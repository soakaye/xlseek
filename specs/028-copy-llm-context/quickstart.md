# Quickstart & Verification Guide: Context-Aware Header Extraction and Prompt Copy for LLM

**Feature Branch**: `028-copy-llm-context` | **Date**: 2026-10-19 | **Spec**: [spec.md](./spec.md)

This document provides runnable validation procedures and automated test suites to verify Feature 028 end-to-end.

---

## 1. Prerequisites & Environment Setup

- **Rust**: 1.75+ with Cargo workspace configured
- **Node.js**: v18+ with `npm` dependencies installed
- **Platform**: Windows, macOS, or Linux

Verify environment readiness:
```bash
cargo --version
npm --version
```

---

## 2. Automated Test Verification (Quality Gates)

### 2.1 Rust Core & Backend Tests
Execute unit and integration tests covering header extraction, heuristics, column windowing, Markdown table generation, and Word context extraction:
```bash
# Run tests for core crate
cargo test --package xlseek-core search::llm_context

# Run complete workspace tests
cargo test --workspace

# Linting and style verification
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --check
```

### 2.2 Frontend Type Checking & Bundling
Confirm that TypeScript models, hooks, components, and constants compile cleanly with zero type errors:
```bash
npm run build
```

---

## 3. End-to-End Functional Test Scenarios

### Scenario 1: One-Click Excel Context Copy with Column & Row Headers
1. Launch the application:
   ```bash
   npm run tauri dev
   ```
2. Search for a keyword that appears in an offset cell (e.g., cell `G42` with value `150,000`).
3. Select the result row in the search results table.
4. Click the **"Copy for LLM"** button in the preview pane toolbar.
5. Confirm that a toast appears: `"Context copied for LLM to clipboard"`.
6. Paste the clipboard contents into a text editor.
7. **Verification**:
   - Metadata lists File, Sheet, Cell Coordinate (`G42`), Column Header (e.g. `Amount`), and Row Key (e.g. `Task 42`).
   - Markdown table presents column headers aligned with row data.
   - The cell value `150,000` is formatted in bold (`**150,000**`).

### Scenario 2: Right-Click Context Menu Selection
1. Right-click an Excel search result row in the search results table.
2. Verify that a context menu appears immediately (<50ms) with options:
   - `Copy for LLM (Table Context)`
   - `Copy for LLM (Header & Cell Only)`
3. Select `Copy for LLM (Header & Cell Only)`.
4. Paste clipboard into editor.
5. **Verification**: The snippet contains pure compact metadata and key/value pairs without the multi-column table.

### Scenario 3: Wide Sheet Column Windowing and Ellipsis
1. Perform a search in a test workbook with 30+ columns where the hit cell is located at column `P` (col index 15).
2. Trigger `Copy for LLM`.
3. Paste clipboard into editor.
4. **Verification**:
   - Column 1 contains the leading key column (Column A).
   - An ellipsis column (`| ... |`) indicates omitted columns between A and K.
   - Columns K through U (5 columns before and after P) are displayed.
   - A trailing ellipsis column (`| ... |`) indicates omitted columns after U.

### Scenario 4: Word Document Context Extraction
1. Search for a keyword located in a Word document (`.docx` or `.docm`).
2. Right-click the Word search result.
3. Select `Copy for LLM (With Headings & Context)`.
4. Paste into editor.
5. **Verification**:
   - Preceding section heading is rendered as `### Heading Title`.
   - Matched paragraph text is rendered as a blockquote (`> ...`).
   - Matched search terms are highlighted in bold.

### Scenario 5: Settings Customization & Persistence
1. Open the Settings dialog.
2. Locate the "AI / LLM Context Copy" section.
3. Change the column window size from `5` to `3`.
4. Close Settings and re-open to confirm the setting persisted.
5. Trigger `Copy for LLM` on a wide sheet and verify that only 3 adjacent columns before and after the hit cell are extracted.
