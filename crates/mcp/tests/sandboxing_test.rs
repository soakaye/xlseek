//! # Sandboxing & Error Hardening Integration Tests (crates/mcp/tests/sandboxing_test.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Verifies directory boundary whitelist enforcement, rejection of unauthorized paths,
//! invalid regex error handling, corrupt file resilience, and non-panicking error reporting.
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

/// Tests that `xlseek_search` rejects paths outside the allowed directory whitelist.
#[test]
fn test_security_whitelist_search_rejection() {
    let fixtures = fixtures_dir();
    // Configure security context to allow only a non-existent subfolder
    let restricted_dir = fixtures.join("nonexistent_sandbox");
    let security = McpSecurityContext::new(vec![restricted_dir]);
    let server = McpServer::new(security);

    let req = serde_json::json!({
        // Constant reference: constants::JSONRPC_VERSION
        // Constant reference: constants::METHOD_TOOLS_CALL
        // Constant reference: constants::TOOL_NAME_SEARCH
        "jsonrpc": constants::JSONRPC_VERSION,
        "id": 100,
        "method": constants::METHOD_TOOLS_CALL,
        "params": {
            "name": constants::TOOL_NAME_SEARCH,
            "arguments": {
                "paths": [fixtures.display().to_string()],
                "query": "Revenue"
            }
        }
    });

    let resp_str = server
        .handle_request(&req.to_string())
        .expect("response generated");
    let resp: Value = serde_json::from_str(&resp_str).expect("valid json response");

    assert_eq!(resp["id"], 100);
    assert_eq!(resp["result"]["isError"], true);
    let err_msg = resp["result"]["content"][0]["text"].as_str().unwrap();
    assert!(err_msg.contains("Access denied"));
}

/// Tests that `xlseek_list_sheets` rejects paths outside the allowed directory whitelist.
#[test]
fn test_security_whitelist_list_sheets_rejection() {
    let fixtures = fixtures_dir();
    let restricted_dir = fixtures.join("nonexistent_sandbox");
    let security = McpSecurityContext::new(vec![restricted_dir]);
    let server = McpServer::new(security);

    let target_file = fixtures.join("sample_report.xlsx");

    let req = serde_json::json!({
        // Constant reference: constants::JSONRPC_VERSION
        // Constant reference: constants::METHOD_TOOLS_CALL
        // Constant reference: constants::TOOL_NAME_LIST_SHEETS
        "jsonrpc": constants::JSONRPC_VERSION,
        "id": 101,
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

    assert_eq!(resp["id"], 101);
    assert_eq!(resp["result"]["isError"], true);
    let err_msg = resp["result"]["content"][0]["text"].as_str().unwrap();
    assert!(err_msg.contains("Access denied"));
}

/// Tests that `xlseek_preview_sheet` rejects paths outside the allowed directory whitelist.
#[test]
fn test_security_whitelist_preview_rejection() {
    let fixtures = fixtures_dir();
    let restricted_dir = fixtures.join("nonexistent_sandbox");
    let security = McpSecurityContext::new(vec![restricted_dir]);
    let server = McpServer::new(security);

    let target_file = fixtures.join("sample_report.xlsx");

    let req = serde_json::json!({
        // Constant reference: constants::JSONRPC_VERSION
        // Constant reference: constants::METHOD_TOOLS_CALL
        // Constant reference: constants::TOOL_NAME_PREVIEW_SHEET
        "jsonrpc": constants::JSONRPC_VERSION,
        "id": 102,
        "method": constants::METHOD_TOOLS_CALL,
        "params": {
            "name": constants::TOOL_NAME_PREVIEW_SHEET,
            "arguments": {
                "path": target_file.display().to_string(),
                "sheet_name": "Sheet1"
            }
        }
    });

    let resp_str = server
        .handle_request(&req.to_string())
        .expect("response generated");
    let resp: Value = serde_json::from_str(&resp_str).expect("valid json response");

    assert_eq!(resp["id"], 102);
    assert_eq!(resp["result"]["isError"], true);
    let err_msg = resp["result"]["content"][0]["text"].as_str().unwrap();
    assert!(err_msg.contains("Access denied"));
}

/// Tests that invalid regex query returns an explicit error without crashing.
#[test]
fn test_invalid_regex_error_handling() {
    let server = McpServer::new(McpSecurityContext::default());
    let fixtures = fixtures_dir();

    let req = serde_json::json!({
        // Constant reference: constants::JSONRPC_VERSION
        // Constant reference: constants::METHOD_TOOLS_CALL
        // Constant reference: constants::TOOL_NAME_SEARCH
        "jsonrpc": constants::JSONRPC_VERSION,
        "id": 103,
        "method": constants::METHOD_TOOLS_CALL,
        "params": {
            "name": constants::TOOL_NAME_SEARCH,
            "arguments": {
                "paths": [fixtures.display().to_string()],
                "query": "[a-z(invalid",
                "is_regex": true
            }
        }
    });

    let resp_str = server
        .handle_request(&req.to_string())
        .expect("response generated");
    let resp: Value = serde_json::from_str(&resp_str).expect("valid json response");

    assert_eq!(resp["id"], 103);
    assert_eq!(resp["result"]["isError"], true);
    let err_msg = resp["result"]["content"][0]["text"].as_str().unwrap();
    assert!(err_msg.contains("regex") || err_msg.contains("Regex"));
}

/// Tests that opening an invalid/corrupted Excel file returns an error cleanly.
#[test]
fn test_corrupt_file_handling() {
    let server = McpServer::new(McpSecurityContext::default());
    // Point to Cargo.toml as a non-Excel file
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");

    let req = serde_json::json!({
        // Constant reference: constants::JSONRPC_VERSION
        // Constant reference: constants::METHOD_TOOLS_CALL
        // Constant reference: constants::TOOL_NAME_LIST_SHEETS
        "jsonrpc": constants::JSONRPC_VERSION,
        "id": 104,
        "method": constants::METHOD_TOOLS_CALL,
        "params": {
            "name": constants::TOOL_NAME_LIST_SHEETS,
            "arguments": {
                "path": manifest.display().to_string()
            }
        }
    });

    let resp_str = server
        .handle_request(&req.to_string())
        .expect("response generated");
    let resp: Value = serde_json::from_str(&resp_str).expect("valid json response");

    assert_eq!(resp["id"], 104);
    assert_eq!(resp["result"]["isError"], true);
    let err_msg = resp["result"]["content"][0]["text"].as_str().unwrap();
    assert!(err_msg.contains("Failed to open Excel workbook"));
}
