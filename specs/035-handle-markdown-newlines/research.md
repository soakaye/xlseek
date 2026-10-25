# Technical Research & Architecture Decisions: Multiline Text Handling in Markdown and CSV Exports

**Feature**: `035-handle-markdown-newlines`  
**Date**: 2026-10-25  
**Status**: Completed  

---

## 1. Technical Context & Problem Statement

Excel Seek (`xlseek`) allows users to export search results in multiple formats, including Markdown (`.md`) and CSV (`.csv`). Both export targets encounter significant syntactic and formatting issues when matched cells contain internal line breaks (`\r\n`, `\n`, `\r`):

1. **Markdown Tables**:
   - In standard GitHub Flavored Markdown (GFM) and CommonMark, table rows are delimited by physical line breaks (`\n`).
   - If a cell contains a physical newline, the Markdown parser treats it as the start of a new row or an invalid table, breaking table layout.
   - The previous implementation in `crates/core/src/search/llm_context.rs` flattened newlines into spaces (`text.replace('\n', " ").replace('\r', "")`). This prevented layout breakage but completely destroyed multiline formatting in user data (e.g. addresses, notes, bullet lists).
   - In "Header & Cell Only" mode, cell values were placed unescaped directly into `- Matched Value: **{value}**`, splitting bold tags across lines and breaking bullet list indentation.

2. **CSV Output**:
   - While RFC 4180 permits multi-line fields enclosed in double quotes, multiline CSV records break line-oriented CLI tools (`grep`, `awk`, `head`, `sed`, `wc -l`, bash `while read line`).
   - CLI stdout streaming (`xlseek-cli -p ./data -q "query"`) outputs CSV by default. A multiline field causes a single match to output across multiple lines, breaking shell pipelines.

---

## 2. Research & Decision Summary

### Decision 1: Markdown Table Cell Line Break Representation

- **Decision**: Replace cell newlines with standard `<br>` tags.
- **Rationale**:
  - GFM and CommonMark table extensions universally support `<br>` as an inline line break within table cells.
  - Markdown previewers (VS Code, GitHub, Obsidian, Notion) and LLM context windows (Claude, ChatGPT, Gemini) correctly render `<br>` as a line break without breaking table rows or columns.
  - `<br>` is more concise and standard than `<br />` (XHTML) in modern web and markdown contexts.
- **Alternatives Considered**:
  - ` \ ` (backslash line break): Not supported inside Markdown table cells by GFM; causes syntax errors.
  - Space `" "` (status quo): Prevents row breakage but loses all multiline information.
  - `<br />`: Valid, but adds unnecessary character overhead for LLM token usage without providing functional benefits over `<br>`.

---

### Decision 2: Markdown Whitespace Trimming and Consecutive Line Breaks

- **Decision**: Trim leading/trailing whitespace and newlines from cell text; preserve internal consecutive newlines as corresponding sequential `<br>` tags (`\n\n` -> `<br><br>`).
- **Rationale**:
  - Excel cells frequently accumulate unintentional leading or trailing carriage returns from user entry. Trimming outer whitespace prevents unsightly blank lines at the top or bottom of table cells.
  - Internal consecutive line breaks represent intentional paragraph or section breaks within a cell and must be preserved to retain user intent.
- **Alternatives Considered**:
  - Collapsing all internal newlines to a single `<br>`: Destroys intentional paragraph spacing in multiline notes.
  - No trimming (literal mechanical replacement): Produces leading `<br>` or trailing `<br>` inside cells, looking messy in rendered tables.

---

### Decision 3: CSV Field Normalization (Single-Line Guarantee)

- **Decision**: Trim leading/trailing newlines and whitespace, then replace internal newlines (`\r\n`, `\n`, `\r`) with a single space `" "`.
- **Rationale**:
  - Guarantees the invariant: **1 search match = strictly 1 physical line in CSV**.
  - `total_lines = 1 (header line) + total_matches`.
  - Ensures 100% reliable piping to `grep`, `awk`, `head`, `wc -l`, and line-by-line processors without requiring complex multiline CSV parsers.
  - Standard CSV quoting rules (quoting fields containing commas or quotes) remain fully active and compliant via `csv::Writer`.
- **Alternatives Considered**:
  - RFC 4180 multiline quoted fields (status quo): Breaks CLI pipes, `grep`, and simple log parsers.
  - Escaped literal `\n` (`\\n`): Adds clutter and requires downstream unescaping; spaces provide cleaner human-readable output in Excel and terminals.
  - `<br>` tag in CSV: Unnatural for plain CSV consumers and spreadsheets.

---

### Decision 4: Centralized Normalization Functions & Constant Extraction

- **Decision**:
  - Create `escape_markdown_cell` / `sanitize_markdown_value` in `crates/core/src/search/llm_context.rs`.
  - Create `sanitize_csv_field` in `crates/core/src/export/csv_export.rs`.
  - Extract all constant strings (`<br>`, `" "`, etc.) into `crates/core/src/constants.rs` conforming to Constitution Principle II.
- **Rationale**:
  - Avoids duplication and ensures identical sanitization rules across GUI file export, CLI file export, CLI stdout streaming, and AI context clipboard copy.
  - All constants are centralized and accompanied by unit tests.

---

## 3. Technology & Dependency Evaluation

| Component | Technology | Impact / Risk |
|:---|:---|:---|
| **Markdown Escaping** | Pure Rust string replacement & trimming | Zero new dependencies; O(N) linear scan; zero allocation when no newlines present. |
| **CSV Sanitization** | Pure Rust string replacement & trimming | Zero new dependencies; O(N) linear scan; fully compatible with `csv::Writer`. |
| **Constants** | `crates/core/src/constants.rs` | Fully conforms to Constitution Principle II with explicit `// Constant reference:` comments. |
