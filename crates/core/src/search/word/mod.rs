//! # Word Document Search & Preview Dispatcher (search/word/mod.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Central dispatcher for Word document processing. Routes search requests and preview extraction
//! to the appropriate parser based on file extension:
//! - `.docx` and `.docm`: Dispatches to `docx` parser (`quick-xml` + `zip`).
//! - `.doc`: Dispatches to `doc` parser (`cfb` + `encoding_rs`).
//!
//! Conforms to Constitution Principle I (English comments), Principle II (Constant references),
//! Principle III (Comprehensive Header Comments), and Principle V (Robust error handling).
//!
//! ## Arguments / Returns
//! Exposes `search_word_file` and `extract_word_document` for the search engine and Tauri IPC commands.
//!
//! ## Errors / Exceptions
//! Returns `Err` with descriptive messages when encountering unsupported file extensions or parsing failures.

pub mod doc;
pub mod docx;

use crate::models::{SearchMatch, SearchQuery, WordPreviewPayload};
use regex::Regex;
use std::path::Path;
use std::sync::atomic::AtomicBool;

/// Error message when an unrecognized or unsupported Word extension is provided.
const ERR_UNSUPPORTED_WORD_EXTENSION: &str = "Unsupported Word document extension";

/// ## Description
/// Routes a search request for a Word document file to the specialized parser based on file extension.
///
/// ## Arguments
/// - `path`: `&Path` - Path to the Word document on disk (.docx, .docm, or .doc).
/// - `query`: `&SearchQuery` - User search parameters including keyword, regex mode, and matching flags.
/// - `regex_opt`: `Option<&Regex>` - Compiled regular expression if regex mode is active.
/// - `cancel_flag`: `Option<&AtomicBool>` - Optional cancellation signal to terminate scanning early.
///
/// ## Returns
/// - `Result<Vec<SearchMatch>, String>`: Vector of matching items found in the document, or error string.
///
/// ## Errors / Exceptions
/// Returns `Err` if the extension is unsupported or if parsing the document fails.
pub fn search_word_file(
    path: &Path,
    query: &SearchQuery,
    regex_opt: Option<&Regex>,
    cancel_flag: Option<&AtomicBool>,
) -> Result<Vec<SearchMatch>, String> {
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_ascii_lowercase())
        .unwrap_or_default();

    // Constant references: crate::constants::EXT_DOCX, EXT_DOCM, EXT_DOC
    let ext_with_dot = format!(".{}", ext);
    if ext_with_dot == crate::constants::EXT_DOCX || ext_with_dot == crate::constants::EXT_DOCM {
        docx::search_docx(path, query, regex_opt, cancel_flag)
    } else if ext_with_dot == crate::constants::EXT_DOC {
        doc::search_doc(path, query, regex_opt, cancel_flag)
    } else {
        Err(format!("{}: .{}", ERR_UNSUPPORTED_WORD_EXTENSION, ext))
    }
}

/// ## Description
/// Extracts ordered paragraphs from a Word document (.docx, .docm, or .doc) for text preview display.
///
/// ## Arguments
/// - `path`: `&Path` - Path to the Word document on disk.
/// - `target_paragraph`: `u32` - 1-based paragraph sequence index to identify the target hit.
///
/// ## Returns
/// - `Result<WordPreviewPayload, String>`: Structured preview payload on success, or error string on failure.
///
/// ## Errors / Exceptions
/// Returns `Err` if the extension is unsupported or if file parsing fails.
pub fn extract_word_document(
    path: &Path,
    target_paragraph: u32,
) -> Result<WordPreviewPayload, String> {
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_ascii_lowercase())
        .unwrap_or_default();

    // Constant references: crate::constants::EXT_DOCX, EXT_DOCM, EXT_DOC
    let ext_with_dot = format!(".{}", ext);
    if ext_with_dot == crate::constants::EXT_DOCX || ext_with_dot == crate::constants::EXT_DOCM {
        docx::extract_docx_paragraphs(path, target_paragraph)
    } else if ext_with_dot == crate::constants::EXT_DOC {
        doc::extract_doc_paragraphs(path, target_paragraph)
    } else {
        Err(format!("{}: .{}", ERR_UNSUPPORTED_WORD_EXTENSION, ext))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsupported_extension_error() {
        let dummy_path = Path::new("test.txt");
        let query = SearchQuery {
            keyword: "test".to_string(),
            target_dir: ".".to_string(),
            directory_mode: crate::models::DirectorySearchMode::Sequential,
            burst_workers: None,
            match_case: false,
            include_value: true,
            use_regex: false,
            include_formula: false,
            include_comment: false,
            include_shape: false,
            include_hidden: false,
            extensions: vec![],
            ignore_patterns: None,
        };
        let result = search_word_file(dummy_path, &query, None, None);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains(ERR_UNSUPPORTED_WORD_EXTENSION));
    }
}
