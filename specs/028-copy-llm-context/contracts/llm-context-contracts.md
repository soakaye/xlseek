# Interface Contracts: Context-Aware Header Extraction and Prompt Copy for LLM

**Feature Branch**: `028-copy-llm-context` | **Date**: 2026-10-19 | **Spec**: [spec.md](./spec.md)

This document specifies the interface contracts between the React frontend and the Rust backend, including Tauri IPC command schemas, data transfer objects, error codes, and centralized constants.

---

## 1. Tauri IPC Command: `generate_llm_context`

### 1.1 Command Signature (Rust Backend)
```rust
#[tauri::command]
pub async fn generate_llm_context(
    matches: Vec<SearchMatch>,
    options: LlmCopyOptions,
) -> Result<String, CommandError>
```

### 1.2 Request Arguments (JSON Schema)
```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "GenerateLlmContextRequest",
  "type": "object",
  "properties": {
    "matches": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "id": { "type": "integer" },
          "file_path": { "type": "string" },
          "file_name": { "type": "string" },
          "sheet_name": { "type": "string" },
          "cell_address": { "type": "string" },
          "match_type": { "type": "string" },
          "snippet": { "type": "string" },
          "full_content": { "type": "string" },
          "row_index": { "type": "integer" },
          "col_index": { "type": "integer" }
        },
        "required": ["file_path", "file_name", "sheet_name", "cell_address", "match_type", "full_content"]
      }
    },
    "options": {
      "type": "object",
      "properties": {
        "column_window_size": { "type": "integer", "minimum": 1, "maximum": 20, "default": 5 },
        "copy_type": { "type": "string", "enum": ["table", "header_cell_only"], "default": "table" },
        "word_mode": { "type": "string", "enum": ["headings_and_paragraph", "paragraph_only"], "default": "headings_and_paragraph" }
      },
      "required": ["column_window_size", "copy_type", "word_mode"]
    }
  },
  "required": ["matches", "options"]
}
```

### 1.3 Response Payload
- **Success (`Ok(String)`)**: A UTF-8 formatted Markdown string containing metadata headers, table representation (with bolded hit values), and optional ellipsis columns.
- **Failure (`Err(CommandError)`)**:
  ```json
  {
    "code": "PreviewFailed"
  }
  ```

---

## 2. Frontend TypeScript Service Contract (`src/services/llm.ts`)

```typescript
import { invoke } from "@tauri-apps/api/core";
import { SearchMatch } from "../types/search";
import { LlmCopyOptions } from "../types/llm";
import { COMMAND_NAMES } from "../constants";

/**
 * ## Description
 * Generates an LLM-ready structured Markdown context snippet from backend.
 *
 * ## Arguments
 * @param matches - Array of search matches to extract context from
 * @param options - Configurable extraction options
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
  // Constant reference: COMMAND_NAMES.GENERATE_LLM_CONTEXT
  return await invoke<string>(COMMAND_NAMES.GENERATE_LLM_CONTEXT, {
    matches,
    options,
  });
}
```

---

## 3. Centralized Constants

### 3.1 Rust Constants (`crates/core/src/constants.rs` / `src-tauri/src/constants.rs`)
- `pub const CMD_GENERATE_LLM_CONTEXT: &str = "generate_llm_context";`
- `pub const DEFAULT_LLM_COLUMN_WINDOW_SIZE: u32 = 5;`
- `pub const MAX_LLM_COLUMN_WINDOW_SIZE: u32 = 20;`
- `pub const MIN_LLM_COLUMN_WINDOW_SIZE: u32 = 1;`
- `pub const LLM_ELLIPSIS_COL: &str = "...";`

### 3.2 Frontend Constants (`src/constants/index.ts`)
```typescript
export const COMMAND_NAMES = {
  // Existing commands...
  SEARCH: "search",
  GET_CELL_PREVIEW: "get_cell_preview",
  GENERATE_LLM_CONTEXT: "generate_llm_context",
} as const;

export const LLM_DEFAULTS = {
  COLUMN_WINDOW_SIZE: 5,
  MIN_WINDOW_SIZE: 1,
  MAX_WINDOW_SIZE: 20,
  DEFAULT_COPY_TYPE: "table" as const,
  DEFAULT_WORD_MODE: "headings_and_paragraph" as const,
} as const;
```

### 3.3 i18n Resource Keys (`crates/core/locales/ja.yml` & `en.yml`)
- `ui.COPY_FOR_LLM`: "Copy for LLM" / "AI向けに文脈付きでコピー"
- `ui.COPY_FOR_LLM_TABLE`: "Copy for LLM (Table Context)" / "AI向けに文脈コピー (表形式)"
- `ui.COPY_FOR_LLM_HEADER_ONLY`: "Copy for LLM (Header & Cell Only)" / "AI向けに文脈コピー (見出し・セルのみ)"
- `ui.COPY_FOR_LLM_WORD_HEADING`: "Copy for LLM (With Headings & Context)" / "AI向けに文脈コピー (章・節見出し付き)"
- `ui.COPY_FOR_LLM_WORD_PARAGRAPH`: "Copy for LLM (Paragraph Only)" / "AI向けに文脈コピー (段落情報のみ)"
- `ui.COPIED_FOR_LLM_SUCCESS`: "Context copied for LLM to clipboard" / "AI向けコンテキストをクリップボードにコピーしました"
- `ui.SETTINGS_LLM_SECTION`: "AI / LLM Context Copy" / "AIコンテキストコピー設定"
- `ui.SETTINGS_LLM_WINDOW_SIZE`: "Adjacent Columns Window Size" / "多列シート時の前後切り出し列数"
