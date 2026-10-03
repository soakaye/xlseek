//! # Export Submodule Definition (export/mod.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Exposes and manages CSV export (csv_export) and Excel (.xlsx) export (xlsx_export)
//! functionality for search results.

pub mod csv_export;
pub mod xlsx_export;

pub use csv_export::{export_to_csv, write_csv_to_writer};
pub use xlsx_export::export_to_xlsx;

use std::borrow::Cow;

/// ## Description
/// Normalizes a path string for user-facing search export (CSV/XLSX/stdout).
/// Strips Windows verbatim prefixes (`\\?\` for drive paths, `\\?\UNC\` for network shares)
/// to produce clean, standard paths without altering internal canonicalization logic.
///
/// ## Arguments
/// - `path`: `&str` - Path string to normalize
///
/// ## Returns
/// - `Cow<'_, str>`: Clean display path string
///
/// ## Errors / Exceptions
/// None. Always succeeds without panicking.
pub fn normalize_export_path(path: &str) -> Cow<'_, str> {
    // Constant reference: crate::constants::VERBATIM_UNC_PATH_PREFIX_WINDOWS
    if let Some(suffix) = path.strip_prefix(crate::constants::VERBATIM_UNC_PATH_PREFIX_WINDOWS) {
        // Constant reference: crate::constants::STANDARD_UNC_PREFIX_WINDOWS
        return Cow::Owned(format!(
            "{}{}",
            crate::constants::STANDARD_UNC_PREFIX_WINDOWS,
            suffix
        ));
    }
    // Constant reference: crate::constants::VERBATIM_PATH_PREFIX_WINDOWS
    if let Some(suffix) = path.strip_prefix(crate::constants::VERBATIM_PATH_PREFIX_WINDOWS) {
        return Cow::Borrowed(suffix);
    }
    Cow::Borrowed(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// ## Description
    /// Verifies that `normalize_export_path` strips verbatim drive and UNC prefixes while leaving regular paths unchanged.
    ///
    /// ## Arguments / Returns
    /// None.
    ///
    /// ## Errors / Exceptions
    /// Panics on assertion failure.
    #[test]
    fn test_normalize_export_path() {
        assert_eq!(
            normalize_export_path(r"\\?\C:\Users\admin\report.xlsx"),
            r"C:\Users\admin\report.xlsx"
        );
        assert_eq!(
            normalize_export_path(r"\\?\UNC\server\share\file.xlsx"),
            r"\\server\share\file.xlsx"
        );
        assert_eq!(
            normalize_export_path(r"C:\Users\admin\report.xlsx"),
            r"C:\Users\admin\report.xlsx"
        );
        assert_eq!(
            normalize_export_path("/Users/admin/report.xlsx"),
            "/Users/admin/report.xlsx"
        );
    }
}
