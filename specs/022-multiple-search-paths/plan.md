# Implementation Plan: Multiple Search Target Paths with Quoted Delimitation

**Branch**: `022-multiple-search-paths` | **Date**: 2026-10-02 | **Spec**: [specs/022-multiple-search-paths/spec.md](file:///Users/soakaye/Develop/Projects/exgrep/specs/022-multiple-search-paths/spec.md)

**Input**: Feature specification from `specs/022-multiple-search-paths/spec.md`

## Summary

Enable users to specify multiple search target paths separated by commas, with support for quoted strings when paths contain whitespace or internal commas. This extends `crates/core` with a robust path tokenizer and multi-root discovery dispatch, adapts `crates/cli` to parse multi-path argument strings, and updates `src/` (GUI) to support append-on-select/drop, active token autocomplete, and non-fatal path warning notifications.

## Technical Context

**Language/Version**: Rust 2021 edition (backend, core, cli), TypeScript / React 18 (frontend), Tauri v2  
**Primary Dependencies**: `calamine`, `rayon`, `rust_xlsxwriter`, `regex`, Tauri v2 API, `@tauri-apps/plugin-dialog`  
**Storage**: Client-side localStorage for search history (storing full multi-path search strings)  
**Testing**: `cargo test --workspace` (unit and integration tests), `npm run build` (TypeScript type checking)  
**Target Platform**: Desktop (macOS, Windows, Linux)  
**Project Type**: Desktop GUI application (`xlseek`) and standalone CLI (`xlseek-cli`)  
**Performance Goals**: Path tokenization completes in < 1ms; multi-root discovery runs without thread overhead using existing Rayon/Burst pools.  
**Constraints**: Zero hardcoded constants (Principle II), comprehensive header comments (Principle III), zero clippy warnings (Principle IV).  
**Scale/Scope**: Up to dozens of search paths in a single query; thousands of Excel workbooks discovered across all paths.  

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] **Principle I: Language-Directed Quality**: All comments, docs, commit messages, and logs are in natural, accurate English. Error codes are translated using source YAML catalogs (`locales/ja.yml`, `locales/en.yml`).
- [x] **Principle II: No Hardcoded Constants**: All delimiters (`,`), quote characters (`"`), error codes, and default flags are defined in constants modules (`crates/core/src/constants.rs`, `src-tauri/src/constants.rs`, `src/constants/index.ts`).
- [x] **Principle III: Comprehensive Header Comments**: Every new function, struct, and method includes a header comment detailing description, arguments/return values, and errors/panics.
- [x] **Principle IV: Modular Design & Code Standards**: Logic is separated cleanly between core tokenization (`crates/core/src/search/path.rs`), search coordination (`crates/core/src/search/engine.rs`), CLI (`crates/cli/src/args.rs`), and GUI (`src/components/search/SearchBar.tsx`, `src/utils/pathTokenizer.ts`).
- [x] **Principle V: Robust Error Handling & Testing**: Safe handling of empty, malformed, or unclosed quote strings. Non-fatal warnings for partially invalid paths. Comprehensive unit and integration test coverage.

## Project Structure

### Documentation (this feature)

```text
specs/022-multiple-search-paths/
├── spec.md              # Feature specification
├── plan.md              # This file (/speckit-plan output)
├── research.md          # Phase 0 research output
├── data-model.md        # Phase 1 data models & state flow
├── quickstart.md        # Phase 1 validation scenarios
├── contracts/           # Phase 1 interface contracts
│   └── path-search-contract.md
└── checklists/
    └── requirements.md  # Spec quality validation checklist
```

### Source Code (repository root)

```text
crates/core/
├── src/
│   ├── constants.rs     # Tokenizer delimiters, quotes, error messages
│   ├── search/
│   │   ├── path.rs      # parse_search_paths, resolve_and_partition_paths
│   │   ├── engine.rs    # Multi-root resolution and discovery dispatch
│   │   └── discovery.rs # Multi-root directory scanning
│   └── tests/           # Unit tests for multi-path tokenization and resolution

crates/cli/
├── src/
│   ├── args.rs          # Expand comma-delimited strings in --path / -p
│   └── constants.rs     # CLI multi-path constants
└── tests/               # Integration tests for multi-path CLI search

src-tauri/
├── src/
│   ├── constants.rs     # GUI multi-path error codes & event names
│   └── commands/
│       └── search_cmd.rs # Validate multi-path target_dir, emit invalid path issues

src/
├── constants/
│   └── index.ts         # Path delimiter and quote constants
├── utils/
│   └── pathTokenizer.ts # Frontend path append, split, and active segment helpers
├── components/search/
│   └── SearchBar.tsx    # Append selected/dropped folders, segment autocomplete
└── hooks/
    └── useSearch.ts     # Multi-path issue handling and warning toast display
```

**Structure Decision**: Fully adheres to the existing multi-crate workspace architecture (`crates/core`, `crates/cli`, `src-tauri`, and `src/`).

## Complexity Tracking

*No constitution violations or unjustified complexities.*
