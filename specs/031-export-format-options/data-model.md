# Data Model: Search Results Export Format Options and UI Reorganization

**Feature**: `031-export-format-options`  
**Date**: 2026-10-24

## 1. ExportFormat Enumeration

Represents the supported file export formats across `xlseek-core`, `src-tauri`, `xlseek-cli`, and the React frontend.

### Rust (`crates/core/src/models/mod.rs`)

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    Csv,
    Xlsx,
    Md,
    Json,
}
```

### TypeScript (`src/types/search.ts`)

```typescript
export type ExportFormat = "csv" | "xlsx" | "md" | "json";
```

| Variant | Serialized Value | File Extension | Supports CLI `stdout` | Description |
| :--- | :--- | :--- | :--- | :--- |
| `Csv` | `"csv"` | `.csv` | Yes | UTF-8 CSV (with BOM for files, without BOM for `stdout`) |
| `Xlsx` | `"xlsx"` | `.xlsx` | No (binary) | Formatted Excel workbook with autofitted columns |
| `Md` | `"md"` | `.md` | Yes | Context-aware AI Markdown generated via `extract_llm_context` |
| `Json` | `"json"` | `.json` | Yes | Pretty-printed UTF-8 JSON with `total_matches` and `matches` array |

---

## 2. ExportRequest Payload

Sent from the frontend to the Tauri backend command `export_results`.

### Rust (`crates/core/src/models/mod.rs`)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRequest {
    pub format: ExportFormat,
    #[serde(default = "default_export_language")]
    pub language: String,
    pub output_path: String,
    pub items: Vec<SearchMatch>,
    #[serde(default)]
    pub llm_options: Option<LlmCopyOptions>,
}
```

### TypeScript (`src/types/search.ts`)

```typescript
export interface ExportRequest {
  format: ExportFormat;
  output_path: string;
  items: SearchMatch[];
  language: DisplayLanguage;
  llm_options?: LlmCopyOptions;
}
```

### Validation Rules

- `language` MUST be `"ja"` or `"en"`; otherwise `ErrorCode::InternalError` is returned.
- `output_path` MUST be a writable file path on the local filesystem.
- `llm_options` defaults to `LlmCopyOptions::default()` when `None` or omitted (used only when `format == ExportFormat::Md`).

---

## 3. JSON Export Schema (`JsonExportPayload` & `JsonExportMatch`)

Defines the structured JSON output produced by `export_to_json` / `write_json_to_writer` in `crates/core/src/export/json_export.rs`.

### Rust (`crates/core/src/export/json_export.rs`)

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct JsonExportPayload {
    pub total_matches: usize,
    pub matches: Vec<JsonExportMatch>,
}

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
```

### Example Synthetic Output (`.json`)

```json
{
  "total_matches": 1,
  "matches": [
    {
      "id": 1,
      "file_name": "sample.xlsx",
      "full_path": "/path/to/synthetic/sample.xlsx",
      "sheet_name": "Sheet1",
      "cell_address": "B2",
      "row_index": 2,
      "col_index": 2,
      "match_type": "CellValue",
      "full_content": "dummy search value",
      "snippet": "dummy search value"
    }
  ]
}
```

---

## 4. UI State & Persistence Model (`StatusBar.tsx`)

- **Active Format State**: `activeFormat: ExportFormat`
  - Initialized from `window.localStorage.getItem(EXPORT_FORMAT_STORAGE_KEY)`.
  - Validated against `["xlsx", "csv", "md", "json"]`; falls back to `"xlsx"` (`DEFAULT_EXPORT_FORMAT`) if absent or invalid.
  - Updated and persisted whenever the user selects a format from the `▾` menu.
- **Menu Open State**: `isExportMenuOpen: boolean`
  - Toggled by clicking the `▾` button when `items.length > 0`.
  - Automatically closed when `items.length === 0`, when clicking outside the menu container, when pressing `Escape`, or when selecting a format option.
