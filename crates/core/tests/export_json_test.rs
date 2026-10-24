//! # JSON Export Integration Test (tests/export_json_test.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Validates the JSON export pipeline: stream writing (`write_json_to_writer`),
//! file export (`export_to_json`), handling of empty match slices,
//! top-level `total_matches` metadata, path normalization, and schema alignment with `xlseek-mcp`.
//! Complies with Constitution Principle I, II, III, and V.
//!
//! ## Arguments / Returns
//! Test binary executed by Cargo test runner.
//!
//! ## Errors / Exceptions
//! Panics on assertion failures or deserialization errors.

use std::fs;
use xlseek_core::export::json_export::{export_to_json, write_json_to_writer, JsonExportPayload};
use xlseek_core::models::{MatchType, SearchMatch};

fn create_sample_json_matches() -> Vec<SearchMatch> {
    vec![
        SearchMatch {
            id: 1,
            file_name: "budget.xlsx".to_string(),
            full_path: "\\\\?\\C:\\Reports\\2026\\budget.xlsx".to_string(),
            sheet_name: "Expenses".to_string(),
            cell_address: "D12".to_string(),
            row_index: 12,
            col_index: 4,
            col_name: "D".to_string(),
            match_type: MatchType::CellValue,
            shape_name: None,
            sheet_hidden: false,
            snippet: "Software Subscription".to_string(),
            full_content: "Software Subscription ($500/mo)".to_string(),
            formula: None,
            sheets_in_workbook: vec!["Expenses".to_string()],
        },
        SearchMatch {
            id: 2,
            file_name: "budget.xlsx".to_string(),
            full_path: "C:\\Reports\\2026\\budget.xlsx".to_string(),
            sheet_name: "Summary".to_string(),
            cell_address: "B2".to_string(),
            row_index: 2,
            col_index: 2,
            col_name: "B".to_string(),
            match_type: MatchType::Formula,
            shape_name: None,
            sheet_hidden: false,
            snippet: "=SUM(Expenses!D12:D50)".to_string(),
            full_content: "=SUM(Expenses!D12:D50)".to_string(),
            formula: Some("=SUM(Expenses!D12:D50)".to_string()),
            sheets_in_workbook: vec!["Expenses".to_string(), "Summary".to_string()],
        },
    ]
}

/// ## Description
/// Tests that writing an empty slice produces valid JSON with total_matches: 0 and an empty array.
///
/// ## Arguments / Returns
/// None. Returns void.
///
/// ## Errors / Exceptions
/// Panics if serialization fails or produced JSON does not match expected structure.
#[test]
fn test_write_json_empty_slice() {
    let mut buffer = Vec::new();
    let result = write_json_to_writer(&mut buffer, &[]);
    assert!(result.is_ok());

    let payload: JsonExportPayload =
        serde_json::from_slice(&buffer).expect("Valid JSON deserialization");
    assert_eq!(payload.total_matches, 0);
    assert!(payload.matches.is_empty());
}

/// ## Description
/// Tests that write_json_to_writer formats sample matches with normalized paths, 2-space pretty printing,
/// and correct handling of optional formula and shape_name fields.
///
/// ## Arguments / Returns
/// None. Returns void.
///
/// ## Errors / Exceptions
/// Panics if JSON deserialization or field verification fails.
#[test]
fn test_write_json_to_writer_content_and_normalization() {
    let matches = create_sample_json_matches();
    let mut buffer = Vec::new();
    let result = write_json_to_writer(&mut buffer, &matches);
    assert!(result.is_ok());

    let json_str = String::from_utf8(buffer).expect("Valid UTF-8 JSON");
    // Verify 2-space indentation
    assert!(json_str.contains("  \"total_matches\": 2"));
    assert!(json_str.contains("  \"matches\": ["));

    let payload: JsonExportPayload =
        serde_json::from_str(&json_str).expect("Valid JSON deserialization");
    assert_eq!(payload.total_matches, 2);
    assert_eq!(payload.matches.len(), 2);

    // Verify path normalization (stripping Windows \\?\ verbatim prefix)
    assert_eq!(
        payload.matches[0].full_path,
        "C:\\Reports\\2026\\budget.xlsx"
    );

    // Verify optional fields: match 0 has no formula, match 1 has formula
    assert_eq!(payload.matches[0].formula, None);
    assert_eq!(
        payload.matches[1].formula,
        Some("=SUM(Expenses!D12:D50)".to_string())
    );
    assert_eq!(payload.matches[0].shape_name, None);
    assert_eq!(payload.matches[1].shape_name, None);
}

/// ## Description
/// Tests that export_to_json writes the JSON output to a file and can be read back.
///
/// ## Arguments / Returns
/// None. Returns void.
///
/// ## Errors / Exceptions
/// Panics if file creation, writing, or reading fails.
#[test]
fn test_export_to_json_file() {
    let output_path = std::env::temp_dir().join("xlseek-test-export-output.json");
    let path_str = output_path.to_str().expect("Path to str");
    let matches = create_sample_json_matches();

    let result = export_to_json(path_str, &matches);
    assert!(result.is_ok());

    let content = fs::read_to_string(path_str).expect("Read exported JSON file");
    let payload: JsonExportPayload =
        serde_json::from_str(&content).expect("Valid JSON deserialization");
    assert_eq!(payload.total_matches, 2);
    assert_eq!(payload.matches[0].file_name, "budget.xlsx");

    let _ = fs::remove_file(output_path);
}
