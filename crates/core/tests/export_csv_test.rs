//! # CSV Export Integration Tests (tests/export_csv_test.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Validates the CSV export pipeline, specifically verifying that multiline cells,
//! multiline formulas, and multiline shape names are converted to single spaces
//! so that N matches strictly yield exactly N + 1 physical lines in the output CSV.
//! Conforms to Constitution Principle I, II, III, and V.
//!
//! ## Arguments / Returns
//! Test binary executed by Cargo test runner.
//!
//! ## Errors / Exceptions
//! Panics on assertion failures or unexpected file I/O errors.

use std::collections::BTreeMap;
use std::fs;
use xlseek_core::export::export_to_csv;
use xlseek_core::models::{MatchType, SearchMatch};

/// ## Description
/// Builds a minimal translation catalog for English CSV headers.
///
/// ## Arguments / Returns
/// None. Returns BTreeMap of catalogs.
///
/// ## Errors / Exceptions
/// Does not panic.
fn create_test_catalogs() -> BTreeMap<String, BTreeMap<String, String>> {
    let mut catalogs = BTreeMap::new();
    let mut english = BTreeMap::new();
    for key in xlseek_core::constants::EXPORT_HEADER_KEYS {
        english.insert(key.to_string(), key.to_string());
    }
    english.insert(
        xlseek_core::constants::EXPORT_MATCH_VALUE_KEY.to_string(),
        "Cell Value".to_string(),
    );
    catalogs.insert(xlseek_core::constants::LANGUAGE_EN.to_string(), english);
    catalogs
}

/// ## Description
/// Tests that exporting N search matches with multiline contents strictly results in N + 1 lines.
///
/// ## Arguments / Returns
/// None. Returns void.
///
/// ## Errors / Exceptions
/// Panics on assertion failure or file I/O error.
#[test]
fn test_export_to_csv_multiline_line_count_invariant() {
    let output_path = std::env::temp_dir().join("xlseek-test-export-multiline-line-count.csv");
    let path_str = output_path.to_str().expect("Path to str");
    let catalogs = create_test_catalogs();

    let matches = vec![
        SearchMatch {
            id: 1,
            file_name: "sample1.xlsx".to_string(),
            full_path: "/data/sample1.xlsx".to_string(),
            sheet_name: "Sheet1".to_string(),
            cell_address: "A1".to_string(),
            row_index: 1,
            col_index: 1,
            col_name: "A".to_string(),
            shape_name: None,
            match_type: MatchType::CellValue,
            sheet_hidden: false,
            snippet: "Line 1".to_string(),
            full_content: "Line 1\nLine 2\r\nLine 3".to_string(),
            formula: None,
            sheets_in_workbook: vec!["Sheet1".to_string()],
        },
        SearchMatch {
            id: 2,
            file_name: "sample2.xlsx".to_string(),
            full_path: "/data/sample2.xlsx".to_string(),
            sheet_name: "Sheet2".to_string(),
            cell_address: "B5".to_string(),
            row_index: 5,
            col_index: 2,
            col_name: "B".to_string(),
            shape_name: Some("Banner\nBox".to_string()),
            match_type: MatchType::CellValue,
            sheet_hidden: false,
            snippet: "Header".to_string(),
            full_content: "\n\nHeader\n\nValue\n\n".to_string(),
            formula: Some("=IF(\nB5>0,\n\"Yes\",\n\"No\")".to_string()),
            sheets_in_workbook: vec!["Sheet2".to_string()],
        },
        SearchMatch {
            id: 3,
            file_name: "sample3.xlsx".to_string(),
            full_path: "/data/sample3.xlsx".to_string(),
            sheet_name: "Sheet3".to_string(),
            cell_address: "C10".to_string(),
            row_index: 10,
            col_index: 3,
            col_name: "C".to_string(),
            shape_name: None,
            match_type: MatchType::CellValue,
            sheet_hidden: false,
            snippet: "Simple".to_string(),
            full_content: "Simple content without newlines".to_string(),
            formula: None,
            sheets_in_workbook: vec!["Sheet3".to_string()],
        },
    ];

    let result = export_to_csv(
        path_str,
        &matches,
        xlseek_core::constants::LANGUAGE_EN,
        &catalogs,
    );
    assert!(result.is_ok());

    let content = fs::read_to_string(&output_path).expect("Read exported CSV");
    // Split into non-empty lines (strip trailing newline)
    let lines: Vec<&str> = content.trim_end().split('\n').collect();

    // Invariant: Exactly N + 1 lines (1 header + 3 match rows)
    assert_eq!(lines.len(), 4);

    // Verify Row 1: Line 1 Line 2 Line 3
    assert!(lines[1].contains("Line 1 Line 2 Line 3"));
    // Verify Row 2: Header  Value and formula without newlines
    assert!(lines[2].contains("Header  Value"));
    assert!(lines[2].contains("Banner Box"));
    assert!(lines[2].contains("=IF( B5>0, \"\"Yes\"\", \"\"No\"\")"));
    // Verify Row 3
    assert!(lines[3].contains("Simple content without newlines"));

    let _ = fs::remove_file(output_path);
}
