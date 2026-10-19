//! # Tool Handler: xlseek_preview_sheet (crates/mcp/src/tools/preview.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Implements Excel worksheet cell preview rendering in clean Markdown table format.
//! Opens workbooks using `calamine`, bounds row output by `start_row` and `max_rows`,
//! sanitizes cell content to prevent table breakage, and renders column letters (A, B, C...).
//! Conforms to Constitution Principle I (English text), Principle II (No Hardcoded Constants),
//! Principle III (Comprehensive Header Comments), and Principle V (Robust Error Handling).

use crate::constants;
use crate::protocol::{ToolCallResult, ToolDefinition};
use crate::security::McpSecurityContext;
use calamine::{open_workbook_auto, Data, Reader, Sheets};
use serde_json::Value;
use xlseek_core::search::parser::col_to_name;

/// Returns the tool definition contract for `xlseek_preview_sheet`.
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
    // Constant reference: constants::TOOL_NAME_PREVIEW_SHEET
    // Constant reference: constants::TOOL_DESC_PREVIEW_SHEET
    // Constant reference: constants::DEFAULT_PREVIEW_START_ROW
    // Constant reference: constants::DEFAULT_PREVIEW_MAX_ROWS
    ToolDefinition {
        name: constants::TOOL_NAME_PREVIEW_SHEET.to_string(),
        description: constants::TOOL_DESC_PREVIEW_SHEET.to_string(),
        input_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the Excel file."
                },
                "sheet_name": {
                    "type": "string",
                    "description": "Name of the sheet to preview."
                },
                "start_row": {
                    "type": "integer",
                    "default": constants::DEFAULT_PREVIEW_START_ROW,
                    "description": "1-based starting row index."
                },
                "max_rows": {
                    "type": "integer",
                    "default": constants::DEFAULT_PREVIEW_MAX_ROWS,
                    "description": "Maximum number of rows to return in the preview window."
                }
            },
            "required": ["path", "sheet_name"]
        }),
    }
}

/// Converts a `calamine::Data` cell value into a clean display string.
///
/// ## Arguments
/// - `data`: `&Data` - Reference to calamine cell data.
///
/// ## Returns
/// String representation sanitized for Markdown table embedding.
///
/// ## Errors
/// Does not panic.
fn format_cell_data(data: &Data) -> String {
    let raw = match data {
        Data::Empty => String::new(),
        Data::String(s) => s.clone(),
        Data::Float(f) => f.to_string(),
        Data::Int(i) => i.to_string(),
        Data::Bool(b) => b.to_string(),
        Data::DateTime(d) => d.to_string(),
        Data::DateTimeIso(d) => d.clone(),
        Data::DurationIso(d) => d.clone(),
        Data::Error(e) => format!("{:?}", e),
    };

    // Sanitize pipes and newlines to preserve Markdown table integrity
    raw.replace('|', "\\|").replace(['\n', '\r'], " ")
}

/// Executes the `xlseek_preview_sheet` tool invocation.
///
/// ## Arguments
/// - `args`: `&Value` - Tool invocation arguments object.
/// - `security`: `&McpSecurityContext` - Active security context for path validation.
///
/// ## Returns
/// `ToolCallResult` containing formatted Markdown table text or error details.
///
/// ## Errors
/// Returns error `ToolCallResult` if parameters are missing, unauthorized,
/// or worksheet cannot be read. Does not panic.
pub fn execute(args: &Value, security: &McpSecurityContext) -> ToolCallResult {
    // Constant reference: constants::ERR_MISSING_PARAM_PATH
    // Constant reference: constants::ERR_MISSING_PARAM_SHEET_NAME
    let raw_path = match args.get("path").and_then(|p| p.as_str()) {
        Some(p) if !p.is_empty() => p,
        _ => return ToolCallResult::error(constants::ERR_MISSING_PARAM_PATH),
    };

    let sheet_name = match args.get("sheet_name").and_then(|s| s.as_str()) {
        Some(s) if !s.is_empty() => s,
        _ => return ToolCallResult::error(constants::ERR_MISSING_PARAM_SHEET_NAME),
    };

    // Constant reference: constants::DEFAULT_PREVIEW_START_ROW
    let start_row = match args.get("start_row").and_then(|v| v.as_u64()) {
        Some(0) | None => constants::DEFAULT_PREVIEW_START_ROW,
        Some(v) => v as u32,
    };

    // Constant reference: constants::DEFAULT_PREVIEW_MAX_ROWS
    let max_rows = match args.get("max_rows").and_then(|v| v.as_u64()) {
        Some(0) | None => constants::DEFAULT_PREVIEW_MAX_ROWS,
        Some(v) => v as u32,
    };

    let validated_path = match security.validate_path(raw_path) {
        Ok(p) => p,
        Err(e) => return ToolCallResult::error(e),
    };

    let normalized_display = xlseek_core::search::path::normalize_canonical_path(&validated_path)
        .display()
        .to_string();

    let mut workbook: Sheets<_> = match open_workbook_auto(&validated_path) {
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

    if !workbook.sheet_names().iter().any(|s| s == sheet_name) {
        // Constant reference: constants::ERR_SHEET_NOT_FOUND
        return ToolCallResult::error(format!(
            "{}: '{}' in {}",
            constants::ERR_SHEET_NOT_FOUND,
            sheet_name,
            normalized_display
        ));
    }

    let range = match workbook.worksheet_range(sheet_name) {
        Ok(r) => r,
        Err(e) => {
            // Constant reference: constants::ERR_SHEET_NOT_FOUND
            return ToolCallResult::error(format!(
                "{}: '{}' ({})",
                constants::ERR_SHEET_NOT_FOUND,
                sheet_name,
                e
            ));
        }
    };

    // Constant reference: constants::MARKDOWN_COL_ROW
    // Constant reference: constants::MARKDOWN_DIVIDER
    // Constant reference: constants::MARKDOWN_PIPE
    // Constant reference: constants::PREVIEW_EMPTY_WORKSHEET
    if range.is_empty() {
        let empty_output = format!(
            "Preview of '{}' in {} ({})\n\n{} {} {}\n{}{}{}\n",
            sheet_name,
            normalized_display,
            constants::PREVIEW_EMPTY_WORKSHEET,
            constants::MARKDOWN_PIPE,
            constants::MARKDOWN_COL_ROW,
            constants::MARKDOWN_PIPE,
            constants::MARKDOWN_PIPE,
            constants::MARKDOWN_DIVIDER,
            constants::MARKDOWN_PIPE,
        );
        return ToolCallResult::success(empty_output);
    }

    let (range_start_row, range_start_col) = range.start().unwrap_or((0, 0));
    let (_, range_end_col) = range.end().unwrap_or((0, 0));
    let total_rows = range.height() as u32 + range_start_row;
    let total_cols = range_end_col + 1;

    let actual_start = start_row;
    let actual_end = (start_row + max_rows - 1).min(total_rows);

    let mut output = format!(
        "Preview of '{}' in {} (Rows {}-{} of {}, {} columns):\n\n",
        sheet_name, normalized_display, actual_start, actual_end, total_rows, total_cols
    );

    // Build header row: | Row | A | B | ... |
    output.push_str(constants::MARKDOWN_PIPE);
    output.push(' ');
    output.push_str(constants::MARKDOWN_COL_ROW);
    output.push(' ');
    output.push_str(constants::MARKDOWN_PIPE);

    for col in 0..total_cols {
        let col_name = col_to_name(col);
        output.push(' ');
        output.push_str(&col_name);
        output.push(' ');
        output.push_str(constants::MARKDOWN_PIPE);
    }
    output.push('\n');

    // Build divider row: |---|---|---|...|
    output.push_str(constants::MARKDOWN_PIPE);
    output.push_str(constants::MARKDOWN_DIVIDER);
    output.push_str(constants::MARKDOWN_PIPE);

    for _ in 0..total_cols {
        output.push_str(constants::MARKDOWN_DIVIDER);
        output.push_str(constants::MARKDOWN_PIPE);
    }
    output.push('\n');

    // Build data rows
    if actual_start <= total_rows {
        for r in actual_start..=actual_end {
            output.push_str(constants::MARKDOWN_PIPE);
            output.push(' ');
            output.push_str(&r.to_string());
            output.push(' ');
            output.push_str(constants::MARKDOWN_PIPE);

            for c in 0..total_cols {
                let cell_text = if (r - 1) >= range_start_row && c >= range_start_col {
                    let rel_r = (r - 1 - range_start_row) as usize;
                    let rel_c = (c - range_start_col) as usize;
                    range
                        .get((rel_r, rel_c))
                        .map(format_cell_data)
                        .unwrap_or_default()
                } else {
                    String::new()
                };

                output.push(' ');
                output.push_str(&cell_text);
                output.push(' ');
                output.push_str(constants::MARKDOWN_PIPE);
            }
            output.push('\n');
        }
    }

    ToolCallResult::success(output.trim_end().to_string())
}
