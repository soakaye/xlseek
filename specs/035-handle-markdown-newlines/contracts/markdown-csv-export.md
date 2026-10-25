# Interface Contract: Multiline Normalization for Markdown & CSV Exports

**Feature**: `035-handle-markdown-newlines`  
**Date**: 2026-10-25  
**Status**: Completed  

All data examples in this document use synthetic dummy data conforming to Constitution Principle III.

---

## 1. Rust Backend Internal Functions Contract

### 1.1 `escape_markdown_cell` (in `crates/core/src/search/llm_context.rs`)

```rust
/// ## Description
/// Trims outer whitespace/newlines, escapes pipe delimiters, and replaces internal
/// newlines with `<br>` tags to preserve line breaks within Markdown table cells and list items.
///
/// ## Arguments
/// - `text`: `&str` - Raw input cell or match text
///
/// ## Returns
/// - `String`: Sanitized text suitable for inline Markdown table cells or bold list values
///
/// ## Errors / Exceptions
/// Does not panic.
pub fn escape_markdown_cell(text: &str) -> String;
```

#### Contract Properties

| Input | Output | Verification Rule |
|:---|:---|:---|
| `"Alpha"` | `"Alpha"` | Pass-through of single-line text without changes |
| `"A \| B"` | `"A \\| B"` | Pipe characters escaped with backslash |
| `"Line 1\nLine 2"` | `"Line 1<br>Line 2"` | `\n` replaced with `<br>` |
| `"Line 1\r\nLine 2"` | `"Line 1<br>Line 2"` | CRLF replaced with `<br>` |
| `"\n\nTrim Me\n\n"` | `"Trim Me"` | Outer whitespace and newlines removed |
| `"Para 1\n\nPara 2"` | `"Para 1<br><br>Para 2"` | Consecutive internal newlines preserved as `<br><br>` |
| `""` | `""` | Empty string returns empty string |

---

### 1.2 `sanitize_csv_field` (in `crates/core/src/export/csv_export.rs`)

```rust
/// ## Description
/// Trims outer whitespace/newlines and replaces all internal newline sequences (`\r\n`, `\n`, `\r`)
/// with a single space to enforce the 1-record-per-line CSV guarantee.
///
/// ## Arguments
/// - `text`: `&str` - Raw field text (cell value, formula, etc.)
///
/// ## Returns
/// - `String`: Single-line text safe for line-oriented CSV export
///
/// ## Errors / Exceptions
/// Does not panic.
pub fn sanitize_csv_field(text: &str) -> String;
```

#### Contract Properties

| Input | Output | Verification Rule |
|:---|:---|:---|
| `"Item 100"` | `"Item 100"` | Unchanged |
| `"Line 1\nLine 2"` | `"Line 1 Line 2"` | Internal newline replaced with `" "` |
| `"Line 1\r\nLine 2"` | `"Line 1 Line 2"` | Internal CRLF replaced with `" "` |
| `"\n\nHeader\n\nValue\n\n"` | `"Header  Value"` | Outer newlines trimmed; internal newlines replaced with `" "` |
| `""` | `""` | Empty string returns empty string |

---

## 2. Export Output Artifact Contracts

### 2.1 Markdown Export Output (`.md`)

- **Table Mode**:
  ```markdown
  ### sample.xlsx - Sheet1

  | Column A | Column B |
  | --- | --- |
  | Item 1 | Detail 1<br>Detail 2 |
  ```
  Every table row starting with `|` MUST NOT contain any unescaped newline before the closing `|`.

- **Header & Cell Only Mode**:
  ```markdown
  ### sample.xlsx - Sheet1
  - Location: Sheet1!B2 (Column: Column B, Row: Row 1)
    - Matched Value: **Detail 1<br>Detail 2**
  ```
  The `- Matched Value: **...**` line MUST NOT contain any raw newline inside the `**` markup.

### 2.2 CSV Export Output (`.csv`)

- **Line Count Guarantee**:
  $$\text{Total Lines} = 1 + N_{\text{matches}}$$
- **Example Record**:
  ```csv
  1,"sample.xlsx","/dummy/path/sample.xlsx","Sheet1","B2","","セル値","Detail 1 Detail 2",""
  ```
  No double-quoted field in the output CSV file contains an embedded newline character.
