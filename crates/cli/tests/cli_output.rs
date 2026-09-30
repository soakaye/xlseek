//! # CLI Output Integration Tests
//!
//! ## Description
//! Verifies CLI output publication overwrite policies and input file safety using actual process execution.
//!
//! ## Arguments / Returns
//! Executes Cargo-provided CLI binary and verifies process exit codes and target file contents.
//!
//! ## Errors
//! Fails test if I/O, execution, or assertions fail.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

const TEST_DIRECTORY_PREFIX: &str = "xlseek-cli-output";
const TEST_EXISTING_CONTENT: &str = "preserve-existing-output";
const TEST_QUERY: &str = "Financial Report Q3";
const TEST_OUTPUT_NAME: &str = "results.csv";

/// Resolves the absolute path to the repository root.
///
/// ## Arguments / Returns
/// Takes no arguments; returns `PathBuf`.
///
/// ## Errors
/// Panics if repository root cannot be determined.
fn repo_root() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    if manifest.join("../../tests/fixtures").exists() {
        manifest.join("../..")
    } else {
        manifest
            .parent()
            .expect("repository root must exist")
            .to_path_buf()
    }
}

/// Verifies default refusal of existing outputs, explicit overwrite behavior, and identity protection against overwriting inputs.
///
/// ## Arguments / Returns
/// Validates temporary test paths and process exit statuses.
///
/// ## Errors
/// Fails test on file manipulation failure or unexpected content modification.
#[test]
fn rejects_existing_output_by_default_and_never_overwrites_input() {
    let root = repo_root();
    let fixture = root.join("tests/fixtures/sample_report.xlsx");
    let directory =
        std::env::temp_dir().join(format!("{TEST_DIRECTORY_PREFIX}-{}", std::process::id()));
    fs::create_dir_all(&directory).expect("temporary directory must be created");
    let output = directory.join(TEST_OUTPUT_NAME);
    fs::write(&output, TEST_EXISTING_CONTENT).expect("existing output must be created");

    let denied = run_cli(&fixture, TEST_QUERY, &output, false);
    assert_eq!(
        denied.status.code(),
        Some(xlseek_cli::constants::CLI_EXIT_FAILURE)
    );
    assert_eq!(fs::read_to_string(&output).unwrap(), TEST_EXISTING_CONTENT);

    let replaced = run_cli(&fixture, TEST_QUERY, &output, true);
    assert_eq!(
        replaced.status.code(),
        Some(xlseek_cli::constants::CLI_EXIT_SUCCESS)
    );
    assert!(fs::read_to_string(&output).unwrap().contains(TEST_QUERY));

    let original_input = fs::read(&fixture).expect("input workbook must be readable");
    let input_output = run_cli(&fixture, TEST_QUERY, &fixture, true);
    assert_eq!(
        input_output.status.code(),
        Some(xlseek_cli::constants::CLI_EXIT_FAILURE)
    );
    assert_eq!(fs::read(&fixture).unwrap(), original_input);
    let _ = fs::remove_dir_all(directory);
}

/// Executes a single CLI invocation with the provided arguments.
///
/// ## Arguments / Returns
/// Accepts input path, query, output path, and overwrite flag, returning `std::process::Output`.
///
/// ## Errors
/// Panics if CLI process fails to launch.
fn run_cli(
    input: &std::path::Path,
    query: &str,
    output: &std::path::Path,
    overwrite: bool,
) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_xlseek-cli"));
    command
        .arg("--path")
        .arg(input)
        .arg("--query")
        .arg(query)
        .arg("--format")
        .arg(
            if output.extension().and_then(|value| value.to_str())
                == Some(xlseek_cli::constants::CLI_FORMAT_XLSX)
            {
                xlseek_cli::constants::CLI_FORMAT_XLSX
            } else {
                xlseek_cli::constants::CLI_FORMAT_CSV
            },
        )
        .arg("--output")
        .arg(output);
    if overwrite {
        command.arg(xlseek_cli::constants::CLI_TEST_OVERWRITE_OPTION);
    }
    command.output().expect("CLI process must start")
}
