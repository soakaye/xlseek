//! # Tool Handler: xlseek_list_sheets (crates/mcp/src/tools/list_sheets.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Implements Excel workbook worksheet listing for `.xlsx`, `.xlsm`, `.xls`, and `.xlsb` files.
//! Validates target file path against directory security boundaries, opens the workbook
//! using `calamine`, and returns an ordered list of worksheet names.
//! Conforms to Constitution Principle I (English text), Principle II (No Hardcoded Constants),
//! Principle III (Comprehensive Header Comments), and Principle V (Robust Error Handling).

use crate::constants;
use crate::protocol::{ToolCallResult, ToolDefinition};
use crate::security::McpSecurityContext;
use calamine::{open_workbook_auto, Reader, Sheets};
use serde_json::Value;

/// Returns the tool definition contract for `xlseek_list_sheets`.
///
/// ## Arguments
/// None.
///
/// ## Returns
/// `ToolDefinition` with JSON Schema input properties.
///
/// ## Errors
/// Does not panic.
pub fn tool_definition() -> ToolDefinition {
    // Constant reference: constants::TOOL_NAME_LIST_SHEETS
    // Constant reference: constants::TOOL_DESC_LIST_SHEETS
    ToolDefinition {
        name: constants::TOOL_NAME_LIST_SHEETS.to_string(),
        description: constants::TOOL_DESC_LIST_SHEETS.to_string(),
        input_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Absolute or relative path to the Excel file."
                }
            },
            "required": ["path"]
        }),
    }
}

/// Executes the `xlseek_list_sheets` tool invocation.
///
/// ## Arguments
/// - `args`: `&Value` - Tool invocation arguments object.
/// - `security`: `&McpSecurityContext` - Active security context for path validation.
///
/// ## Returns
/// `ToolCallResult` containing worksheet names or error details.
///
/// ## Errors
/// Returns error `ToolCallResult` if path is missing, unauthorized, or workbook cannot be opened.
pub fn execute(args: &Value, security: &McpSecurityContext) -> ToolCallResult {
    // Constant reference: constants::ERR_MISSING_PARAM_PATH
    let raw_path = match args.get("path").and_then(|p| p.as_str()) {
        Some(p) if !p.is_empty() => p,
        _ => return ToolCallResult::error(constants::ERR_MISSING_PARAM_PATH),
    };

    let validated_path = match security.validate_path(raw_path) {
        Ok(p) => p,
        Err(e) => return ToolCallResult::error(e),
    };

    let normalized_display = xlseek_core::search::path::normalize_canonical_path(&validated_path)
        .display()
        .to_string();

    let workbook: Result<Sheets<_>, _> = open_workbook_auto(&validated_path);
    let workbook = match workbook {
        Ok(wb) => wb,
        Err(e) => {
            // Constant reference: constants::ERR_OPEN_WORKBOOK_FAILED
            return ToolCallResult::error(format!(
                "{}: {} ({})",
                constants::ERR_OPEN_WORKBOOK_FAILED,
                normalized_display,
                e
            ));
        }
    };

    let sheets = workbook.sheet_names();
    let mut output = format!(
        "Workbook: {}\nTotal sheets: {}\nWorksheets:\n",
        normalized_display,
        sheets.len()
    );

    for (idx, name) in sheets.iter().enumerate() {
        output.push_str(&format!("{}. {}\n", idx + 1, name));
    }

    ToolCallResult::success(output.trim_end().to_string())
}
