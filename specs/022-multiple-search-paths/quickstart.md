# Quickstart: Validating Multiple Search Target Paths

**Feature**: `022-multiple-search-paths`  
**Date**: 2026-10-02

This guide outlines end-to-end scenarios to verify multi-path search functionality across Rust unit tests, the CLI tool, and the GUI application.

## Prerequisites
- Rust toolchain (`cargo`, `clippy`, `rustfmt`)
- Node.js & npm (`npm run build`)
- Existing test fixtures in `tests/fixtures` or temporary test folders.

---

## Scenario 1: Core Unit Tests for Quoted Path Tokenizer

Run the path tokenizer unit tests:
```bash
cargo test -p xlseek-core search::path::tests
```
**Verification Points**:
- Commas within quotes (e.g. `"dir, with, comma"`) are kept intact.
- Outside commas split tokens cleanly (e.g. `dir1, dir2` -> `["dir1", "dir2"]`).
- Whitespace around unquoted tokens is trimmed.
- Empty segments from trailing or consecutive commas are omitted.
- Unclosed quotes return `PathParseError::UnclosedQuote`.

---

## Scenario 2: CLI Multiple Target Path Search

Run `xlseek-cli` with multiple comma-separated and quoted paths:
```bash
# Comma-separated paths in -p
cargo run -p xlseek-cli -- "sample" -p "tests/fixtures, tests/fixtures" -o /tmp/results.csv -w

# Quoted path containing spaces in -p
cargo run -p xlseek-cli -- "sample" -p "\"tests/fixtures\", tests/fixtures" -o /tmp/results.csv -w

# Positional arguments with multiple paths
cargo run -p xlseek-cli -- "sample" tests/fixtures tests/fixtures -o /tmp/results.csv -w
```
**Verification Points**:
- Output CSV contains matching cells found in workbooks across fixture directories.
- Exit code is 0.

---

## Scenario 3: Mixed Valid and Invalid Paths (Non-fatal warning)

Run search specifying one valid fixture path and one non-existent path:
```bash
cargo run -p xlseek-cli -- "sample" -p "tests/fixtures, /path/does/not/exist" -o /tmp/results.csv -w
```
**Verification Points**:
- Warning diagnostic is printed indicating `/path/does/not/exist` was not found.
- Results from `crates/core/tests/fixtures` are still searched and saved to `/tmp/results.csv`.
- Exit code indicates successful search for valid roots.

---

## Scenario 4: GUI Verification

1. Launch `xlseek` desktop GUI via `cargo tauri dev` or `npm run dev`.
2. In the folder path input, enter two directory paths separated by a comma:
   `crates/core/tests/fixtures, crates/cli/tests/fixtures`
3. Click "Select Folder" button and choose a folder with a space in its name (e.g. `My Documents`).
   - Verify it appends as `, "My Documents"`.
4. Run search for a known keyword.
5. Verify matching results appear from workbooks in both directories.
