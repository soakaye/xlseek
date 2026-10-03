//! # Export Command Handler (commands/export_cmd.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Receives export requests (export_results) from the frontend and exports
//! search match items to a file in the requested format (CSV or Excel).
//! Conforms to Constitution Principle I (English comments/errors), Principle II (Constant references),
//! and Principle III (Header comments).

use crate::export::{export_to_csv, export_to_xlsx};
use crate::models::{CommandError, ErrorCode, ExportFormat, ExportRequest};
use tauri_plugin_i18n::PluginI18nExt;

/// ## Description
/// Exports search result items to the specified file path in the requested format (CSV or Excel).
///
/// ## Arguments
/// - `app_handle`: `tauri::AppHandle` - Application handle to access plugin translation catalogs
/// - `request`: `ExportRequest` - Export format, target path, language, and items list
///
/// ## Returns
/// - `Result<(), CommandError>`: `Ok(())` on success, or CommandError on unsupported language or failure
///
/// ## Errors / Exceptions
/// Returns `Err` on unsupported language, file write failure, or asynchronous task failure. Does not panic.
#[tauri::command]
pub async fn export_results(
    app_handle: tauri::AppHandle,
    request: ExportRequest,
) -> Result<(), CommandError> {
    if request.language != crate::constants::LANGUAGE_JA
        && request.language != crate::constants::LANGUAGE_EN
    {
        return Err(CommandError {
            code: ErrorCode::InternalError,
        });
    }
    let catalogs = app_handle.i18n().get_translations_data();
    tauri::async_runtime::spawn_blocking(move || match request.format {
        ExportFormat::Csv => export_to_csv(
            &request.output_path,
            &request.items,
            &request.language,
            &catalogs,
        ),
        ExportFormat::Xlsx => export_to_xlsx(
            &request.output_path,
            &request.items,
            &request.language,
            &catalogs,
        ),
    })
    .await
    .map_err(|_| CommandError {
        code: ErrorCode::ExportFailed,
    })?
    .map_err(|_| CommandError {
        code: ErrorCode::ExportFailed,
    })
}
