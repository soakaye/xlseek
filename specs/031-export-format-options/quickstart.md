# Quickstart & Verification Guide: Search Results Export Format Options and UI Reorganization

**Feature**: `031-export-format-options`  
**Date**: 2026-10-24

## 1. Prerequisites

- Rust toolchain (`cargo`, `rustfmt`, `clippy`)
- Node.js & npm (`npm install`)

## 2. Automated Verification Commands (Quality Gates)

Run all four workspace quality gates from the repository root:

```bash
# 1. Frontend TypeScript & Vite bundle check + unit tests
npm run build
npm test -- --run

# 2. Rust workspace unit and integration tests (core, cli, mcp, src-tauri)
cargo test --workspace

# 3. Rust Clippy static analysis
cargo clippy --workspace --all-targets -- -D warnings

# 4. Rust formatting check
cargo fmt --check
```

## 3. End-to-End Validation Scenarios

### Scenario A: CLI Export in Markdown and JSON (`xlseek-cli`)

```bash
# Stream JSON search results to stdout
cargo run --package xlseek-cli -- -f json "dummy" crates/core/tests/fixtures

# Stream AI Markdown search results to stdout
cargo run --package xlseek-cli -- -f md "dummy" crates/core/tests/fixtures

# Export to .json file (auto-inferred from extension)
cargo run --package xlseek-cli -- -o /tmp/sample_export.json --overwrite "dummy" crates/core/tests/fixtures

# Export to .md file (auto-inferred from extension)
cargo run --package xlseek-cli -- -o /tmp/sample_export.md --overwrite "dummy" crates/core/tests/fixtures
```

### Scenario B: GUI Status Bar Export Split Button

1. Launch the desktop application (`npm run tauri dev`).
2. Verify that when 0 search results are present, the Export Split Button (both main button and `▾` chevron) in the right side of the status bar is disabled.
3. Run a search against a synthetic test directory yielding matches.
4. Click the `▾` toggle on the Export Split Button and verify that an upward menu opens displaying:
   - Excel (`.xlsx`)
   - CSV (`.csv`)
   - AI Markdown (`.md`)
   - JSON (`.json`)
5. Select **AI Markdown (`.md`)** or **JSON (`.json`)** from the menu:
   - Verify the split button updates its active format label/icon.
   - Verify the OS file save dialog opens immediately with `.md` or `.json` default filename and filter.
   - Save the file and verify the completion toast displays the saved path and the file contents match the expected Markdown or JSON schema in [data-model.md](./data-model.md).
