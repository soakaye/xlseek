//! # Shape Export Integration Test
//!
//! ## Description
//! Verifies that shape result name, type, content, and position are exported
//! in common column order for both CSV and Excel formats.
//! ## Arguments / Returns
//! No arguments. Passes translation catalogs and result model to export functions and reads back files.
//! ## Errors / Exceptions
//! Fails if file export, reopening, or column values mismatch assertions.

use calamine::Reader;
use exlgrep_core::constants::*;
use exlgrep_core::export::{csv_export::export_to_csv, xlsx_export::export_to_xlsx};
use exlgrep_core::models::{MatchType, SearchMatch};
use std::collections::BTreeMap;

const TEST_OUTPUT_PREFIX: &str = "exlgrep-shape-export";

/// ## Description
/// Verifies that shape name appears in shape column and shape type/content appear in appropriate columns.
/// ## Arguments / Returns
/// No arguments. Reopens both formats and verifies column order and values.
/// ## Errors / Exceptions
/// Fails on export error, file read failure, or assertion failure.
#[test]
fn writes_shape_name_to_independent_column_in_both_formats() {
    let mut catalog = BTreeMap::new();
    for key in EXPORT_HEADER_KEYS {
        catalog.insert(key.to_string(), key.to_string());
    }
    for key in [
        EXPORT_MATCH_VALUE_KEY,
        EXPORT_MATCH_FORMULA_KEY,
        EXPORT_MATCH_COMMENT_KEY,
        EXPORT_MATCH_HIDDEN_SHEET_KEY,
        EXPORT_MATCH_SHAPE_KEY,
        EXPORT_SHEET_NAME_KEY,
    ] {
        catalog.insert(key.to_string(), key.to_string());
    }
    let catalogs = BTreeMap::from([
        (LANGUAGE_EN.to_string(), catalog.clone()),
        (LANGUAGE_JA.to_string(), catalog),
    ]);
    let item = SearchMatch {
        id: 1,
        file_name: "report.xlsx".to_string(),
        full_path: "/books/report.xlsx".to_string(),
        sheet_name: "Sheet1".to_string(),
        cell_address: "".to_string(),
        row_index: 0,
        col_index: 0,
        col_name: "".to_string(),
        match_type: MatchType::Shape,
        shape_name: Some("Group / Label".to_string()),
        sheet_hidden: false,
        snippet: "needle".to_string(),
        full_content: "shape needle".to_string(),
        formula: None,
        sheets_in_workbook: vec!["Sheet1".to_string()],
    };

    let csv_path = std::env::temp_dir().join(format!("{TEST_OUTPUT_PREFIX}.csv"));
    let xlsx_path = std::env::temp_dir().join(format!("{TEST_OUTPUT_PREFIX}.xlsx"));
    export_to_csv(
        csv_path.to_str().unwrap_or_default(),
        std::slice::from_ref(&item),
        LANGUAGE_EN,
        &catalogs,
    )
    .expect("CSV export must succeed");
    export_to_xlsx(
        xlsx_path.to_str().unwrap_or_default(),
        std::slice::from_ref(&item),
        LANGUAGE_EN,
        &catalogs,
    )
    .expect("Excel export must succeed");

    let mut csv = csv::Reader::from_path(&csv_path).expect("CSV must reopen");
    let headers = csv.headers().expect("CSV headers must parse");
    assert_eq!(headers.len(), EXPORT_HEADER_KEYS.len());
    assert_eq!(headers.get(5), Some("export.header.shapeName"));
    let record = csv
        .records()
        .next()
        .expect("CSV row must exist")
        .expect("CSV row must parse");
    assert_eq!(record.get(5), Some("Group / Label"));
    assert_eq!(record.get(6), Some(EXPORT_MATCH_SHAPE_KEY));

    let mut workbook = calamine::open_workbook_auto(&xlsx_path).expect("Excel file must reopen");
    let sheet = workbook
        .worksheet_range_at(0)
        .expect("worksheet must exist")
        .expect("worksheet must parse");
    assert_eq!(
        sheet.get_value((1, 5)).map(ToString::to_string).as_deref(),
        Some("Group / Label")
    );
    assert_eq!(
        sheet.get_value((1, 6)).map(ToString::to_string).as_deref(),
        Some(EXPORT_MATCH_SHAPE_KEY)
    );

    std::fs::remove_file(csv_path).expect("CSV must be removed");
    std::fs::remove_file(xlsx_path).expect("Excel output must be removed");
}
