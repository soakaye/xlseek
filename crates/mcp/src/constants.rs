//! # MCP Constants Module (crates/mcp/src/constants.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Centralized definitions for MCP server protocol constants, JSON-RPC 2.0 error codes,
//! tool names, default parameters, and CLI argument strings.
//! Conforms to Constitution Principle I (English text), Principle II (No Hardcoded Constants),
//! Principle III (Comprehensive Header Comments), and Principle V (Test Verification).

/// JSON-RPC 2.0 protocol version tag.
pub const JSONRPC_VERSION: &str = "2.0";

/// MCP protocol handshake version supported by this server.
pub const MCP_PROTOCOL_VERSION: &str = "2024-11-05";

/// Server binary and registration name.
pub const SERVER_NAME: &str = "xlseek-mcp";

/// Server version string matching crate package version.
pub const SERVER_VERSION: &str = "1.0.2";

// JSON-RPC standard error codes
/// Standard JSON-RPC parse error code (-32700).
pub const ERROR_CODE_PARSE_ERROR: i32 = -32700;

/// Standard JSON-RPC invalid request error code (-32600).
pub const ERROR_CODE_INVALID_REQUEST: i32 = -32600;

/// Standard JSON-RPC method not found error code (-32601).
pub const ERROR_CODE_METHOD_NOT_FOUND: i32 = -32601;

/// Standard JSON-RPC invalid params error code (-32602).
pub const ERROR_CODE_INVALID_PARAMS: i32 = -32602;

/// Standard JSON-RPC internal error code (-32603).
pub const ERROR_CODE_INTERNAL_ERROR: i32 = -32603;

// MCP Method names
/// Handshake initialization request method.
pub const METHOD_INITIALIZE: &str = "initialize";

/// Notification sent by client after initialization handshake.
pub const METHOD_NOTIFICATIONS_INITIALIZED: &str = "notifications/initialized";

/// Health check ping method.
pub const METHOD_PING: &str = "ping";

/// Tool discovery request method.
pub const METHOD_TOOLS_LIST: &str = "tools/list";

/// Tool execution request method.
pub const METHOD_TOOLS_CALL: &str = "tools/call";

// Exposed tool names
/// Unified Excel and Word cross-document search tool name.
pub const TOOL_NAME_SEARCH: &str = "xlseek_search";

/// Worksheet listing tool name.
pub const TOOL_NAME_LIST_SHEETS: &str = "xlseek_list_sheets";

/// Worksheet cell grid preview tool name.
pub const TOOL_NAME_PREVIEW_SHEET: &str = "xlseek_preview_sheet";

// Default numeric thresholds and boundaries
/// Default maximum search results returned when unspecified.
pub const DEFAULT_MAX_RESULTS: usize = 100;

/// Default 1-based start row for sheet cell preview.
pub const DEFAULT_PREVIEW_START_ROW: u32 = 1;

/// Default maximum preview rows extracted from a worksheet.
pub const DEFAULT_PREVIEW_MAX_ROWS: u32 = 100;

// CLI arguments and environment variables
/// CLI long flag for specifying permitted directory roots.
pub const CLI_FLAG_ALLOWED_DIR: &str = "--allowed-dir";

/// CLI short flag for displaying help text.
pub const CLI_FLAG_HELP_SHORT: &str = "-h";

/// CLI long flag for displaying help text.
pub const CLI_FLAG_HELP_LONG: &str = "--help";

/// CLI short flag for displaying version string.
pub const CLI_FLAG_VERSION_SHORT: &str = "-v";

/// CLI long flag for displaying version string.
pub const CLI_FLAG_VERSION_LONG: &str = "--version";

/// Environment variable for comma-separated allowed directory roots.
pub const ENV_ALLOWED_DIRS: &str = "XLSEEK_ALLOWED_DIRS";

// Content types and diagnostic strings
/// Content type label for textual MCP tool response blocks.
pub const CONTENT_TYPE_TEXT: &str = "text";

/// Human-readable description for tool `xlseek_search`.
pub const TOOL_DESC_SEARCH: &str = "High-speed cross-document search across Excel (.xlsx, .xlsm, .xls, .xlsb) and Word (.docx, .doc) files. Returns matching cells, formulas, comments, and drawing shape text.";

/// Human-readable description for tool `xlseek_list_sheets`.
pub const TOOL_DESC_LIST_SHEETS: &str = "List all worksheet names contained within a specific Excel workbook file (.xlsx, .xlsm, .xls, .xlsb).";

/// Human-readable description for tool `xlseek_preview_sheet`.
pub const TOOL_DESC_PREVIEW_SHEET: &str = "Preview rows and columns of an Excel worksheet formatted as a clean Markdown table with row and column headers.";

/// Error message for access to paths outside permitted whitelist boundaries.
pub const ERR_ACCESS_DENIED_PREFIX: &str = "Access denied: Path is outside permitted directories";

/// Error message when an invalid method name is requested.
pub const ERR_METHOD_NOT_FOUND_PREFIX: &str = "Method not found";

/// Error message when parameters fail deserialization.
pub const ERR_INVALID_PARAMS_PREFIX: &str = "Invalid params";

/// Error message when target workbook cannot be opened.
pub const ERR_OPEN_WORKBOOK_FAILED: &str = "Failed to open Excel workbook";

/// Error message when target worksheet is not found.
pub const ERR_SHEET_NOT_FOUND: &str = "Worksheet not found in workbook";

/// Error message when required parameter 'path' is missing.
pub const ERR_MISSING_PARAM_PATH: &str = "Missing required parameter: 'path'";

/// Error message when required parameter 'sheet_name' is missing.
pub const ERR_MISSING_PARAM_SHEET_NAME: &str = "Missing required parameter: 'sheet_name'";

/// Row column header label in Markdown preview table.
pub const MARKDOWN_COL_ROW: &str = "Row";

/// Divider hyphens in Markdown preview table.
pub const MARKDOWN_DIVIDER: &str = "---";

/// Cell delimiter in Markdown preview table.
pub const MARKDOWN_PIPE: &str = "|";

/// Label for empty worksheets in sheet preview output.
pub const PREVIEW_EMPTY_WORKSHEET: &str = "Empty worksheet";

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that core protocol constants adhere to required string conventions.
    #[test]
    fn test_protocol_constants_validity() {
        assert_eq!(JSONRPC_VERSION, "2.0");
        assert_eq!(MCP_PROTOCOL_VERSION, "2024-11-05");
        assert_eq!(SERVER_NAME, "xlseek-mcp");
        assert!(!SERVER_VERSION.is_empty());
    }

    /// Tests that tool names follow the `xlseek_` prefix convention.
    #[test]
    fn test_tool_names_prefixed() {
        assert!(TOOL_NAME_SEARCH.starts_with("xlseek_"));
        assert!(TOOL_NAME_LIST_SHEETS.starts_with("xlseek_"));
        assert!(TOOL_NAME_PREVIEW_SHEET.starts_with("xlseek_"));
    }

    /// Tests that default limits are non-zero.
    #[test]
    fn test_default_thresholds() {
        const {
            assert!(DEFAULT_MAX_RESULTS > 0);
            assert!(DEFAULT_PREVIEW_START_ROW > 0);
            assert!(DEFAULT_PREVIEW_MAX_ROWS > 0);
        }
    }
}
