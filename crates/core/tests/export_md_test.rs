//! # Markdown Export Integration Test (tests/export_md_test.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Validates the Markdown export pipeline: stream writing (`write_md_to_writer`)
//! and file export (`export_to_md`), handling of empty match slices,
//! and workbook / sheet grouping using the LLM context extraction engine.
//! Complies with Constitution Principle I, II, III, and V.
//!
//! ## Arguments / Returns
//! Test binary executed by Cargo test runner.
//!
//! ## Errors / Exceptions
//! Panics on assertion failures or unexpected I/O errors.

use rust_xlsxwriter::Workbook;
use std::fs;
use std::path::PathBuf;
use xlseek_core::export::md_export::{export_to_md, write_md_to_writer};
use xlseek_core::models::{
    LlmCopyOptions, LlmCopyType, MatchType, SearchMatch, WordExtractionMode,
};

fn create_test_excel_file(test_name: &str) -> (PathBuf, Vec<SearchMatch>) {
    let source_path = std::env::temp_dir().join(format!("xlseek-test-source-{}.xlsx", test_name));
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.set_name("Revenue").expect("set sheet name");
    worksheet
        .write_string(0, 0, "Item")
        .expect("write header 0");
    worksheet
        .write_string(0, 1, "Amount")
        .expect("write header 1");
    worksheet
        .write_string(1, 0, "Widgets")
        .expect("write row 1");
    worksheet
        .write_string(1, 1, "50000")
        .expect("write amount 1");
    workbook.save(&source_path).expect("save test workbook");

    let matches = vec![SearchMatch {
        id: 1,
        file_name: format!("xlseek-test-source-{}.xlsx", test_name),
        full_path: source_path.to_str().expect("to str").to_string(),
        sheet_name: "Revenue".to_string(),
        cell_address: "B2".to_string(),
        row_index: 2,
        col_index: 2,
        col_name: "B".to_string(),
        match_type: MatchType::CellValue,
        shape_name: None,
        sheet_hidden: false,
        snippet: "50000".to_string(),
        full_content: "50000".to_string(),
        formula: None,
        sheets_in_workbook: vec!["Revenue".to_string()],
    }];

    (source_path, matches)
}

/// ## Description
/// Tests that writing an empty slice of matches to a stream results in empty output and Ok(()).
///
/// ## Arguments / Returns
/// None. Returns void.
///
/// ## Errors / Exceptions
/// Panics if write_md_to_writer returns Err or produces bytes.
#[test]
fn test_write_md_empty_slice() {
    let mut buffer = Vec::new();
    let options = LlmCopyOptions::default();
    let result = write_md_to_writer(&mut buffer, &[], &options);
    assert!(result.is_ok());
    assert!(buffer.is_empty());
}

/// ## Description
/// Tests that write_md_to_writer formats sample matches with expected document headers and structure.
///
/// ## Arguments / Returns
/// None. Returns void.
///
/// ## Errors / Exceptions
/// Panics if stream writing fails or if required markdown headers are absent.
#[test]
fn test_write_md_to_writer_content() {
    let (source_path, matches) = create_test_excel_file("writer-content");
    let mut buffer = Vec::new();
    let options = LlmCopyOptions::default();
    let result = write_md_to_writer(&mut buffer, &matches, &options);
    assert!(result.is_ok());

    let output_str = String::from_utf8(buffer).expect("Valid UTF-8 markdown");
    assert!(output_str.contains("Revenue"));
    assert!(output_str.contains("50000"));

    let _ = fs::remove_file(source_path);
}

/// ## Description
/// Tests that export_to_md writes the formatted Markdown to an actual file on disk.
///
/// ## Arguments / Returns
/// None. Returns void.
///
/// ## Errors / Exceptions
/// Panics if temp file creation, writing, or reading back fails.
#[test]
fn test_export_to_md_file() {
    let (source_path, matches) = create_test_excel_file("file-export");
    let output_path = std::env::temp_dir().join("xlseek-test-export-output-file.md");
    let path_str = output_path.to_str().expect("Path to str");

    let options = LlmCopyOptions {
        column_window_size: 3,
        copy_type: LlmCopyType::Table,
        word_mode: WordExtractionMode::HeadingsAndParagraph,
    };

    let result = export_to_md(path_str, &matches, Some(&options));
    assert!(result.is_ok());

    let content = fs::read_to_string(path_str).expect("Read exported markdown file");
    assert!(!content.is_empty());
    assert!(content.contains("Revenue"));
    assert!(content.contains("50000"));

    let _ = fs::remove_file(source_path);
    let _ = fs::remove_file(output_path);
}

/// ## Description
/// Creates a synthetic test workbook with multiline cell data and returns its path and match slice.
///
/// ## Arguments
/// - `test_name`: `&str` - Unique test suffix
///
/// ## Returns
/// - `(PathBuf, Vec<SearchMatch>)`: Temporary file path and single SearchMatch object
///
/// ## Errors / Exceptions
/// Panics if workbook creation or saving fails.
fn create_test_multiline_excel_file(test_name: &str) -> (PathBuf, Vec<SearchMatch>) {
    let source_path =
        std::env::temp_dir().join(format!("xlseek-test-multiline-{}.xlsx", test_name));
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.set_name("Notes").expect("set sheet name");
    worksheet
        .write_string(0, 0, "Title")
        .expect("write header 0");
    worksheet
        .write_string(0, 1, "Details")
        .expect("write header 1");
    worksheet.write_string(1, 0, "Note1").expect("write row 1");
    worksheet
        .write_string(1, 1, "First Line\nSecond Line\r\nThird Line")
        .expect("write multiline");
    workbook.save(&source_path).expect("save test workbook");

    let matches = vec![SearchMatch {
        id: 1,
        file_name: format!("xlseek-test-multiline-{}.xlsx", test_name),
        full_path: source_path.to_str().expect("to str").to_string(),
        sheet_name: "Notes".to_string(),
        cell_address: "B2".to_string(),
        row_index: 2,
        col_index: 2,
        col_name: "B".to_string(),
        match_type: MatchType::CellValue,
        shape_name: None,
        sheet_hidden: false,
        snippet: "Second Line".to_string(),
        full_content: "First Line\nSecond Line\r\nThird Line".to_string(),
        formula: None,
        sheets_in_workbook: vec!["Notes".to_string()],
    }];

    (source_path, matches)
}

/// ## Description
/// Tests that Markdown table export converts multiline cell text to <br> without breaking rows.
///
/// ## Arguments / Returns
/// None. Returns void.
///
/// ## Errors / Exceptions
/// Panics on assertion failures or unexpected write errors.
#[test]
fn test_write_md_multiline_table_export() {
    let (source_path, matches) = create_test_multiline_excel_file("table-multiline");
    let mut buffer = Vec::new();
    let options = LlmCopyOptions {
        column_window_size: 3,
        copy_type: LlmCopyType::Table,
        word_mode: WordExtractionMode::HeadingsAndParagraph,
    };
    let result = write_md_to_writer(&mut buffer, &matches, &options);
    assert!(result.is_ok());

    let output_str = String::from_utf8(buffer).expect("Valid UTF-8 markdown");
    assert!(output_str.contains("First Line<br>Second Line<br>Third Line"));

    // Verify all table rows start and end with '|' and contain no unescaped newlines
    let table_lines: Vec<&str> = output_str
        .lines()
        .filter(|line| line.starts_with('|') && line.ends_with('|'))
        .collect();
    assert!(table_lines.len() >= 3); // Header, separator, data row
    for line in table_lines {
        assert!(!line.contains('\n'));
        assert!(!line.contains('\r'));
    }

    let _ = fs::remove_file(source_path);
}

/// ## Description
/// Tests that Markdown Header & Cell Only export converts multiline text to <br> to protect bullet lists.
///
/// ## Arguments / Returns
/// None. Returns void.
///
/// ## Errors / Exceptions
/// Panics on assertion failures or unexpected write errors.
#[test]
fn test_write_md_multiline_header_only_export() {
    let (source_path, matches) = create_test_multiline_excel_file("header-multiline");
    let mut buffer = Vec::new();
    let options = LlmCopyOptions {
        column_window_size: 3,
        copy_type: LlmCopyType::HeaderCellOnly,
        word_mode: WordExtractionMode::HeadingsAndParagraph,
    };
    let result = write_md_to_writer(&mut buffer, &matches, &options);
    assert!(result.is_ok());

    let output_str = String::from_utf8(buffer).expect("Valid UTF-8 markdown");
    assert!(output_str.contains("- Matched Value: **First Line<br>Second Line<br>Third Line**"));

    let _ = fs::remove_file(source_path);
}
