# Quickstart & Verification Guide: Scrollable Match Detail Area in Search Preview

**Feature Branch**: `029-detail-view-scroll` | **Date**: 2026-10-22 | **Spec**: [spec.md](./spec.md)

This document provides runnable validation procedures, automated test suites, and end-to-end verification scenarios for Feature 029.

---

## 1. Prerequisites & Environment Setup

- **Node.js**: v18+ with `npm` package manager
- **Rust**: 2021 edition (1.75+)
- **Git Branch**: `029-detail-view-scroll`

Confirm development environment readiness:
```bash
node -v
npm -v
cargo --version
```

---

## 2. Automated Quality Gates

All of the following verification commands MUST succeed with zero errors and zero warnings before completion:

### 2.1 Frontend Component Unit Tests (Vitest)
Run the dedicated test suite for `MetaInfoCard`:
```bash
npm test -- tests/meta-info-card.test.tsx
```

Run the entire frontend test suite (170+ tests):
```bash
npm test
```

### 2.2 Frontend Type Checking and Bundle Verification
Verify TypeScript type safety and Vite production bundle creation:
```bash
npm run build
```

### 2.3 Rust Workspace Quality Gates
Verify core engine and Tauri crate health:
```bash
# Run all workspace unit and integration tests
cargo test --workspace

# Compiler and Clippy linting checks
cargo clippy --workspace --all-targets -- -D warnings

# Code formatting compliance
cargo fmt --check
```

---

## 3. Manual Functional Verification Scenarios (E2E)

### Scenario 1: Long Cell Content Scrolling and Spreadsheet Grid Preservation
1. Launch the application in development mode:
   ```bash
   npm run tauri dev
   ```
2. Search for a keyword that appears in an Excel workbook with long multi-line cell values (e.g. 10+ lines or extensive paragraphs).
3. Select the result row in the search results table.
4. **Verification**:
   - The surrounding cell preview grid (`SpreadsheetGrid`) remains clearly visible with at least 3 rows displayed and does not collapse to zero height.
   - The match detail text box is capped at `max-h-36` (144px) and presents a functional vertical scrollbar.
   - Mouse wheel or scrollbar drag allows scrolling smoothly through the entire text from top to bottom.
   - The header ("Match Details / Type") and footer ("Row / Col / Hidden") remain statically visible during scrolling.

### Scenario 2: Short Cell Content Display
1. Select a search result containing a short word or numeric value (1–2 lines).
2. **Verification**:
   - The detail text box sizes naturally to fit the content without showing vertical scrollbars or redundant empty space.

### Scenario 3: Standalone UI Prototype Consistency
1. Open `design/mainui/index.html` in a web browser.
2. Confirm that `#metaFullContent` incorporates the identical scrollable styles and harmonizes with the surrounding preview layout.
