//! # Cell Preview Command Handler (commands/preview_cmd.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Handles cell preview requests (get_cell_preview) from the frontend,
//! extracting surrounding cell data and formulas around the specified cell.
//! Conforms to Constitution Principle I (English comments/errors), Principle II (Constant references),
//! and Principle III (Header comments).

use crate::models::{
    CellPreviewData, CommandError, ErrorCode, LlmCopyOptions, SearchMatch, WordPreviewPayload,
};
use crate::search::llm_context::extract_llm_context;
use crate::search::preview::extract_cell_preview;
use crate::search::word::extract_word_document;

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

/// ## Description
/// Retrieves ordered document paragraphs from a Word document (.docx, .docm, or .doc) for text preview.
///
/// ## Arguments
/// - `file_path`: `String` - Path to the Word document on disk
/// - `target_paragraph`: `u32` - 1-based paragraph sequence index matching the search hit
///
/// ## Returns
/// - `Result<WordPreviewPayload, CommandError>`: Word document preview payload on success, or CommandError on failure
///
/// ## Errors / Exceptions
/// Returns `Err` if the file cannot be opened, is encrypted, or is corrupted. Does not panic.
#[tauri::command]
pub async fn get_word_preview(
    file_path: String,
    target_paragraph: u32,
) -> Result<WordPreviewPayload, CommandError> {
    // Constant reference: crate::constants::CMD_GET_WORD_PREVIEW
    tauri::async_runtime::spawn_blocking(move || {
        let path = std::path::Path::new(&file_path);
        extract_word_document(path, target_paragraph)
    })
    .await
    .map_err(|_| CommandError {
        code: ErrorCode::PreviewFailed,
    })?
    .map_err(|_| CommandError {
        code: ErrorCode::PreviewFailed,
    })
}

/// ## Description
/// Generates context-aware Markdown prompt content from search matches for copying into LLMs.
///
/// ## Arguments
/// - matches: Vec<SearchMatch> - Selected search match results
/// - options: LlmCopyOptions - Options including column window size, copy format, and Word mode
///
/// ## Returns
/// - Result<String, CommandError>: Structured Markdown string on success, or CommandError on failure
///
/// ## Errors / Exceptions
/// Returns Err(CommandError) if extraction fails or background task panics. Does not panic.
#[tauri::command]
pub async fn generate_llm_context(
    matches: Vec<SearchMatch>,
    options: LlmCopyOptions,
) -> Result<String, CommandError> {
    // Constant reference: crate::constants::CMD_GENERATE_LLM_CONTEXT
    tauri::async_runtime::spawn_blocking(move || extract_llm_context(&matches, &options))
        .await
        .map_err(|_| CommandError {
            code: ErrorCode::PreviewFailed,
        })?
        .map_err(|_| CommandError {
            code: ErrorCode::PreviewFailed,
        })
}
