# Implementation Plan: Search Results Export Format Options and UI Reorganization

**Branch**: `031-export-format-options` | **Date**: 2026-10-24 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/031-export-format-options/spec.md`

## Summary

Reorganize the search results export functionality across `xlseek-core`, `xlseek` (Tauri v2 GUI), and `xlseek-cli`:
1. **Core Export Engine (`xlseek-core`)**: Extend `ExportFormat` (`Csv`, `Xlsx`, `Md`, `Json`) and `ExportRequest` (adding optional `llm_options: Option<LlmCopyOptions>`). Add `md_export.rs` (reusing `extract_llm_context` to output AI-ready Markdown files) and `json_export.rs` (serializing `JsonExportPayload` with `total_matches` and normalized `matches` array in 2-space pretty-printed UTF-8 JSON).
2. **GUI Status Bar Split Button (`src/`)**: Replace the separate CSV and Excel buttons in `StatusBar.tsx` with a unified Split Button (`[Export (Active Format) | ▾]`) that remembers the user's selected format in `localStorage` (initial default: `.xlsx`), immediately opens the OS file save dialog when either the main button or a dropdown format item is clicked, and syncs the standalone HTML prototype (`design/mainui/index.html`).
3. **Standalone CLI (`xlseek-cli`)**: Support `-f, --format <csv|xlsx|md|json>`, automatic format inference from `.md` and `.json` file extensions (`-o`), and `stdout` streaming for `md` and `json`.

## Technical Context

**Language/Version**: Rust 2021 Edition (1.75+), TypeScript 5.x / React 18

**Primary Dependencies**: Tauri v2 (`@tauri-apps/api`, `tauri-plugin-dialog`, `tauri-plugin-i18n`), `calamine`, `rust_xlsxwriter`, `csv`, `serde`, `serde_json`, Tailwind CSS, Lucide React

**Storage**: Local filesystem (exported `.csv`, `.xlsx`, `.md`, `.json` files) and browser `localStorage` (`xlseek.exportFormat` for persisted split button format selection)

**Testing**: `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`, `npm run build`, `npm test -- --run` (Vitest)

**Target Platform**: Cross-platform Desktop (Windows, macOS, Linux via Tauri v2) & CLI (`xlseek-cli`)

**Project Type**: Desktop GUI Application + Standalone CLI + Shared Core Library (Cargo Workspace)

**Performance Goals**: Asynchronous background export execution (`spawn_blocking` in Tauri) so exporting up to 1,000 matches does not block the UI thread; atomic temporary file staging in CLI.

**Constraints**: Strictly offline/local execution; zero hardcoded constants (Principle II); comprehensive 3-part header comments (Principle III); synthetic dummy data only in all tests and docs (Principle III).

**Scale/Scope**: 4 export formats (`csv`, `xlsx`, `md`, `json`) across 3 workspace crates (`crates/core`, `crates/cli`, `src-tauri`) and frontend (`src/`, `design/mainui/index.html`).

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Gate Requirement | Pre-Design Status | Post-Design Status |
| :--- | :--- | :--- | :--- |
| **I. Language-Directed Quality** | English source comments/docs; bilingual catalog entries (`ja.yml`, `en.yml`); zero garbled tokens. | PASS | PASS |
| **II. No Hardcoded Constants** | All format strings, translation keys, storage keys, and error messages extracted to `constants.rs` / `src/constants/index.ts` with `// Constant reference:` comments. | PASS | PASS |
| **III. Comprehensive Header Comments & Documentation Hygiene** | Every file, struct, enum, function, and method includes Description, Arguments/Returns, and Errors/Panics; zero real data in artifacts (`sample.xlsx` synthetic data only). | PASS | PASS |
| **IV. Modular Design & Code Standards** | Dedicated `md_export.rs` and `json_export.rs` modules in `crates/core/src/export/`; zero `clippy`, `rustfmt`, or `tsc` warnings/errors. | PASS | PASS |
| **V. Robust Error Handling & Testing** | All export functions return `Result<..., String>` or `Result<(), CommandError>` with zero unhandled panics; unit & integration tests added across core, CLI, and frontend. | PASS | PASS |

## Project Structure

### Documentation (this feature)

```text
specs/031-export-format-options/
├── plan.md              # This file (/speckit-plan command output)
├── research.md          # Phase 0 output (/speckit-plan command)
├── data-model.md        # Phase 1 output (/speckit-plan command)
├── quickstart.md        # Phase 1 output (/speckit-plan command)
├── contracts/           # Phase 1 output (/speckit-plan command)
│   ├── export-ipc.md    # Tauri IPC export_results contract
│   └── cli-export.md    # xlseek-cli format and stdout/file contract
└── tasks.md             # Phase 2 output (/speckit-tasks command)
```

### Source Code (repository root)

```text
crates/
├── core/
│   ├── locales/
│   │   ├── en.yml                       # English UI & CLI translation keys for MD/JSON export and split button
│   │   └── ja.yml                       # Japanese UI & CLI translation keys for MD/JSON export and split button
│   └── src/
│       ├── constants.rs                 # Format constants, error messages, and translation key constants
│       ├── models/
│       │   └── mod.rs                   # ExportFormat (Csv, Xlsx, Md, Json) and ExportRequest (with llm_options)
│       └── export/
│           ├── mod.rs                   # Re-exports csv_export, xlsx_export, md_export, json_export
│           ├── csv_export.rs            # Existing CSV export
│           ├── xlsx_export.rs           # Existing XLSX export
│           ├── md_export.rs             # NEW: AI Markdown export reusing extract_llm_context
│           └── json_export.rs           # NEW: Structured JSON export (JsonExportPayload)
├── cli/
│   ├── src/
│   │   ├── constants.rs                 # Re-exports format constants
│   │   ├── args.rs                      # Support -f <csv|xlsx|md|json> and .md/.json extension inference
│   │   ├── output.rs                    # OutputGuard::publish support for Md and Json
│   │   └── lib.rs                       # Stdout streaming support for Csv, Md, and Json
│   └── tests/
│       └── cli_search.rs                # Integration tests for CLI MD and JSON export (file & stdout)
src-tauri/
└── src/
    ├── constants.rs                     # Re-exports core constants
    └── commands/
        └── export_cmd.rs                # Dispatch ExportFormat::Md and ExportFormat::Json in export_results
src/
├── constants/
│   └── index.ts                         # EXPORT_FORMATS, EXPORT_FORMAT_STORAGE_KEY, DEFAULT_EXPORT_FORMAT
├── types/
│   └── search.ts                        # ExportFormat = "csv" | "xlsx" | "md" | "json", ExportRequest.llm_options
├── components/
│   └── common/
│       └── StatusBar.tsx                # Export Split Button, format menu, localStorage persistence
└── App.tsx                              # Pass defaultOptions-derived LlmCopyOptions to StatusBar
design/
└── mainui/
    └── index.html                       # Synchronized standalone UI prototype with Export Split Button
```

**Structure Decision**: Extends the existing Cargo workspace (`crates/core`, `crates/cli`, `src-tauri`) and React frontend (`src/`) following the established modular architecture in `AGENTS.md`.

## Complexity Tracking

> No Constitution Check violations exist.
