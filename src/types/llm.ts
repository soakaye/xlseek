/**
 * Copyright (c) 2026 soakaye
 *
 * @fileoverview LLM context copy type definitions (src/types/llm.ts)
 *
 * ## Description
 * Defines data structures and option contracts for extracting and formatting
 * LLM prompt snippets and structured spreadsheet context.
 * Complies with Constitution Principle I (English documentation) and Principle III (comprehensive comments).
 */

export type LlmCopyType = "table" | "header_cell_only";

export type WordExtractionMode = "headings_and_paragraph" | "paragraph_only";

/**
 * Configuration options for LLM context extraction.
 */
export interface LlmCopyOptions {
  column_window_size: number;
  copy_type: LlmCopyType;
  word_mode: WordExtractionMode;
}

/**
 * Resolved spatial and semantic header information for an individual cell match.
 */
export interface HeaderContext {
  column_header: string;
  header_row_index: number;
  row_key: string;
  key_col_index: number;
}

/**
 * Formatted prompt payload returned by the backend context extraction engine.
 */
export interface PromptSnippet {
  content: string;
  match_count: number;
  sheet_count: number;
}
