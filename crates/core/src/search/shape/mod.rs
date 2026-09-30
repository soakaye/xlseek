//! # Excel Shape Extraction Module
//!
//! ## Description
//! Extracts shape names, text, sheet names, and anchors from drawing parts.
//! ## Arguments / Returns
//! Format adapters accept file path and cancellation flag, returning common ShapeText list.
//! ## Errors / Exceptions
//! Returns Result error on unsupported format, corrupted ZIP/XML, or exceeded limits.

mod ooxml;
mod xls;
mod xlsb;

use std::path::Path;
use std::sync::atomic::AtomicBool;

/// ## Description
/// Holds information required for searching from a DrawingML shape.
/// ## Arguments / Returns
/// Fields store shape ID, shape name, full text, sheet name, and optional 1-based anchor coordinates.
/// ## Errors / Exceptions
/// Data holder only; does not generate errors or panics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShapeText {
    pub shape_id: String,
    pub shape_name: String,
    pub text: String,
    pub sheet_name: String,
    pub anchor: Option<(u32, u32)>,
}

/// ## Description
/// Executes the shape extraction adapter appropriate for the file extension.
/// ## Arguments / Returns
/// `path` is an Excel workbook, `cancel_flag` is optional cancellation flag, returns shape list.
/// ## Errors / Exceptions
/// Returns error string on corrupted ZIP/XML or unsupported format.
pub fn extract_shapes(
    path: &Path,
    sheet_names: &[String],
    cancel_flag: Option<&AtomicBool>,
) -> Result<Vec<ShapeText>, String> {
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
        Some(crate::constants::EXT_XLSX | crate::constants::EXT_XLSM) => {
            ooxml::extract(path, cancel_flag)
        }
        Some(crate::constants::EXT_XLSB) => xlsb::extract(path, sheet_names, cancel_flag),
        Some(crate::constants::EXT_XLS) => xls::extract(path, sheet_names, cancel_flag),
        _ => Ok(Vec::new()),
    }
}
