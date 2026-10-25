# Research & Architecture Decisions: Minimal Version Bump Files and Consolidated Display Copyright

**Feature Branch**: `033-minimize-version-files`
**Date**: 2026-10-25

## Overview

This document analyzes the current version synchronization and display copyright architecture in Excel Seek (`xlseek`), identifies root causes of multi-file churn and lockfile omissions, and establishes technical architecture decisions to minimize modified files and guarantee atomic version commits.

---

## Current State Analysis

### 1. Version Update Churn
Currently, running `npm version` executes `scripts/sync-version.js`, which modifies **11+ non-lock files**:
1. `package.json` (modified by npm)
2. `package-lock.json` (modified by npm)
3. `src-tauri/tauri.conf.json`
4. `crates/core/Cargo.toml`
5. `crates/cli/Cargo.toml`
6. `crates/mcp/Cargo.toml`
7. `src-tauri/Cargo.toml`
8. `src/constants/index.ts`
9. `crates/core/locales/en.yml`
10. `crates/core/locales/ja.yml`
11. `src-tauri/locales/en.yml`
12. `src-tauri/locales/ja.yml`
13. `design/mainui/index.html`

### 2. Lockfile Omission (`Cargo.lock` Left Uncommitted)
Historically, when `npm version` ran:
- `Cargo.lock` was modified by a background Cargo invocation or post-bump check, but was NOT reliably staged before `npm` created the release commit and Git tag `vX.Y.Z`.
- Consequently, release commits contained version updates in TOML/JSON files, while `Cargo.lock` remained dirty in `git status`, omitting the lockfile change from the tagged release commit.

### 3. Display Copyright Proliferation
The user-facing copyright string `"Copyright © 2026 soakaye"` is statically duplicated across 4 localization files (`crates/core/locales/{en,ja}.yml` and `src-tauri/locales/{en,ja}.yml`). Because copyright notice format is universal rather than language-dependent, static duplication introduces manual yearly maintenance with no localization benefit.

---

## Architectural Decisions

### Decision 1: Cargo Workspace Package Inheritance
- **Decision**: Define canonical version, edition, authors, and license in root `Cargo.toml` under `[workspace.package]`:
  ```toml
  [workspace.package]
  version = "1.0.3"
  edition = "2021"
  authors = ["soakaye"]
  license = "MIT"
  ```
  In all 4 member crates (`crates/core`, `crates/cli`, `crates/mcp`, `src-tauri`), replace individual version declarations with:
  ```toml
  [package]
  version.workspace = true
  edition.workspace = true
  authors.workspace = true
  ```
- **Rationale**: Standard Rust/Cargo feature since 1.64. Reduces 4 independent `Cargo.toml` files into a single root `Cargo.toml` definition.
- **Alternatives Considered**: Retaining individual `Cargo.toml` version fields and updating all 4 with regex (rejected: redundant file changes, high merge conflict risk).

---

### Decision 2: Tauri v2 Package.json Version Reference
- **Decision**: Update `src-tauri/tauri.conf.json` to reference the root `package.json`:
  ```json
  {
    "version": "../package.json"
  }
  ```
- **Rationale**: Tauri v1 and v2 natively support referencing `../package.json` for application versioning. The build pipeline automatically extracts the version from `package.json` at compile time, eliminating `tauri.conf.json` from the version bump file set completely.
- **Alternatives Considered**: Regex synchronization script for `tauri.conf.json` (rejected: unnecessary file churn).

---

### Decision 3: Frontend Version Injection via Vite / package.json
- **Decision**: In `vite.config.ts`, expose `__APP_VERSION__` via Vite's `define` property by reading `package.json.version` (or `process.env.npm_package_version`). In `src/constants/index.ts`, define:
  ```typescript
  // Constant reference: __APP_VERSION__ injected by Vite build configuration
  export const APP_CONSTANTS = {
    ROOT_ELEMENT_ID: "root",
    APP_VERSION: typeof __APP_VERSION__ !== "undefined" ? __APP_VERSION__ : "1.0.0",
    APP_AUTHOR: "soakaye",
  } as const;
  ```
- **Rationale**: Complies with Constitution Principle II (centralized constants definition in `src/constants/index.ts`), while making `src/constants/index.ts` static across version bumps. No file modifications needed when version changes.
- **Alternatives Considered**: Hardcoded string replacement in `src/constants/index.ts` (rejected: requires file edits on every release).

---

### Decision 4: Elimination of Static Version from Locale Files
- **Decision**: Completely delete `about.APP_VERSION: ...` from:
  - `crates/core/locales/en.yml`
  - `crates/core/locales/ja.yml`
  - `src-tauri/locales/en.yml`
  - `src-tauri/locales/ja.yml`
  In `AboutDialog.tsx`, replace `{t("about.APP_VERSION")}` with `{APP_CONSTANTS.APP_VERSION}`.
- **Rationale**: Version numbers are language-neutral numbers. Eliminates 4 redundant file updates per release.
- **Alternatives Considered**: Retaining YAML entries (rejected: useless redundancy).

---

### Decision 5: Dynamic Display Copyright Generation
- **Decision**: Completely delete `about.COPYRIGHT: ...` from all 4 locale YAML files. Retain `about.COPYRIGHT_LABEL` ("Copyright:" in English, "著作権:" in Japanese). In `AboutDialog.tsx`, dynamically render:
  ```tsx
  Copyright © {new Date().getFullYear()} {APP_CONSTANTS.APP_AUTHOR}
  ```
  Per user constraint, file headers across all existing source files (`.rs`, `.ts`, `.tsx`, `.js`) remain untouched.
- **Rationale**: Completely eliminates copyright text duplication across languages. Year rollover is automatic, and author name is centralized in constants. Preserves source headers as mandated.
- **Alternatives Considered**: Static single string in locale catalogs (rejected: still requires annual updates).

---

### Decision 6: Atomic Cargo.lock Synchronization and Staging Guarantee
- **Decision**: Redesign `scripts/sync-version.js` and `scripts/sync-version-utils.js` to execute during `npm version` as follows:
  1. Update root `Cargo.toml` `[workspace.package].version` with target version.
  2. Run `cargo check --workspace` to update `Cargo.lock`.
  3. Verify that `Cargo.lock` has been updated to the target version.
  4. Explicitly stage both `Cargo.toml` and `Cargo.lock`:
     `git add Cargo.toml Cargo.lock`
  5. Validate via `git diff --cached --name-only` that both `Cargo.toml` and `Cargo.lock` are successfully staged in Git's index, and verify `git diff Cargo.lock` reports no unstaged changes.
  6. If any step fails, log diagnostics and exit with code 1. This causes `npm version` to abort before creating the Git commit and release tag.
- **Rationale**: Guarantees that `Cargo.lock` is included in the release commit and tag every single time. Prevents orphaned lockfile diffs.
- **Alternatives Considered**: Relying on post-version commits (rejected: leaves release tag pointing to commit with missing lockfile).

---

### Decision 7: Decoupling of Static Mockup HTML
- **Decision**: Remove `design/mainui/index.html` from `sync-version` target files.
- **Rationale**: `design/mainui/` is an exploratory standalone UI prototype, not runtime production code. Production builds do not package it. Decoupling it avoids noisy diffs during semantic version releases.
- **Alternatives Considered**: Continuing to regex-replace HTML prototype strings (rejected: unnecessary churn).

---

## Summary of File Changes During `npm version`

| File | Before | After |
| :--- | :--- | :--- |
| `package.json` | Modified by npm | Modified by npm |
| `package-lock.json` | Modified by npm | Modified by npm |
| `Cargo.toml` (root) | Not updated | **Updated** (SSOT for Rust workspace) |
| `Cargo.lock` | Omitted / Left dirty | **Updated & Staged** (Guaranteed in version tag) |
| `crates/core/Cargo.toml` | Modified by script | **Unchanged** (inherits workspace version) |
| `crates/cli/Cargo.toml` | Modified by script | **Unchanged** (inherits workspace version) |
| `crates/mcp/Cargo.toml` | Modified by script | **Unchanged** (inherits workspace version) |
| `src-tauri/Cargo.toml` | Modified by script | **Unchanged** (inherits workspace version) |
| `src-tauri/tauri.conf.json` | Modified by script | **Unchanged** (references `../package.json`) |
| `src/constants/index.ts` | Modified by script | **Unchanged** (dynamic `__APP_VERSION__`) |
| `crates/core/locales/en.yml` | Modified by script | **Unchanged** (version/copyright keys removed) |
| `crates/core/locales/ja.yml` | Modified by script | **Unchanged** (version/copyright keys removed) |
| `src-tauri/locales/en.yml` | Modified by script | **Unchanged** (version/copyright keys removed) |
| `src-tauri/locales/ja.yml` | Modified by script | **Unchanged** (version/copyright keys removed) |
| `design/mainui/index.html` | Modified by script | **Unchanged** (decoupled from release lifecycle) |
| **Total Non-Lock Files** | **11 files** | **2 files** (`package.json`, root `Cargo.toml`) |
