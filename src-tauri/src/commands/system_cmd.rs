use std::path::Path;

#[tauri::command]
pub async fn open_in_excel(file_path: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        open::that(&file_path).map_err(|e| format!("Excelでファイルを開けませんでした: {}", e))
    })
    .await
    .map_err(|e| format!("実行エラー: {}", e))?
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

