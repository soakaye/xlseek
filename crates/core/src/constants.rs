//! # Core Constants Module (constants.rs)
//!
//! ## Description
//! Centralizes constant definitions, configuration parameters, supported file extensions,
//! event identifiers, and localized error messages across the application.
//! Conforms to Constitution Principle II (No Hardcoded Constants).
//!
//! ## Arguments / Returns
//! Exposes public constant values used across search, extraction, export, and preview engines.
//!
//! ## Errors
//! As a constant definition module, this does not produce runtime errors.

// ==============================================================================
// 1. File Extensions
// ==============================================================================

/// Default list of target Excel extensions
pub const DEFAULT_EXTENSIONS: [&str; 4] = [".xlsx", ".xlsm", ".xlsb", ".xls"];
/// Backward-compatible default setting for enabling cell value search in SearchQuery.
pub const DEFAULT_INCLUDE_VALUE: bool = true;
/// Field name for value search option in SearchQuery JSON.
pub const SEARCH_QUERY_INCLUDE_VALUE_FIELD: &str = "include_value";
/// Canonical Sequential directory discovery token.
pub const DIRECTORY_MODE_SEQUENTIAL: &str = "sequential";
/// Canonical concurrent Burst directory discovery token.
pub const DIRECTORY_MODE_BURST: &str = "burst";
/// Minimum Burst directory visitor count for Automatic resolution and custom validation.
pub const BURST_WORKERS_MIN: usize = 2;
/// Maximum Automatic Burst directory visitor count.
pub const BURST_WORKERS_AUTO_MAX: usize = 8;
/// Maximum custom Burst directory visitor count.
pub const BURST_WORKERS_MAX: usize = 32;
/// Maximum number of pending directory paths held by the discovery scheduler.
pub const DISCOVERY_QUEUE_CAPACITY: usize = 256;
/// Delay between cancellation-aware retries when delivering to a full file channel.
pub const DISCOVERY_RETRY_DELAY_MS: u64 = 10;
/// Number of directory visitors used by Sequential mode.
pub const DISCOVERY_SEQUENTIAL_WORKERS: usize = 1;
/// Outstanding scheduler tasks added or retired for one directory.
pub const DISCOVERY_ONE_TASK: usize = 1;
/// Fatal error returned when a discovery callback panics.
pub const ERR_DISCOVERY_CALLBACK_PANIC: &str = "Directory discovery callback panicked";
/// Fatal error returned when the bounded workbook input channel is disconnected.
pub const ERR_DISCOVERY_FILE_CHANNEL: &str = "Workbook file channel closed during discovery";
/// Fatal error returned when the scanner thread panics.
pub const ERR_DISCOVERY_WORKER_PANIC: &str = "Directory discovery worker panicked";
/// CLI error used when the workbook consumer has closed the bounded file channel.
pub const ERR_CLI_FILE_CHANNEL: &str = "Workbook file channel closed during discovery";
/// Wire field carrying the selected directory traversal mode.
pub const SEARCH_QUERY_DIRECTORY_MODE_FIELD: &str = "directory_mode";
/// Wire field carrying a custom Burst visitor count.
pub const SEARCH_QUERY_BURST_WORKERS_FIELD: &str = "burst_workers";
/// Descendant directory failure code for denied access.
pub const DISCOVERY_ERROR_PERMISSION_DENIED: &str = "permission_denied";
/// Descendant directory failure code for other read failures.
pub const DISCOVERY_ERROR_READ_FAILED: &str = "read_failed";
/// Long CLI field for directory traversal mode.
pub const CLI_DIRECTORY_MODE_FIELD: &str = "directory-mode";
/// Long CLI field for a custom Burst worker count.
pub const CLI_BURST_WORKERS_FIELD: &str = "burst-workers";
/// Error returned for invalid Burst count or conflicting traversal mode options.
pub const ERR_CLI_DIRECTORY_MODE: &str = "Invalid directory mode or Burst worker count";
/// Error returned if no directory visitor count can be determined.
pub const ERR_DISCOVERY_WORKER_COUNT: &str = "Could not determine a valid directory visitor count";
/// Stable fixture identifier used by shared-core discovery tests.
pub const CLI_TEST_BURST_FIXTURE_ID: &str = "shared-tree";
/// Number of workbook paths in the shared-core nested discovery fixture.
pub const CLI_TEST_NESTED_FILE_COUNT: usize = 4;
/// Number of sibling fixture folders used by the parallel discovery test.
pub const CLI_TEST_BURST_DIRECTORY_COUNT: usize = 8;
/// Delay used by the parallel discovery test callback to expose overlap.
pub const CLI_TEST_BURST_CALLBACK_DELAY_MS: u64 = 20;
/// Expected minimum simultaneous file callbacks for a two-worker Burst test.
pub const CLI_TEST_BURST_MIN_CONCURRENCY: usize = 2;
pub const CLI_QUERY_FIELD: &str = "query";
pub const CLI_PATH_FIELD: &str = "path";
pub const CLI_FORMAT_FIELD: &str = "format";
pub const CLI_OUTPUT_FIELD: &str = "output";
pub const CLI_MATCH_CASE_FIELD: &str = "match-case";
pub const CLI_REGEX_FIELD: &str = "regex";
pub const CLI_VALUES_FIELD: &str = "values";
pub const CLI_FORMULAS_FIELD: &str = "formulas";
pub const CLI_COMMENTS_FIELD: &str = "comments";
pub const CLI_SHAPES_FIELD: &str = "shapes";
pub const CLI_HIDDEN_SHEETS_FIELD: &str = "hidden-sheets";
pub const CLI_EXTENSIONS_FIELD: &str = "extensions";
pub const CLI_LANGUAGE_FIELD: &str = "language";
pub const CLI_OVERWRITE_FIELD: &str = "overwrite";
pub const CLI_HELP_FIELD: &str = "help";
pub const CLI_DEFAULT_LANGUAGE: &str = "ja";
pub const CLI_DEFAULT_INCLUDE_FORMULA: bool = true;
pub const CLI_DEFAULT_INCLUDE_COMMENT: bool = true;
pub const CLI_DEFAULT_INCLUDE_SHAPE: bool = false;
pub const CLI_DEFAULT_INCLUDE_HIDDEN: bool = false;
pub const CLI_FORMAT_CSV: &str = "csv";
pub const CLI_FORMAT_XLSX: &str = "xlsx";
pub const CLI_OUTPUT_SUFFIX: &str = ".tmp";
pub const CLI_TEMP_DIRECTORY_PREFIX: &str = ".xlseek-export-";
pub const CLI_TEMP_FILE_NAME: &str = "result.tmp";
pub const CLI_TEMP_CREATE_ATTEMPTS: u32 = 16;
pub const CLI_EXIT_SUCCESS: i32 = 0;
pub const CLI_EXIT_PARTIAL: i32 = 1;
pub const CLI_EXIT_FAILURE: i32 = 2;
pub const CLI_KEY_PREFIX: &str = "cli.";
pub const CLI_LOCALE_JA: &str = "ja";
pub const CLI_LOCALE_EN: &str = "en";
pub const LOCALE_METADATA_VERSION_KEY: &str = "_version";
pub const ERR_CATALOG_INVALID: &str = "Invalid embedded translation catalog";
pub const CLI_TRANSLATION_HELP_KEY: &str = "cli.HELP";
pub const CLI_TRANSLATION_SUMMARY_KEY: &str = "cli.SUMMARY";
pub const CLI_TRANSLATION_ISSUES_KEY: &str = "cli.ISSUES";
pub const CLI_TRANSLATION_ERROR_KEY: &str = "cli.ERROR";
pub const CLI_REQUIRED_TRANSLATION_KEYS: [&str; 11] = [
    CLI_TRANSLATION_HELP_KEY,
    "cli.ERROR",
    "cli.SUMMARY",
    "cli.ISSUES",
    "cli.ERROR_REQUIRED",
    "cli.ERROR_UNKNOWN",
    "cli.ERROR_DUPLICATE",
    "cli.ERROR_VALUE",
    "cli.ERROR_PATH",
    "cli.ERROR_OUTPUT",
    "cli.ERROR_NO_TYPES",
];
pub const CLI_OPTION_PREFIX: &str = "--";
pub const CLI_ASSIGNMENT_SEPARATOR: &str = "=";
pub const CLI_LIST_SEPARATOR: char = ',';
pub const CLI_EXTENSION_PREFIX: &str = ".";
pub const CLI_SHORT_HELP: &str = "-h";
pub const CLI_LONG_HELP: &str = "--help";
pub const CLI_BOOLEAN_TRUE: &str = "true";
pub const CLI_BOOLEAN_FALSE: &str = "false";
pub const CLI_DEFAULT_FALSE: bool = false;
pub const CLI_CURRENT_DIRECTORY: &str = ".";
pub const HOME_ENVIRONMENT_VARIABLE: &str = "HOME";
pub const WINDOWS_HOME_ENVIRONMENT_VARIABLE: &str = "USERPROFILE";
pub const ERR_CLI_NON_UNICODE: &str = "CLI arguments must be valid Unicode";
pub const ERR_CLI_DUPLICATE: &str = "Duplicate CLI option";
pub const ERR_CLI_UNKNOWN: &str = "Unknown CLI option";
pub const ERR_CLI_REQUIRED: &str = "Missing required CLI option";
pub const ERR_CLI_VALUE: &str = "Invalid CLI option value";
pub const ERR_CLI_HELP_OPTIONS: &str = "Help accepts only --language ja|en";
pub const ERR_CLI_FORMAT: &str = "Output format and extension must match";
pub const ERR_CLI_EXTENSIONS: &str = "Invalid or empty Excel extension list";
pub const ERR_CLI_EMPTY_QUERY: &str = "Search query cannot be empty";
pub const ERR_CLI_NO_SEARCH_TYPES: &str = "At least one search type must be enabled";
pub const ERR_CLI_REGEX: &str = "Invalid regular expression";
pub const ERR_CLI_INPUT_PATH: &str = "Search input must exist and be a file or directory";
pub const ERR_CLI_OUTPUT_PATH: &str =
    "Output must be in an existing directory and not be a symlink";
pub const ERR_CLI_HOME: &str = "Could not resolve home directory";
pub const ERR_CLI_LANGUAGE: &str = "Language must be ja or en";
pub const ERR_CLI_EXISTS: &str = "Output already exists; use --overwrite to replace it";
pub const ERR_CLI_INPUT_COLLISION: &str = "Output must not alias a search input";
pub const ERR_CLI_OUTPUT_CHANGED: &str = "Output changed while the search was running";
pub const ERR_CLI_OUTPUT: &str = "Failed to publish output";
pub const ERR_CLI_TEMP_LIMIT: &str = "Could not create a unique temporary directory";
pub const CLI_SHORT_OPTION_PREFIX: &str = "-";
pub const CLI_OPTION_TERMINATOR: &str = "--";
pub const CLI_SHORT_PATH: &str = "-p";
pub const CLI_SHORT_QUERY: &str = "-q";
pub const CLI_SHORT_FORMAT: &str = "-f";
pub const CLI_SHORT_OUTPUT: &str = "-o";
pub const CLI_SHORT_MATCH_CASE: &str = "-c";
pub const CLI_SHORT_REGEX: &str = "-r";
pub const CLI_SHORT_OVERWRITE: &str = "-w";
pub const CLI_SHORT_LANGUAGE: &str = "-l";
pub const CLI_SHORT_EXTENSIONS: &str = "-e";

pub const CLI_SHORT_TO_LONG_OPTIONS: [(&str, &str); 10] = [
    (CLI_SHORT_PATH, CLI_PATH_FIELD),
    (CLI_SHORT_QUERY, CLI_QUERY_FIELD),
    (CLI_SHORT_FORMAT, CLI_FORMAT_FIELD),
    (CLI_SHORT_OUTPUT, CLI_OUTPUT_FIELD),
    (CLI_SHORT_MATCH_CASE, CLI_MATCH_CASE_FIELD),
    (CLI_SHORT_REGEX, CLI_REGEX_FIELD),
    (CLI_SHORT_OVERWRITE, CLI_OVERWRITE_FIELD),
    (CLI_SHORT_LANGUAGE, CLI_LANGUAGE_FIELD),
    (CLI_SHORT_EXTENSIONS, CLI_EXTENSIONS_FIELD),
    (CLI_SHORT_HELP, CLI_HELP_FIELD),
];

pub const CLI_VALUE_OPTIONS: [&str; 15] = [
    CLI_PATH_FIELD,
    CLI_QUERY_FIELD,
    CLI_FORMAT_FIELD,
    CLI_OUTPUT_FIELD,
    CLI_MATCH_CASE_FIELD,
    CLI_REGEX_FIELD,
    CLI_VALUES_FIELD,
    CLI_FORMULAS_FIELD,
    CLI_COMMENTS_FIELD,
    CLI_SHAPES_FIELD,
    CLI_HIDDEN_SHEETS_FIELD,
    CLI_EXTENSIONS_FIELD,
    CLI_LANGUAGE_FIELD,
    CLI_DIRECTORY_MODE_FIELD,
    CLI_BURST_WORKERS_FIELD,
];
pub const CLI_TEST_OUTPUT_NAME: &str = "xlseek-cli-args-test.csv";
pub const CLI_TEST_OVERWRITE_OPTION: &str = "--overwrite";
pub const CLI_TEST_HYPHEN_QUERY: &str = "--needle";
pub const CLI_TEST_COMMENT_QUERY: &str = "財務報告レビュー対象セル";
pub const CLI_TEST_INVALID_REGEX: &str = "[";
pub const CLI_TEST_INVALID_WORKBOOK_BYTES: &[u8] = b"broken workbook";
pub const CLI_TEST_VALID_WORKBOOK_NAME: &str = "valid.xlsx";
pub const CLI_TEST_EMPTY_OUTPUT_NAME: &str = "empty.csv";
pub const CLI_TEST_BROKEN_OUTPUT_NAME: &str = "all-broken.csv";
pub const CLI_TEST_PARTIAL_OUTPUT_NAME: &str = "partial.csv";
pub const CLI_TEST_REGEX_OUTPUT_NAME: &str = "invalid-regex.csv";
pub const CSV_UTF8_BOM: [u8; 3] = [0xEF, 0xBB, 0xBF];
pub const CLI_TEST_EXPECTED_HEADER_ID_JA: &str = "ID";
pub const CLI_TEST_ENGLISH_USAGE_PREFIX: &str = "Usage:";
pub const CLI_SUMMARY_SCANNED_PLACEHOLDER: &str = "{scanned}";
pub const CLI_SUMMARY_MATCHES_PLACEHOLDER: &str = "{matches}";
pub const CLI_SUMMARY_PATH_PLACEHOLDER: &str = "{path}";
pub const CLI_SUMMARY_COUNT_PLACEHOLDER: &str = "{count}";
pub const CLI_FALLBACK_ENGLISH: &str = "Error";
pub const CLI_ERROR_LABEL_EN: &str = "Error";
pub const SEARCH_STAGE_DISCOVERY: &str = "discovery";
pub const SEARCH_STAGE_WORKBOOK: &str = "workbook";
pub const ERR_CLI_PANIC: &str = "Workbook parser encountered an unexpected panic";
pub const CLI_ERROR_TRANSLATION_KEYS: [(&str, &str); 19] = [
    (ERR_CLI_NON_UNICODE, "cli.ERROR_VALUE"),
    (ERR_CLI_DUPLICATE, "cli.ERROR_DUPLICATE"),
    (ERR_CLI_UNKNOWN, "cli.ERROR_UNKNOWN"),
    (ERR_CLI_REQUIRED, "cli.ERROR_REQUIRED"),
    (ERR_CLI_VALUE, "cli.ERROR_VALUE"),
    (ERR_CLI_FORMAT, "cli.ERROR_VALUE"),
    (ERR_CLI_EXTENSIONS, "cli.ERROR_VALUE"),
    (ERR_CLI_EMPTY_QUERY, "cli.ERROR_VALUE"),
    (ERR_CLI_NO_SEARCH_TYPES, "cli.ERROR_NO_TYPES"),
    (ERR_CLI_REGEX, "cli.ERROR_REGEX"),
    (ERR_CLI_INPUT_PATH, "cli.ERROR_PATH"),
    (ERR_CLI_OUTPUT_PATH, "cli.ERROR_OUTPUT"),
    (ERR_CLI_EXISTS, "cli.ERROR_OUTPUT"),
    (ERR_CLI_INPUT_COLLISION, "cli.ERROR_OUTPUT"),
    (ERR_CLI_OUTPUT_CHANGED, "cli.ERROR_OUTPUT"),
    (ERR_CLI_HELP_OPTIONS, "cli.ERROR_VALUE"),
    (ERR_CLI_HOME, "cli.ERROR_PATH"),
    (ERR_CLI_LANGUAGE, "cli.ERROR_VALUE"),
    (ERR_CLI_DIRECTORY_MODE, "cli.ERROR_DIRECTORY_MODE"),
];
pub const CLI_ERROR_SEPARATOR: &str = ": ";
pub const CLI_TRANSLATION_PATH_PLACEHOLDER: &str = "{path}";
pub const CLI_TRANSLATION_OPTION_PLACEHOLDER: &str = "{option}";
pub const CLI_ROOT_WALK_DEPTH: usize = 0;
pub const OOXML_WORKBOOK_PART: &str = "xl/workbook.xml";
pub const OOXML_WORKBOOK_RELS_PART: &str = "xl/_rels/workbook.xml.rels";
pub const OOXML_WORKBOOK_DIRECTORY: &str = "xl";
pub const OOXML_ABSOLUTE_PATH_PREFIX: &str = "/";
pub const OOXML_COMMENTS_RELATION_SUFFIX: &str = "/comments";
pub const XML_RELATIONSHIP_ELEMENT: &str = "Relationship";
pub const XML_RELATIONSHIP_ID_ATTRIBUTE: &str = "Id";
pub const XML_SHEET_RELATIONSHIP_ID_ATTRIBUTE: &str = "id";
pub const XML_RELATIONSHIP_TARGET_ATTRIBUTE: &str = "Target";
pub const XML_RELATIONSHIP_TYPE_ATTRIBUTE: &str = "Type";
pub const XML_RELATIONSHIP_MODE_ATTRIBUTE: &str = "TargetMode";
pub const XML_RELATIONSHIP_EXTERNAL_VALUE: &str = "External";
pub const XML_SHEET_ELEMENT: &str = "sheet";
pub const XML_SHEET_NAME_ATTRIBUTE: &str = "name";
pub const XML_COMMENT_ELEMENT: &str = "comment";
pub const XML_COMMENT_CELL_ATTRIBUTE: &str = "ref";
pub const XML_TEXT_ELEMENT: &str = "t";
pub const XML_NAMESPACE_SEPARATOR: u8 = b':';
pub const PATH_SEPARATOR: &str = "/";
pub const EXCEL_COLUMN_BASE: u32 = 26;
pub const EXCEL_ASCII_A: u8 = b'A';
pub const EXCEL_ONE_BASED_OFFSET: u32 = 1;
pub const EXCEL_ZERO_INDEX: u32 = 0;
pub const ERR_COMMENT_RELATION: &str = "Invalid or missing workbook relationship";
pub const ERR_COMMENT_EXTERNAL: &str = "External comment relationships are not read";
pub const ERR_COMMENT_CELL: &str = "Invalid comment cell reference";
pub const DEFAULT_EXPORT_LANGUAGE: &str = "en";
pub const LANGUAGE_JA: &str = "ja";
pub const LANGUAGE_EN: &str = "en";
pub const TRANSLATION_UNAVAILABLE_KEY: &str = "common.translationUnavailable";
pub const ERR_TRANSLATION_MISSING: &str = "A required translation is missing";

/// Individual extension constants
pub const EXT_XLSX: &str = ".xlsx";
pub const EXT_XLSM: &str = ".xlsm";
pub const EXT_XLSB: &str = ".xlsb";
pub const EXT_XLS: &str = ".xls";

// ==============================================================================
// 2. Scan & Search Settings
// ==============================================================================

/// Minimum throttle interval for scan progress notifications (ms)
pub const PROGRESS_NOTIFY_INTERVAL_MS: u64 = 50;

/// Row interval for checking cancellation during scanning (bitmask: 0x7F = every 127 rows)
pub const CANCEL_CHECK_ROW_INTERVAL: usize = 0x7F;

/// Surrounding context character count for snippet generation
pub const SNIPPET_CONTEXT_CHARS: usize = 30;

/// Snippet ellipsis marker
pub const SNIPPET_ELLIPSIS: &str = "...";
pub const SNIPPET_MARK_OPEN: &str =
    "<mark class='bg-yellow-500/30 text-yellow-300 px-0.5 rounded font-semibold'>";
pub const SNIPPET_MARK_CLOSE: &str = "</mark>";
pub const HTML_ESCAPE_AMPERSAND: &str = "&amp;";
pub const HTML_ESCAPE_LESS_THAN: &str = "&lt;";
pub const HTML_ESCAPE_GREATER_THAN: &str = "&gt;";
pub const HTML_ESCAPE_DOUBLE_QUOTE: &str = "&quot;";
pub const HTML_ESCAPE_APOSTROPHE: &str = "&#39;";
pub const SHAPE_MAX_XML_BYTES: u64 = 52_428_800;
pub const SHAPE_MAX_TOTAL_XML_BYTES: u64 = 209_715_200;
pub const SHAPE_MAX_XML_DEPTH: usize = 128;
pub const SHAPE_MAX_BINARY_BYTES: u64 = 52_428_800;
pub const XLSB_BRT_BUNDLE_SH_RECORD_ID: u32 = 0x009C;
pub const XLSB_BRT_DRAWING_RECORD_ID: u32 = 0x0226;
pub const XLSB_BUNDLE_SHEET_FIXED_BYTES: usize = 8;
pub const XLSB_STRING_LENGTH_BYTES: usize = 4;
pub const XLSB_UTF16_UNIT_BYTES: usize = 2;
pub const XLSB_VARINT_VALUE_MASK: u8 = 0x7F;
pub const XLSB_VARINT_CONTINUATION_MASK: u8 = 0x80;
pub const XLSB_VARINT_SHIFT: u32 = 7;
pub const XLSB_VARINT_MAX_BYTES: u32 = 5;
pub const XLS_MAX_BIFF_RECORD_BYTES: usize = 8_224;
pub const XLS_BOUNDSHEET_RECORD_ID: u16 = 0x0085;
pub const XLS_TXO_RECORD_ID: u16 = 0x01B6;
pub const XLS_CONTINUE_RECORD_ID: u16 = 0x003C;
pub const XLS_TXO_TEXT_COUNT_OFFSET: usize = 10;
pub const XLS_TXO_FIXED_HEADER_BYTES: usize = 16;
pub const XLS_BIFF_RECORD_HEADER_BYTES: usize = 4;
pub const XLS_SHAPE_NAME_PREFIX: &str = "Shape ";
pub const ERR_SHAPE_READ: &str = "Failed to read Shape drawing data";
pub const ERR_SHAPE_LIMIT: &str = "Shape drawing data exceeds the safety limit";

/// Initial ID for search match items
pub const DEFAULT_MATCH_ID_START: u64 = 1;

/// Excel temporary lock file prefix (excluded from scanning)
pub const EXCEL_TEMP_FILE_PREFIX: &str = "~$";

/// Buffer capacity for bounded sync channel in pipeline parallel processing
pub const CHANNEL_BUFFER_SIZE: usize = 1024;

// ==============================================================================
// 3. Preview Settings
// ==============================================================================

/// Row radius surrounding target cell in preview grid
pub const PREVIEW_ROW_RADIUS: u32 = 3;

/// Column radius surrounding target cell in preview grid
pub const PREVIEW_COL_RADIUS: u32 = 2;

// ==============================================================================
// 4. Events & Menus
// ==============================================================================

/// Event name for search match notification
pub const EVENT_SEARCH_MATCH: &str = "search-match";

/// Event name for scan progress notification
pub const EVENT_SCAN_PROGRESS: &str = "scan-progress";

/// Event name for opening About dialog
pub const EVENT_OPEN_ABOUT_DIALOG: &str = "open-about-dialog";

/// Menu item ID for opening About dialog
pub const MENU_ITEM_ABOUT_ID: &str = "open_about";

pub const MENU_KEY_ABOUT: &str = "menu.about";
pub const MENU_KEY_FILE: &str = "menu.file";
pub const MENU_KEY_EDIT: &str = "menu.edit";
pub const MENU_KEY_VIEW: &str = "menu.view";
pub const MENU_KEY_WINDOW: &str = "menu.window";
pub const MENU_KEY_HELP: &str = "menu.help";

// ==============================================================================
// 5. Export Settings
// ==============================================================================

/// Header background color for Excel export (Teal 700)
pub const XLSX_HEADER_BG_COLOR: u32 = 0x000F_766E;

/// Header foreground color for Excel export (White)
pub const XLSX_HEADER_FG_COLOR: u32 = 0x00FF_FFFF;

/// Export column widths. Translated headers are retrieved from shared catalogs.
pub const XLSX_COLUMN_WIDTHS: [f64; 9] = [8.0, 25.0, 40.0, 20.0, 12.0, 22.0, 14.0, 45.0, 30.0];
pub const ERR_INVALID_LANGUAGE: &str = "Unsupported export language";
pub const EXPORT_HEADER_KEYS: [&str; 9] = [
    "export.header.id",
    "export.header.fileName",
    "export.header.fullPath",
    "export.header.sheetName",
    "export.header.cell",
    "export.header.shapeName",
    "export.header.matchType",
    "export.header.matchedContent",
    "export.header.formula",
];
pub const EXPORT_SHEET_NAME_KEY: &str = "export.sheetName";
pub const EXPORT_MATCH_VALUE_KEY: &str = "export.match.value";
pub const EXPORT_MATCH_FORMULA_KEY: &str = "export.match.formula";
pub const EXPORT_MATCH_COMMENT_KEY: &str = "export.match.comment";
pub const EXPORT_MATCH_HIDDEN_SHEET_KEY: &str = "export.match.hiddenSheet";
pub const EXPORT_MATCH_SHAPE_KEY: &str = "export.match.shape";

// ==============================================================================
// 6. Supported Apps
// ==============================================================================

pub const APP_ID_EXCEL: &str = "excel";
pub const APP_ID_NUMBERS: &str = "numbers";
pub const APP_ID_CALC: &str = "calc";

pub const APP_NAME_EXCEL: &str = "Microsoft Excel";
pub const APP_NAME_NUMBERS: &str = "Numbers";
pub const APP_NAME_CALC: &str = "LibreOffice Calc";

pub const ICON_HINT_EXCEL: &str = "excel";
pub const ICON_HINT_NUMBERS: &str = "numbers";
pub const ICON_HINT_CALC: &str = "calc";
pub const ICON_HINT_GENERIC: &str = "generic";

// ==============================================================================
// 7. Error Messages & Log Templates
// ==============================================================================

pub const ERR_WORKBOOK_OPEN: &str = "Could not open workbook";
pub const ERR_SHEET_NOT_FOUND: &str = "Requested sheet was not found";
pub const ERR_CSV_HEADER_WRITE: &str = "Failed to write CSV header";
pub const ERR_CSV_RECORD_WRITE: &str = "Failed to write CSV record";
pub const ERR_XLSX_CREATE: &str = "Failed to create Excel workbook";
pub const ERR_XLSX_WORKSHEET: &str = "Failed to add worksheet";
pub const ERR_XLSX_WRITE: &str = "Failed to write Excel data";
pub const ERR_XLSX_SAVE: &str = "Failed to save Excel workbook";
pub const ERR_INVALID_REGEX: &str = "Invalid regular expression";
pub const ERR_FILE_NOT_FOUND: &str = "Requested file was not found";
pub const ERR_FOLDER_OPEN: &str = "Could not open folder";
pub const ERR_APP_LAUNCH: &str = "Could not launch application";
pub const ERR_LOCK_FAILED: &str = "Failed to acquire engine lock";
pub const ERR_SYSTEM_APP_NOT_FOUND: &str = "No application is available for this file";
pub const ERR_TASK_EXECUTION: &str = "Task execution failed";
pub const ERR_PATH_NOT_DIR: &str = "Path is not a directory";
pub const LOG_SEARCH_STARTED: &str = "[start_search] Search request received";
pub const LOG_SEARCH_COMPLETED: &str = "[start_search] Search completed";
pub const LOG_SEARCH_FAILED: &str = "[start_search] Search engine failed";
/// Constant reference: Home abbreviation path prefix.
pub const HOME_PATH_PREFIX_UNIX: &str = "~/";
/// Constant reference: Windows home abbreviation path prefix.
pub const HOME_PATH_PREFIX_WINDOWS: &str = "~\\";
/// Constant reference: Windows verbatim extended-length prefix.
pub const VERBATIM_PATH_PREFIX_WINDOWS: &str = r"\\?\";
/// Constant reference: Windows verbatim UNC extended-length prefix.
pub const VERBATIM_UNC_PATH_PREFIX_WINDOWS: &str = r"\\?\UNC\";
/// Constant reference: Standard Windows UNC prefix.
pub const STANDARD_UNC_PREFIX_WINDOWS: &str = r"\\";
/// Constant reference: Maximum suggestions returned for directory completion.
pub const MAX_PATH_COMPLETION_RESULTS: usize = 10;
pub const LOG_EVENT_EMIT_FAILED: &str = "Failed to emit Tauri event";
pub const LOG_CANCEL_REQUESTED: &str = "[cancel_search] Cancellation requested";
/// Constant reference: Maximum batch size for search matches
pub const SEARCH_MATCH_BATCH_SIZE: usize = 50;
/// Constant reference: Batch send interval for search matches (ms)
pub const SEARCH_MATCH_BATCH_INTERVAL_MS: u128 = 25;

/// macOS application bundle identifiers
pub const MAC_BUNDLE_EXCEL: &str = "com.microsoft.Excel";
pub const MAC_BUNDLE_NUMBERS: &str = "com.apple.iWork.Numbers";

// ==============================================================================
// 8. Unit Tests
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Validates supported extension constants and default extensions list.
    ///
    /// ## Arguments / Returns
    /// None.
    ///
    /// ## Errors
    /// Panics on assertion failure.
    #[test]
    fn test_default_extensions_validity() {
        assert_eq!(DEFAULT_EXTENSIONS.len(), 4);
        assert!(DEFAULT_EXTENSIONS.contains(&EXT_XLSX));
        assert!(DEFAULT_EXTENSIONS.contains(&EXT_XLSM));
        assert!(DEFAULT_EXTENSIONS.contains(&EXT_XLSB));
        assert!(DEFAULT_EXTENSIONS.contains(&EXT_XLS));
    }

    /// Validates CSV/Excel export header column counts and key mappings.
    ///
    /// ## Arguments / Returns
    /// None.
    ///
    /// ## Errors
    /// Panics on assertion failure.
    #[test]
    fn test_csv_export_headers() {
        assert_eq!(EXPORT_HEADER_KEYS.len(), XLSX_COLUMN_WIDTHS.len());
        assert_eq!(EXPORT_HEADER_KEYS[0], "export.header.id");
        assert_eq!(EXPORT_HEADER_KEYS[1], "export.header.fileName");
        assert_eq!(EXPORT_HEADER_KEYS[8], "export.header.formula");
        assert_eq!(EXPORT_SHEET_NAME_KEY, "export.sheetName");
        assert_eq!(EXPORT_MATCH_HIDDEN_SHEET_KEY, "export.match.hiddenSheet");
    }

    /// Validates that error messages are non-empty strings.
    ///
    /// ## Arguments / Returns
    /// None.
    ///
    /// ## Errors
    /// Panics on assertion failure.
    #[test]
    fn test_error_messages_non_empty() {
        assert!(!ERR_WORKBOOK_OPEN.is_empty());
        assert!(!ERR_SHEET_NOT_FOUND.is_empty());
        assert!(!ERR_INVALID_REGEX.is_empty());
        assert!(!ERR_FILE_NOT_FOUND.is_empty());
        assert!(!ERR_LOCK_FAILED.is_empty());
    }

    /// Validates menu item IDs and event channel constants.
    ///
    /// ## Arguments / Returns
    /// None.
    ///
    /// ## Errors
    /// Panics on assertion failure.
    #[test]
    fn test_menu_and_event_constants() {
        assert_eq!(EVENT_OPEN_ABOUT_DIALOG, "open-about-dialog");
        assert_eq!(MENU_ITEM_ABOUT_ID, "open_about");
        assert_eq!(MENU_KEY_ABOUT, "menu.about");
        assert_eq!(MENU_KEY_FILE, "menu.file");
        assert_eq!(MENU_KEY_EDIT, "menu.edit");
        assert_eq!(MENU_KEY_VIEW, "menu.view");
        assert_eq!(MENU_KEY_WINDOW, "menu.window");
        assert_eq!(MENU_KEY_HELP, "menu.help");
    }

    /// Validates pipeline parallel processing buffer and batch interval constants.
    ///
    /// ## Arguments / Returns
    /// None.
    ///
    /// ## Errors
    /// Panics on assertion failure.
    #[test]
    fn test_pipeline_constants() {
        assert_eq!(CHANNEL_BUFFER_SIZE, 1024);
        assert_eq!(SEARCH_MATCH_BATCH_SIZE, 50);
        assert_eq!(SEARCH_MATCH_BATCH_INTERVAL_MS, 25);
    }
}
