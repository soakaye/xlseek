# Quickstart & Verification Guide: Explicit Sheet Tab Name in AI Copy Target Cell Metadata

**Feature Branch**: `030-ai-copy-tab-name` | **Date**: 2026-10-23 | **Spec**: [spec.md](./spec.md)

This guide documents how to validate the explicit sheet tab name formatting in LLM context generation.

---

## 1. Prerequisites

- Rust toolchain (2021 edition, `cargo`)
- Node.js & npm (for workspace build checks)
- Working repository at branch `030-ai-copy-tab-name`

---

## 2. Automated Unit Test Verification

Run the targeted unit tests for `llm_context` and `constants`:

```powershell
# Run core LLM context tests
cargo test --package xlseek-core search::llm_context

# Run core constants tests
cargo test --package xlseek-core constants::tests
```

### Expected Outcomes
- Tests verify:
  1. `format_target_cell_line` produces `- Target Cell: [Sheet1] A1 (Column: ..., Row: ...)` when sheet name is present.
  2. `format_target_cell_line` falls back to `- Target Cell: A1 (Column: ..., Row: ...)` when sheet name is empty.
  3. Table mode context (`format_table_excel_context`) includes `[SheetName]` in the `Target Cell` line.
  4. Header-only mode context (`format_header_only_excel_context`) includes `[SheetName]` in the `Target Cell` line.
  5. Japanese and special characters in sheet names (e.g. `シート1`, `FY2026 Q3 (Final)`) format without corruption.
  6. Word context generation remains unchanged and passes all existing assertions.

---

## 3. Full Project Quality Gates

Before finalizing and committing changes, execute the four required project quality gates:

```powershell
# 1. Frontend type check & build
npm run build

# 2. Rust workspace tests
cargo test --workspace

# 3. Rust Clippy warnings check
cargo clippy --workspace --all-targets -- -D warnings

# 4. Rust formatting check
cargo fmt --check
```

All 4 commands must exit with code 0.

---

## 4. Manual / GUI End-to-End Verification

1. Start the desktop development application:
   ```powershell
   npm run tauri dev
   ```
2. Search for any term that produces matches in an Excel file with a named worksheet (e.g. `Sheet1` or `シート1`).
3. Select a match in the search results table.
4. Click the "AI Copy" button (or right-click → "Copy for LLM").
5. Paste the clipboard contents into a text editor.
6. Verify the `Target Cell` metadata line reads:
   `- Target Cell: [<SheetName>] <CellAddress> (Column: <Header>, Row: <Key>)`
7. Switch AI Copy mode in settings to "Header Only", repeat the copy, and confirm the same `[<SheetName>]` prefix is present.
