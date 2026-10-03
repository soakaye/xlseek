//! # Data Models Module (models/mod.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Defines shared data structures for search criteria, search matches, scanning progress,
//! spreadsheet preview information, export requests, and external application associations.
//! Conforms to Constitution Principle II (No Hardcoded Constants) and Principle III (Comprehensive Header Comments).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Query structure holding search execution parameters and options.
///
/// ## Arguments / Returns
/// Holds search keyword, target folder path, matching flags, and targeted file extensions.
///
/// ## Errors
/// Fails Serde deserialization if mandatory keyword or target directory are missing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub keyword: String,
    pub target_dir: String,
    #[serde(default)]
    pub directory_mode: DirectorySearchMode,
    #[serde(default)]
    pub burst_workers: Option<usize>,
    #[serde(default)]
    pub match_case: bool,
    #[serde(default = "default_include_value")]
    pub include_value: bool,
    #[serde(default)]
    pub use_regex: bool,
    #[serde(default = "default_true")]
    pub include_formula: bool,
    #[serde(default = "default_true")]
    pub include_comment: bool,
    #[serde(default = "default_true")]
    pub include_shape: bool,
    #[serde(default)]
    pub include_hidden: bool,
    #[serde(default = "default_extensions")]
    pub extensions: Vec<String>,
}

/// Returns the backward-compatible default value for value search when omitted in legacy JSON.
///
/// ## Arguments / Returns
/// Takes no arguments; returns boolean `DEFAULT_INCLUDE_VALUE`.
///
/// ## Errors
/// Does not panic or fail.
fn default_include_value() -> bool {
    // Constant reference: crate::constants::DEFAULT_INCLUDE_VALUE
    crate::constants::DEFAULT_INCLUDE_VALUE
}

/// Helper function providing boolean `true` as default for Serde deserialization.
///
/// ## Arguments / Returns
/// None. Returns `true`.
///
/// ## Errors
/// Does not panic.
fn default_true() -> bool {
    true
}

/// Returns default Excel extensions (.xlsx, .xlsm, .xlsb, .xls) for search.
///
/// ## Arguments / Returns
/// None. Returns `Vec<String>` of default extensions.
///
/// ## Errors
/// Does not panic.
fn default_extensions() -> Vec<String> {
    // Constant reference: crate::constants::DEFAULT_EXTENSIONS
    crate::constants::DEFAULT_EXTENSIONS
        .iter()
        .map(|&ext| ext.to_string())
        .collect()
}

/// Element type indicating where a search match hit occurred.
///
/// ## Arguments / Returns
/// Variants represent cell value, formula, comment, hidden sheet, or drawing shape.
///
/// ## Errors
/// Deserialization fails if an unknown variant is encountered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatchType {
    CellValue,
    Formula,
    Comment,
    HiddenSheet,
    Shape,
}

/// Result structure storing matched cell or drawing shape item.
///
/// ## Arguments / Returns
/// Holds match metadata, row/col index, sheet visibility, snippet, and optional formula.
///
/// ## Errors
/// Simple data carrier; does not generate runtime errors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMatch {
    pub id: u64,
    pub file_name: String,
    pub full_path: String,
    pub sheet_name: String,
    pub cell_address: String,
    pub row_index: u32,
    pub col_index: u32,
    pub col_name: String,
    pub match_type: MatchType,
    pub shape_name: Option<String>,
    pub sheet_hidden: bool,
    pub snippet: String,
    pub full_content: String,
    pub formula: Option<String>,
    pub sheets_in_workbook: Vec<String>,
}

/// Holds search matches and partial non-fatal issues encountered in a single workbook file.
#[derive(Debug, Default)]
pub struct FileSearchResult {
    pub matches: Vec<SearchMatch>,
    pub issues: Vec<SearchIssue>,
}

/// Records non-fatal errors or warnings encountered during discovery, workbook parsing, or sheet extraction.
#[derive(Debug)]
pub struct SearchIssue {
    pub path: std::path::PathBuf,
    pub stage: String,
    pub sheet_name: Option<String>,
    pub cause: String,
}

/// Summary report aggregating discovered files, scanned counts, matches found, elapsed time, and issues.
#[derive(Debug, Default)]
pub struct SearchReport {
    pub discovered_files: usize,
    pub scanned_files: usize,
    pub readable_files: usize,
    pub failed_files: usize,
    pub matches_found: usize,
    pub elapsed_ms: u64,
    pub issues: Vec<SearchIssue>,
}

/// Status enumeration representing current state of search scanning engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScanState {
    Scanning,
    Completed,
    Cancelled,
    Error,
}

/// Locale-neutral scan phase identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScanPhase {
    Preparing,
    Discovering,
    Scanning,
    Finished,
}

/// Stable error code identifiers for translatable client responses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    InvalidRegex,
    PathNotFound,
    WorkbookOpenFailed,
    PreviewFailed,
    ExportFailed,
    AppLaunchFailed,
    FolderOpenFailed,
    PermissionDenied,
    SearchFailed,
    InternalError,
}

/// Structured error payload returned by Tauri commands.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct CommandError {
    pub code: ErrorCode,
}

impl From<String> for CommandError {
    fn from(_: String) -> Self {
        Self {
            code: ErrorCode::InternalError,
        }
    }
}

/// Scan progress notification payload emitted during search execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    pub state: ScanState,
    pub phase: ScanPhase,
    pub error_code: Option<ErrorCode>,
    pub scanned_files: usize,
    pub total_files: usize,
    pub matches_found: usize,
    pub current_file: String,
    pub elapsed_ms: u64,
}

/// Grid preview data returned for surrounding cell visualization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellPreviewData {
    pub target_row: u32,
    pub target_col: u32,
    pub columns: Vec<PreviewColumn>,
    pub rows: Vec<PreviewRow>,
    pub sheets_in_workbook: Vec<String>,
}

/// Column header metadata for spreadsheet preview grid.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewColumn {
    pub key: String,
    pub label: String,
}

/// Single row representation within the preview grid.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewRow {
    pub row_number: u32,
    pub cells: HashMap<String, CellValueInfo>,
}

/// Cell value and target match status for preview cells.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellValueInfo {
    pub value: String,
    pub is_target: bool,
    pub formula: Option<String>,
}

/// Export format options (CSV or Excel).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    Csv,
    Xlsx,
}

/// Export request parameter payload from front-end client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRequest {
    pub format: ExportFormat,
    #[serde(default = "default_export_language")]
    pub language: String,
    pub output_path: String,
    pub items: Vec<SearchMatch>,
}

/// Default language fallback for legacy export requests.
fn default_export_language() -> String {
    crate::constants::DEFAULT_EXPORT_LANGUAGE.to_string()
}

/// Information model representing a supported external application (Excel, Numbers, Calc).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportedApp {
    pub id: String,
    pub name: String,
    pub executable_path: String,
    pub is_default: bool,
    pub icon_hint: Option<String>,
}

#[cfg(test)]
mod localization_tests {
    use super::{
        CommandError, ErrorCode, ExportRequest, ScanPhase, ScanProgress, ScanState, SearchQuery,
    };

    /// Verifies that progress phases and error codes serialize to language-neutral snake_case.
    #[test]
    fn serializes_language_neutral_codes() {
        assert_eq!(
            serde_json::to_string(&ScanPhase::Discovering).unwrap(),
            "\"discovering\""
        );
        assert_eq!(
            serde_json::to_string(&ErrorCode::InternalError).unwrap(),
            "\"internal_error\""
        );
        assert_eq!(
            serde_json::to_string(&CommandError {
                code: ErrorCode::InternalError
            })
            .unwrap(),
            "{\"code\":\"internal_error\"}"
        );
        let progress = ScanProgress {
            state: ScanState::Scanning,
            phase: ScanPhase::Discovering,
            error_code: None,
            scanned_files: 0,
            total_files: 0,
            matches_found: 0,
            current_file: String::new(),
            elapsed_ms: 0,
        };
        let serialized = serde_json::to_value(progress).unwrap();
        assert_eq!(serialized["phase"], "discovering");
        assert_eq!(serialized["current_file"], "");
        assert!(serialized["error_code"].is_null());
    }

    /// Verifies export request language code resolution and default English fallback.
    #[test]
    fn export_request_reads_language_and_defaults_legacy_requests_to_english() {
        let request: ExportRequest =
            serde_json::from_str(r#"{"format":"csv","output_path":"","items":[],"language":"ja"}"#)
                .unwrap();
        assert_eq!(request.language, "ja");
        let legacy: ExportRequest =
            serde_json::from_str(r#"{"format":"csv","output_path":"","items":[]}"#).unwrap();
        assert_eq!(legacy.language, crate::constants::DEFAULT_EXPORT_LANGUAGE);
    }

    /// Verifies that legacy JSON requests default value search to true while preserving explicit false.
    #[test]
    fn legacy_search_query_defaults_value_search_to_true() {
        let legacy: SearchQuery = serde_json::from_value(serde_json::json!({
            "keyword": "needle",
            "target_dir": "input",
        }))
        .unwrap();
        let serialized = serde_json::to_value(legacy).unwrap();
        assert_eq!(
            serialized[crate::constants::SEARCH_QUERY_INCLUDE_VALUE_FIELD],
            true
        );

        let explicit: SearchQuery = serde_json::from_value(serde_json::json!({
            "keyword": "needle",
            "target_dir": "input",
            (crate::constants::SEARCH_QUERY_INCLUDE_VALUE_FIELD): false,
        }))
        .unwrap();
        let serialized = serde_json::to_value(explicit).unwrap();
        assert_eq!(
            serialized[crate::constants::SEARCH_QUERY_INCLUDE_VALUE_FIELD],
            false
        );
    }

    /// Verifies legacy query JSON receives Sequential mode and Automatic count defaults.
    ///
    /// ## Arguments / Returns
    /// Deserializes a legacy query and checks its backward-compatible discovery settings.
    ///
    /// ## Errors
    /// Panics if the legacy query cannot deserialize or defaults differ.
    #[test]
    fn legacy_query_defaults_to_sequential_automatic_discovery() {
        let legacy: SearchQuery =
            serde_json::from_str(r#"{"keyword":"needle","target_dir":"input"}"#).unwrap();
        let serialized = serde_json::to_value(legacy).unwrap();
        assert_eq!(serialized["directory_mode"], "sequential");
        assert!(serialized["burst_workers"].is_null());
    }
}

/// Selects serial or bounded concurrent directory discovery.
///
/// ## Arguments / Returns
/// `Sequential` visits one directory at a time; `Burst` permits bounded overlap.
/// Serde uses stable lowercase values for IPC and saved request compatibility.
///
/// ## Errors
/// Deserialization returns an error for an unknown explicit mode.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DirectorySearchMode {
    #[default]
    Sequential,
    Burst,
}

/// Resolves Automatic or validates a custom directory visitor count.
///
/// ## Arguments / Returns
/// `requested` is an optional custom count; returns the bounded effective count.
///
/// ## Errors
/// Returns `Err` for counts outside the configured custom bounds or unavailable parallelism.
pub fn resolve_burst_workers(requested: Option<usize>) -> Result<usize, String> {
    match requested {
        Some(count)
            if (crate::constants::BURST_WORKERS_MIN..=crate::constants::BURST_WORKERS_MAX)
                .contains(&count) =>
        {
            Ok(count)
        }
        Some(_) => Err(crate::constants::ERR_CLI_DIRECTORY_MODE.to_string()),
        None => std::thread::available_parallelism()
            .map(|available| {
                available.get().clamp(
                    crate::constants::BURST_WORKERS_MIN,
                    crate::constants::BURST_WORKERS_AUTO_MAX,
                )
            })
            .map_err(|_| crate::constants::ERR_DISCOVERY_WORKER_COUNT.to_string()),
    }
}

/// Tests custom count boundaries and Automatic resolution against shared constants.
///
/// ## Arguments / Returns
/// No arguments; returns `()` after checking supported and rejected counts.
///
/// ## Errors
/// Panics if count resolution violates the configured bounds.
#[cfg(test)]
#[test]
fn burst_worker_resolution_enforces_custom_bounds_and_caps_automatic() {
    assert_eq!(
        resolve_burst_workers(Some(crate::constants::BURST_WORKERS_MIN)).unwrap(),
        crate::constants::BURST_WORKERS_MIN
    );
    assert_eq!(
        resolve_burst_workers(Some(crate::constants::BURST_WORKERS_MAX)).unwrap(),
        crate::constants::BURST_WORKERS_MAX
    );
    assert!(resolve_burst_workers(Some(crate::constants::BURST_WORKERS_MIN - 1)).is_err());
    assert!(resolve_burst_workers(Some(crate::constants::BURST_WORKERS_MAX + 1)).is_err());
    let automatic = resolve_burst_workers(None).unwrap();
    assert!(
        (crate::constants::BURST_WORKERS_MIN..=crate::constants::BURST_WORKERS_AUTO_MAX)
            .contains(&automatic)
    );
}
