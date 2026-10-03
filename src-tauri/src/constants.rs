//! # Constants Definition Module (constants.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Centrally manages fixed values, configuration settings, file extensions,
//! event names, and error messages used across the application.
//! Conforms to Constitution Principle II (Externalization of constants).
// ==============================================================================
// 1. File Extension Constants
// ==============================================================================

/// Default list of search target Excel extensions
pub const DEFAULT_EXTENSIONS: [&str; 4] = [".xlsx", ".xlsm", ".xlsb", ".xls"];
/// Compatibility default to enable value search in SearchQuery.
pub const DEFAULT_INCLUDE_VALUE: bool = true;
/// Field name for value search setting in SearchQuery JSON.
pub const SEARCH_QUERY_INCLUDE_VALUE_FIELD: &str = "include_value";
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

/// Minimum throttle interval for scan progress notification (milliseconds)
pub const PROGRESS_NOTIFY_INTERVAL_MS: u64 = 50;

/// Row interval for checking cancellation during scan (bitmask: 0x7F = every 128 rows)
pub const CANCEL_CHECK_ROW_INTERVAL: usize = 0x7F;

/// Number of surrounding context characters when generating snippets
pub const SNIPPET_CONTEXT_CHARS: usize = 30;

/// Snippet ellipsis string
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

/// Initial value for search match item ID counter
pub const DEFAULT_MATCH_ID_START: u64 = 1;

/// Prefix for temporary Excel lock files (excluded from scan)
pub const EXCEL_TEMP_FILE_PREFIX: &str = "~$";

/// Buffer capacity of the bounded synchronous channel in pipeline processing
pub const CHANNEL_BUFFER_SIZE: usize = 1024;

// ==============================================================================
// 3. Preview Settings
// ==============================================================================

/// Row radius before and after target cell in preview display
pub const PREVIEW_ROW_RADIUS: u32 = 3;

/// Column radius before and after target cell in preview display
pub const PREVIEW_COL_RADIUS: u32 = 2;

// ==============================================================================
// 4. Tauri Events & Menu Constants
// ==============================================================================

/// Search match notification event name
pub const EVENT_SEARCH_MATCH: &str = "search-match";

/// Scan progress notification event name
pub const EVENT_SCAN_PROGRESS: &str = "scan-progress";

/// Search descendant directory discovery issue event name.
pub const EVENT_SEARCH_ISSUE: &str = "search-issue";

/// Open About dialog event name
pub const EVENT_OPEN_ABOUT_DIALOG: &str = "open-about-dialog";

/// Menu item ID: About dialog
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

/// Excel export header background color (Teal 700)
pub const XLSX_HEADER_BG_COLOR: u32 = 0x000F_766E;

/// Excel export header font color (White)
pub const XLSX_HEADER_FG_COLOR: u32 = 0x00FF_FFFF;

/// Column widths for export. Translated headers are retrieved from shared catalogs.
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
// 6. Application Identifiers and Display Names
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
// 7. Error Messages and Log Templates
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
/// Home shorthand prefix for Unix paths.
pub const HOME_PATH_PREFIX_UNIX: &str = "~/";
/// Home shorthand prefix for Windows paths.
pub const HOME_PATH_PREFIX_WINDOWS: &str = "~\\";
/// Maximum number of candidates returned by directory completion.
pub const MAX_PATH_COMPLETION_RESULTS: usize = 10;
pub const LOG_EVENT_EMIT_FAILED: &str = "Failed to emit Tauri event";
pub const LOG_CANCEL_REQUESTED: &str = "[cancel_search] Cancellation requested";
/// Maximum batch size for emitting search matches
pub const SEARCH_MATCH_BATCH_SIZE: usize = 50;
/// Batch interval for emitting search matches (milliseconds)
pub const SEARCH_MATCH_BATCH_INTERVAL_MS: u128 = 25;

/// macOS application bundle identifiers
pub const MAC_BUNDLE_EXCEL: &str = "com.microsoft.Excel";
pub const MAC_BUNDLE_NUMBERS: &str = "com.apple.iWork.Numbers";

// ==============================================================================
// 7.1 Multi-Path Search Constants
// ==============================================================================

/// Delimiter separating multiple search paths.
pub const MULTI_PATH_DELIMITER: char = ',';
/// Delimiter string separating multiple search paths.
pub const MULTI_PATH_DELIMITER_STR: &str = ",";
/// Standard double quote character used for path grouping.
pub const PATH_QUOTE_CHAR: char = '"';
/// Error message returned when a quoted path segment is not closed.
pub const ERR_PATH_UNCLOSED_QUOTE: &str = "Unclosed double quote in search path";
/// Error message returned when no valid search path is specified.
pub const ERR_NO_VALID_SEARCH_PATHS: &str = "No valid search path specified";

// ==============================================================================
// 7.2 Linux Desktop Command Constants
// ==============================================================================

/// Command name for invoking DBus calls on Linux
pub const CMD_DBUS_SEND: &str = "dbus-send";

/// DBus reply printing flag (required to wait for reply and detect errors)
pub const DBUS_ARG_PRINT_REPLY: &str = "--print-reply";

/// DBus method call reply timeout argument (2000 milliseconds)
pub const DBUS_ARG_REPLY_TIMEOUT: &str = "--reply-timeout=2000";

/// DBus session bus argument
pub const DBUS_ARG_SESSION: &str = "--session";

/// DBus destination argument for file manager
pub const DBUS_DEST_FILEMANAGER: &str = "--dest=org.freedesktop.FileManager1";

/// DBus method call type argument
pub const DBUS_TYPE_METHOD_CALL: &str = "--type=method_call";

/// DBus object path for file manager
pub const DBUS_PATH_FILEMANAGER: &str = "/org/freedesktop/FileManager1";

/// DBus method to reveal and highlight items in file manager
pub const DBUS_METHOD_SHOW_ITEMS: &str = "org.freedesktop.FileManager1.ShowItems";

/// DBus parameter prefix for string array argument
pub const DBUS_PARAM_ARRAY_STRING_PREFIX: &str = "array:string:";

/// DBus startup id parameter placeholder
pub const DBUS_PARAM_STARTUP_ID_EMPTY: &str = "string:";

/// Command name for invoking GNOME Files (Nautilus) file manager
pub const CMD_NAUTILUS: &str = "nautilus";

/// Command name for invoking KDE Dolphin file manager
pub const CMD_DOLPHIN: &str = "dolphin";

/// Command name for invoking Cinnamon Nemo file manager
pub const CMD_NEMO: &str = "nemo";

/// CLI flag to select and highlight a target file in supported file managers
pub const ARG_SELECT: &str = "--select";

/// CLI flag for Nemo to open without managing the desktop
pub const ARG_NO_DESKTOP: &str = "--no-desktop";

/// Command name for GIO desktop launcher on Linux
pub const CMD_GIO: &str = "gio";

/// Subcommand for GIO to open a location
pub const GIO_SUBCOMMAND_OPEN: &str = "open";

/// Command name for XDG desktop opener on Linux
pub const CMD_XDG_OPEN: &str = "xdg-open";

/// URI prefix for local file paths
pub const FILE_URI_PREFIX: &str = "file://";

/// Windows Explorer executable command name
pub const CMD_EXPLORER: &str = "explorer";

/// Windows Explorer executable command name for WSL environments
pub const CMD_WSL_EXPLORER: &str = "explorer.exe";

/// Windows Explorer /select, parameter prefix to highlight an item
pub const ARG_EXPLORER_SELECT_PREFIX: &str = "/select,";

/// Command name for WSL path translation utility
pub const CMD_WSLPATH: &str = "wslpath";

/// Command line argument for wslpath to convert to Windows format
pub const WSLPATH_ARG_WINDOWS: &str = "-w";

// ==============================================================================
// 8. Unit Tests
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// ## Description
    /// Verifies the validity of extension constants and the default list.
    ///
    /// ## Arguments / Returns
    /// None
    ///
    /// ## Errors / Exceptions
    /// Panics if assertions fail.
    #[test]
    fn test_default_extensions_validity() {
        assert_eq!(DEFAULT_EXTENSIONS.len(), 4);
        assert!(DEFAULT_EXTENSIONS.contains(&EXT_XLSX));
        assert!(DEFAULT_EXTENSIONS.contains(&EXT_XLSM));
        assert!(DEFAULT_EXTENSIONS.contains(&EXT_XLSB));
        assert!(DEFAULT_EXTENSIONS.contains(&EXT_XLS));
    }

    /// ## Description
    /// Verifies the column count of CSV export headers and the presence of primary column keys.
    ///
    /// ## Arguments / Returns
    /// None
    ///
    /// ## Errors / Exceptions
    /// Panics if assertions fail.
    #[test]
    fn test_csv_export_headers() {
        assert_eq!(EXPORT_HEADER_KEYS.len(), XLSX_COLUMN_WIDTHS.len());
        assert_eq!(EXPORT_HEADER_KEYS[0], "export.header.id");
        assert_eq!(EXPORT_HEADER_KEYS[1], "export.header.fileName");
        assert_eq!(EXPORT_HEADER_KEYS[8], "export.header.formula");
        assert_eq!(EXPORT_SHEET_NAME_KEY, "export.sheetName");
        assert_eq!(EXPORT_MATCH_HIDDEN_SHEET_KEY, "export.match.hiddenSheet");
    }

    /// ## Description
    /// Verifies that all error messages are non-empty strings.
    ///
    /// ## Arguments / Returns
    /// None
    ///
    /// ## Errors / Exceptions
    /// Panics if assertions fail.
    #[test]
    fn test_error_messages_non_empty() {
        assert!(!ERR_WORKBOOK_OPEN.is_empty());
        assert!(!ERR_SHEET_NOT_FOUND.is_empty());
        assert!(!ERR_INVALID_REGEX.is_empty());
        assert!(!ERR_FILE_NOT_FOUND.is_empty());
        assert!(!ERR_LOCK_FAILED.is_empty());
    }

    /// ## Description
    /// Verifies that menu and event-related constants are non-empty and match expected strings.
    ///
    /// ## Arguments / Returns
    /// None
    ///
    /// ## Errors / Exceptions
    /// Panics if assertions fail.
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

    /// ## Description
    /// Verifies that buffer constants for pipeline parallel execution are properly defined.
    ///
    /// ## Arguments / Returns
    /// None
    ///
    /// ## Errors / Exceptions
    /// Panics if assertions fail.
    #[test]
    fn test_pipeline_constants() {
        assert_eq!(CHANNEL_BUFFER_SIZE, 1024);
        assert_eq!(SEARCH_MATCH_BATCH_SIZE, 50);
        assert_eq!(SEARCH_MATCH_BATCH_INTERVAL_MS, 25);
    }

    /// ## Description
    /// Verifies that Linux desktop integration constants are properly defined.
    ///
    /// ## Arguments / Returns
    /// None
    ///
    /// ## Errors / Exceptions
    /// Panics if assertions fail.
    #[test]
    fn test_linux_desktop_constants() {
        assert_eq!(CMD_DBUS_SEND, "dbus-send");
        assert_eq!(DBUS_ARG_PRINT_REPLY, "--print-reply");
        assert_eq!(DBUS_ARG_REPLY_TIMEOUT, "--reply-timeout=2000");
        assert_eq!(DBUS_ARG_SESSION, "--session");
        assert_eq!(DBUS_DEST_FILEMANAGER, "--dest=org.freedesktop.FileManager1");
        assert_eq!(DBUS_TYPE_METHOD_CALL, "--type=method_call");
        assert_eq!(DBUS_PATH_FILEMANAGER, "/org/freedesktop/FileManager1");
        assert_eq!(
            DBUS_METHOD_SHOW_ITEMS,
            "org.freedesktop.FileManager1.ShowItems"
        );
        assert_eq!(DBUS_PARAM_ARRAY_STRING_PREFIX, "array:string:");
        assert_eq!(DBUS_PARAM_STARTUP_ID_EMPTY, "string:");
        assert_eq!(CMD_NAUTILUS, "nautilus");
        assert_eq!(CMD_DOLPHIN, "dolphin");
        assert_eq!(CMD_NEMO, "nemo");
        assert_eq!(ARG_SELECT, "--select");
        assert_eq!(ARG_NO_DESKTOP, "--no-desktop");
        assert_eq!(CMD_GIO, "gio");
        assert_eq!(GIO_SUBCOMMAND_OPEN, "open");
        assert_eq!(CMD_XDG_OPEN, "xdg-open");
        assert_eq!(FILE_URI_PREFIX, "file://");
        assert_eq!(CMD_EXPLORER, "explorer");
        assert_eq!(CMD_WSL_EXPLORER, "explorer.exe");
        assert_eq!(ARG_EXPLORER_SELECT_PREFIX, "/select,");
        assert_eq!(CMD_WSLPATH, "wslpath");
        assert_eq!(WSLPATH_ARG_WINDOWS, "-w");
    }

    /// ## Description
    /// Verifies that `tauri.conf.json` defines `productName` as "xlseek" across all platforms
    /// so installer package file names and installation directories are unified to "xlseek".
    ///
    /// ## Arguments / Returns
    /// None
    ///
    /// ## Errors / Exceptions
    /// Panics if assertion or JSON parsing fails.
    #[test]
    fn test_tauri_config_product_name() {
        let manifest_dir =
            std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| "src-tauri".to_string());
        let conf_path = std::path::Path::new(&manifest_dir).join("tauri.conf.json");
        let content = std::fs::read_to_string(&conf_path).expect("tauri.conf.json should exist");
        let parsed: serde_json::Value =
            serde_json::from_str(&content).expect("tauri.conf.json should be valid JSON");
        assert_eq!(
            parsed["productName"], "xlseek",
            "Tauri package productName must be xlseek"
        );
    }

    /// ## Description
    /// Verifies that `tauri.linux.conf.json` exists and defines `productName` as "xlseek"
    /// so Debian packages do not contain spaces in their filename or package identifiers.
    ///
    /// ## Arguments / Returns
    /// None
    ///
    /// ## Errors / Exceptions
    /// Panics if assertion or JSON parsing fails.
    #[test]
    fn test_tauri_linux_config_product_name() {
        let manifest_dir =
            std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| "src-tauri".to_string());
        let linux_conf_path = std::path::Path::new(&manifest_dir).join("tauri.linux.conf.json");
        let content =
            std::fs::read_to_string(&linux_conf_path).expect("tauri.linux.conf.json should exist");
        let parsed: serde_json::Value =
            serde_json::from_str(&content).expect("tauri.linux.conf.json should be valid JSON");
        assert_eq!(
            parsed["productName"], "xlseek",
            "Linux package productName must be xlseek without spaces"
        );
    }
}
