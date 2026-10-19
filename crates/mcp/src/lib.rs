//! # MCP Server Core Library (crates/mcp/src/lib.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Core orchestrator and standard input/output (stdio) event loop for `xlseek-mcp`.
//! Dispatches JSON-RPC 2.0 requests, handles protocol lifecycle (`initialize`, `ping`),
//! tools discovery (`tools/list`), and tool execution (`tools/call`).
//! Conforms to Constitution Principle I (English text), Principle II (No Hardcoded Constants),
//! Principle III (Comprehensive Header Comments), Principle IV (Modular Architecture),
//! and Principle V (Robust Error Handling & Test Verification).

pub mod constants;
pub mod protocol;
pub mod security;
pub mod tools;

use protocol::{
    InitializeResult, JsonRpcError, JsonRpcRequest, JsonRpcResponse, ServerCapabilities,
    ServerInfo, ToolCallParams, ToolListResult, ToolsCapability,
};
use security::McpSecurityContext;
use serde_json::Value;
use std::io::{BufRead, Write};

/// MCP Server dispatcher and connection state manager.
///
/// ## Description
/// Maintains active security boundaries and routes incoming JSON-RPC lines
/// from `stdin` to appropriate protocol and tool handlers, outputting responses to `stdout`.
#[derive(Debug, Clone)]
pub struct McpServer {
    /// Active security context enforcing directory whitelist constraints.
    security: McpSecurityContext,
}

impl McpServer {
    /// Constructs a new `McpServer` instance.
    ///
    /// ## Arguments
    /// - `security`: `McpSecurityContext` - Configured path security context.
    ///
    /// ## Returns
    /// An initialized `McpServer`.
    ///
    /// ## Errors
    /// Does not panic.
    pub fn new(security: McpSecurityContext) -> Self {
        Self { security }
    }

    /// Accessor for the active security context.
    ///
    /// ## Arguments
    /// None.
    ///
    /// ## Returns
    /// Reference to `McpSecurityContext`.
    ///
    /// ## Errors
    /// Does not panic.
    pub fn security(&self) -> &McpSecurityContext {
        &self.security
    }

    /// Processes a single incoming line from the client.
    ///
    /// ## Arguments
    /// - `line`: `&str` - Raw single-line string read from `stdin`.
    ///
    /// ## Returns
    /// - `Some(String)`: Serialized JSON-RPC 2.0 response to send back to client.
    /// - `None`: If the incoming message was a notification requiring no response, or empty whitespace.
    ///
    /// ## Errors
    /// Parse failures and unknown methods generate standard JSON-RPC error responses without crashing.
    pub fn handle_request(&self, line: &str) -> Option<String> {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return None;
        }

        let request: JsonRpcRequest = match serde_json::from_str(trimmed) {
            Ok(req) => req,
            Err(e) => {
                let err_resp = JsonRpcResponse::error(
                    Value::Null,
                    JsonRpcError::parse_error(format!("Failed to parse JSON: {e}")),
                );
                return serde_json::to_string(&err_resp).ok();
            }
        };

        // If message is a notification (no id provided), do not send a response
        let id = match request.id {
            Some(id) => id,
            None => {
                // Constant reference: crate::constants::METHOD_NOTIFICATIONS_INITIALIZED
                if request.method == crate::constants::METHOD_NOTIFICATIONS_INITIALIZED {
                    eprintln!("Client notification: initialized");
                }
                return None;
            }
        };

        let response = match request.method.as_str() {
            // Constant reference: crate::constants::METHOD_INITIALIZE
            crate::constants::METHOD_INITIALIZE => {
                let result = InitializeResult {
                    // Constant reference: crate::constants::MCP_PROTOCOL_VERSION
                    protocol_version: crate::constants::MCP_PROTOCOL_VERSION.to_string(),
                    capabilities: ServerCapabilities {
                        tools: Some(ToolsCapability::default()),
                    },
                    server_info: ServerInfo {
                        // Constant reference: crate::constants::SERVER_NAME
                        // Constant reference: crate::constants::SERVER_VERSION
                        name: crate::constants::SERVER_NAME.to_string(),
                        version: crate::constants::SERVER_VERSION.to_string(),
                    },
                };
                match serde_json::to_value(result) {
                    Ok(val) => JsonRpcResponse::success(id, val),
                    Err(e) => {
                        JsonRpcResponse::error(id, JsonRpcError::internal_error(e.to_string()))
                    }
                }
            }
            // Constant reference: crate::constants::METHOD_PING
            crate::constants::METHOD_PING => JsonRpcResponse::success(id, serde_json::json!({})),
            // Constant reference: crate::constants::METHOD_TOOLS_LIST
            crate::constants::METHOD_TOOLS_LIST => {
                let list_result = ToolListResult {
                    tools: tools::list_tool_definitions(),
                };
                match serde_json::to_value(list_result) {
                    Ok(val) => JsonRpcResponse::success(id, val),
                    Err(e) => {
                        JsonRpcResponse::error(id, JsonRpcError::internal_error(e.to_string()))
                    }
                }
            }
            // Constant reference: crate::constants::METHOD_TOOLS_CALL
            crate::constants::METHOD_TOOLS_CALL => {
                let params = match request.params {
                    Some(p) => p,
                    None => {
                        return serde_json::to_string(&JsonRpcResponse::error(
                            id,
                            JsonRpcError::invalid_params("Missing params object for tools/call"),
                        ))
                        .ok();
                    }
                };
                let tool_call_params: ToolCallParams = match serde_json::from_value(params) {
                    Ok(p) => p,
                    Err(e) => {
                        return serde_json::to_string(&JsonRpcResponse::error(
                            id,
                            JsonRpcError::invalid_params(format!("Invalid tools/call params: {e}")),
                        ))
                        .ok();
                    }
                };
                let tool_result = tools::dispatch_tool_call(
                    &tool_call_params.name,
                    &tool_call_params.arguments,
                    &self.security,
                );
                match serde_json::to_value(tool_result) {
                    Ok(val) => JsonRpcResponse::success(id, val),
                    Err(e) => {
                        JsonRpcResponse::error(id, JsonRpcError::internal_error(e.to_string()))
                    }
                }
            }
            unknown => JsonRpcResponse::error(id, JsonRpcError::method_not_found(unknown)),
        };

        serde_json::to_string(&response).ok()
    }

    /// Runs the synchronous stdio message loop until EOF is encountered on the reader.
    ///
    /// ## Arguments
    /// - `reader`: `R` - Any input stream implementing `BufRead` (typically `stdin.lock()`).
    /// - `writer`: `W` - Any output stream implementing `Write` (typically `stdout.lock()`).
    ///
    /// ## Returns
    /// - `Ok(())`: When reader closes normally on EOF.
    /// - `Err(std::io::Error)`: On fatal stream I/O errors.
    ///
    /// ## Errors
    /// I/O write failures return `Err`. Does not panic.
    pub fn run<R: BufRead, W: Write>(&self, reader: R, mut writer: W) -> std::io::Result<()> {
        for line_result in reader.lines() {
            let line = line_result?;
            if let Some(response_json) = self.handle_request(&line) {
                writer.write_all(response_json.as_bytes())?;
                writer.write_all(b"\n")?;
                writer.flush()?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests initialize method handshake returns correct server metadata and tools capability.
    #[test]
    fn test_server_initialize() {
        let server = McpServer::new(McpSecurityContext::default());
        let req = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{}}}"#;
        let resp = server.handle_request(req).expect("response generated");

        assert!(resp.contains(r#""id":1"#));
        assert!(resp.contains(crate::constants::MCP_PROTOCOL_VERSION));
        assert!(resp.contains(crate::constants::SERVER_NAME));
    }

    /// Tests ping method responds with empty object.
    #[test]
    fn test_server_ping() {
        let server = McpServer::new(McpSecurityContext::default());
        let req = r#"{"jsonrpc":"2.0","id":2,"method":"ping"}"#;
        let resp = server.handle_request(req).expect("response generated");

        assert!(resp.contains(r#""id":2"#));
        assert!(resp.contains(r#""result":{}"#));
    }

    /// Tests unknown method returns method not found error (-32601).
    #[test]
    fn test_server_unknown_method() {
        let server = McpServer::new(McpSecurityContext::default());
        let req = r#"{"jsonrpc":"2.0","id":3,"method":"unknown/method"}"#;
        let resp = server.handle_request(req).expect("response generated");

        assert!(resp.contains(r#""id":3"#));
        assert!(resp.contains("-32601"));
        assert!(resp.contains(crate::constants::ERR_METHOD_NOT_FOUND_PREFIX));
    }

    /// Tests notification message does not emit a response.
    #[test]
    fn test_server_notification_silent() {
        let server = McpServer::new(McpSecurityContext::default());
        let notification = r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#;
        let resp = server.handle_request(notification);
        assert!(resp.is_none());
    }
}
