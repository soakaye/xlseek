# Research: Clean Changelog Headers and English Source Comments

**Feature**: `017-clean-changelog-english-comments`
**Date**: 2026-09-30

## Research Questions & Findings

### 1. Header Docstring Standards & Changelog Removal

- **Context**: Constitution v3.0.0 Principle III requires removing in-file changelogs (`変更履歴`, `Revision History`) from file, struct, and function comments while preserving the 3 mandatory elements:
  1. Detailed description of the purpose / behavior
  2. Arguments and return types with explanations
  3. Possible errors, `Result`/`Option` handling, and panic/exception conditions
- **Decision**: Remove all `## 変更履歴` and `## Revision History` blocks, bullet points, version numbers, dates, and author logs from Rust doc comments (`//!`, `///`), TypeScript JSDoc (`/** ... */`), and HTML prototype headers (`<!-- ... -->`). Retain clear sections for `## Purpose / Description`, `## Arguments / Returns` (or `@param` / `@returns`), and `## Errors / Panics` (or `@throws`).
- **Rationale**: Git commit logs and PR history provide an immutable, automated audit trail. In-file revision histories create high maintenance friction and frequently go stale or cause merge conflicts.
- **Alternatives Considered**:
  - *Retaining a single line "Last Updated"*: Rejected because it still violates Constitution v3.0.0 Principle III ("MUST NOT be recorded in code header comments; defer to version control").

### 2. English Comment Standardization

- **Context**: Constitution v3.0.0 Principle I requires that unspecified user responses and generated documents/code comments be written in natural, accurate English, while preserving Japanese localization catalog entries in `crates/core/locales/ja.yml`.
- **Decision**:
  - Translate all docstrings, inline comments (`//`), block comments (`/* ... */`), and TODO notes in `crates/`, `src-tauri/`, `src/`, and `design/mainui/index.html` into idiomatic, concise technical English.
  - Standardize constant reference annotations to: `// Constant reference: <CONSTANT_NAME>`.
  - Preserve `crates/core/locales/ja.yml` content entirely (since it contains application runtime localization strings, not code comments).
  - Update `AGENTS.md` to reflect Constitution v3.0.0 (3-element header comments, English default).
- **Rationale**: Eliminates language inconsistency across files, aligns with global open-source conventions, and ensures AI coding agents and automated linters parse docstrings without character encoding or natural language ambiguity.
- **Alternatives Considered**:
  - *Bilingual comments*: Rejected as overly verbose and difficult to synchronize.

### 3. File Inventory and Scope of Changes

- **Rust Back-end & CLI**:
  - `crates/core/src/`: `lib.rs`, `constants.rs`, `i18n.rs`, `models/*.rs`, `search/*.rs`, `export/*.rs`
  - `crates/cli/src/`: `main.rs`, `lib.rs`, `constants.rs`, `args.rs`, `output.rs`
  - `src-tauri/src/`: `main.rs`, `lib.rs`, `constants.rs`, `commands/*.rs`
  - `crates/**/tests/*.rs`, `src-tauri/**/tests/*.rs`
- **TypeScript / React Front-end**:
  - `src/`: `App.tsx`, `main.tsx`, `constants/index.ts`, `types/*.ts`, `hooks/*.ts`, `components/**/*.tsx`
- **UI Prototype & Guides**:
  - `design/mainui/index.html` (header comments and inline script comments translated, mock data preserved)
  - `AGENTS.md` (guideline alignment with Constitution v3.0.0)
- **Exclusions**:
  - `crates/core/locales/ja.yml` (runtime Japanese dictionary)
  - `src/constants/licenses.json` (third-party legal notices)

### 4. Quality Gate Verification

- **Command Pipeline**:
  - `npm run build`: verifies TypeScript compilation and bundle packaging.
  - `cargo test --workspace`: verifies unit and integration tests across all crates.
  - `cargo clippy --workspace --all-targets -- -D warnings`: verifies zero warnings.
  - `cargo fmt --check`: verifies strict Rust formatting compliance.
- **Validation**: Run all four gates post-refactor to ensure zero regression.
