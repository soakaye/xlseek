# Data Model: Multiline Text Handling in Markdown and CSV Exports

**Feature**: `035-handle-markdown-newlines`  
**Date**: 2026-10-25  
**Status**: Completed  

---

## 1. Entities & Structural Transformations

All examples in this document use synthetic dummy data conforming to Constitution Principle III.

### 1.1 `SearchMatch` (Existing Core Model)

The core data structure representing an individual search hit across Excel, Word, or Shape contents.

```rust
pub struct SearchMatch {
    pub id: usize,
    pub file_name: String,
    pub full_path: String,
    pub sheet_name: String,
    pub cell_address: String,
    pub row_index: u32,
    pub col_index: u32,
    pub col_name: String,
    pub match_type: MatchType,
    pub shape_name: Option<String>,
    pub sheet_hidden: bool,
    pub snippet: String,
    pub full_content: String,
    pub formula: Option<String>,
    pub sheets_in_workbook: Vec<String>,
}
```

---

### 1.2 Markdown Table Cell Transformation

When formatting cells for GitHub Flavored Markdown (GFM) tables:

- **Input**: Raw cell text string potentially containing newlines (`\r\n`, `\n`, `\r`), pipes (`|`), and surrounding whitespace.
- **Transformation Pipeline**:
  1. Trim leading and trailing whitespace and newlines: `raw.trim()`
  2. Escape pipe characters to prevent column splintering: `replace('|', "\\|")`
  3. Normalize Windows CRLF to LF: `replace("\r\n", "\n")`
  4. Normalize Mac CR to LF: `replace('\r', "\n")`
  5. Replace remaining LF characters with `<br>`: `replace('\n', "<br>")`
- **Output**: Sanitized single-line Markdown table cell string.

#### Synthetic Data Examples

| Input Raw Cell Value | Transformed Markdown Cell | Rendered Markdown View |
|:---|:---|:---|
| `"Item Name"` | `"Item Name"` | Item Name |
| `"Line 1\nLine 2"` | `"Line 1<br>Line 2"` | Line 1<br>Line 2 |
| `"\n\nHeader\n\nValue\n\n"` | `"Header<br><br>Value"` | Header<br><br>Value |
| `"A \| B\nC \| D"` | `"A \| B<br>C \| D"` | A \| B<br>C \| D |
| `""` (empty) | `""` | *(empty)* |
| `"\n   \n"` (whitespace only) | `""` | *(empty)* |

---

### 1.3 Markdown List Item Transformation ("Header & Cell Only" Mode)

When formatting matches for compact hierarchical list context:

- **Pattern**:
  ```markdown
  - Location: Sheet1!B2 (Column: ColumnB, Row: Row1)
    - Matched Value: **{sanitized_value}**
  ```
- **Transformation Pipeline**:
  - `sanitized_value` is processed with the same newline and trimming pipeline as Markdown table cells.
  - Converting newlines to `<br>` ensures that the bold markup `**...**` and bullet indentation are not split across physical lines.

#### Synthetic Data Example

```markdown
### sample.xlsx - Sheet1
- Location: Sheet1!B2 (Column: ColumnB, Row: Row1)
  - Matched Value: **First Line<br>Second Line**
```

---

### 1.4 CSV Record Transformation

When writing records to a CSV file or standard output:

- **Fields Processed**: `full_content`, `formula`, `shape_name`
- **Transformation Pipeline**:
  1. Trim leading and trailing whitespace and newlines: `field.trim()`
  2. Normalize CRLF and CR to LF: `replace("\r\n", "\n").replace('\r', "\n")`
  3. Replace internal newlines with a single space: `replace('\n', " ")`
- **Output**: Single-line string safe for standard RFC 4180 CSV writers.
- **Invariant**: Every search match record occupies strictly 1 physical line in the output CSV.

#### Synthetic Data Examples

| Input `full_content` | Transformed CSV Field | CSV Line Output |
|:---|:---|:---|
| `"Alpha\nBeta"` | `"Alpha Beta"` | `1,"sample.xlsx","/dummy/sample.xlsx","Sheet1","A1","","セル値","Alpha Beta",""` |
| `"\n\nHeader\n\nValue\n\n"` | `"Header  Value"` | `2,"sample.xlsx","/dummy/sample.xlsx","Sheet1","A2","","セル値","Header  Value",""` |
| `"Simple Text"` | `"Simple Text"` | `3,"sample.xlsx","/dummy/sample.xlsx","Sheet1","A3","","セル値","Simple Text",""` |

---

## 2. Invariant Contracts

1. **Table Integrity**: For any input text containing arbitrary newlines, the generated Markdown table row MUST NEVER contain physical line breaks inside the row string (`!row.contains('\n')`).
2. **CSV Line Count Invariant**: For any export of $N$ search matches, the total physical line count of the exported CSV file MUST equal exactly $N + 1$ (1 header line + $N$ records).
3. **Pipe Delimiter Protection**: Escaped pipes `\|` MUST NEVER be modified or broken by newline substitution.
