# Implementation Plan: Clean Changelog Headers and English Source Comments

**Branch**: `017-clean-changelog-english-comments` | **Date**: 2026-09-30 | **Spec**: [spec.md](file:///Users/soakaye/Develop/Projects/workspaces/exgrep/hamlet/specs/017-clean-changelog-english-comments/spec.md)

**Input**: Feature specification from `/specs/017-clean-changelog-english-comments/spec.md`

## Summary

In accordance with Constitution v3.0.0 (Principle I & Principle III), update the codebase to:
1. Completely remove in-file changelog sections (`変更履歴`, `Revision History`, author/version tables) from all file and item header doc comments across Rust, TypeScript, and HTML prototype files.
2. Translate all source code comments (doc comments, inline comments, constant reference annotations) from Japanese to natural, accurate English across `crates/`, `src-tauri/`, `src/`, and `design/mainui/index.html`.
3. Preserve Japanese localization resources in `crates/core/locales/ja.yml` intact.
4. Align `AGENTS.md` and developer guidelines with Constitution v3.0.0.
5. Verify that all 4 mandatory quality gates (`cargo test`, `cargo clippy`, `cargo fmt`, `npm run build`) pass with 0 errors and 0 warnings.

## Technical Context

**Language/Version**: Rust 2021 edition, TypeScript 5.x / React 18, HTML5 / Tailwind CSS
**Primary Dependencies**: Tauri v2, `calamine`, `rayon`, `rust_xlsxwriter`, `regex`, Lucide React
**Storage**: N/A (in-memory parsing / streaming search)
**Testing**: `cargo test --workspace`, `npm run build` (`tsc --noEmit`), `cargo clippy`, `cargo fmt`
**Target Platform**: Desktop (macOS, Windows, Linux via Tauri v2)
**Project Type**: Desktop GUI + CLI application + Core library
**Performance Goals**: Zero runtime performance impact (docstrings and comments only)
**Constraints**: Zero compiler/linter warnings, preserve all runtime localization strings in `ja.yml`
**Scale/Scope**: ~65 source files across Rust crates, Tauri backend, React frontend, plus `design/mainui/index.html` and `AGENTS.md`

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Principle I (Language-Directed Quality)**: Default language for unspecified user responses and generated documents/code comments is English. Japanese UI strings remain in `ja.yml`. -> **PASS**
- **Principle II (No Hardcoded Constants)**: Constant reference annotations are preserved and translated to English (`// Constant reference: ...`). No constants modified or hardcoded. -> **PASS**
- **Principle III (Comprehensive Header Comments)**: 3-element header schema enforced (Purpose, Arguments/Returns, Errors/Exceptions). In-file changelog headers strictly removed in favor of Git version control. -> **PASS**
- **Principle IV (Modular Design & Code Standards)**: Standard coding style and formatting enforced (`cargo fmt`, `cargo clippy`, `npm run build`). -> **PASS**
- **Principle V (Robust Error Handling & Testing)**: All existing tests and error handling logic preserved intact. -> **PASS**

## Project Structure

### Documentation (this feature)

```text
specs/017-clean-changelog-english-comments/
├── plan.md              # This file (/speckit-plan command output)
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
│   └── documentation-contract.md
├── checklists/
│   └── requirements.md
└── tasks.md             # Phase 2 output (/speckit-tasks command)
```

### Source Code (repository root)

```text
crates/
├── core/src/            # Core search, models, export, i18n
│   ├── lib.rs, constants.rs, i18n.rs
│   ├── models/*.rs
│   ├── search/*.rs
│   └── export/*.rs
├── cli/src/             # Standalone CLI binary
│   └── main.rs, lib.rs, constants.rs, args.rs, output.rs
src-tauri/src/           # Tauri desktop backend
│   └── main.rs, lib.rs, constants.rs, commands/*.rs
src/                     # React / TypeScript frontend
│   ├── App.tsx, main.tsx
│   ├── constants/index.ts
│   ├── types/*.ts
│   ├── hooks/*.ts
│   └── components/**/*.tsx
design/mainui/           # UI Prototype
│   └── index.html
AGENTS.md                # Agent guidelines
```

**Structure Decision**: Multi-crate Rust workspace + React/Tauri frontend. Refactoring touches comments across all submodules in distinct, well-bounded modules.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| None      | N/A        | Standard refactoring within guidelines |
