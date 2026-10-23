# Interface Contracts: LLM Context Output Formatting

**Feature Branch**: `030-ai-copy-tab-name` | **Date**: 2026-10-23 | **Spec**: [spec.md](../spec.md)

This document formalizes the interface contracts for LLM context generation in `xlseek-core`.
Per Constitution Principle III and Section 2.3/3.3, all examples use synthetic dummy data.

---

## 1. IPC Command Interface Contract

### Command: `generate_llm_context`

Exposed via Tauri IPC command `generate_llm_context` and implemented by `xlseek_core::search::llm_context::generate_llm_context`.

#### Input
```json
{
  "matches": [
    {
      "file_path": "C:\\MockData\\SampleWorkbook.xlsx",
      "file_name": "SampleWorkbook.xlsx",
      "sheet_name": "Sheet1",
      "cell_address": "B2",
      "matched_value": "Active",
      "full_content": "Active Project Status",
      "row_index": 1,
      "col_index": 1
    }
  ],
  "options": {
    "copy_type": "table",
    "column_window_size": 5,
    "word_mode": "headings_and_paragraph"
  }
}
```

#### Output (Markdown String)

##### Case 1: Standard Excel Match with Worksheet Name (Table Mode)
```markdown
### SampleWorkbook.xlsx - Sheet1
- Target Cell: [Sheet1] B2 (Column: Status, Row: Item 1)
  - Matched Value: **Active Project Status**

| # | A | B |
|---|---|---|
| 1 | Name | Status |
| 2 | Project Alpha | **Active Project Status** |
```

##### Case 2: Standard Excel Match with Worksheet Name (Header-Only Mode)
```markdown
### SampleWorkbook.xlsx - Sheet1
- Target Cell: [Sheet1] B2 (Column: Status, Row: Item 1)
  - Matched Value: **Active Project Status**
```

##### Case 3: Empty Sheet Name Fallback
When `sheet_name` is empty or whitespace-only:
```markdown
### SampleWorkbook.xlsx - 
- Target Cell: B2 (Column: Status, Row: Item 1)
  - Matched Value: **Active Project Status**
```

##### Case 4: Special Characters in Sheet Name
When `sheet_name` is `FY2026 Q3 (Final)`:
```markdown
### SampleWorkbook.xlsx - FY2026 Q3 (Final)
- Target Cell: [FY2026 Q3 (Final)] B2 (Column: Status, Row: Item 1)
  - Matched Value: **Active Project Status**
```

---

## 2. Invariants & Error Handling

- **Idempotency**: Given identical matches and options, `generate_llm_context` produces deterministic Markdown text.
- **Panic Freedom**: Does not panic on unreadable files, corrupt workbooks, or missing sheets; returns `Err(String)` with localized error messaging per Constitution Principle V.
- **Word Document Invariant**: Matches from `.docx`, `.docm`, `.doc` files omit the `Target Cell` concept entirely and continue outputting `Heading` and `Paragraph` context blocks without modification.
