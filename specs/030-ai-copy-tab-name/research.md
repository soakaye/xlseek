# Research & Architecture Decisions: Explicit Sheet Tab Name in AI Copy Target Cell Metadata

**Feature Branch**: `030-ai-copy-tab-name` | **Date**: 2026-10-23 | **Spec**: [spec.md](./spec.md)

This document records the research findings, design tradeoffs, and technical decisions made during Phase 0 for Feature 030 (Explicit Sheet Tab Name in AI Copy Target Cell Metadata).

---

## 1. Target Cell Formatting Syntax

### Problem
In `crates/core/src/search/llm_context.rs`, the AI Copy (LLM context copy) generator currently outputs the matched cell location as:
```markdown
- Target Cell: B2 (Column: Status, Row: Item 1)
```
While the section header includes the worksheet name (`### SampleWorkbook.xlsx - Sheet1`), individual target cell descriptors lack the worksheet name. When multiple matches across different sheets or workbooks are aggregated into a single clipboard payload, or when prompts are excerpted, neither human readers nor LLMs can easily associate `B2` with `Sheet1` without scanning backwards through markdown headers.

### Decision
Format the worksheet tab name directly within the `Target Cell` line using the bracketed prefix syntax:
```markdown
- Target Cell: [SheetName] CellAddress (Column: ColHeader, Row: RowKey)
```
Example:
```markdown
- Target Cell: [Sheet1] B2 (Column: Status, Row: Item 1)
```
If the sheet name is empty or whitespace-only (fallback edge case), omit the brackets and fall back to the existing coordinate-only format:
```markdown
- Target Cell: B2 (Column: Status, Row: Item 1)
```

### Rationale
- **High Readability for LLMs and Humans**: Enclosing the sheet name in square brackets `[SheetName]` creates an unambiguous token boundary. LLMs recognize this immediately as a scoping container without confusion with formula syntax or cell coordinates.
- **Handling Spaces and Special Characters**: Excel sheet names commonly contain spaces, Japanese characters, hyphens, parentheses, and dates (e.g. `FY2026 Q3 (Final)`). Bracket notation cleanly wraps these characters without requiring Excel quote-escaping conventions (such as doubling single quotes).
- **Approved in Clarification Session**: Confirmed with the user in Clarification Session 2026-10-23 (Option A).

### Alternatives Considered
- **Alternative B: Excel Formula Reference Syntax (`'SheetName'!A16`)**:
  - *Pros*: Familiar to Excel formula power users.
  - *Cons*: Requires single-quote escaping when sheet names contain apostrophes, and punctuation like `'` and `!` can be harder for LLMs to cleanly parse in plain-text markdown summaries.
- **Alternative C: Key-Value Format (`Sheet: SheetName, Cell: A16`)**:
  - *Pros*: Verbose and explicit.
  - *Cons*: Adds extra token overhead to LLM prompts, diluting concise prompt context.

---

## 2. Centralized Constants Extraction (Constitution Principle II)

### Problem
Constitution Principle II strictly prohibits hardcoded constants (strings, format templates, magic numbers) in code outside `0` and `""`. All constants must be centralized in `crates/core/src/constants.rs` and referenced with explicit code comments (`// Constant reference: ...`).

### Decision
Define the format strings and metadata label constants in `crates/core/src/constants.rs` under section `7.2 AI / LLM Context Copy Constants`:
```rust
/// Format template for Target Cell metadata line with explicit sheet name
pub const LLM_TARGET_CELL_WITH_SHEET_FMT: &str = "- Target Cell: [{}] {} (Column: {}, Row: {})";

/// Format template for Target Cell metadata line without sheet name (fallback)
pub const LLM_TARGET_CELL_WITHOUT_SHEET_FMT: &str = "- Target Cell: {} (Column: {}, Row: {})";
```
Add corresponding unit assertions to the `tests` module in `crates/core/src/constants.rs`.

### Rationale
- Complies fully with Constitution Principle II.
- Allows immediate centralized maintenance if prompt conventions evolve in the future.
- Guarantees identical formatting between table mode and header-only mode.

---

## 3. Modular Helper Function in `llm_context.rs` (Constitution Principles III & IV)

### Problem
Both `format_table_excel_context` and `format_header_only_excel_context` independently format the `Target Cell` metadata line. Duplicating the string formatting and fallback logic violates DRY and creates maintenance drift risk.

### Decision
Extract the formatting logic into a private helper function in `crates/core/src/search/llm_context.rs`:
```rust
/// ## Description
/// Formats the Target Cell metadata line including the worksheet tab name enclosed in brackets,
/// or omitting the brackets if the worksheet name is empty.
///
/// ## Arguments
/// - `sheet_name`: `&str` - Worksheet tab name
/// - `cell_address`: `&str` - Cell coordinate address (e.g., "A1")
/// - `column_header`: `&str` - Detected column header text
/// - `row_key`: `&str` - Detected row key text
///
/// ## Returns
/// - `String`: Formatted "- Target Cell: ..." metadata line
///
/// ## Errors / Exceptions
/// Does not panic.
fn format_target_cell_line(
    sheet_name: &str,
    cell_address: &str,
    column_header: &str,
    row_key: &str,
) -> String {
    let trimmed_sheet = sheet_name.trim();
    if trimmed_sheet.is_empty() {
        // Constant reference: crate::constants::LLM_TARGET_CELL_WITHOUT_SHEET_FMT
        format!(
            crate::constants::LLM_TARGET_CELL_WITHOUT_SHEET_FMT,
            cell_address, column_header, row_key
        )
    } else {
        // Constant reference: crate::constants::LLM_TARGET_CELL_WITH_SHEET_FMT
        format!(
            crate::constants::LLM_TARGET_CELL_WITH_SHEET_FMT,
            trimmed_sheet, cell_address, column_header, row_key
        )
    }
}
```

### Rationale
- Follows the Single Responsibility Principle (SRP) per Principle IV.
- Provides a comprehensive 3-part header comment complying with Principle III.
- Ensures identical behavior between Table mode and Header-only mode per User Story 2.

---

## 4. Scope and Non-Regression Boundaries

### Findings
- **Excel Matches**: Handled by `format_table_excel_context` and `format_header_only_excel_context`. The `sheet_name: &str` parameter is already available in both functions, as well as `m.sheet_name: String` inside `SearchMatch`.
- **Word Matches**: Handled by `format_word_context` (`format_word_paragraph_only` and `format_word_headings_and_paragraph`). Word documents do not have worksheets or cell coordinates; their metadata uses `Heading` and `Paragraph` indicators. They remain untouched.
- **Frontend / Tauri IPC**: `generate_llm_context` receives `Vec<SearchMatch>` and `LlmCopyOptions`. The change is purely in the string formatting engine of `xlseek-core`. No frontend components or Tauri command signatures need modification.
