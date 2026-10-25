//! # CLI Search Integration Tests
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Verifies that the CLI binary exports Excel search results without initializing a GUI.
//!
//! ## Arguments / Returns
//! Executes Cargo-provided CLI binary and verifies process status and output content.
//!
//! ## Errors
//! Fails test if process execution, file operations, or assertions fail.

use calamine::Reader;
use std::fs;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

const TEST_DIRECTORY_PREFIX: &str = "xlseek-cli-search";
const QUERY_TEXT: &str = "Financial Report Q3";
const FORMAT_CSV: &str = "csv";
const OUTPUT_CSV_NAME: &str = "result.csv";
const OUTPUT_XLSX_NAME: &str = "result.xlsx";
const TEST_EMPTY_DIRECTORY_NAME: &str = "empty";
const TEST_BROKEN_DIRECTORY_NAME: &str = "broken";
const TEST_PARTIAL_DIRECTORY_NAME: &str = "partial";
const TEST_BROKEN_WORKBOOK_NAME: &str = "broken.xlsx";
const TEST_DIRECTORY_SEPARATOR: &str = "-";
static TEST_DIRECTORY_SEQUENCE: AtomicUsize = AtomicUsize::new(0);

/// Generates a unique temporary directory that will not conflict across concurrent tests.
///
/// ## Arguments / Returns
/// Returns a `PathBuf` incorporating process ID and atomic sequence number.
///
/// ## Errors
/// Simple path construction; does not fail or panic.
fn unique_test_directory() -> std::path::PathBuf {
    let sequence = TEST_DIRECTORY_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "{TEST_DIRECTORY_PREFIX}{TEST_DIRECTORY_SEPARATOR}{}{TEST_DIRECTORY_SEPARATOR}{sequence}",
        std::process::id()
    ))
}

/// Resolves the absolute path to the repository root.
///
/// ## Arguments / Returns
/// Takes no arguments; returns `PathBuf`.
///
/// ## Errors
/// Panics if repository root cannot be determined.
fn repo_root() -> std::path::PathBuf {
    let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    if manifest.join("../../tests/fixtures").exists() {
        manifest.join("../..")
    } else {
        manifest
            .parent()
            .expect("repository root must exist")
            .to_path_buf()
    }
}

/// Searches a valid single workbook and saves matches to CSV and XLSX without a GUI.
///
/// ## Arguments / Returns
/// Tests CLI CSV and Excel export against expected search hits.
///
/// ## Errors
/// Panics if process fails or output content does not match.
#[test]
fn exports_search_results_from_a_single_workbook_without_gui() {
    let root = repo_root();
    let fixture = root.join("tests/fixtures/sample_report.xlsx");
    let output_dir = unique_test_directory();
    fs::create_dir_all(&output_dir).expect("temporary test directory must be created");
    let output = output_dir.join(OUTPUT_CSV_NAME);

    let result = Command::new(env!("CARGO_BIN_EXE_xlseek-cli"))
        .arg("--path")
        .arg(fixture)
        .arg("--query")
        .arg(QUERY_TEXT)
        .arg("--format")
        .arg(FORMAT_CSV)
        .arg("--output")
        .arg(&output)
        .output()
        .expect("CLI process must start");

    assert_eq!(result.status.code(), Some(0));
    let contents = fs::read_to_string(&output).expect("CSV result must be written");
    assert!(contents.contains(QUERY_TEXT));
    let xlsx = output_dir.join(OUTPUT_XLSX_NAME);
    let excel_result = Command::new(env!("CARGO_BIN_EXE_xlseek-cli"))
        .arg("--path")
        .arg(root.join("tests/fixtures/sample_report.xlsx"))
        .arg("--query")
        .arg(QUERY_TEXT)
        .arg("--format")
        .arg(xlseek_cli::constants::CLI_FORMAT_XLSX)
        .arg("--output")
        .arg(&xlsx)
        .output()
        .expect("CLI Excel process must start");
    assert_eq!(
        excel_result.status.code(),
        Some(xlseek_cli::constants::CLI_EXIT_SUCCESS)
    );
    let mut workbook = calamine::open_workbook_auto(&xlsx).expect("Excel output must open");
    let sheet_name = workbook.sheet_names().first().cloned().unwrap();
    let range = workbook.worksheet_range(&sheet_name).unwrap();
    assert!(range
        .rows()
        .flatten()
        .any(|cell| cell.to_string().contains(QUERY_TEXT)));
    let _ = fs::remove_dir_all(output_dir);
}

/// Differentiates empty folders, unreadable workbooks, partial successes, and invalid regexes via exit codes.
///
/// ## Arguments / Returns
/// Validates process exit codes and saved output files across diverse edge cases.
///
/// ## Errors
/// Panics if exit codes or file outputs deviate from specification.
#[test]
fn distinguishes_empty_partial_total_and_invalid_regex_runs() {
    let root = repo_root();
    let fixture = root.join("tests/fixtures/sample_report.xlsx");
    let work = unique_test_directory();
    let empty = work.join(TEST_EMPTY_DIRECTORY_NAME);
    let broken = work.join(TEST_BROKEN_DIRECTORY_NAME);
    let partial = work.join(TEST_PARTIAL_DIRECTORY_NAME);
    fs::create_dir_all(&empty).unwrap();
    fs::create_dir_all(&broken).unwrap();
    fs::create_dir_all(&partial).unwrap();
    let broken_file = broken.join(TEST_BROKEN_WORKBOOK_NAME);
    fs::write(
        &broken_file,
        xlseek_cli::constants::CLI_TEST_INVALID_WORKBOOK_BYTES,
    )
    .unwrap();
    fs::copy(
        &fixture,
        partial.join(xlseek_cli::constants::CLI_TEST_VALID_WORKBOOK_NAME),
    )
    .unwrap();
    fs::copy(&broken_file, partial.join(TEST_BROKEN_WORKBOOK_NAME)).unwrap();

    let empty_output = work.join(xlseek_cli::constants::CLI_TEST_EMPTY_OUTPUT_NAME);
    assert_eq!(
        run_cli(&empty, QUERY_TEXT, &empty_output, FORMAT_CSV)
            .status
            .code(),
        Some(0)
    );
    assert!(fs::read(&empty_output)
        .unwrap()
        .starts_with(&xlseek_cli::constants::CSV_UTF8_BOM));

    let broken_output = work.join(xlseek_cli::constants::CLI_TEST_BROKEN_OUTPUT_NAME);
    assert_eq!(
        run_cli(&broken, QUERY_TEXT, &broken_output, FORMAT_CSV)
            .status
            .code(),
        Some(2)
    );
    assert!(fs::read_to_string(&broken_output)
        .unwrap()
        .contains(xlseek_cli::constants::CLI_TEST_EXPECTED_HEADER_ID_JA));

    let partial_output = work.join(xlseek_cli::constants::CLI_TEST_PARTIAL_OUTPUT_NAME);
    assert_eq!(
        run_cli(&partial, QUERY_TEXT, &partial_output, FORMAT_CSV)
            .status
            .code(),
        Some(1)
    );
    assert!(fs::read_to_string(&partial_output)
        .unwrap()
        .contains(QUERY_TEXT));

    let invalid_regex_output = work.join(xlseek_cli::constants::CLI_TEST_REGEX_OUTPUT_NAME);
    let invalid_regex = Command::new(env!("CARGO_BIN_EXE_xlseek-cli"))
        .arg("--path")
        .arg(&fixture)
        .arg("--query")
        .arg(xlseek_cli::constants::CLI_TEST_INVALID_REGEX)
        .arg("--regex")
        .arg(xlseek_cli::constants::CLI_BOOLEAN_TRUE)
        .arg("--format")
        .arg(FORMAT_CSV)
        .arg("--output")
        .arg(&invalid_regex_output)
        .output()
        .unwrap();
    assert_eq!(invalid_regex.status.code(), Some(2));
    assert!(!invalid_regex_output.exists());
    let _ = fs::remove_dir_all(work);
}

/// Verifies that positional arguments output plain CSV to stdout with summary text suppressed.
///
/// ## Arguments / Returns
/// Validates process stdout/stderr contents and exit codes.
///
/// ## Errors
/// Panics if summary lines leak into stdout or if CSV content is incorrect.
#[test]
fn exports_search_results_to_stdout_with_positional_arguments() {
    let root = repo_root();
    let fixture1 = root.join("tests/fixtures/sample_report.xlsx");
    let fixture2 = root.join("tests/fixtures");

    let result = Command::new(env!("CARGO_BIN_EXE_xlseek-cli"))
        .arg(QUERY_TEXT)
        .arg(&fixture1)
        .arg(&fixture2)
        .output()
        .expect("CLI process must start");

    assert_eq!(result.status.code(), Some(0));

    // Verify stdout is plain UTF-8 CSV without BOM
    assert!(!result
        .stdout
        .starts_with(&xlseek_cli::constants::CSV_UTF8_BOM));

    let stdout_str = String::from_utf8(result.stdout).expect("stdout must be valid UTF-8");
    assert!(stdout_str.contains(QUERY_TEXT));

    // Verify summary text is suppressed for UNIX pipelines
    assert!(!stdout_str.contains("検索完了:"));
    assert!(!stdout_str.contains("Files scanned:"));
}

/// Verifies short options (-q, -p, -o) and automatic format inference from .xlsx extension.
///
/// ## Arguments / Returns
/// Verifies exit codes and generated Excel spreadsheet contents.
///
/// ## Errors
/// Panics if inference fails or output Excel cannot be opened.
#[test]
fn supports_short_options_and_format_inference() {
    let root = repo_root();
    let fixture = root.join("tests/fixtures/sample_report.xlsx");
    let output_dir = unique_test_directory();
    fs::create_dir_all(&output_dir).expect("temporary test directory must be created");
    let xlsx_output = output_dir.join("inferred.xlsx");

    let result = Command::new(env!("CARGO_BIN_EXE_xlseek-cli"))
        .arg("-q")
        .arg(QUERY_TEXT)
        .arg("-p")
        .arg(&fixture)
        .arg("-o")
        .arg(&xlsx_output)
        .output()
        .expect("CLI process must start");

    assert_eq!(result.status.code(), Some(0));
    assert!(xlsx_output.exists());

    // Verify summary is output to stdout when exporting to file
    let stdout_str = String::from_utf8_lossy(&result.stdout);
    assert!(stdout_str.contains("検索完了:"));

    let mut workbook = calamine::open_workbook_auto(&xlsx_output).expect("Excel output must open");
    let sheet_name = workbook.sheet_names().first().cloned().unwrap();
    let range = workbook.worksheet_range(&sheet_name).unwrap();
    assert!(range
        .rows()
        .flatten()
        .any(|cell| cell.to_string().contains(QUERY_TEXT)));

    let _ = fs::remove_dir_all(output_dir);
}

/// Verifies that errors are reported in real time to stderr during async pipelined traversal.
///
/// ## Arguments / Returns
/// Verifies stderr output and partial success exit code 1.
///
/// ## Errors
/// Panics if broken file is not notified or exit code is not 1.
#[test]
fn notifies_errors_in_realtime_during_async_pipeline() {
    let root = repo_root();
    let fixture = root.join("tests/fixtures/sample_report.xlsx");
    let work = unique_test_directory();
    let mixed_dir = work.join("mixed");
    fs::create_dir_all(&mixed_dir).unwrap();

    let broken_file = mixed_dir.join(TEST_BROKEN_WORKBOOK_NAME);
    fs::write(
        &broken_file,
        xlseek_cli::constants::CLI_TEST_INVALID_WORKBOOK_BYTES,
    )
    .unwrap();
    fs::copy(
        &fixture,
        mixed_dir.join(xlseek_cli::constants::CLI_TEST_VALID_WORKBOOK_NAME),
    )
    .unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_xlseek-cli"))
        .arg(QUERY_TEXT)
        .arg(&mixed_dir)
        .output()
        .expect("CLI process must start");

    // Partial success exit code 1
    assert_eq!(result.status.code(), Some(1));

    // Error for broken file reported to stderr
    let stderr_str = String::from_utf8_lossy(&result.stderr);
    assert!(stderr_str.contains(TEST_BROKEN_WORKBOOK_NAME));

    // Matches from valid file present in stdout
    let stdout_str = String::from_utf8_lossy(&result.stdout);
    assert!(stdout_str.contains(QUERY_TEXT));

    let _ = fs::remove_dir_all(work);
}

/// Verifies that the CLI accepts multiple comma-separated paths via --path / -p.
#[test]
fn test_cli_search_multiple_paths_in_option() {
    let work = unique_test_directory();
    let dir_a = work.join("dir_a");
    let dir_b = work.join("dir_b");
    fs::create_dir_all(&dir_a).unwrap();
    fs::create_dir_all(&dir_b).unwrap();

    let fixture = repo_root().join("tests/fixtures/sample_report.xlsx");
    fs::copy(&fixture, dir_a.join("sample_a.xlsx")).unwrap();
    fs::copy(&fixture, dir_b.join("sample_b.xlsx")).unwrap();

    let output_csv = work.join("output.csv");
    let path_arg = format!("{}, {}", dir_a.display(), dir_b.display());

    let result = Command::new(env!("CARGO_BIN_EXE_xlseek-cli"))
        .arg("-p")
        .arg(&path_arg)
        .arg("-q")
        .arg(QUERY_TEXT)
        .arg("-o")
        .arg(&output_csv)
        .output()
        .expect("CLI process must start");

    assert_eq!(result.status.code(), Some(0));
    assert!(output_csv.exists());

    let content = fs::read_to_string(&output_csv).unwrap();
    assert!(content.contains("sample_a.xlsx"));
    assert!(content.contains("sample_b.xlsx"));

    let _ = fs::remove_dir_all(work);
}

/// Verifies that running CLI search with overlapping parent and child paths produces zero duplicate matches.
///
/// ## Arguments / Returns
/// None. Invokes CLI with parent and subpath arguments and asserts scanned files count and match deduplication.
///
/// ## Errors
/// Panics if CLI invocation fails, exit code is non-zero, or duplicate entries exist.
#[test]
fn test_cli_search_overlapping_paths_deduplication() {
    let work = unique_test_directory();
    let parent_dir = work.join("parent");
    let child_dir = parent_dir.join("child");
    fs::create_dir_all(&child_dir).unwrap();

    let fixture = repo_root().join("tests/fixtures/sample_report.xlsx");
    fs::copy(&fixture, parent_dir.join("sample_parent.xlsx")).unwrap();
    fs::copy(&fixture, child_dir.join("sample_child.xlsx")).unwrap();

    let output_csv = work.join("output.csv");
    let path_arg = format!("{}, {}", parent_dir.display(), child_dir.display());

    let result = Command::new(env!("CARGO_BIN_EXE_xlseek-cli"))
        .arg("-p")
        .arg(&path_arg)
        .arg("-q")
        .arg(QUERY_TEXT)
        .arg("-o")
        .arg(&output_csv)
        .output()
        .expect("CLI process must start");

    assert_eq!(result.status.code(), Some(0));
    assert!(output_csv.exists());

    let stdout_str = String::from_utf8_lossy(&result.stdout);
    // Summary contains scanned files count; since child is pruned, scanned files must be 2, not 3
    assert!(stdout_str.contains("2"));

    let content = fs::read_to_string(&output_csv).unwrap();
    let parent_matches = content.matches("sample_parent.xlsx").count();
    let child_matches = content.matches("sample_child.xlsx").count();
    assert!(parent_matches > 0);
    assert_eq!(parent_matches, child_matches);

    let _ = fs::remove_dir_all(work);
}

/// Tests that CLI exports search results to a Markdown (.md) file with -f md and via extension inference.
///
/// ## Arguments / Returns
/// No arguments. Returns void.
///
/// ## Errors
/// Panics if CLI fails or exported markdown file is missing or invalid.
#[test]
fn test_cli_export_md_file_and_inference() {
    let root = repo_root();
    let fixture = root.join("tests/fixtures/sample_report.xlsx");
    let output_dir = unique_test_directory();
    fs::create_dir_all(&output_dir).expect("temporary test directory must be created");

    // 1. Explicit -f md
    let output_md = output_dir.join("result_explicit.md");
    let res = Command::new(env!("CARGO_BIN_EXE_xlseek-cli"))
        .arg("--path")
        .arg(&fixture)
        .arg("--query")
        .arg(QUERY_TEXT)
        .arg("--format")
        .arg("md")
        .arg("--output")
        .arg(&output_md)
        .output()
        .expect("CLI process must start");
    assert_eq!(res.status.code(), Some(0));
    let content = fs::read_to_string(&output_md).expect("Read exported markdown");
    assert!(content.contains(QUERY_TEXT));

    // 2. Extension inference with -o result_inferred.md (no -f)
    let output_inferred = output_dir.join("result_inferred.md");
    let res2 = Command::new(env!("CARGO_BIN_EXE_xlseek-cli"))
        .arg("--path")
        .arg(&fixture)
        .arg("--query")
        .arg(QUERY_TEXT)
        .arg("--output")
        .arg(&output_inferred)
        .output()
        .expect("CLI process must start");
    assert_eq!(res2.status.code(), Some(0));
    let content2 = fs::read_to_string(&output_inferred).expect("Read inferred markdown");
    assert!(content2.contains(QUERY_TEXT));

    let _ = fs::remove_dir_all(output_dir);
}

/// Tests that CLI exports search results to a JSON (.json) file with -f json and via extension inference.
///
/// ## Arguments / Returns
/// No arguments. Returns void.
///
/// ## Errors
/// Panics if CLI fails or exported JSON file cannot be deserialized.
#[test]
fn test_cli_export_json_file_and_inference() {
    let root = repo_root();
    let fixture = root.join("tests/fixtures/sample_report.xlsx");
    let output_dir = unique_test_directory();
    fs::create_dir_all(&output_dir).expect("temporary test directory must be created");

    // 1. Explicit -f json
    let output_json = output_dir.join("result_explicit.json");
    let res = Command::new(env!("CARGO_BIN_EXE_xlseek-cli"))
        .arg("--path")
        .arg(&fixture)
        .arg("--query")
        .arg(QUERY_TEXT)
        .arg("--format")
        .arg("json")
        .arg("--output")
        .arg(&output_json)
        .output()
        .expect("CLI process must start");
    assert_eq!(res.status.code(), Some(0));
    let content = fs::read_to_string(&output_json).expect("Read exported json");
    assert!(content.contains("\"total_matches\":"));
    assert!(content.contains("\"matches\":"));
    assert!(content.contains(QUERY_TEXT));

    // 2. Extension inference with -o result_inferred.json (no -f)
    let output_inferred = output_dir.join("result_inferred.json");
    let res2 = Command::new(env!("CARGO_BIN_EXE_xlseek-cli"))
        .arg("--path")
        .arg(&fixture)
        .arg("--query")
        .arg(QUERY_TEXT)
        .arg("--output")
        .arg(&output_inferred)
        .output()
        .expect("CLI process must start");
    assert_eq!(res2.status.code(), Some(0));
    let content2 = fs::read_to_string(&output_inferred).expect("Read inferred json");
    assert!(content2.contains("\"total_matches\":"));
    assert!(content2.contains("\"matches\":"));
    assert!(content2.contains(QUERY_TEXT));

    let _ = fs::remove_dir_all(output_dir);
}

/// Tests that CLI streams Markdown or JSON directly to stdout when --output is omitted.
///
/// ## Arguments / Returns
/// No arguments. Returns void.
///
/// ## Errors
/// Panics if CLI stdout streaming fails or output does not match expected format.
#[test]
fn test_cli_stdout_streaming_md_and_json() {
    let root = repo_root();
    let fixture = root.join("tests/fixtures/sample_report.xlsx");

    // 1. Stdout streaming for Markdown
    let md_res = Command::new(env!("CARGO_BIN_EXE_xlseek-cli"))
        .arg("--path")
        .arg(&fixture)
        .arg("--query")
        .arg(QUERY_TEXT)
        .arg("--format")
        .arg("md")
        .output()
        .expect("CLI process must start");
    assert_eq!(md_res.status.code(), Some(0));
    let md_stdout = String::from_utf8_lossy(&md_res.stdout);
    assert!(md_stdout.contains(QUERY_TEXT));

    // 2. Stdout streaming for JSON
    let json_res = Command::new(env!("CARGO_BIN_EXE_xlseek-cli"))
        .arg("--path")
        .arg(&fixture)
        .arg("--query")
        .arg(QUERY_TEXT)
        .arg("--format")
        .arg("json")
        .output()
        .expect("CLI process must start");
    assert_eq!(json_res.status.code(), Some(0));
    let json_stdout = String::from_utf8_lossy(&json_res.stdout);
    assert!(json_stdout.contains("\"total_matches\":"));
    assert!(json_stdout.contains("\"matches\":"));
    assert!(json_stdout.contains(QUERY_TEXT));
}

/// ## Description
/// Tests that streaming CSV to stdout replaces newlines in multiline cells with spaces,
/// guaranteeing strictly 1 physical line per match.
///
/// ## Arguments / Returns
/// Validates process exit code, stdout line count, and sanitized field text.
///
/// ## Errors
/// Panics if CLI fails or stdout does not match the 1-record-per-line guarantee.
#[test]
fn test_cli_stdout_csv_multiline_cells() {
    let temp_dir = unique_test_directory();
    fs::create_dir_all(&temp_dir).expect("create temp dir");
    let workbook_path = temp_dir.join("multiline_sample.xlsx");

    let mut workbook = rust_xlsxwriter::Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.write_string(0, 0, "ColA").expect("write header");
    worksheet
        .write_string(1, 0, "Multiline\nTarget\r\nCell")
        .expect("write cell");
    workbook.save(&workbook_path).expect("save workbook");

    let res = Command::new(env!("CARGO_BIN_EXE_xlseek-cli"))
        .arg("--path")
        .arg(&workbook_path)
        .arg("--query")
        .arg("Target")
        .arg("--format")
        .arg("csv")
        .output()
        .expect("CLI process must start");

    assert_eq!(res.status.code(), Some(0));
    let stdout_str = String::from_utf8(res.stdout).expect("Valid UTF-8 stdout");
    let lines: Vec<&str> = stdout_str.trim_end().split('\n').collect();

    // 1 header line + 1 match line = 2 lines total
    assert_eq!(lines.len(), 2);
    assert!(lines[1].contains("Multiline Target Cell"));

    let _ = fs::remove_dir_all(temp_dir);
}

/// Helper that invokes the CLI with path, query, format, and output arguments.
///
/// ## Arguments / Returns
/// Returns process `Output`.
///
/// ## Errors
/// Panics if CLI fails to launch.
fn run_cli(
    input: &std::path::Path,
    query: &str,
    output: &std::path::Path,
    format: &str,
) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_xlseek-cli"))
        .arg("--path")
        .arg(input)
        .arg("--query")
        .arg(query)
        .arg("--format")
        .arg(format)
        .arg("--output")
        .arg(output)
        .output()
        .expect("CLI process must start")
}
