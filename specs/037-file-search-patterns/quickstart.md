# Quickstart: File Search Patterns Validation Guide

**Feature**: `037-file-search-patterns`  
**Date**: 2026-10-27  
**Status**: Complete  

---

## 1. Overview & Prerequisites

This guide describes end-to-end verification procedures to confirm that inclusion file search patterns, automatic extension complement, sequential filtering (Include Patterns → Ignore Patterns), pattern history, CLI arguments, and MCP integration function reliably across all platforms.

### Synthetic Test Environment (Principle III Compliance)
All scenarios use synthetic dummy data created in a temporary test directory:
- `dummy_budget_2024.xlsx` (contains needle keyword `"TargetValue"`)
- `dummy_quarterly_report.xlsx` (contains `"TargetValue"`)
- `dummy_inventory.xlsx` (contains `"TargetValue"`)
- `dummy_legacy_data.xls` (contains `"TargetValue"`)
- `~$dummy_budget_2024.xlsx` (temporary lock file, contains `"TargetValue"`)
- `archive/dummy_budget_2024.xlsx` (archive directory copy, contains `"TargetValue"`)

---

## 2. Automated Quality Gates

Verify that all automated code quality and linting gates pass with zero errors and zero warnings:

```bash
# 1. Frontend TypeScript typecheck and Vite build
npm run build

# 2. Rust workspace unit and integration tests
cargo test --workspace

# 3. Rust workspace Clippy static analysis (zero warnings)
cargo clippy --workspace --all-targets -- -D warnings

# 4. Rust formatting verification
cargo fmt --check
```

---

## 3. End-to-End Validation Scenarios

### Scenario 1: Extensionless Pattern with Automatic Extension Complement
1. **Target**: Directory containing synthetic files.
2. **Options**: Ensure `.xlsx` and `.xlsm` are enabled in extension toggles.
3. **Input**:
   - File Search Patterns: `dummy_budget*` (no extension specified)
   - Search Query: `TargetValue`
4. **Execution**: Click **Search** (or press Enter).
5. **Expected Outcome**:
   - System complements `dummy_budget*` to `dummy_budget*.xlsx` and `dummy_budget*.xlsm`.
   - `dummy_budget_2024.xlsx` is scanned and displayed in search results.
   - `dummy_quarterly_report.xlsx` and `dummy_inventory.xlsx` are skipped.

---

### Scenario 2: Multiple Comma-Separated Patterns (OR Logic)
1. **Input**:
   - File Search Patterns: `*quarterly*, *inventory*`
   - Search Query: `TargetValue`
2. **Execution**: Execute search.
3. **Expected Outcome**:
   - Both `dummy_quarterly_report.xlsx` and `dummy_inventory.xlsx` are matched and scanned.
   - `dummy_budget_2024.xlsx` is skipped.

---

### Scenario 3: Sequential Filtering (Include Filter First, Then Ignore Filter)
1. **Input**:
   - File Search Patterns: `*budget*`
   - Ignore Patterns: `~$*, archive/*`
   - Search Query: `TargetValue`
2. **Execution**: Execute search.
3. **Expected Outcome**:
   - `dummy_budget_2024.xlsx` matches inclusion pattern and does not match ignore pattern → Scanned and matched.
   - `~$dummy_budget_2024.xlsx` matches inclusion pattern BUT matches ignore pattern `~$*` → Excluded and skipped.
   - `archive/dummy_budget_2024.xlsx` matches inclusion pattern BUT matches ignore pattern `archive/*` → Excluded and skipped.
   - `dummy_inventory.xlsx` does not match inclusion pattern → Excluded immediately during Step 1 (never evaluated by ignore filter).

---

### Scenario 4: Explicit Extension Precedence
1. **Setup**: In extension toggles, uncheck `.xls` (leave only `.xlsx` checked).
2. **Input**:
   - File Search Patterns: `dummy_legacy_data.xls` (explicit `.xls` extension)
   - Search Query: `TargetValue`
3. **Execution**: Execute search.
4. **Expected Outcome**:
   - Explicitly specified extension overrides the unchecked `.xls` toggle for supported formats.
   - `dummy_legacy_data.xls` is scanned and matches are displayed.

---

### Scenario 5: CLI Invocation with `-g` and `--file-patterns`
1. **Command**:
   ```bash
   cargo run -p xlseek-cli -- -q "TargetValue" -p "dummy/path" -g "*budget*" -i "~$*"
   ```
2. **Expected Outcome**:
   - CLI parses `-g` as file search patterns and `-i` as ignore patterns.
   - Only `dummy_budget_2024.xlsx` is processed and reported.

---

### Scenario 6: MCP Tool Invocation with `file_patterns`
1. **Request Payload**:
   ```json
   {
     "name": "search",
     "arguments": {
       "paths": ["dummy/path"],
       "query": "TargetValue",
       "file_patterns": "*budget*, *report*",
       "ignore_patterns": "~$*"
     }
   }
   ```
2. **Expected Outcome**:
   - Tool returns structured matches only for files matching the specified file search patterns and not matching `~$*`.

---

### Scenario 7: History Recall & Settings Default Configuration
1. **History Dropdown**:
   - Run a search with pattern `*budget*, *report*`.
   - Clear input, click the history dropdown icon on the File Search Patterns row.
   - Verify that `*budget*, *report*` is displayed in the dropdown and selecting it repopulates the input.
2. **Settings Saved Defaults**:
   - Open Settings > Saved Defaults.
   - Enter `*budget*` in "Default File Search Patterns". Save and close.
   - Reset the search bar or reload the app; verify the File Search Patterns field initializes with `*budget*`.
