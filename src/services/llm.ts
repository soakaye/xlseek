/**
 * Copyright (c) 2026 soakaye
 *
 * @fileoverview LLM Context Generation Service (src/services/llm.ts)
 *
 * ## Description
 * Provides frontend IPC invocation methods to request structured Markdown
 * context snippets from the Rust backend for selected search matches.
 * Complies with Constitution Principle I (English documentation), Principle II (external constants),
 * and Principle III (comprehensive comments).
 */

import { invoke } from "@tauri-apps/api/core";
import { SearchMatch } from "../types/search";
import { LlmCopyOptions } from "../types/llm";
import { COMMANDS } from "../constants";

/**
 * ## Description
 * Requests the Rust backend to assemble and format an LLM-ready context snippet
 * for the provided search matches based on specified extraction options.
 *
 * ## Arguments
 * @param matches - List of search matches to extract context from
 * @param options - Extraction configuration options (column window, copy type, word mode)
 *
 * ## Returns
 * @returns Formatted Markdown string ready for clipboard transfer
 *
 * ## Errors / Exceptions
 * Throws an Error if backend command execution fails.
 */
export async function generateLlmContext(
  matches: SearchMatch[],
  options: LlmCopyOptions
): Promise<string> {
  // Constant reference: COMMANDS.GENERATE_LLM_CONTEXT
  return await invoke<string>(COMMANDS.GENERATE_LLM_CONTEXT, {
    matches,
    options,
  });
}
