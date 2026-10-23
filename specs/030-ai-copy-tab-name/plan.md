# Implementation Plan: Explicit Sheet Tab Name in AI Copy Target Cell Metadata

**Branch**: `030-ai-copy-tab-name` | **Date**: 2026-10-23 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/030-ai-copy-tab-name/spec.md`

## Summary

When copying search matches via "AI Copy" (LLM context copy), the `Target Cell` metadata line currently formats as `- Target Cell: A16 (Column: ColHeader, Row: RowKey)`, which omits the worksheet tab name. This causes ambiguity when prompts are pasted into AI assistants or note-taking tools.

This feature modifies `xlseek-core` to format the `Target Cell` line with an explicit bracketed worksheet name prefix:
`- Target Cell: [SheetName] CellAddress (Column: ColHeader, Row: RowKey)`
If the sheet name is empty or whitespace-only, it falls back gracefully to `- Target Cell: CellAddress (Column: ColHeader, Row: RowKey)`.
This formatting applies uniformly across both Table context mode and Header-only context mode, while keeping Word document formatting untouched.

All format strings are extracted to `crates/core/src/constants.rs` per Constitution Principle II, a unified helper function `format_target_cell_line` is introduced per Principle IV with comprehensive 3-part header comments per Principle III, and unit tests are added per Principle V.

## Technical Context

**Language/Version**: Rust 2021 (Rust 1.77+), TypeScript 5.x / React 18  
**Primary Dependencies**: `calamine`, `regex`, `tauri` v2  
**Storage**: N/A  
**Testing**: `cargo test --package xlseek-core`, `npm run build`, `cargo test --workspace`  
**Target Platform**: Windows / macOS / Linux desktop  
**Project Type**: Desktop application & shared core library (`xlseek-core`)  
**Performance Goals**: <1ms formatting latency overhead per search result batch  
**Constraints**: Fully conform to Constitution Principles I - V (English artifacts, no hardcoded constants, 3-part header comments, Clippy `-D warnings`, `cargo fmt`)  
**Scale/Scope**: `crates/core/src/constants.rs`, `crates/core/src/search/llm_context.rs`, and unit tests  

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Check Item | Status | Verification Note |
|---|---|---|---|
| **Principle I: Language-Directed Quality** | All plan, research, data-model, contracts, and quickstart artifacts written in natural, accurate English. Source code comments in English. User communication in Japanese. No unwanted special tokens. | **PASS** | Artifacts verified in English; user responses in Japanese. |
| **Principle II: No Hardcoded Constants** | No magic strings or format templates hardcoded outside `0` and `""`. Format strings extracted to `crates/core/src/constants.rs`. Code call sites annotate `// Constant reference: ...`. | **PASS** | Constants `LLM_TARGET_CELL_WITH_SHEET_FMT` and `LLM_TARGET_CELL_WITHOUT_SHEET_FMT` defined in `constants.rs`. |
| **Principle III: Comprehensive Header Comments** | All new or modified functions have a 3-part header comment (Description, Arguments/Returns, Errors/Exceptions). No per-file change histories. | **PASS** | `format_target_cell_line` and test functions will be fully commented with required 3-part format. |
| **Principle IV: Modular Design & Code Standards** | Modular Single Responsibility Principle (SRP) helper function. Full compliance with `cargo clippy --workspace --all-targets -- -D warnings` and `cargo fmt --check`. | **PASS** | Helper function isolates formatting logic; clippy and rustfmt checks required before completion. |
| **Principle V: Robust Error Handling & Testing** | No unhandled panics. Safe fallback on missing sheet names. Unit tests for standard formatting, fallback formatting, special characters, table mode, and header-only mode. | **PASS** | Automated test suite in `llm_context.rs` covers all paths. |

## Project Structure

### Documentation (this feature)

```text
specs/030-ai-copy-tab-name/
├── spec.md              # Feature specification
├── checklists/
│   └── requirements.md  # Spec quality checklist (16/16 pass)
├── plan.md              # This file (/speckit-plan output)
├── research.md          # Architecture research & decisions (Phase 0 output)
├── data-model.md        # Entities & Markdown schema (Phase 1 output)
├── quickstart.md        # Validation guide & quality gates (Phase 1 output)
└── contracts/
    └── llm-context-contracts.md # Interface contract for LLM context generation
```

### Source Code (repository root)

```text
crates/core/
├── src/
│   ├── constants.rs           # Centralized constants: LLM_TARGET_CELL_WITH_SHEET_FMT, etc.
│   └── search/
│       └── llm_context.rs     # Target cell formatting helper & context builders
└── tests/                     # Integration tests
```

**Structure Decision**: This feature is entirely contained within `crates/core` (`constants.rs` and `llm_context.rs`). It does not modify frontend UI components or Tauri IPC command signatures.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

*No violations. All principles fully satisfied.*
