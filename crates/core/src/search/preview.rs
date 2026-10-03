//! # Cell Surrounding Preview Extraction Module (search/preview.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Generates a preview grid centered around a specific sheet and cell address
//! in an Excel file by extracting cell values and formulas within bounding rows and columns.
//! Conforms to Constitution Principle I (English comments/errors), Principle II (Constant references),
//! Principle III (Header comments), and Principle V (Robust error handling).

use crate::models::{CellPreviewData, CellValueInfo, PreviewColumn, PreviewRow};
use crate::search::parser::col_to_name;
use calamine::{open_workbook_auto, Data, Reader, Sheets};
use std::collections::HashMap;
use std::path::Path;

/// ## Description
/// Extracts worksheet data surrounding a target cell (radius defined by constants)
/// and builds a preview struct containing column headers, row data, and all sheet names.
///
/// ## Arguments
/// - `file_path`: `P` - Path to the Excel file
/// - `sheet_name`: `&str` - Name of the worksheet to preview
/// - `target_row_1based`: `u32` - 1-based target cell row index
/// - `target_col_1based`: `u32` - 1-based target cell column index
///
/// ## Returns
/// - `Result<CellPreviewData, String>`: Preview grid data on success, or an English error message on failure
///
/// ## Errors / Exceptions
/// - Returns `Err` if the file cannot be opened or the specified sheet does not exist.
/// - Does not panic.
pub fn extract_cell_preview<P: AsRef<Path>>(
    file_path: P,
    sheet_name: &str,
    target_row_1based: u32,
    target_col_1based: u32,
) -> Result<CellPreviewData, String> {
    let path_ref = file_path.as_ref();
    let mut workbook: Sheets<_> = open_workbook_auto(path_ref).map_err(|e| {
        // Constant reference: crate::constants::ERR_WORKBOOK_OPEN
        format!(
            "{}: {} ({})",
            crate::constants::ERR_WORKBOOK_OPEN,
            path_ref.display(),
            e
        )
    })?;

    let sheets_in_workbook = workbook.sheet_names().to_vec();

    let target_row_0based = target_row_1based.saturating_sub(1);
    let target_col_0based = target_col_1based.saturating_sub(1);

    // Constant reference: crate::constants::PREVIEW_ROW_RADIUS
    let min_row = target_row_0based.saturating_sub(crate::constants::PREVIEW_ROW_RADIUS);
    let max_row = target_row_0based + crate::constants::PREVIEW_ROW_RADIUS;

    // Constant reference: crate::constants::PREVIEW_COL_RADIUS
    let min_col = target_col_0based.saturating_sub(crate::constants::PREVIEW_COL_RADIUS);
    let max_col = target_col_0based + crate::constants::PREVIEW_COL_RADIUS;

    // Define column headers
    let mut columns = Vec::new();
    for col_idx in min_col..=max_col {
        let label = col_to_name(col_idx);
        columns.push(PreviewColumn {
            key: label.clone(),
            label,
        });
    }

    // Read worksheet cell values
    let range = workbook.worksheet_range(sheet_name).map_err(|e| {
        // Constant reference: crate::constants::ERR_SHEET_NOT_FOUND
        format!(
            "{}: '{}' ({})",
            crate::constants::ERR_SHEET_NOT_FOUND,
            sheet_name,
            e
        )
    })?;

    let (range_start_row, range_start_col) = range.start().unwrap_or_default();

    // Read formula map if available
    let formula_map = if let Ok(f_range) = workbook.worksheet_formula(sheet_name) {
        let (f_start_row, f_start_col) = f_range.start().unwrap_or_default();
        let mut map: HashMap<(u32, u32), String> = HashMap::new();
        for (r, row) in f_range.rows().enumerate() {
            for (c, formula) in row.iter().enumerate() {
                if !formula.is_empty() {
                    let absolute_r = r as u32 + f_start_row;
                    let absolute_c = c as u32 + f_start_col;
                    map.insert((absolute_r, absolute_c), formula.clone());
                }
            }
        }
        Some(map)
    } else {
        None
    };

    let mut rows = Vec::new();

    for r in min_row..=max_row {
        let row_number = r + 1;
        let mut cells = HashMap::new();

        for c in min_col..=max_col {
            let col_name = col_to_name(c);
            let is_target = r == target_row_0based && c == target_col_0based;

            let value_str = if r >= range_start_row && c >= range_start_col {
                let rel_r = (r - range_start_row) as usize;
                let rel_c = (c - range_start_col) as usize;
                if let Some(cell_data) = range.get((rel_r, rel_c)) {
                    match cell_data {
                        Data::Empty => String::new(),
                        Data::String(s) => s.clone(),
                        Data::Float(f) => f.to_string(),
                        Data::Int(i) => i.to_string(),
                        Data::Bool(b) => b.to_string(),
                        Data::DateTime(d) => d.to_string(),
                        Data::DateTimeIso(d) => d.clone(),
                        Data::DurationIso(d) => d.clone(),
                        Data::Error(e) => format!("{:?}", e),
                    }
                } else {
                    String::new()
                }
            } else {
                String::new()
            };

            let formula = formula_map.as_ref().and_then(|m| m.get(&(r, c)).cloned());

            cells.insert(
                col_name,
                CellValueInfo {
                    value: value_str,
                    is_target,
                    formula,
                },
            );
        }

        rows.push(PreviewRow { row_number, cells });
    }

    Ok(CellPreviewData {
        target_row: target_row_1based,
        target_col: target_col_1based,
        columns,
        rows,
        sheets_in_workbook,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// ## Description
    /// Verifies that calling extract_cell_preview with a nonexistent file path
    /// returns an Err containing an error message safely without panicking.
    ///
    /// ## Arguments
    /// None
    ///
    /// ## Returns
    /// None
    ///
    /// ## Errors / Exceptions
    /// Panics if an assertion fails.
    #[test]
    fn test_extract_cell_preview_nonexistent_file() {
        let invalid_path = PathBuf::from("nonexistent_test_workbook_12345.xlsx");
        let result = extract_cell_preview(&invalid_path, "Sheet1", 1, 1);
        assert!(result.is_err());
        let err_msg = result.unwrap_err();
        assert!(err_msg.contains(crate::constants::ERR_WORKBOOK_OPEN));
    }

    /// ## Description
    /// Verifies that extract_cell_preview extracts cells correctly even when the worksheet range
    /// begins after A1 (non-zero start offset).
    ///
    /// ## Arguments
    /// None
    ///
    /// ## Returns
    /// None
    ///
    /// ## Errors / Exceptions
    /// Panics if an assertion fails or fixture cannot be read.
    #[test]
    fn test_extract_cell_preview_range_with_start_offset() {
        let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let root = if manifest.join("../../tests/fixtures").exists() {
            manifest.join("../..")
        } else {
            manifest.parent().unwrap().to_path_buf()
        };
        let fixture = root.join("tests/fixtures/sample_report.xlsx");
        // In sample_report.xlsx, Data range starts at A12 (0-based row 11)
        let result = extract_cell_preview(&fixture, "Data", 12, 1).expect("preview should succeed");
        assert_eq!(result.target_row, 12);
        assert_eq!(result.target_col, 1);
        let target_row = result
            .rows
            .iter()
            .find(|r| r.row_number == 12)
            .expect("row 12 must be present");
        let target_cell = target_row.cells.get("A").expect("cell A must be present");
        assert!(target_cell.is_target);
        assert_eq!(target_cell.value, "Financial Report Q3");
    }
}
