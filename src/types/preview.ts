/**
 * Copyright (c) 2026 soakaye
 *
 * @fileoverview Word Document Preview Type Definitions (src/types/preview.ts)
 *
 * ## Description
 * Defines the TypeScript data models for Word document text preview payloads and paragraphs.
 * Complies with Constitution Principle I (English documentation) and Principle III (comprehensive documentation).
 */

/**
 * Represents a single extracted paragraph from a Word document for preview.
 */
export interface WordParagraph {
  index: number;
  text: string;
  is_target: boolean;
}

/**
 * Complete Word document text preview payload returned from Tauri IPC command `get_word_preview`.
 */
export interface WordPreviewPayload {
  file_path: string;
  file_name: string;
  target_paragraph: number;
  total_paragraphs: number;
  paragraphs: WordParagraph[];
}
