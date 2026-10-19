//! # JSON-RPC 2.0 Error Module (crates/mcp/src/protocol/error.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Implements standard JSON-RPC 2.0 error structures and factory helpers
//! according to the specification and MCP guidelines.
//! Conforms to Constitution Principle I (English text), Principle II (No Hardcoded Constants),
//! Principle III (Comprehensive Header Comments), and Principle V (Robust Error Handling).

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Structured JSON-RPC 2.0 error object.
///
/// ## Description
/// Encapsulates error codes, descriptive messages, and optional contextual data
/// returned in the `error` field of an invalid JSON-RPC response.
///
/// ## Errors
/// Serializes to valid JSON-RPC 2.0 error representation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JsonRpcError {
    /// Integer error code identifying the failure category.
    pub code: i32,
    /// Concise, human-readable summary of the error.
    pub message: String,
    /// Optional supplementary data payload providing debugging context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl JsonRpcError {
    /// Constructs a standard parse error (-32700).
    ///
    /// ## Arguments
    /// - `details`: `D` - Description of the parse failure.
    ///
    /// ## Returns
    /// `JsonRpcError` with standard parse error code.
    ///
    /// ## Errors
    /// Does not panic.
    pub fn parse_error<D: Into<String>>(details: D) -> Self {
        // Constant reference: crate::constants::ERROR_CODE_PARSE_ERROR
        Self {
            code: crate::constants::ERROR_CODE_PARSE_ERROR,
            message: details.into(),
            data: None,
        }
    }

    /// Constructs a standard invalid request error (-32600).
    ///
    /// ## Arguments
    /// - `details`: `D` - Description of why the request is malformed.
    ///
    /// ## Returns
    /// `JsonRpcError` with invalid request error code.
    ///
    /// ## Errors
    /// Does not panic.
    pub fn invalid_request<D: Into<String>>(details: D) -> Self {
        // Constant reference: crate::constants::ERROR_CODE_INVALID_REQUEST
        Self {
            code: crate::constants::ERROR_CODE_INVALID_REQUEST,
            message: details.into(),
            data: None,
        }
    }

    /// Constructs a standard method not found error (-32601).
    ///
    /// ## Arguments
    /// - `method`: `&str` - The requested unknown method name.
    ///
    /// ## Returns
    /// `JsonRpcError` with method not found code.
    ///
    /// ## Errors
    /// Does not panic.
    pub fn method_not_found(method: &str) -> Self {
        // Constant reference: crate::constants::ERROR_CODE_METHOD_NOT_FOUND
        // Constant reference: crate::constants::ERR_METHOD_NOT_FOUND_PREFIX
        Self {
            code: crate::constants::ERROR_CODE_METHOD_NOT_FOUND,
            message: format!(
                "{}: {}",
                crate::constants::ERR_METHOD_NOT_FOUND_PREFIX,
                method
            ),
            data: None,
        }
    }

    /// Constructs a standard invalid params error (-32602).
    ///
    /// ## Arguments
    /// - `details`: `D` - Description of parameter mismatch or validation failure.
    ///
    /// ## Returns
    /// `JsonRpcError` with invalid params error code.
    ///
    /// ## Errors
    /// Does not panic.
    pub fn invalid_params<D: Into<String>>(details: D) -> Self {
        // Constant reference: crate::constants::ERROR_CODE_INVALID_PARAMS
        // Constant reference: crate::constants::ERR_INVALID_PARAMS_PREFIX
        Self {
            code: crate::constants::ERROR_CODE_INVALID_PARAMS,
            message: format!(
                "{}: {}",
                crate::constants::ERR_INVALID_PARAMS_PREFIX,
                details.into()
            ),
            data: None,
        }
    }

    /// Constructs a standard internal server error (-32603).
    ///
    /// ## Arguments
    /// - `details`: `D` - Description of internal execution error.
    ///
    /// ## Returns
    /// `JsonRpcError` with internal error code.
    ///
    /// ## Errors
    /// Does not panic.
    pub fn internal_error<D: Into<String>>(details: D) -> Self {
        // Constant reference: crate::constants::ERROR_CODE_INTERNAL_ERROR
        Self {
            code: crate::constants::ERROR_CODE_INTERNAL_ERROR,
            message: details.into(),
            data: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests creation and field mapping of standard error constructors.
    #[test]
    fn test_error_factories() {
        let err = JsonRpcError::method_not_found("unknown_cmd");
        assert_eq!(err.code, crate::constants::ERROR_CODE_METHOD_NOT_FOUND);
        assert!(err.message.contains("unknown_cmd"));

        let parse_err = JsonRpcError::parse_error("invalid json");
        assert_eq!(parse_err.code, crate::constants::ERROR_CODE_PARSE_ERROR);
    }
}
