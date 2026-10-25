# Implementation Plan: Handle Multiline Text in Markdown and CSV Search Results Export

**Branch**: `035-handle-markdown-newlines` | **Date**: 2026-10-25 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/035-handle-markdown-newlines/spec.md`

---

## Summary

Handle newline characters (`\r\n`, `\n`, `\r`) in search result cell strings and text values across Markdown and CSV export pipelines in `xlseek-core`:
1. **Markdown Export**: Replace newlines with standard `<br>` tags, trim leading/trailing whitespace, preserve internal consecutive newlines (`<br><br>`), and escape pipe characters, maintaining valid GFM table rows and list items.
2. **CSV Export**: Trim leading/trailing whitespace and replace internal newlines with a single space (`" "`), guaranteeing that every search result strictly occupies exactly one physical line in CSV outputs (`total_lines = 1 + match_count`).

---

## Technical Context

**Language/Version**: Rust (2021 edition), TypeScript 5.6  
**Primary Dependencies**: `xlseek-core`, `calamine`, `rust_xlsxwriter`, `csv`, `rayon`  
**Storage**: Local file system (read-only search; atomic staging for exports)  
**Testing**: `cargo test --workspace`, `npm test`  
**Target Platform**: macOS, Windows, Linux (Desktop & CLI)  
**Project Type**: Desktop application (Tauri v2) & Standalone CLI  
**Performance Goals**: < 5% runtime overhead for 1,000 matches during export  
**Constraints**: Zero hardcoded constants (Principle II), comprehensive header comments (Principle III), zero compiler/clippy warnings (Principle IV), synthetic dummy data only in documentation (Principle III).  
**Scale/Scope**: Core export modules in `crates/core` (`llm_context.rs`, `csv_export.rs`, `constants.rs`) and test coverage.  

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Check Item | Status | Notes |
|:---|:---|:---:|:---|
| **Principle I: Language-Directed Quality** | English default in codebase, comments, commits; Japanese for user interactions | **PASS** | Source code comments, docs, and tests in English; user replies in Japanese. |
| **Principle II: No Hardcoded Constants** | All constants defined in centralized `constants.rs` with usage comments | **PASS** | Constants (`MD_LINE_BREAK`, `SPACE_STR`, etc.) defined in `crates/core/src/constants.rs` with unit tests and `// Constant reference:` comments. |
| **Principle III: Header Comments & Hygiene** | 3-element header comments on all functions; zero real data in docs | **PASS** | Every function has Description, Arguments/Returns, and Errors. All documents use synthetic dummy data (`sample.xlsx`). |
| **Principle IV: Modular Design & Code Standards** | Comply with clippy, rustfmt, SRP | **PASS** | Logic isolated in respective export modules; zero clippy warnings. |
| **Principle V: Robust Error Handling & Testing** | No panics, comprehensive unit/integration tests | **PASS** | Pure string transformation functions that do not panic; unit and integration tests added. |

---

## Project Structure

### Documentation (this feature)

```text
specs/035-handle-markdown-newlines/
├── spec.md              # Feature specification (/speckit-specify output)
├── plan.md              # This file (/speckit-plan command output)
├── research.md          # Technical research & architecture decisions (Phase 0 output)
├── data-model.md        # Data models & transformations (Phase 1 output)
├── quickstart.md        # Verification & test execution guide (Phase 1 output)
├── contracts/           # Interface and artifact contracts (Phase 1 output)
│   └── markdown-csv-export.md
├── checklists/          # Quality checklist
│   └── requirements.md
└── tasks.md             # Implementation tasks (/speckit-tasks output)
```

### Source Code (repository root)

```text
crates/
└── core/
    ├── src/
    │   ├── constants.rs               # Define MD_LINE_BREAK and string constants
    │   ├── export/
    │   │   ├── csv_export.rs          # Add sanitize_csv_field and apply to full_content, formula
    │   │   └── md_export.rs           # Markdown export pipeline
    │   └── search/
    │       └── llm_context.rs         # Update escape_markdown_cell to convert newlines to <br>
    └── tests/
        ├── export_md_test.rs          # Test multiline markdown table & list export
        └── export_csv_test.rs         # Test multiline CSV single-line guarantee
```

**Structure Decision**:
Changes are strictly isolated to `crates/core` backend export and formatting modules. The CLI (`crates/cli`) and GUI desktop app (`src-tauri` / `src`) depend directly on `crates/core` export routines and automatically benefit from the improved sanitization without redundant frontend changes.

---

## Complexity Tracking

*No violations. All Constitution principles are strictly followed without exceptions.*
