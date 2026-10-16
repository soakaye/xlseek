# Implementation Plan: Search Path Normalization and Subpath Deduplication

**Branch**: `026-dedup-search-paths` | **Date**: 2026-10-16 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/026-dedup-search-paths/spec.md`

## Summary

Normalize all user-specified search target paths into clean, canonical absolute paths and deduplicate any path that is an exact duplicate or a descendant subpath of another specified directory before starting directory traversal. This prevents redundant directory scans and duplicate search results across both the GUI (`xlseek-core` / `src-tauri`) and CLI (`xlseek-cli`) workflows, while preserving non-fatal warning diagnostics for invalid paths and human-readable path formatting (without Windows `\\?\` verbatim prefixes).

## Technical Context

**Language/Version**: Rust 2021 edition, TypeScript 5.x (React 18)

**Primary Dependencies**: `std::fs`, `std::path`, `walkdir`, `rayon`, `same-file`, Tauri v2 (`xlseek-core`, `xlseek-cli`, `xlseek`)

**Storage**: Local filesystem (read-only directory traversal and path canonicalization)

**Testing**: `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`, `npm run build`

**Target Platform**: Windows, macOS, Linux (Desktop GUI & CLI)

**Project Type**: Desktop application (Tauri v2) + standalone CLI (`xlseek-cli`) sharing core library (`xlseek-core`)

**Performance Goals**: Path normalization and subpath pruning complete in < 50ms for up to 50 target paths; zero redundant directory walks or duplicate file evaluations when overlapping ancestor/descendant paths are provided.

**Constraints**: Offline-only local execution; strict adherence to project constitution (no hardcoded constants outside `constants.rs`, comprehensive 3-part header comments, zero Clippy/TypeScript warnings).

**Scale/Scope**: Core path resolution module (`crates/core/src/search/path.rs`), search engine root resolution (`crates/core/src/search/engine.rs`), CLI argument path resolution (`crates/cli/src/args.rs`), and unit/integration test suites.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Principle I (Language-Directed Quality)**: PASS — Code comments, logs, and error constants in Rust/TS use accurate English; user responses and Japanese artifacts follow user instructions; no garbled tokens.
- **Principle II (No Hardcoded Constants)**: PASS — Any path prefix or environment variable constants (`HOME_PATH_SHORTHAND_EXACT`, `HOME_ENVIRONMENT_VARIABLE`, `WINDOWS_HOME_ENVIRONMENT_VARIABLE`, `VERBATIM_PATH_PREFIX_WINDOWS`, etc.) are defined in `crates/core/src/constants.rs` and referenced with explicit `// Constant reference:` comments.
- **Principle III (Comprehensive Header Comments)**: PASS — Every new or modified function in `path.rs`, `engine.rs`, and `args.rs` includes Description, Arguments/Returns, and Errors/Panics sections without per-file change histories.
- **Principle IV (Modular Design & Code Standards)**: PASS — Shared path normalization and subpath deduplication live in `crates/core/src/search/path.rs` and are reused by both `engine.rs` and `cli/src/args.rs`; compliant with `clippy` and `rustfmt`.
- **Principle V (Robust Error Handling & Testing)**: PASS — Uses `Result` and non-panicking fallbacks; comprehensive unit and integration tests cover parent/child ordering, relative segments, `~` expansion, sibling prefix collision, invalid subpaths, and CLI positional/comma-separated paths.

## Project Structure

### Documentation (this feature)

```text
specs/026-dedup-search-paths/
├── plan.md              # This file (/speckit-plan command output)
├── research.md          # Phase 0 output (/speckit-plan command)
├── data-model.md        # Phase 1 output (/speckit-plan command)
├── quickstart.md        # Phase 1 output (/speckit-plan command)
├── contracts/           # Phase 1 output (/speckit-plan command)
│   └── path-normalization.md
└── tasks.md             # Phase 2 output (/speckit-tasks command - NOT created by /speckit-plan)
```

### Source Code (repository root)

```text
crates/
├── core/
│   └── src/
│       ├── constants.rs        # Path shorthand & normalization constants
│       └── search/
│           ├── path.rs         # Canonical path normalization, OS home fallback, and subpath pruning
│           └── engine.rs       # SearchEngine multi-path resolution and deduplication integration tests
└── cli/
    ├── src/
    │   └── args.rs             # Unified CLI multi-path & positional path normalization and subpath pruning
    └── tests/
        └── cli_search.rs       # CLI integration tests for overlapping parent/child and directory/file paths
```

**Structure Decision**: Centralize path normalization (`normalize_canonical_path`) and subpath deduplication (`deduplicate_and_prune_subpaths`) in `crates/core/src/search/path.rs` so both `crates/core/src/search/engine.rs` (used by Tauri GUI) and `crates/cli/src/args.rs` (standalone CLI) share the exact same implementation.

## Complexity Tracking

> No Constitution Check violations.
