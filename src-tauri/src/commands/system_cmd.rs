use crate::models::SupportedApp;
use std::path::Path;

#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;

#[tauri::command]
pub async fn open_in_excel(file_path: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        open::that(&file_path).map_err(|e| format!("Excelでファイルを開けませんでした: {}", e))
    })
    .await
    .map_err(|e| format!("実行エラー: {}", e))?
}

/// 指定拡張子（.xlsx/.xls等）をサポートするインストール済みアプリケーションの一覧を取得する
#[tauri::command]
pub async fn get_supported_apps(extension: Option<String>) -> Result<Vec<SupportedApp>, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let ext = extension.unwrap_or_else(|| ".xlsx".to_string());
        let ext_normalized = if ext.starts_with('.') { ext } else { format!(".{}", ext) };

        #[cfg(target_os = "windows")]
        {
            let mut apps = Vec::new();
            let mut seen_paths = std::collections::HashSet::new();

            // 1. 既定アプリケーションの取得 (UserChoice)
            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            let mut default_progid = String::new();
            let user_choice_path = format!(
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\FileExts\{}\UserChoice",
                ext_normalized
            );
            if let Ok(key) = hkcu.open_subkey(&user_choice_path) {
                if let Ok(prog_id) = key.get_value::<String, _>("ProgId") {
                    default_progid = prog_id;
                }
            }

            // 2. OpenWithProgids から ProgID を収集
            let mut progids = Vec::new();
            if !default_progid.is_empty() {
                progids.push(default_progid.clone());
            }

            let open_with_progids_path = format!(
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\FileExts\{}\OpenWithProgids",
                ext_normalized
            );
            if let Ok(key) = hkcu.open_subkey(&open_with_progids_path) {
                for (name, _) in key.enum_values().flatten() {
                    if !progids.contains(&name) {
                        progids.push(name);
                    }
                }
            }

            let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
            if let Ok(key) = hkcr.open_subkey(format!(r"{}\OpenWithProgids", ext_normalized)) {
                for (name, _) in key.enum_values().flatten() {
                    if !progids.contains(&name) {
                        progids.push(name);
                    }
                }
            }

            // 代表的な標準ProgID / フォールバック候補 (Excel, LibreOffice Calc等)
            for common_progid in &["Excel.Sheet.12", "Excel.Sheet.8", "LibreOffice.CalcDocument.1", "WPS.Workbooks.6"] {
                let s = common_progid.to_string();
                if !progids.contains(&s) {
                    progids.push(s);
                }
            }

            // 3. ProgID から実行ファイルパスと表示名を抽出
            for progid in progids {
                let cmd_path = format!(r"{}\shell\open\command", progid);
                if let Ok(key) = hkcr.open_subkey(&cmd_path) {
                    if let Ok(cmd_str) = key.get_value::<String, _>("") {
                        if let Some((exe_path, app_name)) = parse_command_to_exe(&cmd_str) {
                            if exe_path.exists() {
                                let path_canon = exe_path.to_string_lossy().to_string().to_lowercase();
                                if !seen_paths.contains(&path_canon) {
                                    seen_paths.insert(path_canon);
                                    let is_default = progid == default_progid;
                                    let icon_hint = if app_name.to_lowercase().contains("excel") {
                                        Some("excel".to_string())
                                    } else if app_name.to_lowercase().contains("calc") {
                                        Some("calc".to_string())
                                    } else {
                                        Some("generic".to_string())
                                    };

                                    apps.push(SupportedApp {
                                        id: progid,
                                        name: app_name,
                                        executable_path: exe_path.to_string_lossy().to_string(),
                                        is_default,
                                        icon_hint,
                                    });
                                }
                            }
                        }
                    }
                }
            }

            // 4. OpenWithList (実行ファイル名ベース) の走査
            let open_with_list_path = format!(
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\FileExts\{}\OpenWithList",
                ext_normalized
            );
            if let Ok(key) = hkcu.open_subkey(&open_with_list_path) {
                for (name, val) in key.enum_values().flatten() {
                    if name != "MRUList" {
                        let exe_name = match val {
                            winreg::RegValue { bytes, .. } => {
                                String::from_utf8_lossy(&bytes).trim_matches('\0').trim().to_string()
                            }
                        };
                        if !exe_name.is_empty() && exe_name.to_lowercase().ends_with(".exe") {
                            let app_cmd_path = format!(r"Applications\{}\shell\open\command", exe_name);
                            if let Ok(app_key) = hkcr.open_subkey(&app_cmd_path) {
                                if let Ok(cmd_str) = app_key.get_value::<String, _>("") {
                                    if let Some((exe_path, app_name)) = parse_command_to_exe(&cmd_str) {
                                        if exe_path.exists() {
                                            let path_canon = exe_path.to_string_lossy().to_string().to_lowercase();
                                            if !seen_paths.contains(&path_canon) {
                                                seen_paths.insert(path_canon);
                                                apps.push(SupportedApp {
                                                    id: exe_name.clone(),
                                                    name: app_name,
                                                    executable_path: exe_path.to_string_lossy().to_string(),
                                                    is_default: false,
                                                    icon_hint: Some("generic".to_string()),
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 既定アプリが先頭に来るように並び替え
            apps.sort_by(|a, b| b.is_default.cmp(&a.is_default));
            Ok(apps)
        }

        #[cfg(not(target_os = "windows"))]
        {
            Ok(vec![])
        }
    })
    .await
    .map_err(|e| format!("実行エラー: {}", e))?
}

/// 指定アプリまたは既定アプリでファイルを開く
#[tauri::command]
pub async fn launch_associated_app(
    file_path: String,
    app_path: Option<String>,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let p = Path::new(&file_path);
        if !p.exists() {
            return Err("対象ファイルが存在しません".to_string());
        }

        if let Some(exe) = app_path {
            let exe_path = Path::new(&exe);
            if !exe_path.exists() {
                return Err(format!("指定されたアプリケーションが存在しません: {}", exe));
            }
            std::process::Command::new(exe_path)
                .arg(&file_path)
                .spawn()
                .map_err(|e| format!("アプリケーションの起動に失敗しました: {}", e))?;
            Ok(())
        } else {
            // OS既定のアプリで開く
            open::that(&file_path)
                .map_err(|e| format!("既定のアプリケーションでファイルを開けませんでした: {}", e))
        }
    })
    .await
    .map_err(|e| format!("実行エラー: {}", e))?
}

/// OS標準の「プログラムから開く」ダイアログを表示する
#[tauri::command]
pub async fn show_open_with_dialog(file_path: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let p = Path::new(&file_path);
        if !p.exists() {
            return Err("対象ファイルが存在しません".to_string());
        }

        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("rundll32.exe")
                .args(["shell32.dll,OpenAs_RunDLL", &file_path])
                .spawn()
                .map_err(|e| format!("プログラムから開くダイアログの起動に失敗しました: {}", e))?;
            Ok(())
        }

        #[cfg(not(target_os = "windows"))]
        {
            open::that(&file_path).map_err(|e| format!("ファイルを開けませんでした: {}", e))
        }
    })
    .await
    .map_err(|e| format!("実行エラー: {}", e))?
}

/// コマンドライン文字列から実行ファイルパスとアプリ表示名をパースする補助関数
#[cfg(target_os = "windows")]
fn parse_command_to_exe(cmd_str: &str) -> Option<(std::path::PathBuf, String)> {
    let trimmed = cmd_str.trim();
    let exe_path_str = if trimmed.starts_with('"') {
        let after_first = &trimmed[1..];
        if let Some(end_idx) = after_first.find('"') {
            &after_first[..end_idx]
        } else {
            trimmed
        }
    } else {
        trimmed.split_whitespace().next().unwrap_or(trimmed)
    };

    let path = std::path::PathBuf::from(exe_path_str);
    let stem = path.file_stem()?.to_string_lossy().to_string();
    let display_name = match stem.to_lowercase().as_str() {
        "excel" => "Microsoft Excel".to_string(),
        "soffice" | "scalc" => "LibreOffice Calc".to_string(),
        "et" => "WPS Spreadsheets".to_string(),
        "notepad" => "メモ帳 (Notepad)".to_string(),
        _ => stem,
    };

    Some((path, display_name))
}


#[tauri::command]
pub async fn open_in_folder(file_path: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let path = Path::new(&file_path);
        if !path.exists() {
            return Err("対象ファイルが存在しません".to_string());
        }

        #[cfg(target_os = "windows")]
        {
            // Windowsではエクスプローラーで該当ファイルを選択状態で開く
            let status = std::process::Command::new("explorer")
                .arg(format!("/select,{}", path.display()))
                .status();

            if let Ok(s) = status {
                if s.success() {
                    return Ok(());
                }
            }
        }

        // フォールバック: 親ディレクトリを開く
        if let Some(parent) = path.parent() {
            open::that(parent).map_err(|e| format!("フォルダを開けませんでした: {}", e))
        } else {
            open::that(path).map_err(|e| format!("フォルダを開けませんでした: {}", e))
        }
    })
    .await
    .map_err(|e| format!("実行エラー: {}", e))?
}

/// ドロップされたパスから有効なディレクトリパスを解決する
/// - ディレクトリの場合はそのままのパスを返す
/// - ファイルの場合はその親ディレクトリのパスを返す
#[tauri::command]
pub async fn resolve_dropped_path(path: String) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let p = Path::new(&path);
        if !p.exists() {
            return Err("指定されたパスが存在しません".to_string());
        }

        if p.is_dir() {
            Ok(p.to_string_lossy().to_string())
        } else if let Some(parent) = p.parent() {
            Ok(parent.to_string_lossy().to_string())
        } else {
            Err("ディレクトリパスを特定できませんでした".to_string())
        }
    })
    .await
    .map_err(|e| format!("実行エラー: {}", e))?
}

