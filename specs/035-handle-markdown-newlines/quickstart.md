# Quickstart & Verification Guide: Multiline Text in Markdown and CSV Exports

**Feature**: `035-handle-markdown-newlines`  
**Date**: 2026-10-25  
**Status**: Ready for Implementation  

All data examples in this document use synthetic dummy data conforming to Constitution Principle III.

---

## 1. Prerequisites & Environment

1. Repository root:
   ```bash
   cd /Users/soakaye/Develop/Projects/workspaces/exgrep/coelacanth
   ```
2. Build CLI sidecar for tests:
   ```bash
   npm run build:cli
   ```

---

## 2. Automated Test Verification

### 2.1 Unit Tests for Markdown and CSV Sanitization

Run targeted unit tests verifying `escape_markdown_cell` and `sanitize_csv_field`:

```bash
cargo test -p xlseek-core --lib escape_markdown_cell
cargo test -p xlseek-core --lib sanitize_csv_field
```

**Expected Result**: All tests pass, proving that:
- `<br>` replaces `\n` in Markdown.
- Pipes `|` are properly escaped.
- Leading/trailing whitespace and newlines are trimmed.
- Space `" "` replaces `\n` in CSV fields.

---

### 2.2 Integration Tests for Markdown and CSV Exports

Run integration test suites:

```bash
cargo test -p xlseek-core --test export_md_test
cargo test -p xlseek-core --test export_csv_test
cargo test -p xlseek-cli --test cli_search
```

**Expected Result**:
- Exported `.md` files contain `<br>` instead of flat spaces or broken rows.
- Exported `.csv` files contain exactly $N + 1$ lines without multiline record fragmentation.

---

## 3. End-to-End Verification Scenarios

### Scenario A: Markdown Table Export with Multiline Cells

1. Generate a test workbook `sample.xlsx` with a multiline cell `"Revenue\n(Projected)"`.
2. Run search and export to Markdown:
   ```bash
   cargo run -p xlseek-cli -- -p sample.xlsx -q "Revenue" -f md -o results.md
   ```
3. Inspect `results.md`:
   ```bash
   cat results.md
   ```
   **Verification**:
   - The cell appears as `Revenue<br>(Projected)`.
   - The table row is intact on a single line starting and ending with `|`.

---

### Scenario B: CSV Export Line Count Verification

1. Search the same workbook and export to CSV:
   ```bash
   cargo run -p xlseek-cli -- -p sample.xlsx -q "Revenue" -f csv -o results.csv
   ```
2. Count lines in the exported CSV:
   ```bash
   wc -l results.csv
   ```
   **Verification**:
   - If there is 1 match, line count is exactly `2` (1 header row + 1 match row).
   - Piping to grep works seamlessly:
     ```bash
     cargo run -p xlseek-cli -- -p sample.xlsx -q "Revenue" | grep "Revenue (Projected)"
     ```
