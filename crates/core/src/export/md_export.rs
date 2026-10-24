//! # Markdown Export Module (export/md_export.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Writes matched search items as an AI-ready Markdown (.md) document.
//! Reuses the core LLM context extraction engine to preserve table and heading structure.
//! Conforms to Constitution Principle I (English code/comments), Principle II (Constant references),
//! Principle III (Header comments), and Principle IV (Clippy compliance).

use crate::models::{LlmCopyOptions, SearchMatch};
use std::fs::File;
use std::io::Write;

/// ## Description
/// Outputs a slice of search result items to any `Write` stream in structured Markdown format.
///
/// ## Arguments
/// - `writer`: `&mut W` - Target writable stream
/// - `items`: `&[SearchMatch]` - Slice of search match items to format
/// - `options`: `&LlmCopyOptions` - AI copy formatting options
///
/// ## Returns
/// - `Result<(), String>`: `Ok(())` on success, or an error string on failure
///
/// ## Errors / Exceptions
/// - Returns `Err` if context extraction or stream writing fails.
/// - Does not panic.
pub fn write_md_to_writer<W: Write>(
    writer: &mut W,
    items: &[SearchMatch],
    options: &LlmCopyOptions,
) -> Result<(), String> {
    if items.is_empty() {
        return Ok(());
    }
    let markdown_content = crate::search::llm_context::extract_llm_context(items, options)?;
    writer.write_all(markdown_content.as_bytes()).map_err(|e| {
        // Constant reference: crate::constants::ERR_MD_EXPORT_WRITE
        format!("{}: {}", crate::constants::ERR_MD_EXPORT_WRITE, e)
    })?;
    writer.write_all(b"\n").map_err(|e| {
        // Constant reference: crate::constants::ERR_MD_EXPORT_WRITE
        format!("{}: {}", crate::constants::ERR_MD_EXPORT_WRITE, e)
    })?;
    writer.flush().map_err(|e| {
        // Constant reference: crate::constants::ERR_MD_EXPORT_WRITE
        format!("{}: {}", crate::constants::ERR_MD_EXPORT_WRITE, e)
    })?;
    Ok(())
}

/// ## Description
/// Outputs a slice of search result items to a Markdown (.md) file.
///
/// ## Arguments
/// - `path`: `&str` - Output Markdown file path
/// - `items`: `&[SearchMatch]` - Slice of search match items to export
/// - `options`: `Option<&LlmCopyOptions>` - Extraction options (falls back to defaults if None)
///
/// ## Returns
/// - `Result<(), String>`: `Ok(())` on success, or an English error string on failure
///
/// ## Errors / Exceptions
/// - Returns `Err` if file creation or content formatting fails.
/// - Does not panic.
pub fn export_to_md(
    path: &str,
    items: &[SearchMatch],
    options: Option<&LlmCopyOptions>,
) -> Result<(), String> {
    let file = File::create(path).map_err(|e| {
        // Constant reference: crate::constants::ERR_MD_EXPORT_WRITE
        format!("{}: {}", crate::constants::ERR_MD_EXPORT_WRITE, e)
    })?;
    let mut writer = std::io::BufWriter::new(file);
    let default_options = LlmCopyOptions::default();
    let effective_options = options.unwrap_or(&default_options);
    write_md_to_writer(&mut writer, items, effective_options)
}
