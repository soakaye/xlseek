# Data Model: Context-Aware Header Extraction and Prompt Copy for LLM

**Feature Branch**: `028-copy-llm-context` | **Date**: 2026-10-19 | **Spec**: [spec.md](./spec.md)

This document formalizes the entities, data structures, and state representations required for Feature 028 across both the Rust backend (`crates/core`, `src-tauri`) and TypeScript frontend (`src/`).

---

## 1. Core Domain Entities (Rust & TypeScript)

### 1.1 `LlmCopyOptions`
Represents the user configuration and contextual parameters governing how LLM context snippets are extracted and formatted.

#### Fields (Rust `crates/core/src/models/llm.rs` & TS `src/types/llm.ts`)
| Field | Type (Rust) | Type (TS) | Default | Description |
|---|---|---|---|---|
| `column_window_size` | `u32` | `number` | `5` | Number of adjacent columns to extract before and after the hit cell (1–20) |
| `copy_type` | `LlmCopyType` | `LlmCopyType` | `"table"` | Mode of context formatting (`"table"` or `"header_cell_only"`) |
| `word_mode` | `WordExtractionMode` | `WordExtractionMode` | `"headings_and_paragraph"` | Mode for Word context (`"headings_and_paragraph"` or `"paragraph_only"`) |

#### Enums
- **`LlmCopyType`**:
  - `Table`: Full row context formatted as a Markdown table with column headers.
  - `HeaderCellOnly`: Compact context focusing only on detected header label, row key, cell coordinate, and value.
- **`WordExtractionMode`**:
  - `HeadingsAndParagraph`: Extracts preceding section heading and quotes the matched paragraph.
  - `ParagraphOnly`: Extracts paragraph number and matched paragraph text without heading analysis.

---

### 1.2 `HeaderContext`
Represents the resolved spatial and semantic headers identified for an individual Excel cell match.

#### Fields (Rust `crates/core/src/models/llm.rs`)
| Field | Type | Description |
|---|---|---|
| `column_header` | `String` | Resolved column header title (e.g. `"Status"`, `"Amount"`, or `"Column G"`) |
| `header_row_index` | `u32` | 0-based row index where the column header was located |
| `row_key` | `String` | Resolved row identifier / item name (e.g. `"TASK-102"`, `"Acme Corp"`) |
| `key_col_index` | `u32` | 0-based column index where the row key was located (typically col 0 / Column A) |

---

### 1.3 `CellMatchContext`
Represents the source match context provided to the context generator.

#### Fields (Rust & TS)
| Field | Type | Description |
|---|---|---|
| `file_path` | `String` | Absolute path to the source workbook or document |
| `file_name` | `String` | Display file name (e.g. `budget_2026.xlsx`) |
| `sheet_name` | `String` | Target worksheet name |
| `cell_address` | `String` | Excel cell coordinate (e.g. `G42`) or Word location indicator |
| `row_index` | `u32` | 1-based row index (Excel) or paragraph index (Word) |
| `col_index` | `u32` | 1-based column index (Excel) or 0 (Word) |
| `value` | `String` | Unformatted or stringified cell value |
| `snippet` | `String` | Matched snippet with search query highlighted |

---

### 1.4 `PromptSnippet`
Represents the generated plain-text output ready for system clipboard transfer.

#### Fields (Rust & TS)
| Field | Type | Description |
|---|---|---|
| `content` | `String` | Complete formatted Markdown text payload |
| `match_count` | `usize` | Number of search matches included in the snippet |
| `sheet_count` | `usize` | Number of distinct sheets/files covered in the snippet |

---

## 2. Frontend State & Settings Integration

### 2.1 Extension to `DefaultSearchOptions` (`src/types/defaultOptions.ts`)
The persistent settings model is extended with LLM copy preferences:
```typescript
export interface DefaultSearchOptions {
  // Existing fields...
  match_case: boolean;
  use_regex: boolean;
  include_formula: boolean;
  include_shape: boolean;
  include_comment: boolean;
  include_hidden: boolean;
  extensions: string[];
  directory_mode: "sequential" | "burst";
  burst_workers: number | null;
  enable_word_search: boolean;
  ignore_patterns: string;

  // New fields for Feature 028:
  llm_column_window_size: number; // default: 5
  llm_default_copy_type: "table" | "header_cell_only"; // default: "table"
  llm_word_extraction_mode: "headings_and_paragraph" | "paragraph_only"; // default: "headings_and_paragraph"
}
```

### 2.2 Context Menu State Model (`ContextMenuState`)
Manages UI positioning and payload for right-click interaction on result rows:
```typescript
export interface ContextMenuState {
  isOpen: boolean;
  x: number;
  y: number;
  targetMatch: SearchMatch | null;
  selectedMatches: SearchMatch[];
}
```

---

## 3. Data Flow Diagram

```text
[User Action: Right-Click / Toolbar "Copy for LLM"]
                   │
                   ▼
       [Frontend React: ResultTable / PreviewHeader]
                   │
                   │ invokes IPC command with matches & LlmCopyOptions
                   ▼
         [Tauri IPC Handler: generate_llm_context]
                   │
                   ▼
          [crates/core: extract_llm_context]
         ┌─────────┴─────────┐
         ▼                   ▼
  [Excel Extraction]  [Word Extraction]
  - Scan upward       - Locate preceding heading
    for header_row    - Quote paragraph
  - Scan leftward     - Bold match text
    for key_column
  - Window columns
    [col - W .. col + W]
  - Format Markdown table
  - Bold hit cell (**val**)
         └─────────┬─────────┘
                   │
                   ▼
     [Assembled Markdown String]
                   │
                   ▼
       [Frontend: navigator.clipboard.writeText]
                   │
                   ▼
    [Display Toast: "Copied for LLM"]
```
