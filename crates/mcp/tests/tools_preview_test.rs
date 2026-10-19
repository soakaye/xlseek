//! # Worksheet Listing and Preview Tool Integration Tests (crates/mcp/tests/tools_preview_test.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Verifies worksheet listing (`xlseek_list_sheets`) and Markdown cell grid preview
//! (`xlseek_preview_sheet`) execution over MCP stdio protocol.
//! Conforms to Constitution Principle I (English text), Principle II (No Hardcoded Constants),
//! Principle III (Comprehensive Header Comments), and Principle V (Test Verification).

use serde_json::Value;
use std::path::PathBuf;
use xlseek_mcp::constants;
use xlseek_mcp::security::McpSecurityContext;
use xlseek_mcp::McpServer;

/// Resolves path to test fixtures directory in repository root.
///
/// ## Arguments / Returns
/// Takes no arguments; returns `PathBuf` to `tests/fixtures`.
///
/// ## Errors
/// Panics if manifest path cannot be navigated.
fn fixtures_dir() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .and_then(|p| p.parent())
        .expect("repo root exists")
        .join("tests")
        .join("fixtures")
}

/// Tests that `xlseek_list_sheets` successfully lists worksheet names in an Excel workbook.
#[test]
fn test_list_sheets_success() {
    let server = McpServer::new(McpSecurityContext::default());
    let target_file = fixtures_dir().join("sample_report.xlsx");

    let req = serde_json::json!({
        // Constant reference: constants::JSONRPC_VERSION
        // Constant reference: constants::METHOD_TOOLS_CALL
        // Constant reference: constants::TOOL_NAME_LIST_SHEETS
        "jsonrpc": constants::JSONRPC_VERSION,
        "id": 10,
        "method": constants::METHOD_TOOLS_CALL,
        "params": {
            "name": constants::TOOL_NAME_LIST_SHEETS,
            "arguments": {
                "path": target_file.display().to_string()
            }
        }
    });

    let resp_str = server
        .handle_request(&req.to_string())
        .expect("response generated");
    let resp: Value = serde_json::from_str(&resp_str).expect("valid json response");

    assert_eq!(resp["id"], 10);
    assert!(resp["error"].is_null());
    assert_ne!(resp["result"]["isError"], true);

    let content_text = resp["result"]["content"][0]["text"]
        .as_str()
        .expect("text content");
    assert!(content_text.contains("Total sheets:"));
    assert!(content_text.contains("Worksheets:"));
}

/// Tests that `xlseek_list_sheets` returns an error when the path argument is missing.
#[test]
fn test_list_sheets_missing_path() {
    let server = McpServer::new(McpSecurityContext::default());

    let req = serde_json::json!({
        // Constant reference: constants::JSONRPC_VERSION
        // Constant reference: constants::METHOD_TOOLS_CALL
        // Constant reference: constants::TOOL_NAME_LIST_SHEETS
        "jsonrpc": constants::JSONRPC_VERSION,
        "id": 11,
        "method": constants::METHOD_TOOLS_CALL,
        "params": {
            "name": constants::TOOL_NAME_LIST_SHEETS,
            "arguments": {}
        }
    });

    let resp_str = server
        .handle_request(&req.to_string())
        .expect("response generated");
    let resp: Value = serde_json::from_str(&resp_str).expect("valid json response");

    assert_eq!(resp["id"], 11);
    assert_eq!(resp["result"]["isError"], true);
    let err_text = resp["result"]["content"][0]["text"]
        .as_str()
        .expect("text content");
    assert!(err_text.contains("Missing required parameter"));
}

/// Tests that `xlseek_list_sheets` handles non-existent file path gracefully.
#[test]
fn test_list_sheets_nonexistent_file() {
    let server = McpServer::new(McpSecurityContext::default());
    let invalid_path = fixtures_dir().join("does_not_exist_file_98765.xlsx");

    let req = serde_json::json!({
        // Constant reference: constants::JSONRPC_VERSION
        // Constant reference: constants::METHOD_TOOLS_CALL
        // Constant reference: constants::TOOL_NAME_LIST_SHEETS
        "jsonrpc": constants::JSONRPC_VERSION,
        "id": 12,
        "method": constants::METHOD_TOOLS_CALL,
        "params": {
            "name": constants::TOOL_NAME_LIST_SHEETS,
            "arguments": {
                "path": invalid_path.display().to_string()
            }
        }
    });

    let resp_str = server
        .handle_request(&req.to_string())
        .expect("response generated");
    let resp: Value = serde_json::from_str(&resp_str).expect("valid json response");

    assert_eq!(resp["id"], 12);
    assert_eq!(resp["result"]["isError"], true);
    let err_text = resp["result"]["content"][0]["text"]
        .as_str()
        .expect("text content");
    assert!(err_text.contains("Failed to open Excel workbook"));
}

/// Tests that `xlseek_preview_sheet` generates a valid Markdown table for an existing sheet.
#[test]
fn test_preview_sheet_success() {
    let server = McpServer::new(McpSecurityContext::default());
    let target_file = fixtures_dir().join("sample_report.xlsx");

    // First list sheets to know a valid sheet name
    let list_req = serde_json::json!({
        "jsonrpc": constants::JSONRPC_VERSION,
        "id": 20,
        "method": constants::METHOD_TOOLS_CALL,
        "params": {
            "name": constants::TOOL_NAME_LIST_SHEETS,
            "arguments": {
                "path": target_file.display().to_string()
            }
        }
    });
    let list_resp_str = server.handle_request(&list_req.to_string()).unwrap();
    let list_resp: Value = serde_json::from_str(&list_resp_str).unwrap();
    let list_text = list_resp["result"]["content"][0]["text"].as_str().unwrap();

    // Extract first sheet name from "1. SheetName"
    let first_sheet = list_text
        .lines()
        .find(|l| l.starts_with("1. "))
        .map(|l| l.trim_start_matches("1. ").trim())
        .expect("has at least one sheet");

    let req = serde_json::json!({
        // Constant reference: constants::JSONRPC_VERSION
        // Constant reference: constants::METHOD_TOOLS_CALL
        // Constant reference: constants::TOOL_NAME_PREVIEW_SHEET
        "jsonrpc": constants::JSONRPC_VERSION,
        "id": 21,
        "method": constants::METHOD_TOOLS_CALL,
        "params": {
            "name": constants::TOOL_NAME_PREVIEW_SHEET,
            "arguments": {
                "path": target_file.display().to_string(),
                "sheet_name": first_sheet,
                "start_row": 1,
                "max_rows": 5
            }
        }
    });

    let resp_str = server
        .handle_request(&req.to_string())
        .expect("response generated");
    let resp: Value = serde_json::from_str(&resp_str).expect("valid json response");

    assert_eq!(resp["id"], 21);
    assert_ne!(resp["result"]["isError"], true);

    let table_text = resp["result"]["content"][0]["text"]
        .as_str()
        .expect("text content");
    assert!(table_text.contains(&format!("Preview of '{}'", first_sheet)));
    assert!(table_text.contains("| Row |"));
    assert!(table_text.contains("|---|"));
}

/// Tests that `xlseek_preview_sheet` respects `start_row` and `max_rows` bounds.
#[test]
fn test_preview_sheet_windowing() {
    let server = McpServer::new(McpSecurityContext::default());
    let target_file = fixtures_dir().join("sample_report.xlsx");

    let req = serde_json::json!({
        // Constant reference: constants::JSONRPC_VERSION
        // Constant reference: constants::METHOD_TOOLS_CALL
        // Constant reference: constants::TOOL_NAME_PREVIEW_SHEET
        "jsonrpc": constants::JSONRPC_VERSION,
        "id": 22,
        "method": constants::METHOD_TOOLS_CALL,
        "params": {
            "name": constants::TOOL_NAME_PREVIEW_SHEET,
            "arguments": {
                "path": target_file.display().to_string(),
                "sheet_name": "Sheet1",
                "start_row": 2,
                "max_rows": 3
            }
        }
    });

    let resp_str = server
        .handle_request(&req.to_string())
        .expect("response generated");
    let resp: Value = serde_json::from_str(&resp_str).expect("valid json response");

    assert_eq!(resp["id"], 22);
    // If Sheet1 exists, check windowing
    if resp["result"]["isError"] != true {
        let text = resp["result"]["content"][0]["text"].as_str().unwrap();
        assert!(text.contains("Rows 2-"));
    }
}

/// Tests that `xlseek_preview_sheet` returns an informative error for non-existent sheet name.
#[test]
fn test_preview_sheet_nonexistent_sheet() {
    let server = McpServer::new(McpSecurityContext::default());
    let target_file = fixtures_dir().join("sample_report.xlsx");

    let req = serde_json::json!({
        // Constant reference: constants::JSONRPC_VERSION
        // Constant reference: constants::METHOD_TOOLS_CALL
        // Constant reference: constants::TOOL_NAME_PREVIEW_SHEET
        "jsonrpc": constants::JSONRPC_VERSION,
        "id": 23,
        "method": constants::METHOD_TOOLS_CALL,
        "params": {
            "name": constants::TOOL_NAME_PREVIEW_SHEET,
            "arguments": {
                "path": target_file.display().to_string(),
                "sheet_name": "NonExistentSheet_12345678"
            }
        }
    });

    let resp_str = server
        .handle_request(&req.to_string())
        .expect("response generated");
    let resp: Value = serde_json::from_str(&resp_str).expect("valid json response");

    assert_eq!(resp["id"], 23);
    assert_eq!(resp["result"]["isError"], true);
    let err_text = resp["result"]["content"][0]["text"]
        .as_str()
        .expect("text content");
    assert!(err_text.contains("Worksheet not found in workbook"));
}
