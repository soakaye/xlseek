//! # JSON Export Module (export/json_export.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Formats and outputs matched search items into a structured JSON (.json) document.
//! Generates top-level summary metadata (`total_matches`) and a `matches` array with
//! normalized filesystem paths matching the format used by the `xlseek-mcp` search tool.
//! Conforms to Constitution Principle I (English code/comments), Principle II (Constant references),
//! Principle III (Header comments), and Principle IV (Clippy compliance).

use crate::models::{MatchType, SearchMatch};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

/// Top-level payload structure representing the complete JSON export document.
///
/// ## Arguments / Returns
/// Holds total matches count and ordered list of formatted match items.
///
/// ## Errors
/// Data transfer struct that does not produce runtime errors.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct JsonExportPayload {
    pub total_matches: usize,
    pub matches: Vec<JsonExportMatch>,
}

/// Structured record representing an individual search match within exported JSON.
///
/// ## Arguments / Returns
/// Contains document location, match coordinates, match type, cell content, snippet,
/// and optional formula and drawing shape metadata.
///
/// ## Errors
/// Data transfer struct that does not produce runtime errors.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct JsonExportMatch {
    pub id: u64,
    pub file_name: String,
    pub full_path: String,
    pub sheet_name: String,
    pub cell_address: String,
    pub row_index: u32,
    pub col_index: u32,
    pub match_type: MatchType,
    pub full_content: String,
    pub snippet: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formula: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape_name: Option<String>,
}

/// ## Description
/// Formats a slice of search result items as 2-space pretty-printed JSON to a writable stream.
///
/// ## Arguments
/// - `writer`: `&mut W` - Target writable stream
/// - `items`: `&[SearchMatch]` - Slice of search matches to format
///
/// ## Returns
/// - `Result<(), String>`: `Ok(())` on success, or an error string on serialization failure
///
/// ## Errors / Exceptions
/// - Returns `Err` if JSON serialization or stream writing fails.
/// - Does not panic.
pub fn write_json_to_writer<W: Write>(writer: &mut W, items: &[SearchMatch]) -> Result<(), String> {
    let matches: Vec<JsonExportMatch> = items
        .iter()
        .map(|item| {
            let normalized_path = crate::export::normalize_export_path(&item.full_path);
            JsonExportMatch {
                id: item.id,
                file_name: item.file_name.clone(),
                full_path: normalized_path.into_owned(),
                sheet_name: item.sheet_name.clone(),
                cell_address: item.cell_address.clone(),
                row_index: item.row_index,
                col_index: item.col_index,
                match_type: item.match_type,
                full_content: item.full_content.clone(),
                snippet: item.snippet.clone(),
                formula: item.formula.clone(),
                shape_name: item.shape_name.clone(),
            }
        })
        .collect();

    let payload = JsonExportPayload {
        total_matches: matches.len(),
        matches,
    };

    serde_json::to_writer_pretty(&mut *writer, &payload).map_err(|e| {
        // Constant reference: crate::constants::ERR_JSON_EXPORT_WRITE
        format!("{}: {}", crate::constants::ERR_JSON_EXPORT_WRITE, e)
    })?;
    writer.write_all(b"\n").map_err(|e| {
        // Constant reference: crate::constants::ERR_JSON_EXPORT_WRITE
        format!("{}: {}", crate::constants::ERR_JSON_EXPORT_WRITE, e)
    })?;
    writer.flush().map_err(|e| {
        // Constant reference: crate::constants::ERR_JSON_EXPORT_WRITE
        format!("{}: {}", crate::constants::ERR_JSON_EXPORT_WRITE, e)
    })?;
    Ok(())
}

/// ## Description
/// Outputs a slice of search result items to a structured JSON (.json) file.
///
/// ## Arguments
/// - `path`: `&str` - Target output JSON file path
/// - `items`: `&[SearchMatch]` - Slice of search matches to export
///
/// ## Returns
/// - `Result<(), String>`: `Ok(())` on success, or an error string on failure
///
/// ## Errors / Exceptions
/// - Returns `Err` if file creation or JSON writing fails.
/// - Does not panic.
pub fn export_to_json(path: &str, items: &[SearchMatch]) -> Result<(), String> {
    let file = File::create(path).map_err(|e| {
        // Constant reference: crate::constants::ERR_JSON_EXPORT_WRITE
        format!("{}: {}", crate::constants::ERR_JSON_EXPORT_WRITE, e)
    })?;
    let mut writer = std::io::BufWriter::new(file);
    write_json_to_writer(&mut writer, items)
}
