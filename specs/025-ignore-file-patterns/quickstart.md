# Quickstart: Ignore File Patterns Validation Guide

**Feature**: `025-ignore-file-patterns`  
**Date**: 2026-10-15  
**Status**: Complete

---

## 1. Overview & Prerequisites

This guide outlines step-by-step verification procedures to validate that gitignore-style ignore patterns, directory pruning, persistent history recall, and Settings default configurations function correctly across GUI, Core, and CLI components.

### Prerequisites
- Node.js (v18+) and npm installed
- Rust toolchain (stable, 2021 edition) installed
- Local test directory containing sample files:
  - `NormalWorkbook.xlsx` (contains target keyword "Revenue")
  - `~$NormalWorkbook.xlsx` (temporary lock file)
  - `Archive/OldReport.xlsx` (nested inside `Archive/` folder, contains "Revenue")
  - `Drafts/Draft1.xlsx` (nested inside `Drafts/` folder, contains "Revenue")

---

## 2. Automated Quality Gates

Run all repository quality gates to ensure clean compilation and conformance to Constitution principles:

```powershell
# 1. Frontend type checking and bundle build
npm run build

# 2. Rust workspace unit and integration tests
cargo test --workspace

# 3. Rust workspace Clippy linter (zero warnings)
cargo clippy --workspace --all-targets -- -D warnings

# 4. Rust formatting verification
cargo fmt --check
```

---

## 3. End-to-End Validation Scenarios

### Scenario 1: Default Lock File Exclusion (`~$*`)
1. **Launch**: Start the application (`npm run tauri dev`).
2. **Observe**: Verify the ignore patterns input field directly beneath the search directory row is pre-populated with `~$*`.
3. **Setup**: Target the test directory containing `NormalWorkbook.xlsx` and `~$NormalWorkbook.xlsx`.
4. **Action**: Search for keyword `Revenue`.
5. **Expected Outcome**:
   - `NormalWorkbook.xlsx` is matched and displayed in the results table.
   - `~$NormalWorkbook.xlsx` is skipped completely without throwing a workbook read error or appearing in the results.

---

### Scenario 2: Comma-Separated Multiple Patterns & Quoting
1. **Action**: In the ignore patterns field, enter: `~$*, Drafts/, "*.tmp"`
2. **Search**: Search for keyword `Revenue`.
3. **Expected Outcome**:
   - Matches from `NormalWorkbook.xlsx` and `Archive/OldReport.xlsx` appear.
   - Any files matching `Drafts/` or `~$*` are excluded.
   - Discovery skips entering the `Drafts/` directory entirely (early pruning).

---

### Scenario 3: Ignore Pattern History Persistence & Recall
1. **Action**:
   - Clear the ignore patterns field, enter `backup/*, draft/*`, and execute a search.
   - Open a new search or clear the input field.
   - Click the history button (Clock/History icon) next to the ignore patterns input.
2. **Expected Outcome**:
   - A dropdown menu displays previously used pattern combinations, with `backup/*, draft/*` at the top.
   - Clicking `backup/*, draft/*` or navigating with `ArrowDown` + `Enter` fills the input field.
3. **Session Persistence**:
   - Close the app and re-launch.
   - Open the history dropdown and verify `backup/*, draft/*` is still available.

---

### Scenario 4: Settings Dialog Customization
1. **Action**:
   - Open Settings dialog (Gear icon or Menu > Settings).
   - Select the "Saved Defaults" tab.
   - Locate "Default Ignore Patterns" (labeled in Japanese or English based on active language).
   - Change `~$*` to `~$*, *.bak`, and click Save.
2. **Expected Outcome**:
   - The search bar ignore patterns input updates to `~$*, *.bak`.
   - Reloading the app retains `~$*, *.bak` as the baseline initial value.

---

### Scenario 5: Standalone CLI Verification
Run the CLI binary to verify command-line pattern exclusion:

```powershell
# Run search excluding lock files and Archive folder
cargo run --bin xlseek-cli -- -p ".\tests\fixtures" -q "Revenue" -i "~$*, Archive/"
```

**Expected Outcome**:
- Output shows matches from unignored workbooks only.
- Exit code is 0.
