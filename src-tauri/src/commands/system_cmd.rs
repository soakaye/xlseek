//! # System and External Application Command Handler (commands/system_cmd.rs)
//!
//! ## Description
//! Provides system integration including launching files in Excel or associated apps,
//! discovering supported spreadsheet apps (Windows Registry / macOS Bundles),
//! revealing files in Explorer/Finder, and resolving drag-and-drop paths.
//! Conforms to Constitution Principle I (English code/comments), Principle II (Constant references),
//! Principle III (Header comments), and Principle IV (Clippy compliance).

use crate::models::{CommandError, SupportedApp};
use std::path::Path;

#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;

/// ## Description
/// Opens a specified Excel file using the OS default file association.
///
/// ## Arguments
/// - `file_path`: `String` - Target file path to open
///
/// ## Returns
/// - `Result<(), CommandError>`: Ok(()) on success, or CommandError on failure
///
/// ## Errors / Exceptions
/// Returns `Err` if the file does not exist or OS launch fails. Does not panic.
#[tauri::command]
pub async fn open_in_excel(file_path: String) -> Result<(), CommandError> {
    if !Path::new(&file_path).exists() {
        return Err(CommandError {
            code: crate::models::ErrorCode::PathNotFound,
        });
    }
    tauri::async_runtime::spawn_blocking(move || {
        open::that(&file_path).map_err(|e| {
            // Constant reference: crate::constants::ERR_APP_LAUNCH
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

/// ## Description
/// Retrieves the list of installed applications supporting the specified file extension.
///
/// ## Arguments
/// - `extension`: `Option<String>` - Target extension (defaults to `.xlsx` if omitted)
///
/// ## Returns
/// - `Result<Vec<SupportedApp>, CommandError>`: List of supported applications
///
/// ## Errors / Exceptions
/// Returns empty list or error if registry/API calls fail. Does not panic.
#[tauri::command]
pub async fn get_supported_apps(
    extension: Option<String>,
) -> Result<Vec<SupportedApp>, CommandError> {
    tauri::async_runtime::spawn_blocking(move || -> Result<Vec<SupportedApp>, String> {
        // Constant reference: crate::constants::EXT_XLSX
        let ext = extension.unwrap_or_else(|| crate::constants::EXT_XLSX.to_string());
        let ext_normalized = if ext.starts_with('.') {
            ext
        } else {
            format!(".{}", ext)
        };

        #[cfg(target_os = "windows")]
        {
            let mut apps = Vec::new();
            let mut seen_paths = std::collections::HashSet::new();

            // 1. Retrieve default application (UserChoice)
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

            // 2. Collect ProgIDs from OpenWithProgids
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

            // Standard ProgID fallback candidates (Excel, LibreOffice Calc, etc.)
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

            // 3. Extract executable path and display name from ProgID
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
                                    // Constant reference: crate::constants::ICON_HINT_*
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

            // Stable sort prioritizing default app
            // Stable sort via sort_by_key
            apps.sort_by_key(|a| std::cmp::Reverse(a.is_default));
            Ok(apps)
        }

        #[cfg(target_os = "macos")]
        {
            let _ = ext_normalized;
            let mut apps = Vec::new();
            // Check common spreadsheet applications
            // Constant reference: crate::constants::MAC_BUNDLE_*
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

            // Determine default: Excel preferred, then Numbers, else first found app
            if !apps.is_empty() {
                let mut default_idx = 0;
                for (idx, app) in apps.iter().enumerate() {
                    // Constant reference: crate::constants::MAC_BUNDLE_*
                    if app.id == crate::constants::MAC_BUNDLE_EXCEL {
                        default_idx = idx;
                        break;
                    } else if app.id == crate::constants::MAC_BUNDLE_NUMBERS && default_idx == 0 {
                        default_idx = idx;
                    }
                }
                apps[default_idx].is_default = true;
                // Sort using sort_by_key
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

/// ## Description
/// Launches a file with the specified application executable path or OS default app.
///
/// ## Arguments
/// - `file_path`: `String` - File path to launch
/// - `app_path`: `Option<String>` - Executable path of the target app (or OS default if omitted)
///
/// ## Returns
/// - `Result<(), CommandError>`: Ok(()) on success, or CommandError on failure
///
/// ## Errors / Exceptions
/// Returns `Err` if the file or executable does not exist, or process launch fails. Does not panic.
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
            // Constant reference: crate::constants::ERR_FILE_NOT_FOUND
            return Err(format!(
                "{}: {}",
                crate::constants::ERR_FILE_NOT_FOUND,
                file_path
            ));
        }

        if let Some(exe) = app_path {
            let exe_path = Path::new(&exe);
            if !exe_path.exists() {
                // Constant reference: crate::constants::ERR_APP_LAUNCH
                return Err(format!("{}: {}", crate::constants::ERR_APP_LAUNCH, exe));
            }

            #[cfg(target_os = "macos")]
            {
                std::process::Command::new("open")
                    .args(["-a", &exe, &file_path])
                    .spawn()
                    .map_err(|e| {
                        // Constant reference: crate::constants::ERR_APP_LAUNCH
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
                        // Constant reference: crate::constants::ERR_APP_LAUNCH
                        format!("{}: {}", crate::constants::ERR_APP_LAUNCH, e)
                    })?;
                Ok(())
            }
        } else {
            // Open with OS default application
            open::that(&file_path).map_err(|e| {
                // Constant reference: crate::constants::ERR_APP_LAUNCH
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

/// ## Description
/// Displays the OS "Open With" dialog (or file picker) and opens the file with the selected application.
///
/// ## Arguments
/// - `app`: `tauri::AppHandle` - Tauri application handle
/// - `file_path`: `String` - Target file path
///
/// ## Returns
/// - `Result<(), CommandError>`: Ok(()) on success, or CommandError on failure
///
/// ## Errors / Exceptions
/// Returns `Err` if the file does not exist or dialog display fails. Does not panic.
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
        tauri::async_runtime::spawn_blocking(move || -> Result<(), String> {
            std::process::Command::new("rundll32.exe")
                .args(["shell32.dll,OpenAs_RunDLL", &file_path])
                .spawn()
                .map_err(|e| {
                    // Constant reference: crate::constants::ERR_APP_LAUNCH
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
                    // Constant reference: crate::constants::ERR_APP_LAUNCH
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
                // Constant reference: crate::constants::ERR_APP_LAUNCH
                format!("{}: {}", crate::constants::ERR_APP_LAUNCH, e)
            })
            .map_err(Into::into)
    }
}

/// ## Description
/// Extracts executable file path and display name from Windows registry command line strings.
///
/// ## Arguments
/// - `cmd_str`: `&str` - Command string from registry
///
/// ## Returns
/// - `Option<(std::path::PathBuf, String)>`: Tuple of (executable path, display name)
///
/// ## Errors / Exceptions
/// Returns None if unparseable; does not panic.
#[cfg(target_os = "windows")]
fn parse_command_to_exe(cmd_str: &str) -> Option<(std::path::PathBuf, String)> {
    let trimmed = cmd_str.trim();
    let exe_path_str = if let Some(after_first) = trimmed.strip_prefix('"') {
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
    // Constant reference: crate::constants::APP_NAME_*
    let display_name = match stem.to_lowercase().as_str() {
        "excel" => crate::constants::APP_NAME_EXCEL.to_string(),
        "soffice" | "scalc" => crate::constants::APP_NAME_CALC.to_string(),
        "et" => "WPS Spreadsheets".to_string(),
        "notepad" => "Notepad".to_string(),
        _ => stem,
    };

    Some((path, display_name))
}

/// ## Description
/// Opens the directory containing the specified file in the OS file manager (Explorer or Finder)
/// and selects the file.
///
/// ## Arguments
/// - `file_path`: `String` - File path to reveal
///
/// ## Returns
/// - `Result<(), CommandError>`: Ok(()) on success, or CommandError on failure
///
/// ## Errors / Exceptions
/// Returns `Err` if the file does not exist or file manager launch fails. Does not panic.
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
            // Constant reference: crate::constants::ERR_FILE_NOT_FOUND
            return Err(format!(
                "{}: {}",
                crate::constants::ERR_FILE_NOT_FOUND,
                file_path
            ));
        }

        #[cfg(target_os = "windows")]
        {
            // On Windows, open in Explorer with file selected
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
            // On macOS, open in Finder with file selected
            let status = std::process::Command::new("open")
                .args(["-R", &file_path])
                .status();

            if let Ok(s) = status {
                if s.success() {
                    return Ok(());
                }
            }
        }

        // Fallback: Open parent directory
        if let Some(parent) = path.parent() {
            open::that(parent).map_err(|e| {
                // Constant reference: crate::constants::ERR_FOLDER_OPEN
                format!("{}: {}", crate::constants::ERR_FOLDER_OPEN, e)
            })
        } else {
            open::that(path).map_err(|e| {
                // Constant reference: crate::constants::ERR_FOLDER_OPEN
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

/// ## Description
/// Resolves a valid directory path from a drag-and-dropped path string.
/// Returns the path if it is a directory, or its parent directory if it is a file.
///
/// ## Arguments
/// - `path`: `String` - Dropped path string
///
/// ## Returns
/// - `Result<String, CommandError>`: Resolved directory path or CommandError
///
/// ## Errors / Exceptions
/// Returns `Err` if the path does not exist or parent directory cannot be resolved. Does not panic.
#[tauri::command]
pub async fn resolve_dropped_path(path: String) -> Result<String, CommandError> {
    tauri::async_runtime::spawn_blocking(move || {
        let p = Path::new(&path);
        if !p.exists() {
            // Constant reference: crate::constants::ERR_FILE_NOT_FOUND
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
            // Constant reference: crate::constants::ERR_PATH_NOT_DIR
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
