# Quickstart & Verification Guide: Horizontal Side-by-Side Pattern Inputs Layout

**Feature Branch**: `038-horizontal-pattern-inputs`  
**Date**: 2026-10-28  
**Spec Reference**: [spec.md](./spec.md)  
**Layout Contract**: [ui-layout-contract.md](./contracts/ui-layout-contract.md)

---

## 1. Prerequisites & Environment Setup

Verify local node and cargo toolchains:

```bash
# Verify Node and npm
node --version
npm --version

# Verify Rust
cargo --version
```

---

## 2. Automated Validation Commands (Quality Gates)

Run the mandatory project verification commands:

```bash
# 1. Frontend automated tests
npm test

# 2. Frontend typecheck and bundle build
npm run build

# 3. Rust backend test suite
cargo test --workspace

# 4. Rust clippy linting
cargo clippy --workspace --all-targets -- -D warnings

# 5. Rust formatting check
cargo fmt --check
```

---

## 3. Visual & Interactive Acceptance Scenarios

### Scenario A: Side-by-Side Layout on Standard Desktop Window ($\ge 768$px)

1. Run `npm run tauri dev` or open `design/mainui/index.html` in a web browser.
2. Observe the search options header directly below the Search Directory row:
   - **Left Column**: "検索パターン:" (`Search patterns:`) with `FileText` icon, input field, and action buttons.
   - **Right Column**: "除外パターン:" (`Ignore patterns:`) with `Filter` icon, input field, and action buttons.
3. Verify both controls are aligned horizontally on the same line with equal column widths.

### Scenario B: Input Typing & Clearing

1. Type `*budget*, sample*.xlsx` in the left input:
   - The clear (`X`) button appears immediately on the right side of the left input box.
2. Click the clear (`X`) button:
   - The left input value is cleared to empty `""`.
   - The right input value is untouched.
3. Repeat the same test in the right input with `~$*, *.tmp`.

### Scenario C: History Dropdown Stacking & Isolation

1. Click the `History` button on the left input:
   - The `FilePatternHistoryDropdown` opens directly underneath the left input box.
   - The dropdown width matches the input field.
   - The right input and other UI elements remain clearly visible and interactive.
2. Click the `History` button on the right input:
   - The left dropdown closes immediately.
   - The `IgnoreHistoryDropdown` opens directly underneath the right input box.
3. Click anywhere outside the dropdown or press `Escape`:
   - The active dropdown closes.

### Scenario D: Responsive Stacking on Narrow Window ($< 768$px)

1. Resize the browser / application window to less than $768$px width.
2. Observe that the pattern controls cleanly stack into single rows:
   - File Search Patterns row on top.
   - Ignore Patterns row directly beneath.
3. Confirm that no buttons or labels are truncated or horizontally overflowing.
