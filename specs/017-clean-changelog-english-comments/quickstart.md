# Quickstart: Validating Clean Changelog Headers and English Source Comments

**Feature**: `017-clean-changelog-english-comments`
**Date**: 2026-09-30

This guide outlines runnable verification steps to confirm that in-file changelog headers have been completely removed, comments have been converted to English, and all automated quality gates pass.

## 1. Prerequisites

- Node.js & npm installed
- Rust toolchain (2021 edition, `cargo`, `clippy`, `rustfmt`)
- Working tree on branch `017-clean-changelog-english-comments`

## 2. Automated Static Verification

### A. Verify Changelog Blocks Removal

Run search commands across source code to confirm zero matches for changelog / revision patterns:

```bash
# Check for any remaining Japanese changelog headers
git grep -n "変更履歴" -- 'crates/' 'src-tauri/' 'src/' 'design/mainui/index.html'

# Check for any remaining English revision history blocks
git grep -n -i "revision history" -- 'crates/' 'src-tauri/' 'src/' 'design/mainui/index.html'
```

**Expected Outcome**: Both commands return empty (exit code 1 or zero lines matched).

### B. Verify English Comments in Source Code

Verify that no Japanese character sequences remain within comments across Rust and TypeScript source files (excluding `crates/core/locales/`):

```bash
# Verify no Japanese characters remain in code comments in crates/
git grep -P '//.*[\p{Hiragana}\p{Katakana}\p{Han}]' -- 'crates/' 'src-tauri/' 'src/'
```

**Expected Outcome**: Zero matches.

## 3. Quality Gate Validation Commands

Run the full project verification suite:

```bash
# 1. Frontend Build & Typecheck
npm run build

# 2. Rust Unit and Integration Tests
cargo test --workspace

# 3. Rust Clippy Linters
cargo clippy --workspace --all-targets -- -D warnings

# 4. Rust Formatting
cargo fmt --check
```

**Expected Outcome**: All four commands exit with code 0 and report 0 warnings / 0 failures.
