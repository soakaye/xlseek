//! # AI / LLM Context Copy Models (models/llm.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Defines data models for configuring and representing context-aware header extractions
//! and prompt snippets formatted for LLM consumption.
//! Conforms to Constitution Principle I (English documentation), Principle II (Constant references),
//! and Principle III (Comprehensive Header Comments).

use serde::{Deserialize, Serialize};

/// ## Description
/// Selectable copy format types for LLM context generation.
///
/// ## Arguments / Returns
/// - `Table`: Full row aligned with column headers in Markdown table format.
/// - `HeaderCellOnly`: Compact key-value representation focusing on headers and cell coordinates.
///
/// ## Errors / Exceptions
/// Serde fails deserialization if an unrecognized string token is provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LlmCopyType {
    /// Full row context formatted as a Markdown table
    Table,
    /// Compact context focusing only on detected header label, row key, and cell value
    HeaderCellOnly,
}

impl Default for LlmCopyType {
    /// ## Description
    /// Returns the default copy type (`LlmCopyType::Table`).
    ///
    /// ## Arguments / Returns
    /// - Returns: `LlmCopyType::Table`
    ///
    /// ## Errors / Exceptions
    /// Does not panic.
    fn default() -> Self {
        LlmCopyType::Table
    }
}

/// ## Description
/// Selectable extraction modes for Word document search matches.
///
/// ## Arguments / Returns
/// - `HeadingsAndParagraph`: Extracts preceding section heading and quotes the matched paragraph.
/// - `ParagraphOnly`: Quotes the paragraph text and paragraph index without heading analysis.
///
/// ## Errors / Exceptions
/// Serde fails deserialization if an unrecognized string token is provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WordExtractionMode {
    /// Preceding heading and paragraph blockquote
    HeadingsAndParagraph,
    /// Plain paragraph text with paragraph location
    ParagraphOnly,
}

impl Default for WordExtractionMode {
    /// ## Description
    /// Returns the default Word extraction mode (`WordExtractionMode::HeadingsAndParagraph`).
    ///
    /// ## Arguments / Returns
    /// - Returns: `WordExtractionMode::HeadingsAndParagraph`
    ///
    /// ## Errors / Exceptions
    /// Does not panic.
    fn default() -> Self {
        WordExtractionMode::HeadingsAndParagraph
    }
}

/// ## Description
/// Configuration options governing how LLM context is extracted and formatted.
///
/// ## Arguments / Returns
/// - `column_window_size`: Number of adjacent columns to extract before and after the hit cell.
/// - `copy_type`: Mode of context formatting (`Table` or `HeaderCellOnly`).
/// - `word_mode`: Mode for Word context (`HeadingsAndParagraph` or `ParagraphOnly`).
///
/// ## Errors / Exceptions
/// Serde fails deserialization if mandatory fields are missing or typed incorrectly.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmCopyOptions {
    /// Number of adjacent columns to extract before and after target cell (default: 5)
    #[serde(default = "default_llm_column_window_size")]
    pub column_window_size: u32,
    /// Selected copy format type
    #[serde(default)]
    pub copy_type: LlmCopyType,
    /// Selected Word document extraction mode
    #[serde(default)]
    pub word_mode: WordExtractionMode,
}

/// ## Description
/// Returns the default column window size from centralized constants.
///
/// ## Arguments / Returns
/// - Returns: `u32` constant value (5).
///
/// ## Errors / Exceptions
/// Does not panic.
fn default_llm_column_window_size() -> u32 {
    // Constant reference: crate::constants::DEFAULT_LLM_COLUMN_WINDOW_SIZE
    crate::constants::DEFAULT_LLM_COLUMN_WINDOW_SIZE
}

impl Default for LlmCopyOptions {
    /// ## Description
    /// Provides default LLM copy options conforming to centralized configuration constants.
    ///
    /// ## Arguments / Returns
    /// - Returns: Initialized `LlmCopyOptions` struct.
    ///
    /// ## Errors / Exceptions
    /// Does not panic.
    fn default() -> Self {
        Self {
            // Constant reference: crate::constants::DEFAULT_LLM_COLUMN_WINDOW_SIZE
            column_window_size: crate::constants::DEFAULT_LLM_COLUMN_WINDOW_SIZE,
            copy_type: LlmCopyType::default(),
            word_mode: WordExtractionMode::default(),
        }
    }
}

/// ## Description
/// Spatial and semantic header information discovered for a matched Excel cell.
///
/// ## Arguments / Returns
/// - `column_header`: Resolved column header label (e.g. "Status" or "Column G").
/// - `header_row_index`: 0-based row index where the column header was located.
/// - `row_key`: Resolved row identifier / item name (e.g. "TASK-102").
/// - `key_col_index`: 0-based column index where the row key was located.
///
/// ## Errors / Exceptions
/// Serde serialization/deserialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeaderContext {
    /// Resolved column header label
    pub column_header: String,
    /// 0-based row index of the header row
    pub header_row_index: u32,
    /// Resolved row identifier or leftmost descriptive item label
    pub row_key: String,
    /// 0-based column index of the key column
    pub key_col_index: u32,
}

/// ## Description
/// Formatted plain-text payload containing metadata and Markdown table representation.
///
/// ## Arguments / Returns
/// - `content`: Complete Markdown text formatted for clipboard transfer.
/// - `match_count`: Number of search matches included in the snippet.
/// - `sheet_count`: Number of distinct sheets/files covered in the snippet.
///
/// ## Errors / Exceptions
/// Serde serialization/deserialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptSnippet {
    /// Formatted Markdown content string
    pub content: String,
    /// Total matches represented
    pub match_count: usize,
    /// Total distinct worksheets/documents represented
    pub sheet_count: usize,
}
