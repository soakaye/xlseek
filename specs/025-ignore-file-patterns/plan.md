# Implementation Plan: Ignore File Patterns with Gitignore-Style Matching and History

**Branch**: `025-ignore-file-patterns` | **Date**: 2026-10-15 | **Spec**: [spec.md](file:///F:/source/orca/workspaces/xlseek/.orca/workspaces/xlseek/ponyfish/specs/025-ignore-file-patterns/spec.md)

**Input**: Feature specification from `specs/025-ignore-file-patterns/spec.md`

---

## Summary

This feature adds support for comma-separated `.gitignore`-style file and directory exclusion patterns across `xlseek` (GUI, Core engine, and CLI). Users can supply patterns (e.g. `~$*, *.tmp, backup/`, pre-populated with `~$*` by default) in an input field placed directly beneath the search directory row in the search interface. The directory discovery engine prunes excluded folders early and filters matching files before parsing. Executed pattern combinations are persistently saved in local storage (up to 20 entries) and selectable via an accessible dropdown history menu. In addition, users can configure their preferred default ignore patterns in the Settings dialog under Saved Search Defaults.

Technical approach:
- Rust backend: Encapsulate `.gitignore` semantics using `ignore::gitignore::GitignoreBuilder` in a new module `crates/core/src/search/ignore.rs`, pruning directories and skipping files early in `crates/core/src/search/discovery.rs`.
- Tauri IPC: Extend `SearchQuery` in `crates/core/src/models/mod.rs` and `src-tauri/src/commands/search_cmd.rs` with `ignore_patterns: Option<String>`.
- CLI: Add `-i` / `--ignore-patterns` option in `crates/cli/src/args.rs`.
- React Frontend: Place an ignore pattern input on a dedicated row beneath the search directory in `src/components/search/SearchBar.tsx`, backed by an LRU history store in `src/utils/ignoreHistory.ts` and persistent Settings default options in `src/settings-core.ts` / `src/components/settings/SettingsDialog.tsx`.

---

## Technical Context

**Language/Version**: Rust 2021 edition (Rust 1.75+), TypeScript 5.x, React 18  
**Primary Dependencies**:
- Backend: `ignore = "0.4"`, `calamine`, `rayon`, `rust_xlsxwriter`, `regex`, `serde`, `tauri` v2
- Frontend: `@tauri-apps/api` v2, `lucide-react`, `tailwindcss`  
**Storage**: Client-side `localStorage` (`xlseek.ignorePatternHistory`, `xlseek.default_search_options`)  
**Testing**: `cargo test --workspace` (unit & integration tests), `npm run build` (TypeScript type check & bundle)  
**Target Platform**: Windows, macOS, Linux (cross-platform desktop Tauri application & standalone CLI binary)  
**Project Type**: Desktop GUI application (`src-tauri`, `src/`) and standalone CLI tool (`crates/cli/`) powered by shared core library (`crates/core/`)  
**Performance Goals**:
- Early directory pruning skips entire excluded folder subtrees during traversal without scheduling tasks or reading child directories.
- Zero perceptible latency impact on directory discovery (<10ms matching overhead for 100,000 files).  
**Constraints**:
- Strict conformance to Project Constitution (No hardcoded constants, comprehensive header comments, modular design, zero warnings).
- Completely offline, local filesystem execution only.  
**Scale/Scope**:
- Directories containing up to 100,000+ files and deeply nested subtrees.
- Persistent history capped at 20 most recently used entries.
- Supports 4 Excel file types (`.xlsx`, `.xlsm`, `.xlsb`, `.xls`) and 3 Word file types (`.docx`, `.docm`, `.doc`).

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Requirement | Plan Conformance / Verification | Status |
| :--- | :--- | :--- | :--- |
| **I. Language-Directed Quality** | English default for code comments, logs, spec docs; Japanese for user communication; accurate translations in `ja.yml` / `en.yml`. | All spec artifacts and code comments written in English. UI strings added to both `crates/core/locales/ja.yml` and `en.yml`. Zero garbled text. | PASS |
| **II. No Hardcoded Constants** | No magic numbers or strings (except `0` and `""`). All constants extracted into central modules with explicit reference comments. | All storage keys, delimiters, limits, input IDs, and default patterns extracted to `src/constants/index.ts` and `crates/core/src/constants.rs`. | PASS |
| **III. Comprehensive Header Comments** | Every file, module, struct, function, and method must have a header comment with (1) description, (2) arguments/returns, and (3) error/panic behavior. No change histories. | All new Rust modules (`ignore.rs`) and TypeScript components (`ignoreHistory.ts`, `SearchBar.tsx` updates) will include full triple-element header comments. | PASS |
| **IV. Modular Design & Code Standards** | Single responsibility, standard style guides, zero TypeScript errors (`tsc --noEmit`), zero Clippy warnings (`cargo clippy --workspace --all-targets -- -D warnings`), formatted code (`cargo fmt --check`). | Modular architecture dividing matching, discovery, CLI parsing, and UI history components cleanly. Verified with official linters. | PASS |
| **V. Robust Error Handling & Testing** | No unhandled exceptions or unexpected panics; safe `Result`/`Option` handling; comprehensive unit and integration tests. | Invalid glob expressions fail gracefully with non-fatal issue logging; unit tests cover tokenization, matching, directory pruning, and history limits. | PASS |

---

## Project Structure

### Documentation (this feature)

```text
specs/025-ignore-file-patterns/
├── spec.md              # Feature specification with clarifications and scenarios
├── plan.md              # Implementation plan (this file)
├── research.md          # Phase 0 technical research and decisions
├── data-model.md        # Phase 1 data entities and traversal lifecycle
├── quickstart.md        # Phase 1 validation scenarios and quality gates
├── contracts/           # Phase 1 interface contracts
│   └── ignore-patterns-contract.md
├── checklists/          # Requirements checklists
│   └── requirements.md
└── tasks.md             # Phase 2 output (/speckit-tasks command)
```

### Source Code (repository root)

```text
crates/
├── core/
│   ├── Cargo.toml                         # Add `ignore = "0.4"` dependency
│   ├── locales/
│   │   ├── en.yml                         # English UI labels and placeholders
│   │   └── ja.yml                         # Japanese UI labels and placeholders
│   └── src/
│       ├── constants.rs                   # Core ignore pattern constants & CLI flags
│       ├── models/
│       │   └── mod.rs                     # SearchQuery.ignore_patterns field
│       └── search/
│           ├── mod.rs                     # Export ignore module
│           ├── ignore.rs                  # NEW: IgnoreMatcher domain struct & logic
│           └── discovery.rs               # Early directory pruning & file filtering
├── cli/
│   └── src/
│       ├── constants.rs                   # Re-exports core CLI constants
│       └── args.rs                        # Add -i / --ignore-patterns CLI argument parsing
src-tauri/
└── src/
    ├── constants.rs                       # GUI-specific constants
    └── commands/
        └── search_cmd.rs                  # Forward ignore_patterns to search engine
src/
├── constants/
│   └── index.ts                           # Storage keys, default patterns, limits
├── types/
│   ├── search.ts                          # SearchQuery.ignore_patterns type
│   └── settings.ts                        # DefaultSearchOptions.ignore_patterns type
├── utils/
│   ├── ignoreHistory.ts                   # NEW: LocalStorage history persistence & LRU logic
│   └── ignoreTokenizer.ts                 # NEW: Comma-separated pattern tokenizer
├── settings-core.ts                       # Default search options normalization & persistence
└── components/
    ├── search/
    │   ├── SearchBar.tsx                  # Dedicated ignore patterns input row & history dropdown
    │   └── IgnoreHistoryDropdown.tsx      # NEW: Accessible dropdown for recent patterns
    └── settings/
        └── SettingsDialog.tsx             # Saved Defaults tab ignore patterns setting
```

**Structure Decision**:
Follows the established three-tier architecture of xlseek (`crates/core` for domain logic, `crates/cli` for standalone CLI, `src-tauri` / `src` for desktop GUI), adhering to strict modular separation and centralized constant declarations.

---

## Complexity Tracking

> No constitution violations detected. Standard modular design patterns applied.
