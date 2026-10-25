# Quickstart Validation Guide: Clarify Search Results Save Button

**Feature**: `034-clarify-save-results-button`
**Date**: 2026-10-25

## 1. Overview

This guide provides end-to-end verification scenarios to validate that the search results save split button explicitly conveys its purpose in both Japanese and English, properly updates its label upon format change, and introduces zero regressions.

*Note*: In strict compliance with Constitution Principle III, all file paths and queries used in validation scenarios are synthetic dummy examples (e.g., `tests/fixtures/synthetic_sample.xlsx`).

---

## 2. Prerequisites & Build Verification

Ensure all development tooling passes cleanly before manual or automated testing:

```bash
# 1. Frontend type checking and bundle validation
npm run build

# 2. Frontend unit tests
npm test -- src/components/common/__tests__/StatusBar.test.tsx --run

# 3. Rust workspace checks
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --check
```

---

## 3. Automated Test Scenarios

Run the updated Vitest suite:

```bash
npm test -- src/components/common/__tests__/StatusBar.test.tsx --run
```

Expected Automated Results:
1. `renders disabled export split button when results are empty`: Primary button has accessible name `Save search results (Excel (.xlsx))` and visible text `Save Results: Excel (.xlsx)` in disabled state.
2. `toggles format menu open and closed`: Menu toggles open with 4 format choices (`Excel (.xlsx)`, `CSV (.csv)`, `AI Markdown (.md)`, `JSON (.json)`).
3. `persists selected format and updates primary button`: Choosing Markdown updates the button label to `Save Results: AI Markdown (.md)`.

---

## 4. Manual Verification Scenarios (Desktop GUI)

Launch the desktop GUI:

```bash
npm run tauri dev
```

### Scenario 1: Initial Empty / Idle State

1. Open the application.
2. Observe the bottom status bar without performing a search.
3. **Verify**:
   - The export split button is disabled.
   - The button visibly displays the spreadsheet icon followed by `結果を保存: Excel (.xlsx)` (Japanese) or `Save Results: Excel (.xlsx)` (English).
   - Tooltip reads `検索結果を保存 (Excel (.xlsx))` / `Save search results (Excel (.xlsx))`.
   - The status bar height is exactly 40px with no vertical distortion.

### Scenario 2: Active Results State & Primary Click

1. Enter a synthetic test path (e.g. `tests/fixtures`) and search for a known synthetic term (e.g. `sample`).
2. When search completes with matches:
   - The export button becomes enabled.
   - Click the primary button.
3. **Verify**:
   - The native OS file save dialog opens immediately with `.xlsx` filter.
   - The default file name is `ExcelSeek_Results_<timestamp>.xlsx`.
   - Cancel the save dialog; verify the button returns to normal ready state.

### Scenario 3: Format Menu Switching

1. Click the dropdown toggle button (`▾`).
2. The format menu opens upwards, listing 4 options.
3. Click `CSV (.csv)`.
4. **Verify**:
   - The menu closes.
   - The primary button immediately updates its text to `結果を保存: CSV (.csv)` (or `Save Results: CSV (.csv)`) and icon updates to the text document icon.
   - The native OS file save dialog opens immediately with `.csv` filter.
   - Close the application and re-open; verify that the persisted format (`CSV (.csv)`) is restored.

### Scenario 4: Window Resizing & Responsive Layout

1. Resize the application window to its minimum supported width (800px).
2. **Verify**:
   - The status text truncates with ellipsis.
   - The export split button and settings button remain fully visible, aligned, and unobstructed.
   - No vertical layout expansion occurs.

---

## 5. Standalone Prototype Verification

Open `design/mainui/index.html` in a web browser:

```bash
open design/mainui/index.html
```

**Verify**:
- The status bar shows the updated button label `結果を保存: Excel (.xlsx)` with tooltip `検索結果を保存 (Excel (.xlsx))`.
