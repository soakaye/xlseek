//! # JSON-RPC 2.0 & MCP Messages Module (crates/mcp/src/protocol/messages.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Data structures and Serde serializers for JSON-RPC 2.0 wire framing
//! and Model Context Protocol (MCP) standard payloads.
//! Conforms to Constitution Principle I (English text), Principle II (No Hardcoded Constants),
//! Principle III (Comprehensive Header Comments), and Principle V (Robust Error Handling).

use super::error::JsonRpcError;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Top-level incoming JSON-RPC 2.0 request frame.
///
/// ## Description
/// Received over standard input from an MCP client.
/// Represents method calls or notifications (when `id` is None).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    /// Protocol specification version (must be "2.0").
    pub jsonrpc: String,
    /// Identifier established by the client, or None for notifications.
    pub id: Option<Value>,
    /// Target method to invoke.
    pub method: String,
    /// Optional parameter object or array.
    #[serde(default)]
    pub params: Option<Value>,
}

/// Outgoing JSON-RPC 2.0 response frame.
///
/// ## Description
/// Serialized and written to standard output.
/// Contains either a valid `result` or an `error` object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    /// Protocol specification version (always "2.0").
    pub jsonrpc: String,
    /// Identifier matching the original request, or Value::Null.
    pub id: Value,
    /// Result payload for successful invocations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    /// Error payload for failed invocations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

impl JsonRpcResponse {
    /// Creates a successful response frame echoing the request ID.
    ///
    /// ## Arguments
    /// - `id`: `Value` - Request identifier to echo back.
    /// - `result`: `Value` - Result payload data.
    ///
    /// ## Returns
    /// `JsonRpcResponse` with result payload.
    ///
    /// ## Errors
    /// Does not panic.
    pub fn success(id: Value, result: Value) -> Self {
        // Constant reference: crate::constants::JSONRPC_VERSION
        Self {
            jsonrpc: crate::constants::JSONRPC_VERSION.to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    /// Creates an error response frame echoing the request ID.
    ///
    /// ## Arguments
    /// - `id`: `Value` - Request identifier to echo back (or Value::Null).
    /// - `error`: `JsonRpcError` - Error structure describing the failure.
    ///
    /// ## Returns
    /// `JsonRpcResponse` with error payload.
    ///
    /// ## Errors
    /// Does not panic.
    pub fn error(id: Value, error: JsonRpcError) -> Self {
        // Constant reference: crate::constants::JSONRPC_VERSION
        Self {
            jsonrpc: crate::constants::JSONRPC_VERSION.to_string(),
            id,
            result: None,
            error: Some(error),
        }
    }
}

/// Information identifying the client sending an `initialize` request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    /// Human-readable client application name.
    pub name: String,
    /// Client application version string.
    pub version: String,
}

/// Parameters passed to the `initialize` method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeParams {
    /// MCP protocol version requested by the client.
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    /// Capabilities advertised by the client.
    pub capabilities: Value,
    /// Optional metadata identifying the client application.
    #[serde(rename = "clientInfo")]
    pub client_info: Option<ClientInfo>,
}

/// Server identification details returned during handshake.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    /// Registered server binary name.
    pub name: String,
    /// Server release version string.
    pub version: String,
}

/// Tools capability declaration block.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToolsCapability {}

/// Server capabilities declared during initialization.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServerCapabilities {
    /// Indicates whether tool discovery and execution are supported.
    pub tools: Option<ToolsCapability>,
}

/// Response payload returned by the `initialize` method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeResult {
    /// Protocol version accepted and used by the server.
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    /// Declared capabilities supported by this server instance.
    pub capabilities: ServerCapabilities,
    /// Metadata identifying the server name and version.
    #[serde(rename = "serverInfo")]
    pub server_info: ServerInfo,
}

/// Description and JSON Schema contract of an exposed MCP tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    /// Unique identifier for invoking the tool.
    pub name: String,
    /// Human-readable explanation of tool behavior.
    pub description: String,
    /// JSON Schema object describing valid arguments.
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
}

/// Response payload returned by the `tools/list` method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolListResult {
    /// Array of tool definitions supported by this server.
    pub tools: Vec<ToolDefinition>,
}

/// Parameters passed to the `tools/call` method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallParams {
    /// Name of the tool to execute.
    pub name: String,
    /// Arguments payload passed to the tool.
    #[serde(default)]
    pub arguments: Value,
}

/// Content block returned inside a tool execution result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolContent {
    /// MIME or category type, typically "text".
    #[serde(rename = "type")]
    pub content_type: String,
    /// Formatted content text string.
    pub text: String,
}

impl ToolContent {
    /// Creates a text content block.
    ///
    /// ## Arguments
    /// - `text`: `T` - Textual content to send to client.
    ///
    /// ## Returns
    /// `ToolContent` of type "text".
    ///
    /// ## Errors
    /// Does not panic.
    pub fn text<T: Into<String>>(text: T) -> Self {
        // Constant reference: crate::constants::CONTENT_TYPE_TEXT
        Self {
            content_type: crate::constants::CONTENT_TYPE_TEXT.to_string(),
            text: text.into(),
        }
    }
}

/// Result payload returned by the `tools/call` method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallResult {
    /// Array of content fragments.
    pub content: Vec<ToolContent>,
    /// Optional flag indicating a domain-level tool failure.
    #[serde(rename = "isError", skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
}

impl ToolCallResult {
    /// Creates a successful tool execution result containing a single text block.
    ///
    /// ## Arguments
    /// - `text`: `T` - Output text to deliver to the LLM.
    ///
    /// ## Returns
    /// `ToolCallResult` with single text item and `is_error` unset.
    ///
    /// ## Errors
    /// Does not panic.
    pub fn success<T: Into<String>>(text: T) -> Self {
        Self {
            content: vec![ToolContent::text(text)],
            is_error: None,
        }
    }

    /// Creates an error tool execution result containing an error description.
    ///
    /// ## Arguments
    /// - `error_text`: `T` - Explanatory error text to deliver to the LLM.
    ///
    /// ## Returns
    /// `ToolCallResult` with single text item and `is_error = Some(true)`.
    ///
    /// ## Errors
    /// Does not panic.
    pub fn error<T: Into<String>>(error_text: T) -> Self {
        Self {
            content: vec![ToolContent::text(error_text)],
            is_error: Some(true),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests serialization and deserialization roundtrip of JSON-RPC request and response.
    #[test]
    fn test_jsonrpc_request_response_roundtrip() {
        let req_json = r#"{"jsonrpc":"2.0","id":42,"method":"ping"}"#;
        let req: JsonRpcRequest = serde_json::from_str(req_json).expect("valid request");
        assert_eq!(req.jsonrpc, crate::constants::JSONRPC_VERSION);
        assert_eq!(req.id, Some(Value::from(42)));
        assert_eq!(req.method, crate::constants::METHOD_PING);

        let resp = JsonRpcResponse::success(Value::from(42), serde_json::json!({ "status": "ok" }));
        let serialized = serde_json::to_string(&resp).expect("valid response serialization");
        assert!(serialized.contains(r#""id":42"#));
        assert!(serialized.contains(r#""jsonrpc":"2.0""#));
        assert!(serialized.contains(r#""result":{"status":"ok"}"#));
    }

    /// Tests tool call result constructor helpers.
    #[test]
    fn test_tool_call_result_factories() {
        let ok = ToolCallResult::success("hello");
        assert_eq!(ok.content.len(), 1);
        assert_eq!(ok.content[0].text, "hello");
        assert!(ok.is_error.is_none());

        let err = ToolCallResult::error("failed");
        assert_eq!(err.content[0].text, "failed");
        assert_eq!(err.is_error, Some(true));
    }
}
