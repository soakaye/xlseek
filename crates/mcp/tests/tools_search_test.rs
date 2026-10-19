//! # Search Tool Integration Tests (crates/mcp/tests/tools_search_test.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Verifies cross-document search execution via the `xlseek_search` tool over MCP.
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

/// Tests that `xlseek_search` successfully finds matching cells in test fixtures.
#[test]
fn test_search_tool_basic_hit() {
    let server = McpServer::new(McpSecurityContext::default());
    let fixtures = fixtures_dir();

    let req = serde_json::json!({
        // Constant reference: constants::JSONRPC_VERSION
        // Constant reference: constants::METHOD_TOOLS_CALL
        // Constant reference: constants::TOOL_NAME_SEARCH
        "jsonrpc": constants::JSONRPC_VERSION,
        "id": 1,
        "method": constants::METHOD_TOOLS_CALL,
        "params": {
            "name": constants::TOOL_NAME_SEARCH,
            "arguments": {
                "paths": [fixtures.display().to_string()],
                "query": "Revenue",
                "match_case": false
            }
        }
    });

    let resp_str = server
        .handle_request(&req.to_string())
        .expect("response generated");
    let resp: Value = serde_json::from_str(&resp_str).expect("valid json response");

    assert_eq!(resp["id"], 1);
    let text = resp["result"]["content"][0]["text"]
        .as_str()
        .expect("text content");
    assert!(text.contains("Found") || text.contains("match"));
    assert!(resp["result"]["isError"].is_null());
}

/// Tests that searching for a non-existent keyword returns a 0-match success response.
#[test]
fn test_search_tool_no_matches() {
    let server = McpServer::new(McpSecurityContext::default());
    let fixtures = fixtures_dir();

    let req = serde_json::json!({
        // Constant reference: constants::JSONRPC_VERSION
        // Constant reference: constants::METHOD_TOOLS_CALL
        // Constant reference: constants::TOOL_NAME_SEARCH
        "jsonrpc": constants::JSONRPC_VERSION,
        "id": 2,
        "method": constants::METHOD_TOOLS_CALL,
        "params": {
            "name": constants::TOOL_NAME_SEARCH,
            "arguments": {
                "paths": fixtures.display().to_string(),
                "query": "NonExistentString12345XYZ"
            }
        }
    });

    let resp_str = server
        .handle_request(&req.to_string())
        .expect("response generated");
    let resp: Value = serde_json::from_str(&resp_str).expect("valid json response");

    assert_eq!(resp["id"], 2);
    let text = resp["result"]["content"][0]["text"]
        .as_str()
        .expect("text content");
    assert!(text.contains("No matches found"));
    assert!(resp["result"]["isError"].is_null());
}

/// Tests that `max_results` limits the number of returned hit items.
#[test]
fn test_search_tool_max_results_truncation() {
    let server = McpServer::new(McpSecurityContext::default());
    let fixtures = fixtures_dir();

    let req = serde_json::json!({
        // Constant reference: constants::JSONRPC_VERSION
        // Constant reference: constants::METHOD_TOOLS_CALL
        // Constant reference: constants::TOOL_NAME_SEARCH
        "jsonrpc": constants::JSONRPC_VERSION,
        "id": 3,
        "method": constants::METHOD_TOOLS_CALL,
        "params": {
            "name": constants::TOOL_NAME_SEARCH,
            "arguments": {
                "paths": [fixtures.display().to_string()],
                "query": "e",
                "max_results": 1
            }
        }
    });

    let resp_str = server
        .handle_request(&req.to_string())
        .expect("response generated");
    let resp: Value = serde_json::from_str(&resp_str).expect("valid json response");

    assert_eq!(resp["id"], 3);
    let text = resp["result"]["content"][0]["text"]
        .as_str()
        .expect("text content");
    assert!(text.contains("truncated to maximum 1 results"));
}

/// Tests that missing mandatory arguments produces an error response without crashing.
#[test]
fn test_search_tool_missing_query_error() {
    let server = McpServer::new(McpSecurityContext::default());

    let req = serde_json::json!({
        // Constant reference: constants::JSONRPC_VERSION
        // Constant reference: constants::METHOD_TOOLS_CALL
        // Constant reference: constants::TOOL_NAME_SEARCH
        "jsonrpc": constants::JSONRPC_VERSION,
        "id": 4,
        "method": constants::METHOD_TOOLS_CALL,
        "params": {
            "name": constants::TOOL_NAME_SEARCH,
            "arguments": {
                "paths": ["crates/core/tests"]
            }
        }
    });

    let resp_str = server
        .handle_request(&req.to_string())
        .expect("response generated");
    let resp: Value = serde_json::from_str(&resp_str).expect("valid json response");

    assert_eq!(resp["id"], 4);
    assert_eq!(resp["result"]["isError"], true);
    let text = resp["result"]["content"][0]["text"]
        .as_str()
        .expect("text content");
    assert!(text.contains("query"));
}

/// Tests searching with a direct single file target.
#[test]
fn test_search_tool_single_file_target() {
    let server = McpServer::new(McpSecurityContext::default());
    let target_file = fixtures_dir().join("sample_report.xlsx");

    let req = serde_json::json!({
        // Constant reference: constants::JSONRPC_VERSION
        // Constant reference: constants::METHOD_TOOLS_CALL
        // Constant reference: constants::TOOL_NAME_SEARCH
        "jsonrpc": constants::JSONRPC_VERSION,
        "id": 5,
        "method": constants::METHOD_TOOLS_CALL,
        "params": {
            "name": constants::TOOL_NAME_SEARCH,
            "arguments": {
                "paths": target_file.display().to_string(),
                "query": "Report"
            }
        }
    });

    let resp_str = server
        .handle_request(&req.to_string())
        .expect("response generated");
    let resp: Value = serde_json::from_str(&resp_str).expect("valid json response");

    assert_eq!(resp["id"], 5);
    let text = resp["result"]["content"][0]["text"]
        .as_str()
        .expect("text content");
    assert!(text.contains("sample_report.xlsx"));
}
