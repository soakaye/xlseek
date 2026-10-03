//! # Cell Preview Command Handler (commands/preview_cmd.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Handles cell preview requests (get_cell_preview) from the frontend,
//! extracting surrounding cell data and formulas around the specified cell.
//! Conforms to Constitution Principle I (English comments/errors), Principle II (Constant references),
//! and Principle III (Header comments).

use crate::models::{CellPreviewData, CommandError, ErrorCode};
use crate::search::preview::extract_cell_preview;

/// ## Description
/// Retrieves surrounding cell preview data centered on a specific sheet and cell coordinates in an Excel file.
///
/// ## Arguments
/// - `file_path`: `String` - Path to the Excel file
/// - `sheet_name`: `String` - Target sheet name
/// - `row_index`: `u32` - 1-based target cell row index
/// - `col_index`: `u32` - 1-based target cell column index
///
/// ## Returns
/// - `Result<CellPreviewData, CommandError>`: Preview grid data on success, or CommandError on failure
///
/// ## Errors / Exceptions
/// Returns `Err` if opening file, finding sheet, or asynchronous task fails. Does not panic.
#[tauri::command]
pub async fn get_cell_preview(
    file_path: String,
    sheet_name: String,
    row_index: u32,
    col_index: u32,
) -> Result<CellPreviewData, CommandError> {
    tauri::async_runtime::spawn_blocking(move || {
        extract_cell_preview(&file_path, &sheet_name, row_index, col_index)
    })
    .await
    .map_err(|_| CommandError {
        code: ErrorCode::PreviewFailed,
    })?
    .map_err(|_| CommandError {
        code: ErrorCode::PreviewFailed,
    })
}
