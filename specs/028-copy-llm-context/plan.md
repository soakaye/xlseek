# Implementation Plan: Context-Aware Header Extraction and Prompt Copy for LLM

**Branch**: `028-copy-llm-context` | **Date**: 2026-10-19 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/028-copy-llm-context/spec.md`

---

## Summary

Implement an intelligent "Copy for LLM" capability that automatically extracts tabular column headers and row item keys for matched cells in Excel spreadsheets, formatting the target row into a clean, token-efficient Markdown table with bold hit cell values (`**value**`). For Word documents, it extracts the nearest preceding section heading and quotes the matched paragraph. The feature is accessible from both the preview toolbar and a new right-click context menu in the search results table, with configurable column window limits and extraction modes persisted across sessions. All operations execute strictly locally without external network transmission.

---

## Technical Context

**Language/Version**: Rust 2021 edition (1.75+), TypeScript 5.x / React 18

**Primary Dependencies**:
- Backend: `calamine` (0.36), `rust_xlsxwriter`, `serde` (1.0), `serde_json` (1.0), `tauri` v2
- Frontend: React 18, `@tanstack/react-virtual`, `lucide-react`, `tailwindcss`

**Storage**: Local filesystem (read-only Excel/Word inspection), LocalStorage (settings persistence)

**Testing**:
- Rust: `cargo test --package xlseek-core`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`
- Frontend: `npm run build` (`tsc --noEmit` and Vite bundle)

**Target Platform**: Windows, macOS, Linux (Desktop GUI)

**Project Type**: Desktop application (Tauri v2 + React 18 + Rust core engine)

**Performance Goals**:
- Extraction and clipboard copy latency < 200ms
- Right-click context menu presentation < 50ms
- Zero UI thread blocking during file reading and AST parsing

**Constraints**:
- Strict adherence to the project constitution:
  - Principle I: Documentation, comments, and internal error strings in natural English; UI localized via `ja.yml`/`en.yml`.
  - Principle II: No hardcoded magic strings or numbers outside centralized constants.
  - Principle III: Comprehensive 3-part header comments on all functions and modules.
  - Principle IV: Clean modular architecture conforming to `clippy` and `rustfmt`.
  - Principle V: Robust error handling, non-panicking fallbacks, and 100% test coverage for edge cases.
- Strict local privacy: 100% offline, zero network requests or third-party AI API keys required.

**Scale/Scope**:
- Backend core: `crates/core/src/models/llm.rs`, `crates/core/src/search/llm_context.rs`, `crates/core/src/constants.rs`, `crates/core/locales/`
- Tauri IPC: `src-tauri/src/commands/preview_cmd.rs` (or dedicated `llm_cmd.rs`), `src-tauri/src/constants.rs`
- Frontend: `src/types/llm.ts`, `src/types/defaultOptions.ts`, `src/constants/index.ts`, `src/services/llm.ts`, `src/components/results/ContextMenu.tsx`, `src/components/results/ResultTable.tsx`, `src/components/preview/PreviewHeader.tsx`, `src/components/settings/`

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Principle I (Language-Directed Quality)**: PASS — All documentation artifacts (`spec.md`, `plan.md`, `research.md`, `data-model.md`, `contracts/`, `quickstart.md`), source code comments, and error types are written in accurate English. UI localization strings for Japanese and English are maintained in `crates/core/locales/ja.yml` and `en.yml`.
- **Principle II (No Hardcoded Constants)**: PASS — Command names (`"generate_llm_context"`), default window sizes (`5`), bounds (`1`–`20`), ellipsis markers (`"..."`), and UI keys are centralized in `crates/core/src/constants.rs`, `src-tauri/src/constants.rs`, and `src/constants/index.ts` with explicit `// Constant reference:` comments.
- **Principle III (Comprehensive Header Comments)**: PASS — Every added/modified Rust function and TypeScript component will contain the required 3-part header comment (Description, Arguments/Returns, Errors/Exceptions) without per-file change histories.
- **Principle IV (Modular Design & Code Standards)**: PASS — Heuristic extraction logic is isolated in `crates/core/src/search/llm_context.rs`, IPC commands in `src-tauri/src/commands/`, and the context menu in a reusable `ContextMenu.tsx` component.
- **Principle V (Robust Error Handling & Testing)**: PASS — All file parsing returns `Result<T, E>`. Non-standard sheets gracefully fall back to default row/column coordinates without panicking. Unit tests in `llm_context.rs` cover edge cases (A1 hit, merged headers, banner offsets, multi-sheet grouping).

---

## Project Structure

### Documentation (this feature)

```text
specs/028-copy-llm-context/
├── spec.md                     # Feature specification with clarifications
├── plan.md                     # This file (/speckit-plan output)
├── research.md                 # Phase 0 research and architectural decisions
├── data-model.md               # Phase 1 data entities and flow diagrams
├── quickstart.md               # Phase 1 verification and run guide
├── contracts/                  # Phase 1 interface contracts
│   └── llm-context-contracts.md # Tauri IPC command & schema definitions
├── checklists/
│   └── requirements.md         # Spec quality checklist (100% pass)
└── tasks.md                    # Phase 2 output (/speckit-tasks output - downstream)
```

### Source Code (repository root)

```text
crates/core/
├── locales/
│   ├── ja.yml                  # Japanese localization keys for LLM copy & menu
│   └── en.yml                  # English localization keys for LLM copy & menu
└── src/
    ├── constants.rs            # Centralized constants (window size, command names)
    ├── models/
    │   ├── mod.rs
    │   └── llm.rs              # LlmCopyOptions, HeaderContext, PromptSnippet models
    └── search/
        ├── mod.rs
        └── llm_context.rs      # Header heuristics, row windowing, Markdown table builder

src-tauri/src/
├── constants.rs                # Tauri-specific command name constants
├── lib.rs                      # Tauri command registration
└── commands/
    ├── mod.rs
    └── preview_cmd.rs          # generate_llm_context command handler

src/
├── constants/
│   └── index.ts                # Frontend constants (command names, default values)
├── types/
│   ├── defaultOptions.ts       # Extension of DefaultSearchOptions for LLM copy
│   ├── llm.ts                  # TypeScript interfaces for LlmCopyOptions
│   └── index.ts
├── services/
│   └── llm.ts                  # generateLlmContext IPC client
└── components/
    ├── common/
    │   └── Toast.tsx           # Success notification feedback
    ├── preview/
    │   └── PreviewHeader.tsx   # "Copy for LLM" toolbar button
    ├── results/
    │   ├── ContextMenu.tsx     # Custom right-click context menu component
    │   └── ResultTable.tsx     # Context menu integration and row handlers
    └── settings/
        └── LlmSettingsTab.tsx  # Settings UI for column window size and defaults
```

---

## Complexity Tracking

> **No Constitution violations detected. All gates passed without exceptions.**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|---|---|---|
| None | N/A | N/A |
