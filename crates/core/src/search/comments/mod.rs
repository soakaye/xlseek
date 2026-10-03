//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Exposes cell comment extraction and common comment models across Excel formats.
//! ## Arguments / Returns
//! Passes workbook path and sheet names to format-specific extractors and returns 1-based coordinate comments.
//! ## Errors / Exceptions
//! Returns Err on missing parts, external relationships, malformed XML, or boundary violations.

mod ooxml;

/// ## Description
/// Represents comment text attached to a cell along with its Excel coordinates.
/// ## Arguments / Returns
/// `sheet_name` and `text` are Strings; `row` and `col` are 1-based u32 indices.
/// ## Errors / Exceptions
/// Holds values only; does not generate errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentText {
    pub sheet_name: String,
    pub row: u32,
    pub col: u32,
    pub text: String,
}

/// ## Description
/// Extracts legacy cell notes from OOXML workbooks (.xlsx, .xlsm).
/// ## Arguments / Returns
/// Accepts workbook path and returns `Result<Vec<CommentText>, String>`.
/// ## Errors / Exceptions
/// Returns Err on corrupted ZIP/XML/relationships or exceeded limits.
pub fn extract(path: &std::path::Path) -> Result<Vec<CommentText>, String> {
    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| {
            format!(
                "{}{}",
                crate::constants::CLI_EXTENSION_PREFIX,
                value.to_ascii_lowercase()
            )
        });
    match extension.as_deref() {
        Some(crate::constants::EXT_XLSX | crate::constants::EXT_XLSM) => ooxml::extract(path),
        _ => Ok(Vec::new()),
    }
}
