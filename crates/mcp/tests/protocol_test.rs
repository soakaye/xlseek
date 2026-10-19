//! # Protocol Integration Tests (crates/mcp/tests/protocol_test.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Integration tests verifying MCP JSON-RPC 2.0 lifecycle, handshake, and error handling.
//! Conforms to Constitution Principle I (English text), Principle II (No Hardcoded Constants),
//! Principle III (Comprehensive Header Comments), and Principle V (Test Verification).

use serde_json::Value;
use xlseek_mcp::constants;
use xlseek_mcp::security::McpSecurityContext;
use xlseek_mcp::McpServer;

/// Tests the end-to-end `initialize` handshake returning server capabilities and metadata.
#[test]
fn test_initialize_handshake_integration() {
    let server = McpServer::new(McpSecurityContext::default());
    let req = serde_json::json!({
        // Constant reference: constants::JSONRPC_VERSION
        // Constant reference: constants::METHOD_INITIALIZE
        // Constant reference: constants::MCP_PROTOCOL_VERSION
        "jsonrpc": constants::JSONRPC_VERSION,
        "id": 100,
        "method": constants::METHOD_INITIALIZE,
        "params": {
            "protocolVersion": constants::MCP_PROTOCOL_VERSION,
            "capabilities": {},
            "clientInfo": { "name": "test-suite", "version": "1.0" }
        }
    });

    let resp_str = server
        .handle_request(&req.to_string())
        .expect("response generated");
    let resp: Value = serde_json::from_str(&resp_str).expect("valid json response");

    assert_eq!(resp["jsonrpc"], constants::JSONRPC_VERSION);
    assert_eq!(resp["id"], 100);
    assert_eq!(
        resp["result"]["protocolVersion"],
        constants::MCP_PROTOCOL_VERSION
    );
    assert_eq!(resp["result"]["serverInfo"]["name"], constants::SERVER_NAME);
    assert!(resp["result"]["capabilities"]["tools"].is_object());
}

/// Tests `tools/list` discovery returns all required tool schemas with valid input definitions.
#[test]
fn test_tools_list_integration() {
    let server = McpServer::new(McpSecurityContext::default());
    let req = serde_json::json!({
        // Constant reference: constants::JSONRPC_VERSION
        // Constant reference: constants::METHOD_TOOLS_LIST
        "jsonrpc": constants::JSONRPC_VERSION,
        "id": 101,
        "method": constants::METHOD_TOOLS_LIST,
        "params": {}
    });

    let resp_str = server
        .handle_request(&req.to_string())
        .expect("response generated");
    let resp: Value = serde_json::from_str(&resp_str).expect("valid json response");

    assert_eq!(resp["id"], 101);
    let tools = resp["result"]["tools"].as_array().expect("tools array");
    assert_eq!(tools.len(), 3);

    let tool_names: Vec<&str> = tools.iter().filter_map(|t| t["name"].as_str()).collect();

    // Constant reference: constants::TOOL_NAME_SEARCH
    // Constant reference: constants::TOOL_NAME_LIST_SHEETS
    // Constant reference: constants::TOOL_NAME_PREVIEW_SHEET
    assert!(tool_names.contains(&constants::TOOL_NAME_SEARCH));
    assert!(tool_names.contains(&constants::TOOL_NAME_LIST_SHEETS));
    assert!(tool_names.contains(&constants::TOOL_NAME_PREVIEW_SHEET));

    for tool in tools {
        assert!(tool["inputSchema"]["type"].is_string());
        assert!(tool["description"].is_string());
    }
}

/// Tests that broken JSON payloads return standard JSON-RPC parse error (-32700).
#[test]
fn test_invalid_json_parse_error() {
    let server = McpServer::new(McpSecurityContext::default());
    let invalid_json = "{ invalid_json_payload ";

    let resp_str = server
        .handle_request(invalid_json)
        .expect("response generated");
    let resp: Value = serde_json::from_str(&resp_str).expect("valid json response");

    // Constant reference: constants::ERROR_CODE_PARSE_ERROR
    assert_eq!(resp["error"]["code"], constants::ERROR_CODE_PARSE_ERROR);
}

/// Tests that requesting an unregistered method returns -32601.
#[test]
fn test_unknown_method_error() {
    let server = McpServer::new(McpSecurityContext::default());
    let req = serde_json::json!({
        // Constant reference: constants::JSONRPC_VERSION
        "jsonrpc": constants::JSONRPC_VERSION,
        "id": 999,
        "method": "system/shutdown",
        "params": {}
    });

    let resp_str = server
        .handle_request(&req.to_string())
        .expect("response generated");
    let resp: Value = serde_json::from_str(&resp_str).expect("valid json response");

    // Constant reference: constants::ERROR_CODE_METHOD_NOT_FOUND
    assert_eq!(resp["id"], 999);
    assert_eq!(
        resp["error"]["code"],
        constants::ERROR_CODE_METHOD_NOT_FOUND
    );
}
