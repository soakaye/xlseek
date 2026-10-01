# Implementation Plan: Unify Project and Package Naming to xlseek

**Branch**: `021-rename-to-xlseek` | **Date**: 2026-10-02 | **Spec**: [spec.md](file:///Users/soakaye/Develop/Projects/workspaces/exgrep/cuskeel/specs/021-rename-to-xlseek/spec.md)

**Input**: Feature specification from `/specs/021-rename-to-xlseek/spec.md`

## Summary

Migrate all occurrences of legacy naming `exgrep` and `exlgrep` to `xlseek` across Rust Cargo package manifests, crate import paths, constants, temporary directory prefixes, storage keys, build scripts, and project documentation. Preserve historical specifications (`specs/001-` through `specs/020-`) as immutable historical records.

## Technical Context

**Language/Version**: Rust 2021 edition, TypeScript 5.x / React 18  
**Primary Dependencies**: Cargo workspace (`xlseek-core`, `xlseek-cli`, `xlseek`), Tauri v2, Vite  
**Storage**: Web `localStorage` (migrating `exlgrep.*` keys to `xlseek.*` keys)  
**Testing**: `cargo test --workspace`, `npm run build`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`  
**Target Platform**: macOS, Windows, Linux  
**Project Type**: Multi-crate desktop application and CLI tool  
**Performance Goals**: N/A (compile-time & configuration naming refactoring)  
**Constraints**: Zero hardcoded strings outside constants (Constitution II), comprehensive header comments (Constitution III), no compiler/clippy warnings (Constitution IV)  
**Scale/Scope**: ~15 files modified in active codebase  

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Principle I: Language-Directed Quality**: All comments, documentation, and commits in natural, accurate English. Clean typography with no special tokens. **PASSED**.
- **Principle II: No Hardcoded Constants**: Temporary prefixes, storage keys, and test output file names defined in constants (`crates/core/src/constants.rs`, `src/constants/index.ts`). **PASSED**.
- **Principle III: Comprehensive Header Comments**: Every modified file and function preserves or updates description, arguments/return values, and error behavior without file-level change logs. **PASSED**.
- **Principle IV: Modular Design & Code Standards**: Cargo workspace properly separates `xlseek-core`, `xlseek-cli`, and `src-tauri` (`xlseek`). Strict `clippy` and `rustfmt` compliance. **PASSED**.
- **Principle V: Robust Error Handling & Testing**: All unit and integration tests updated and passing without regressions. **PASSED**.

## Project Structure

### Documentation (this feature)

```text
specs/021-rename-to-xlseek/
├── spec.md              # Feature specification
├── plan.md              # This file (/speckit-plan output)
├── research.md          # Phase 0 research and decisions
├── data-model.md        # Data models and key mappings
├── quickstart.md        # Validation scenarios
├── contracts/           # Interface contracts
│   └── naming-contract.md
├── checklists/
│   └── requirements.md
└── tasks.md             # Phase 2 tasks (/speckit-tasks output)
```

### Source Code (repository root)

```text
crates/
├── core/                # Package: xlseek-core (lib: xlseek_core)
│   ├── Cargo.toml
│   ├── src/             # constants.rs, lib.rs, search/, export/, models/, i18n.rs
│   └── tests/           # integration tests
└── cli/                 # Package: xlseek-cli (bin: xlseek-cli)
    ├── Cargo.toml       # depends on xlseek-core
    └── src/             # main.rs, lib.rs, args.rs, output.rs, constants.rs

src/                     # React / TypeScript frontend
├── constants/           # index.ts (storage keys, layout constants)
├── hooks/               # useSearchHistory.ts, etc.
└── components/          # UI components

src-tauri/               # Tauri v2 desktop application (xlseek)
├── Cargo.toml           # depends on xlseek-core
└── src/                 # lib.rs, main.rs, commands/, constants.rs

scripts/
└── generate-licenses.py # License generation tool
```

**Structure Decision**: Standard Cargo workspace structure with renamed core crate `xlseek-core` / `xlseek_core`.

## Complexity Tracking

No violations. Standard refactoring strictly adheres to existing architecture and project constitution.
