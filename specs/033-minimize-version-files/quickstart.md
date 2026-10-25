# Quickstart & Verification Guide: Minimal Version Bump Files and Consolidated Display Copyright

**Feature Branch**: `033-minimize-version-files`
**Date**: 2026-10-25

This document outlines the step-by-step verification procedures to validate that version bumping modifies only the minimal set of authoritative files, that `Cargo.lock` is guaranteed to be staged in the version commit, and that display copyright is dynamically rendered.

---

## Prerequisites

- Node.js 18+ and npm installed
- Rust 1.75+ and Cargo installed
- Clean Git working tree (`git status` reports clean)

---

## Verification Scenarios

### Scenario 1: Version Synchronization Unit Tests

Run the dedicated test suite for version synchronization:
```bash
npm run test:sync-version
```
**Expected Outcome**:
- All test cases pass.
- Tests verify:
  - SemVer validation and normalization.
  - Root `Cargo.toml` `[workspace.package].version` updates.
  - Staging verification of `Cargo.lock` and `Cargo.toml`.
  - Elimination of auxiliary target files (`src/constants/index.ts`, `locales/*.yml`, `design/mainui/index.html`).

---

### Scenario 2: Rust Workspace Build and Inheritance Verification

Validate that all member crates compile and inherit metadata from the workspace root:
```bash
npm run build:cli
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```
**Expected Outcome**:
- All workspace members (`xlseek-core`, `xlseek-cli`, `xlseek-mcp`, `xlseek`) compile with zero errors.
- All unit and integration tests pass.
- Clippy reports zero warnings.

---

### Scenario 3: Frontend Type Check and Build Verification

Validate that `__APP_VERSION__` is properly injected via Vite and that TypeScript compilation succeeds:
```bash
npm run build
```
**Expected Outcome**:
- TypeScript type checking (`tsc`) passes with zero errors.
- Vite bundle builds successfully with exit code `0`.

---

### Scenario 4: End-to-End Version Bump Simulation

Test the minimal file bump workflow:
```bash
# 1. Check current git status
git status --porcelain

# 2. Run a patch version bump without git tag to inspect diff
npm version patch --no-git-tag-version

# 3. Verify exactly 4 files are modified
git status -s
```
**Expected Outcome**:
- `git status -s` displays ONLY:
  - `M  package.json`
  - `M  package-lock.json`
  - `M  Cargo.toml`
  - `M  Cargo.lock`
- Zero changes in `crates/core/`, `crates/cli/`, `crates/mcp/`, `src-tauri/tauri.conf.json`, `src/constants/index.ts`, or `locales/*.yml`.
- Both `Cargo.toml` and `Cargo.lock` are staged in the Git index ready for commit.

```bash
# Clean up simulation changes
git restore --staged Cargo.toml Cargo.lock
git restore Cargo.toml Cargo.lock package.json package-lock.json
```

---

### Scenario 5: About Dialog Display Verification

Launch the application in development mode or preview:
```bash
npm run dev
```
1. Open the application window.
2. Click the gear / info button to open the **About Dialog**.
3. Verify:
   - Version displays: `Version <active_version>` matching `package.json`.
   - In English locale:
     - Label: `Copyright:`
     - Value: `Copyright © 2026 soakaye` (with active calendar year).
   - Switch language to Japanese:
     - Label: `著作権:`
     - Value: `Copyright © 2026 soakaye`.
