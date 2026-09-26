//! # システム・外部アプリケーション連携コマンドハンドラ (commands/system_cmd.rs)
//!
//! ## 処理内容
//! OSとの各種連携処理を提供する。Excelや既定アプリケーションでのファイル起動、
//! サポートされているスプレッドシートアプリ一覧の取得（Windowsレジストリ / macOS Bundle）、
//! Explorer / Finder でのファイル所在フォルダ表示、およびドラッグ＆ドロップパス解決を行う。
//! 憲章原則I（日本語エラー）、原則II（定数参照）、原則III（ヘッダコメント）、原則IV（Clippy完全準拠）に準拠。
//!
//! ## 変更履歴
//! - v1.0.0 (2026-09-26, AI Agent): 初版策定。Clippy指摘修正（sort_by_key）、定数参照化、4要素ヘッダコメント付与。

use crate::models::{CommandError, SupportedApp};
use std::path::Path;

#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;

/// ## 処理内容
/// 指定されたExcelファイルをOSの関連付けに従って起動する。
///
/// ## 引数
/// - `file_path`: `String` - 開く対象のファイルパス
///
/// ## 戻り値
/// - `Result<(), String>`: 起動成功時は `Ok(())`、失敗時は日本語エラーメッセージ
///
/// ## エラー / 例外発生条件
/// ファイルが存在しない場合やOSによる起動に失敗した際に `Err` を返却する。panicは発生しない。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化。
#[tauri::command]
pub async fn open_in_excel(file_path: String) -> Result<(), CommandError> {
    if !Path::new(&file_path).exists() {
        return Err(CommandError {
            code: crate::models::ErrorCode::PathNotFound,
        });
    }
    tauri::async_runtime::spawn_blocking(move || {
        open::that(&file_path).map_err(|e| {
            // 定数参照: crate::constants::ERR_APP_LAUNCH を使用
            format!("{}: {}", crate::constants::ERR_APP_LAUNCH, e)
        })
    })
    .await
    .map_err(|_| CommandError {
        code: crate::models::ErrorCode::AppLaunchFailed,
    })?
    .map_err(|_| CommandError {
        code: crate::models::ErrorCode::AppLaunchFailed,
    })
}

/// ## 処理内容
/// 指定拡張子（.xlsx/.xls等）をサポートするOS上のインストール済みアプリケーション一覧を取得する。
///
/// ## 引数
/// - `extension`: `Option<String>` - 対象の拡張子（省略時はデフォルトで .xlsx）
///
/// ## 戻り値
/// - `Result<Vec<SupportedApp>, String>`: アプリケーション情報一覧
///
/// ## エラー / 例外発生条件
/// レジストリ探索またはOS API呼び出し失敗時は空リストまたはエラーを返却する。panicは発生しない。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。Clippy指摘修正（sort_by_key）、定数参照化。
#[tauri::command]
pub async fn get_supported_apps(
    extension: Option<String>,
) -> Result<Vec<SupportedApp>, CommandError> {
    tauri::async_runtime::spawn_blocking(move || -> Result<Vec<SupportedApp>, String> {
        // 定数参照: crate::constants::EXT_XLSX を使用
        let ext = extension.unwrap_or_else(|| crate::constants::EXT_XLSX.to_string());
        let _ext_normalized = if ext.starts_with('.') {
            ext
        } else {
            format!(".{}", ext)
        };

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
            for common_progid in &[
                "Excel.Sheet.12",
                "Excel.Sheet.8",
                "LibreOffice.CalcDocument.1",
                "WPS.Workbooks.6",
            ] {
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
                                let path_canon =
                                    exe_path.to_string_lossy().to_string().to_lowercase();
                                if !seen_paths.contains(&path_canon) {
                                    seen_paths.insert(path_canon);
                                    let is_default = progid == default_progid;
                                    // 定数参照: crate::constants::ICON_HINT_* を使用
                                    let icon_hint = if app_name.to_lowercase().contains("excel") {
                                        Some(crate::constants::ICON_HINT_EXCEL.to_string())
                                    } else if app_name.to_lowercase().contains("calc") {
                                        Some(crate::constants::ICON_HINT_CALC.to_string())
                                    } else {
                                        Some(crate::constants::ICON_HINT_GENERIC.to_string())
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

            // 既定アプリの優先ソート (Clippy: unnecessary_sort_by 解消のため sort_by_key 使用)
            // 定数参照: sort_by_key による安定ソート
            apps.sort_by_key(|a| std::cmp::Reverse(a.is_default));
            Ok(apps)
        }

        #[cfg(target_os = "macos")]
        {
            let mut apps = Vec::new();
            // 代表的なスプレッドシートアプリのチェック
            // 定数参照: crate::constants::MAC_BUNDLE_* を使用
            let known_apps = [
                (
                    crate::constants::MAC_BUNDLE_EXCEL,
                    crate::constants::APP_NAME_EXCEL,
                    "/Applications/Microsoft Excel.app",
                    crate::constants::ICON_HINT_EXCEL,
                ),
                (
                    crate::constants::MAC_BUNDLE_NUMBERS,
                    crate::constants::APP_NAME_NUMBERS,
                    "/Applications/Numbers.app",
                    crate::constants::ICON_HINT_NUMBERS,
                ),
                (
                    "org.libreoffice.script",
                    crate::constants::APP_NAME_CALC,
                    "/Applications/LibreOffice.app",
                    crate::constants::ICON_HINT_CALC,
                ),
            ];

            for (id, name, path_str, icon) in &known_apps {
                let p = Path::new(path_str);
                if p.exists() {
                    apps.push(SupportedApp {
                        id: id.to_string(),
                        name: name.to_string(),
                        executable_path: path_str.to_string(),
                        is_default: false,
                        icon_hint: Some(icon.to_string()),
                    });
                }
            }

            // 既定アプリの優先判定: Excel があれば最優先、無ければ Numbers を既定、いずれもなければ最初の検出アプリ
            if !apps.is_empty() {
                let mut default_idx = 0;
                for (idx, app) in apps.iter().enumerate() {
                    // 定数参照: crate::constants::MAC_BUNDLE_* を使用
                    if app.id == crate::constants::MAC_BUNDLE_EXCEL {
                        default_idx = idx;
                        break;
                    } else if app.id == crate::constants::MAC_BUNDLE_NUMBERS && default_idx == 0 {
                        default_idx = idx;
                    }
                }
                apps[default_idx].is_default = true;
                // Clippy 指摘修正 (T032): sort_by を sort_by_key に置換
                apps.sort_by_key(|a| std::cmp::Reverse(a.is_default));
            }

            Ok(apps)
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            let _ = ext_normalized;
            Ok(vec![])
        }
    })
    .await
    .map_err(|_| CommandError {
        code: crate::models::ErrorCode::InternalError,
    })?
    .map_err(|_| CommandError {
        code: crate::models::ErrorCode::InternalError,
    })
}

/// ## 処理内容
/// 指定された実行パスのアプリケーション、またはOSの既定アプリケーションでファイルを起動する。
///
/// ## 引数
/// - `file_path`: `String` - 起動対象のファイルパス
/// - `app_path`: `Option<String>` - 起動するアプリケーションのパス（省略時はOS既定アプリ）
///
/// ## 戻り値
/// - `Result<(), String>`: 起動成功時は `Ok(())`、失敗時は日本語エラーメッセージ
///
/// ## エラー / 例外発生条件
/// ファイルが存在しない場合やプロセスの起動に失敗した場合に `Err` を返却する。panicは発生しない。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化。
#[tauri::command]
pub async fn launch_associated_app(
    file_path: String,
    app_path: Option<String>,
) -> Result<(), CommandError> {
    if !Path::new(&file_path).exists() {
        return Err(CommandError {
            code: crate::models::ErrorCode::PathNotFound,
        });
    }
    tauri::async_runtime::spawn_blocking(move || {
        let p = Path::new(&file_path);
        if !p.exists() {
            // 定数参照: crate::constants::ERR_FILE_NOT_FOUND を使用
            return Err(format!(
                "{}: {}",
                crate::constants::ERR_FILE_NOT_FOUND,
                file_path
            ));
        }

        if let Some(exe) = app_path {
            let exe_path = Path::new(&exe);
            if !exe_path.exists() {
                // 定数参照: crate::constants::ERR_APP_LAUNCH を使用
                return Err(format!("{}: {}", crate::constants::ERR_APP_LAUNCH, exe));
            }

            #[cfg(target_os = "macos")]
            {
                std::process::Command::new("open")
                    .args(["-a", &exe, &file_path])
                    .spawn()
                    .map_err(|e| {
                        // 定数参照: crate::constants::ERR_APP_LAUNCH を使用
                        format!("{}: {}", crate::constants::ERR_APP_LAUNCH, e)
                    })?;
                Ok(())
            }

            #[cfg(not(target_os = "macos"))]
            {
                std::process::Command::new(exe_path)
                    .arg(&file_path)
                    .spawn()
                    .map_err(|e| {
                        // 定数参照: crate::constants::ERR_APP_LAUNCH を使用
                        format!("{}: {}", crate::constants::ERR_APP_LAUNCH, e)
                    })?;
                Ok(())
            }
        } else {
            // OS既定のアプリで開く
            open::that(&file_path).map_err(|e| {
                // 定数参照: crate::constants::ERR_APP_LAUNCH を使用
                format!("{}: {}", crate::constants::ERR_APP_LAUNCH, e)
            })
        }
    })
    .await
    .map_err(|_| CommandError {
        code: crate::models::ErrorCode::InternalError,
    })?
    .map_err(|_| CommandError {
        code: crate::models::ErrorCode::AppLaunchFailed,
    })
}

/// ## 処理内容
/// OS標準の「プログラムから開く」ダイアログ（またはファイル選択ダイアログ）を表示し、
/// 選択されたアプリケーションでファイルを開く。
///
/// ## 引数
/// - `app`: `tauri::AppHandle` - Tauriアプリケーションハンドル
/// - `file_path`: `String` - 開く対象のファイルパス
///
/// ## 戻り値
/// - `Result<(), String>`: 成功時は `Ok(())`、失敗時は日本語エラーメッセージ
///
/// ## エラー / 例外発生条件
/// ファイル不在時やダイアログ表示エラー時に `Err` を返却する。panicは発生しない。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化。
#[tauri::command]
pub async fn show_open_with_dialog(
    app: tauri::AppHandle,
    file_path: String,
) -> Result<(), CommandError> {
    let p = Path::new(&file_path);
    if !p.exists() {
        return Err(CommandError {
            code: crate::models::ErrorCode::PathNotFound,
        });
    }

    #[cfg(target_os = "windows")]
    {
        let _ = app;
        tauri::async_runtime::spawn_blocking(move || {
            std::process::Command::new("rundll32.exe")
                .args(["shell32.dll,OpenAs_RunDLL", &file_path])
                .spawn()
                .map_err(|e| {
                    // 定数参照: crate::constants::ERR_APP_LAUNCH を使用
                    format!("{}: {}", crate::constants::ERR_APP_LAUNCH, e)
                })?;
            Ok(())
        })
        .await
        .map_err(|_| CommandError {
            code: crate::models::ErrorCode::InternalError,
        })?
        .map_err(|_| CommandError {
            code: crate::models::ErrorCode::AppLaunchFailed,
        })
    }

    #[cfg(target_os = "macos")]
    {
        use tauri_plugin_dialog::DialogExt;
        let chosen = app
            .dialog()
            .file()
            .set_directory("/Applications")
            .blocking_pick_file();

        if let Some(app_path) = chosen {
            let app_str = app_path.to_string();
            std::process::Command::new("open")
                .args(["-a", &app_str, &file_path])
                .spawn()
                .map_err(|e| {
                    // 定数参照: crate::constants::ERR_APP_LAUNCH を使用
                    format!("{}: {}", crate::constants::ERR_APP_LAUNCH, e)
                })?;
        }
        Ok(())
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        let _ = app;
        open::that(&file_path)
            .map_err(|e| {
                // 定数参照: crate::constants::ERR_APP_LAUNCH を使用
                format!("{}: {}", crate::constants::ERR_APP_LAUNCH, e)
            })
            .map_err(Into::into)
    }
}

/// ## 処理内容
/// Windowsレジストリのコマンドライン文字列から実行ファイルパスと表示名を抽出する。
///
/// ## 引数
/// - `cmd_str`: `&str` - レジストリから取得したコマンド文字列
///
/// ## 戻り値
/// - `Option<(std::path::PathBuf, String)>`: (実行ファイルパス, アプリ表示名)
///
/// ## エラー / 例外発生条件
/// パース不可時はNoneを返却する。panicは発生しない。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化。
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
    // 定数参照: crate::constants::APP_NAME_* を使用
    let display_name = match stem.to_lowercase().as_str() {
        "excel" => crate::constants::APP_NAME_EXCEL.to_string(),
        "soffice" | "scalc" => crate::constants::APP_NAME_CALC.to_string(),
        "et" => "WPS Spreadsheets".to_string(),
        "notepad" => "メモ帳 (Notepad)".to_string(),
        _ => stem,
    };

    Some((path, display_name))
}

/// ## 処理内容
/// 指定されたファイルが存在するディレクトリをOSのファイルマネージャー（ExplorerまたはFinder）で開き、
/// 該当ファイルを選択状態にする。
///
/// ## 引数
/// - `file_path`: `String` - 選択状態にする対象ファイルのパス
///
/// ## 戻り値
/// - `Result<(), String>`: 成功時は `Ok(())`、失敗時は日本語エラーメッセージ
///
/// ## エラー / 例外発生条件
/// ファイル不在時やファイルマネージャー起動失敗時に `Err` を返却する。panicは発生しない。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化。
#[tauri::command]
pub async fn open_in_folder(file_path: String) -> Result<(), CommandError> {
    if !Path::new(&file_path).exists() {
        return Err(CommandError {
            code: crate::models::ErrorCode::PathNotFound,
        });
    }
    tauri::async_runtime::spawn_blocking(move || {
        let path = Path::new(&file_path);
        if !path.exists() {
            // 定数参照: crate::constants::ERR_FILE_NOT_FOUND を使用
            return Err(format!(
                "{}: {}",
                crate::constants::ERR_FILE_NOT_FOUND,
                file_path
            ));
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

        #[cfg(target_os = "macos")]
        {
            // macOSではFinderで該当ファイルを選択状態で開く
            let status = std::process::Command::new("open")
                .args(["-R", &file_path])
                .status();

            if let Ok(s) = status {
                if s.success() {
                    return Ok(());
                }
            }
        }

        // フォールバック: 親ディレクトリを開く
        if let Some(parent) = path.parent() {
            open::that(parent).map_err(|e| {
                // 定数参照: crate::constants::ERR_FOLDER_OPEN を使用
                format!("{}: {}", crate::constants::ERR_FOLDER_OPEN, e)
            })
        } else {
            open::that(path).map_err(|e| {
                // 定数参照: crate::constants::ERR_FOLDER_OPEN を使用
                format!("{}: {}", crate::constants::ERR_FOLDER_OPEN, e)
            })
        }
    })
    .await
    .map_err(|_| CommandError {
        code: crate::models::ErrorCode::InternalError,
    })?
    .map_err(|_| CommandError {
        code: crate::models::ErrorCode::FolderOpenFailed,
    })
}

/// ## 処理内容
/// ドラッグ＆ドロップされたパス文字列から、有効なディレクトリパスを解決する。
/// パスがディレクトリの場合はそのまま返し、ファイルの場合は親ディレクトリのパスを返す。
///
/// ## 引数
/// - `path`: `String` - ドロップされたパス文字列
///
/// ## 戻り値
/// - `Result<String, String>`: 解決されたディレクトリパス、または日本語エラーメッセージ
///
/// ## エラー / 例外発生条件
/// パスが存在しない場合や親ディレクトリが特定できない場合に `Err` を返却する。panicは発生しない。
///
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化。
#[tauri::command]
pub async fn resolve_dropped_path(path: String) -> Result<String, CommandError> {
    tauri::async_runtime::spawn_blocking(move || {
        let p = Path::new(&path);
        if !p.exists() {
            // 定数参照: crate::constants::ERR_FILE_NOT_FOUND を使用
            return Err(format!(
                "{}: {}",
                crate::constants::ERR_FILE_NOT_FOUND,
                path
            ));
        }

        if p.is_dir() {
            Ok(p.to_string_lossy().to_string())
        } else if let Some(parent) = p.parent() {
            Ok(parent.to_string_lossy().to_string())
        } else {
            // 定数参照: crate::constants::ERR_PATH_NOT_DIR を使用
            Err(crate::constants::ERR_PATH_NOT_DIR.to_string())
        }
    })
    .await
    .map_err(|_| CommandError {
        code: crate::models::ErrorCode::InternalError,
    })?
    .map_err(|_| CommandError {
        code: crate::models::ErrorCode::PathNotFound,
    })
}
