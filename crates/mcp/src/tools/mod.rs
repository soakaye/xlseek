//! # MCP Tools Registry Module (crates/mcp/src/tools/mod.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Central registry declaring exposed MCP tools and dispatching tool invocations.
//! Conforms to Constitution Principle I (English text), Principle II (No Hardcoded Constants),
//! Principle III (Comprehensive Header Comments), and Principle V (Robust Error Handling).

use crate::protocol::{ToolCallResult, ToolDefinition};
use crate::security::McpSecurityContext;
use serde_json::Value;

pub mod list_sheets;
pub mod preview;
pub mod search;

/// Returns the complete list of exposed MCP tool definitions for `tools/list`.
///
/// ## Arguments
/// None.
///
/// ## Returns
/// Vector of `ToolDefinition` instances.
///
/// ## Errors
/// Does not panic.
pub fn list_tool_definitions() -> Vec<ToolDefinition> {
    vec![
        search::tool_definition(),
        list_sheets::tool_definition(),
        preview::tool_definition(),
    ]
}

/// Dispatches a tool invocation request to the corresponding tool handler.
///
/// ## Arguments
/// - `name`: `&str` - Name of the requested tool.
/// - `arguments`: `&Value` - Tool invocation arguments object.
/// - `security`: `&McpSecurityContext` - Active security context for path validation.
///
/// ## Returns
/// `ToolCallResult` containing either the output content or a domain error.
///
/// ## Errors
/// Unknown tool names return a tool error result without crashing.
pub fn dispatch_tool_call(
    name: &str,
    arguments: &Value,
    security: &McpSecurityContext,
) -> ToolCallResult {
    // Constant reference: crate::constants::TOOL_NAME_SEARCH
    // Constant reference: crate::constants::TOOL_NAME_LIST_SHEETS
    // Constant reference: crate::constants::TOOL_NAME_PREVIEW_SHEET
    match name {
        crate::constants::TOOL_NAME_SEARCH => search::execute(arguments, security),
        crate::constants::TOOL_NAME_LIST_SHEETS => list_sheets::execute(arguments, security),
        crate::constants::TOOL_NAME_PREVIEW_SHEET => preview::execute(arguments, security),
        unknown => ToolCallResult::error(format!("Unknown tool: '{unknown}'")),
    }
}
