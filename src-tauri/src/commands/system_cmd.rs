//! # System and External Application Command Handler (commands/system_cmd.rs)
//!
//! Copyright (c) 2026 soakaye
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
            // Constant reference: crate::constants::CMD_EXPLORER
            // Constant reference: crate::constants::ARG_EXPLORER_SELECT_PREFIX
            let select_arg = format!(
                "{}{}",
                crate::constants::ARG_EXPLORER_SELECT_PREFIX,
                path.display()
            );
            let status = std::process::Command::new(crate::constants::CMD_EXPLORER)
                .arg(&select_arg)
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

        #[cfg(target_os = "linux")]
        {
            if let Ok(()) = open_file_in_folder_linux(path) {
                return Ok(());
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
/// Encodes an absolute local path into a percent-encoded `file://` URI according to RFC 3986.
/// Non-alphanumeric ASCII characters (except `-`, `_`, `.`, `~`, and `/`) and multibyte UTF-8 bytes
/// are percent-encoded.
///
/// ## Arguments
/// - `path`: `&Path` - Local filesystem path to convert
///
/// ## Returns
/// - `String`: Formatted `file://` URI string
///
/// ## Errors / Exceptions
/// Does not panic or fail.
pub fn path_to_file_uri(path: &Path) -> String {
    // Constant reference: crate::constants::FILE_URI_PREFIX
    let mut uri = String::from(crate::constants::FILE_URI_PREFIX);
    let path_str = path.to_string_lossy();

    for byte in path_str.as_bytes() {
        match *byte {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' | b'/' => {
                uri.push(*byte as char);
            }
            b => {
                use std::fmt::Write;
                let _ = write!(uri, "%{:02X}", b);
            }
        }
    }

    uri
}

/// ## Description
/// Builds the argument list for invoking `dbus-send` to request the file manager to reveal
/// and highlight a target file item via `org.freedesktop.FileManager1.ShowItems`.
///
/// ## Arguments
/// - `file_uri`: `&str` - The `file://` formatted URI of the target file
///
/// ## Returns
/// - `Vec<String>`: List of argument strings passed to `dbus-send`
///
/// ## Errors / Exceptions
/// Does not panic or fail.
pub fn build_dbus_show_items_args(file_uri: &str) -> Vec<String> {
    // Constant reference: crate::constants::DBUS_ARG_PRINT_REPLY
    // Constant reference: crate::constants::DBUS_ARG_REPLY_TIMEOUT
    // Constant reference: crate::constants::DBUS_ARG_SESSION
    // Constant reference: crate::constants::DBUS_DEST_FILEMANAGER
    // Constant reference: crate::constants::DBUS_TYPE_METHOD_CALL
    // Constant reference: crate::constants::DBUS_PATH_FILEMANAGER
    // Constant reference: crate::constants::DBUS_METHOD_SHOW_ITEMS
    // Constant reference: crate::constants::DBUS_PARAM_ARRAY_STRING_PREFIX
    // Constant reference: crate::constants::DBUS_PARAM_STARTUP_ID_EMPTY
    vec![
        crate::constants::DBUS_ARG_PRINT_REPLY.to_string(),
        crate::constants::DBUS_ARG_REPLY_TIMEOUT.to_string(),
        crate::constants::DBUS_ARG_SESSION.to_string(),
        crate::constants::DBUS_DEST_FILEMANAGER.to_string(),
        crate::constants::DBUS_TYPE_METHOD_CALL.to_string(),
        crate::constants::DBUS_PATH_FILEMANAGER.to_string(),
        crate::constants::DBUS_METHOD_SHOW_ITEMS.to_string(),
        format!(
            "{}{}",
            crate::constants::DBUS_PARAM_ARRAY_STRING_PREFIX,
            file_uri
        ),
        crate::constants::DBUS_PARAM_STARTUP_ID_EMPTY.to_string(),
    ]
}

/// ## Description
/// Converts a Linux path to a Windows UNC or drive letter path using the `wslpath` utility
/// if running in a WSL (Windows Subsystem for Linux) environment.
///
/// ## Arguments
/// - `path`: `&Path` - Target path in the Linux filesystem
///
/// ## Returns
/// - `Option<String>`: Converted Windows path string, or None if conversion fails or wslpath is unavailable
///
/// ## Errors / Exceptions
/// Returns None if `wslpath` execution fails or returns non-zero status. Does not panic.
#[cfg(target_os = "linux")]
pub fn convert_wsl_path_to_windows(path: &Path) -> Option<String> {
    // Constant reference: crate::constants::CMD_WSLPATH
    // Constant reference: crate::constants::WSLPATH_ARG_WINDOWS
    let output = std::process::Command::new(crate::constants::CMD_WSLPATH)
        .arg(crate::constants::WSLPATH_ARG_WINDOWS)
        .arg(path)
        .output()
        .ok()?;

    if output.status.success() {
        let win_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !win_path.is_empty() {
            return Some(win_path);
        }
    }
    None
}

/// ## Description
/// Attempts to open and highlight the target file or containing directory using Windows Explorer
/// in a WSL environment via `explorer.exe`.
///
/// ## Arguments
/// - `path`: `&Path` - Target file or directory path
///
/// ## Returns
/// - `Result<(), String>`: Ok(()) on success, or Err on failure
///
/// ## Errors / Exceptions
/// Returns `Err` if `wslpath` conversion fails or `explorer.exe` cannot be spawned. Does not panic.
#[cfg(target_os = "linux")]
fn try_open_in_wsl_explorer(path: &Path) -> Result<(), String> {
    let win_path = convert_wsl_path_to_windows(path).ok_or_else(|| {
        // Constant reference: crate::constants::ERR_FOLDER_OPEN
        crate::constants::ERR_FOLDER_OPEN.to_string()
    })?;

    // Primary: Highlight file with /select,
    // Constant reference: crate::constants::CMD_WSL_EXPLORER
    // Constant reference: crate::constants::ARG_EXPLORER_SELECT_PREFIX
    let select_arg = format!(
        "{}{}",
        crate::constants::ARG_EXPLORER_SELECT_PREFIX,
        win_path
    );

    let spawn_res = std::process::Command::new(crate::constants::CMD_WSL_EXPLORER)
        .arg(&select_arg)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();

    if spawn_res.is_ok() {
        return Ok(());
    }

    // Fallback: Open directory without file selection
    let parent_path = path.parent().unwrap_or(path);
    if let Some(win_parent) = convert_wsl_path_to_windows(parent_path) {
        let dir_spawn = std::process::Command::new(crate::constants::CMD_WSL_EXPLORER)
            .arg(&win_parent)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn();

        if dir_spawn.is_ok() {
            return Ok(());
        }
    }

    // Constant reference: crate::constants::ERR_FOLDER_OPEN
    Err(crate::constants::ERR_FOLDER_OPEN.to_string())
}

/// ## Description
/// Attempts to open the containing folder of a file on Linux, prioritizing file highlighting
/// via the DBus `org.freedesktop.FileManager1.ShowItems` interface, dedicated file manager
/// executables with selection flags, Windows Explorer in WSL environments, and falling back
/// sequentially to `gio open` and `xdg-open` on the parent folder.
///
/// ## Arguments
/// - `path`: `&Path` - Target file path
///
/// ## Returns
/// - `Result<(), String>`: Ok(()) on success, or Err on failure
///
/// ## Errors / Exceptions
/// Returns `Err` if all launch attempts fail. Does not panic.
#[cfg(target_os = "linux")]
fn open_file_in_folder_linux(path: &Path) -> Result<(), String> {
    if !path.exists() {
        // Constant reference: crate::constants::ERR_FILE_NOT_FOUND
        return Err(format!(
            "{}: {}",
            crate::constants::ERR_FILE_NOT_FOUND,
            path.display()
        ));
    }

    // 1. Primary Attempt: Highlight file using DBus ShowItems
    let file_uri = path_to_file_uri(path);
    let dbus_args = build_dbus_show_items_args(&file_uri);

    // Constant reference: crate::constants::CMD_DBUS_SEND
    let dbus_status = std::process::Command::new(crate::constants::CMD_DBUS_SEND)
        .args(&dbus_args)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();

    if let Ok(s) = dbus_status {
        if s.success() {
            return Ok(());
        }
    }

    // 2. Secondary Attempt: Direct file manager execution with item selection
    // Constant reference: crate::constants::CMD_NAUTILUS
    // Constant reference: crate::constants::CMD_DOLPHIN
    // Constant reference: crate::constants::CMD_NEMO
    // Constant reference: crate::constants::ARG_SELECT
    // Constant reference: crate::constants::ARG_NO_DESKTOP
    let direct_managers = [
        (crate::constants::CMD_NAUTILUS, crate::constants::ARG_SELECT),
        (crate::constants::CMD_DOLPHIN, crate::constants::ARG_SELECT),
        (crate::constants::CMD_NEMO, crate::constants::ARG_NO_DESKTOP),
    ];

    for (cmd, arg) in direct_managers {
        let direct_status = std::process::Command::new(cmd)
            .arg(arg)
            .arg(path)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();

        if let Ok(s) = direct_status {
            if s.success() {
                return Ok(());
            }
        }
    }

    // 3. Tertiary Attempt: WSL2 Windows Explorer integration
    if let Ok(()) = try_open_in_wsl_explorer(path) {
        return Ok(());
    }

    // 4. Quaternary Fallback: Open parent directory with gio open
    let target_dir = path.parent().unwrap_or(path);

    // Constant reference: crate::constants::CMD_GIO
    // Constant reference: crate::constants::GIO_SUBCOMMAND_OPEN
    let gio_status = std::process::Command::new(crate::constants::CMD_GIO)
        .arg(crate::constants::GIO_SUBCOMMAND_OPEN)
        .arg(target_dir)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();

    if let Ok(s) = gio_status {
        if s.success() {
            return Ok(());
        }
    }

    // 5. Quinary Fallback: Open parent directory with xdg-open
    // Constant reference: crate::constants::CMD_XDG_OPEN
    let xdg_status = std::process::Command::new(crate::constants::CMD_XDG_OPEN)
        .arg(target_dir)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();

    if let Ok(s) = xdg_status {
        if s.success() {
            return Ok(());
        }
    }

    // Constant reference: crate::constants::ERR_FOLDER_OPEN
    Err(crate::constants::ERR_FOLDER_OPEN.to_string())
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// ## Description
    /// Verifies that `path_to_file_uri` converts ASCII and non-ASCII paths (including spaces and Japanese characters)
    /// into RFC 3986 percent-encoded `file://` URIs correctly.
    ///
    /// ## Arguments / Returns
    /// None
    ///
    /// ## Errors / Exceptions
    /// Panics if assertion fails.
    #[test]
    fn test_path_to_file_uri() {
        let ascii_path = PathBuf::from("/home/user/documents/report.xlsx");
        assert_eq!(
            path_to_file_uri(&ascii_path),
            "file:///home/user/documents/report.xlsx"
        );

        let space_path = PathBuf::from("/home/user/my documents/report 2026.xlsx");
        assert_eq!(
            path_to_file_uri(&space_path),
            "file:///home/user/my%20documents/report%202026.xlsx"
        );

        let non_ascii_path = PathBuf::from("/tmp/資料/売上.xlsx");
        let uri = path_to_file_uri(&non_ascii_path);
        // Constant reference: crate::constants::FILE_URI_PREFIX
        assert!(uri.starts_with(crate::constants::FILE_URI_PREFIX));
        assert!(uri.contains("%E8%B3%87%E6%96%99"));
        assert!(uri.contains("%E5%A3%B2%E4%B8%8A"));
    }

    /// ## Description
    /// Verifies that `open_in_folder` immediately returns `ErrorCode::PathNotFound`
    /// when the requested file does not exist on the filesystem.
    ///
    /// ## Arguments / Returns
    /// None
    ///
    /// ## Errors / Exceptions
    /// Panics if assertion fails.
    #[test]
    fn test_open_in_folder_nonexistent_file() {
        let non_existent = "/non/existent/path/for/unit/test/12345.xlsx".to_string();
        tauri::async_runtime::block_on(async {
            let result = open_in_folder(non_existent).await;
            assert!(result.is_err());
            let err = result.unwrap_err();
            assert_eq!(err.code, crate::models::ErrorCode::PathNotFound);
        });
    }

    /// ## Description
    /// Verifies that `open_file_in_folder_linux` returns an Err when given a path
    /// whose parent does not exist or fails to open with system openers.
    ///
    /// ## Arguments / Returns
    /// None
    ///
    /// ## Errors / Exceptions
    /// Panics if assertion fails.
    #[cfg(target_os = "linux")]
    #[test]
    fn test_open_file_in_folder_linux_nonexistent() {
        let invalid_path = PathBuf::from("/non/existent/dir/12345/dummy.xlsx");
        let result = open_file_in_folder_linux(&invalid_path);
        assert!(result.is_err());
    }

    /// ## Description
    /// Verifies that `build_dbus_show_items_args` correctly includes `--print-reply`,
    /// `--reply-timeout=2000`, the session bus flag, destination, method, and URI array.
    ///
    /// ## Arguments / Returns
    /// None
    ///
    /// ## Errors / Exceptions
    /// Panics if assertion fails.
    #[test]
    fn test_build_dbus_show_items_args() {
        let uri = "file:///home/user/test.xlsx";
        let args = build_dbus_show_items_args(uri);

        // Constant reference: crate::constants::DBUS_ARG_PRINT_REPLY
        assert!(args.contains(&crate::constants::DBUS_ARG_PRINT_REPLY.to_string()));
        // Constant reference: crate::constants::DBUS_ARG_REPLY_TIMEOUT
        assert!(args.contains(&crate::constants::DBUS_ARG_REPLY_TIMEOUT.to_string()));
        // Constant reference: crate::constants::DBUS_ARG_SESSION
        assert!(args.contains(&crate::constants::DBUS_ARG_SESSION.to_string()));
        // Constant reference: crate::constants::DBUS_DEST_FILEMANAGER
        assert!(args.contains(&crate::constants::DBUS_DEST_FILEMANAGER.to_string()));
        // Constant reference: crate::constants::DBUS_METHOD_SHOW_ITEMS
        assert!(args.contains(&crate::constants::DBUS_METHOD_SHOW_ITEMS.to_string()));

        // Constant reference: crate::constants::DBUS_PARAM_ARRAY_STRING_PREFIX
        let expected_param = format!(
            "{}{}",
            crate::constants::DBUS_PARAM_ARRAY_STRING_PREFIX,
            uri
        );
        assert!(args.contains(&expected_param));
    }

    /// ## Description
    /// Verifies that `convert_wsl_path_to_windows` successfully converts a Linux path
    /// to a Windows format string when `wslpath` is available in the environment.
    ///
    /// ## Arguments / Returns
    /// None
    ///
    /// ## Errors / Exceptions
    /// Panics if assertion fails.
    #[cfg(target_os = "linux")]
    #[test]
    fn test_convert_wsl_path_to_windows() {
        let temp_dir = std::env::temp_dir();
        if let Some(converted) = convert_wsl_path_to_windows(&temp_dir) {
            assert!(!converted.is_empty());
        }
    }

    /// ## Description
    /// Verifies that `open_file_in_folder_linux` successfully handles an existing file
    /// via available desktop openers (e.g., WSL Explorer or Linux file managers), or
    /// returns Err(ERR_FOLDER_OPEN) if no opener is available.
    ///
    /// ## Arguments / Returns
    /// None
    ///
    /// ## Errors / Exceptions
    /// Panics if assertion fails.
    #[cfg(target_os = "linux")]
    #[test]
    fn test_open_file_in_folder_linux_existing() {
        let manifest_dir =
            std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| "src-tauri".to_string());
        let cargo_toml = PathBuf::from(manifest_dir).join("Cargo.toml");
        let result = open_file_in_folder_linux(&cargo_toml);
        // In WSL or desktop environment with openers, result is Ok(()).
        // In headless environment without openers, result is Err(ERR_FOLDER_OPEN).
        if let Err(err_msg) = result {
            // Constant reference: crate::constants::ERR_FOLDER_OPEN
            assert_eq!(err_msg, crate::constants::ERR_FOLDER_OPEN);
        }
    }
}
