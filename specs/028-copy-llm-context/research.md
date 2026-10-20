# Research & Architecture Decisions: Context-Aware Header Extraction and Prompt Copy for LLM

**Feature Branch**: `028-copy-llm-context` | **Date**: 2026-10-19 | **Spec**: [spec.md](./spec.md)

This document records the research findings, design tradeoffs, and technical decisions made during Phase 0 for Feature 028.

---

## 1. Excel Header and Key Column Detection Heuristic

### Problem
In Excel worksheets, matched cells (e.g. cell `G42` containing `150,000`) often reside far away from row 1 and column A. Real-world business sheets frequently include title banners (rows 1–2), merged category headings, or blank margin columns before the actual tabular data starts.

### Decision
Implement an intelligent upward-scan and leftward-scan heuristic within `crates/core`:
1. **Column Header Detection**:
   - Given worksheet dimensions and target cell `(r, c)`:
   - Inspect cells in column `c` upward from `r - 1` to find candidate header cells.
   - Compare candidates against the sheet's top populated rows (rows within the worksheet's non-empty range).
   - If a candidate cell contains non-empty text, verify if adjacent columns in the same row also contain labels (indicative of a table header row).
   - If found, that row is designated as the `header_row`.
   - Fallback: If `r == 0` (1-based row 1), the hit cell itself resides on the header row. If no header row is identified, default to row 0 (row 1 in 1-based indexing) or use the column label (`Column G`).
2. **Key Column (Row Header) Detection**:
   - Inspect row `r` from left to right starting at the sheet's minimum populated column (typically column A, index 0).
   - The first non-empty cell to the left of column `c` containing descriptive text (or an alphanumeric identifier/code) is designated as the `key_column`.
   - Fallback: Column 0 (Column A).
3. **Merged Header Handling**:
   - If the detected header cell is part of a horizontal merged range, resolve the merged top-left cell text.

### Rationale
- Operates in $O(H + W)$ time without requiring a full sheet AST or ML model.
- Accurately identifies header rows in sheets with 1–5 banner rows preceding the data table.
- Gracefully degrades to standard spreadsheet coordinates (`Row 1`, `Col A`) without throwing panics or failing.

### Alternatives Considered
- **Full Table Range Parsing via calamine dimensions only**: Calamine provides bounding ranges, but title banners at row 0 cause naive row 0 headers to misattribute banner titles (e.g., "Company Annual Report 2026") as the column header instead of row 3's "Price". Rejected as insufficient for banner-bearing sheets.
- **Requiring User-Configured Header Rows per Sheet**: Burdens users with configuration dialogs and breaks the zero-friction one-click requirement. Rejected.

---

## 2. Wide Sheet Column Windowing and Ellipsis Formatting

### Problem
Large enterprise spreadsheets often span 50 to 100+ columns (e.g., monthly financials, ERP extracts). Exporting all 100 columns into a Markdown table produces unreadable output that wraps excessively and consumes unnecessary LLM token context.

### Decision
Implement configurable column windowing with ellipsis preservation:
- Always preserve the **leading key column** (e.g., Column A / row identifier) at table position 1.
- Extract a symmetric window around the hit column: `[target_col - W, target_col + W]`, where $W$ is the user-configured `column_window_size` (default: 5 columns before and after).
- If there is a gap between the leading key column and `target_col - W`, insert an ellipsis column (`| ... |`) representing omitted columns.
- If `target_col + W` is less than the sheet's last column, append a trailing ellipsis column (`| ... |`).
- Inside the Markdown table data row, format the hit cell value with Markdown bold syntax (`**value**`).
- Multi-hit rows from the same sheet are consolidated into a single Markdown table sharing the windowed column headers.

### Rationale
- Keeps table width strictly bounded (typically 12–15 columns total), ensuring clean Markdown rendering in ChatGPT, Claude, and Copilot.
- Retains both identity context (Key Column) and local attribute context (adjacent columns).
- Clearly signals to the LLM that columns were omitted via standardized `...` notation.

### Alternatives Considered
- **Hard truncation from Column A up to column N**: If the search hit is in Column AZ (col 52), hard truncation from Column A excludes the hit cell completely! Rejected.
- **Unbounded full-row copy**: Causes LLM context overflow, broken table formatting, and slow clipboard operations. Rejected.

---

## 3. Word Document Context Extraction Architecture

### Problem
`xlseek` searches Word documents (`.docx`, `.docm`, `.doc`). Unlike Excel sheets, Word files lack two-dimensional cell coordinates and column headers.

### Decision
Provide two selectable context extraction modes:
1. **Mode A (`headings_and_paragraph`)**:
   - Extract the nearest preceding heading (e.g., Heading 1/2/3 style, or bold paragraph) above the match.
   - Format output with section title and blockquote:
     ```markdown
     - File: sample.docx
     - Location: Paragraph 42
     - Section: 3.2 System Architecture

     ### 3.2 System Architecture
     > ... surrounding sentence **matched text** surrounding sentence ...
     ```
2. **Mode B (`paragraph_only`)**:
   - Extract paragraph index and quote:
     ```markdown
     - File: sample.docx
     - Location: Paragraph 42

     > ... surrounding sentence **matched text** surrounding sentence ...
     ```
- Both modes are selectable via the search results right-click context menu and configurable as a default in application settings.

### Rationale
- Aligns Word search hits with the LLM prompt copying feature without forcing inappropriate spreadsheet metaphors.
- Directly leverages existing Word OOXML paragraph extraction utilities in `crates/core/src/search/word/`.

### Alternatives Considered
- **Excluding Word documents from LLM copy**: Fails User Story requirements and causes user confusion when right-clicking Word results. Rejected.

---

## 4. IPC Architecture and System Clipboard Integration

### Decision
- Backend (`crates/core` + `src-tauri`):
  - Expose Tauri command: `generate_llm_context(matches: Vec<SearchMatch>, options: LlmCopyOptions) -> Result<String, CommandError>`.
  - Perform all file reading, header detection, row extraction, Markdown table formatting, and Word context assembly in Rust.
- Frontend (`src/`):
  - On triggering "Copy for LLM" (via preview toolbar or right-click context menu), invoke `generate_llm_context`.
  - On receiving the formatted string, copy it to the clipboard using browser-standard `navigator.clipboard.writeText(text)`.
  - Dispatch a success toast notification: `"Copied context for LLM to clipboard"`.

### Rationale
- Keeps parsing and formatting high-speed, parallel-capable, and non-blocking in Rust.
- Utilizing `navigator.clipboard` in the frontend ensures seamless cross-platform desktop compatibility (Windows, macOS, Linux X11/Wayland) without requiring platform-specific Rust clipboard crates.

---

## 5. Right-Click Context Menu Architecture

### Decision
- Implement a lightweight, custom React context menu component `ContextMenu.tsx` within `src/components/results/`.
- Listens to `onContextMenu` on result rows in `ResultTable.tsx`.
- Captures click coordinate `(x, y)`, clamps to window bounds, and renders copy choices:
  - Excel hits:
    - `Copy for LLM (Table Context)`
    - `Copy for LLM (Header & Cell Only)`
  - Word hits:
    - `Copy for LLM (With Headings & Context)`
    - `Copy for LLM (Paragraph Only)`
- Automatically closes on outside click or Esc key press.
- Instant rendering (<10ms), fully themed with Tailwind CSS and Lucide icons.

### Rationale
- Completely avoids the latency and OS-level limitations of Tauri native menus.
- Easily extensible for future context actions.
