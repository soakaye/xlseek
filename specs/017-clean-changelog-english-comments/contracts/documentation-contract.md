# Interface & Documentation Contracts

**Feature**: `017-clean-changelog-english-comments`
**Date**: 2026-09-30

This document defines the interface standards and verification contracts for header comments, docstrings, and in-code annotations.

## 1. Rust Source Documentation Contract

- **Target Files**: All `.rs` files under `crates/core/src/`, `crates/cli/src/`, `src-tauri/src/`, and integration tests.
- **Language**: English only (ASCII / standard Unicode technical terms).
- **Prohibited Patterns**:
  - `## 変更履歴`
  - `## Revision History`
  - `* v[0-9]+\.[0-9]+\.[0-9]+` (in file headers)
  - Japanese character ranges `[\u3000-\u303f\u3040-\u309f\u30a0-\u30ff\uff00-\uffef\u4e00-\u9faf]` within comment markers (`//`, `///`, `//!`, `/*`, `*`).
- **Required Header Sections**:
  - Purpose & overview
  - Arguments / Return value specification
  - Errors / Panics / Result handling

## 2. TypeScript / React Documentation Contract

- **Target Files**: All `.ts` and `.tsx` files under `src/`.
- **Language**: English only.
- **Prohibited Patterns**:
  - `変更履歴` or file-level revision history blocks in JSDoc headers.
  - Japanese text in code comments (`//`, `/* */`, `/** */`).
- **Required Header Sections**:
  - Component / Hook / Function purpose
  - Props / Parameter definitions
  - Return / Exceptions (if any)

## 3. UI Prototype Documentation Contract

- **Target Files**: `design/mainui/index.html`
- **Language**: English comments (`<!-- ... -->`, `//`, `/* ... */`).
- **Prohibited Patterns**:
  - `## 変更履歴` or version history blocks in HTML comments.
  - Japanese comments in embedded `<script>` or `<style>` blocks.
- **Exemptions**:
  - Mock UI data strings representing Japanese Excel spreadsheet cells (e.g., sample table rows) are preserved.

## 4. Constant Reference Annotation Contract

- When referencing constants from `constants.rs` or `constants/index.ts`, use the English annotation format:
  ```rust
  // Constant reference: constants::APP_NAME
  ```
  ```typescript
  // Constant reference: APP_TITLE
  ```
