//! # MCP Protocol Module (crates/mcp/src/protocol/mod.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Aggregates JSON-RPC 2.0 framing, standard error types, and MCP handshake/tool definitions.
//! Conforms to Constitution Principle I (English text), Principle III (Comprehensive Header Comments),
//! and Principle IV (Modular Architecture).

pub mod error;
pub mod messages;

pub use error::JsonRpcError;
pub use messages::{
    ClientInfo, InitializeParams, InitializeResult, JsonRpcRequest, JsonRpcResponse,
    ServerCapabilities, ServerInfo, ToolCallParams, ToolCallResult, ToolContent, ToolDefinition,
    ToolListResult, ToolsCapability,
};
