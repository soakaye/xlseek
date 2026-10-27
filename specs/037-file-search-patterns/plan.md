# Implementation Plan: File Search Patterns with Extension Complement and Sequential Filtering

**Branch**: `037-file-search-patterns` | **Date**: 2026-10-27 | **Spec**: [spec.md](file:///Users/soakaye/Develop/Projects/workspaces/exgrep/shrimpfish/specs/037-file-search-patterns/spec.md)

**Input**: Feature specification from `specs/037-file-search-patterns/spec.md`

---

## Summary

This feature adds inclusion file search pattern filtering with automatic extension complement and sequential evaluation (File Search Patterns → Ignore Patterns) across the Excel Seek suite (GUI, Core engine, standalone CLI, and MCP server).

Users can enter one or more comma-separated search patterns (such as `*budget*, quarterly_*`). If a pattern token lacks a file extension, the system automatically complements it using all currently active extension toggle options. If an explicit extension is specified (e.g. `*.xls`), it is preserved and takes precedence over unchecked extension toggles for supported document formats. During filesystem traversal, candidate files are first evaluated against the inclusion patterns; only candidates that pass are subsequently tested against the active ignore patterns.

The GUI search bar integrates a dedicated File Search Patterns row positioned directly between the Search Directory row and the Ignore Patterns row (`Directory` → `File Search Patterns` → `Ignore Patterns` → `Search Query / Options`), backed by an LRU history dropdown (capped at 20 entries) and persistent baseline configuration in the Settings dialog (Saved Defaults tab). The CLI exposes `-g` and `--file-patterns`, and the MCP server exposes the `file_patterns` parameter in its `search` tool schema.

---

## Technical Context

**Language/Version**: Rust 2021 edition (Rust 1.75+), TypeScript 5.x, React 18  
**Primary Dependencies**:
- Backend: `ignore = "0.4"` (reused for glob compilation), `calamine`, `rayon`, `rust_xlsxwriter`, `regex`, `serde`, `serde_json`, `tauri` v2
- Frontend: `@tauri-apps/api` v2, `lucide-react`, `tailwindcss`  
**Storage**: Client-side `localStorage` (`xlseek.filePatternHistory`, `xlseek.default_search_options`)  
**Testing**: `cargo test --workspace` (unit & integration tests), `npm run build` (TypeScript typecheck & bundle)  
**Target Platform**: Windows, macOS, Linux (cross-platform desktop Tauri application & standalone CLI binary)  
**Project Type**: Desktop GUI application (`src-tauri`, `src/`) and standalone CLI tool (`crates/cli/`) powered by shared core library (`crates/core/`) and MCP server (`crates/mcp/`)  
**Performance Goals**:
- Include pattern evaluation adds less than 5% overhead to total directory scanning time across 10,000 files.
- Reduces scanned file count and search duration by >50% for targeted sub-patterns compared to full directory scans.  
**Constraints**:
- Strict conformance to Project Constitution (Zero hardcoded constants, comprehensive header comments, modular single responsibility, zero warnings).
- Completely offline, local filesystem execution only. Zero real user data in persisted artifacts (Principle III).  
**Scale/Scope**:
- Directories with 100,000+ files and deeply nested directory trees.
- Pattern history capped at 20 most recently used entries.
- Supports 4 Excel file types (`.xlsx`, `.xlsm`, `.xlsb`, `.xls`) and 3 Word file types (`.docx`, `.docm`, `.doc`).

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Requirement | Plan Conformance / Verification | Status |
| :--- | :--- | :--- | :--- |
| **I. Language-Directed Quality** | English default for code comments, logs, spec docs; natural UI translations in `ja.yml` / `en.yml`. | All spec artifacts, plan docs, and code comments written in English. UI labels and placeholders added to both `crates/core/locales/ja.yml` and `en.yml`. Zero garbled text. | PASS |
| **II. No Hardcoded Constants** | No magic numbers or strings (except `0` and `""`). All constants extracted into centralized constants files with explicit reference comments. | All storage keys, delimiters, limits, input IDs, CLI flags (`-g`, `--file-patterns`), and default patterns extracted to `src/constants/index.ts` and `crates/core/src/constants.rs`. | PASS |
| **III. Comprehensive Header Comments & Documentation Hygiene** | Every file, module, struct, function, and method must have a header comment with (1) description, (2) arguments/returns, and (3) error/panic behavior. Zero real search data in documents. | All new Rust modules (`pattern.rs`) and TypeScript components (`filePatternHistory.ts`, `SearchBar.tsx` updates) will include full triple-element header comments. All test scenarios and artifacts use synthetic dummy names only. | PASS |
| **IV. Modular Design & Code Standards** | Single responsibility, standard style guides, zero TypeScript errors (`tsc --noEmit`), zero Clippy warnings (`cargo clippy --workspace --all-targets -- -D warnings`), formatted code (`cargo fmt --check`). | Modular design dividing pattern compilation (`pattern.rs`), traversal pipeline (`discovery.rs`), CLI parsing (`args.rs`), MCP tool schemas (`tools/search.rs`), and UI history (`filePatternHistory.ts`). | PASS |
| **V. Robust Error Handling & Testing** | No unhandled exceptions or unexpected panics; safe `Result`/`Option` handling; comprehensive unit and integration tests. | Malformed globs fail gracefully with non-fatal issue logging; unit and integration tests cover tokenization, extension complement, sequential filtering, CLI args, and history. | PASS |

---

## Project Structure

### Documentation (this feature)

```text
specs/037-file-search-patterns/
├── spec.md              # Feature specification with clarifications and acceptance scenarios
├── plan.md              # Implementation plan (this file)
├── research.md          # Phase 0 technical research and architecture decisions
├── data-model.md        # Phase 1 data entities and sequential traversal lifecycle
├── quickstart.md        # Phase 1 end-to-end synthetic validation scenarios
├── contracts/           # Phase 1 interface contracts
│   └── file-search-patterns-contract.md
├── checklists/          # Requirements validation checklists
│   └── requirements.md
└── tasks.md             # Phase 2 output (/speckit-tasks command)
```

### Source Code (repository root)

```text
crates/
├── core/
│   ├── locales/
│   │   ├── en.yml                         # English UI labels and placeholders
│   │   └── ja.yml                         # Japanese UI labels and placeholders
│   └── src/
│       ├── constants.rs                   # Core file pattern constants & CLI flags
│       ├── models/
│       │   └── mod.rs                     # SearchQuery.file_patterns field
│       └── search/
│           ├── mod.rs                     # Export pattern module
│           ├── pattern.rs                 # NEW: FilePatternMatcher & complement logic
│           ├── discovery.rs               # Sequential filtering: pattern -> ignore
│           └── engine.rs                  # Wire file_patterns into discovery pipeline
├── cli/
│   └── src/
│       └── args.rs                        # Parse -g / --file-patterns CLI flags
├── mcp/
│   └── src/
│       └── tools/
│           └── search.rs                  # Add file_patterns to tool schema & handler
src-tauri/
└── src/
    ├── constants.rs                       # GUI-specific constants
    └── commands/
        └── search_cmd.rs                  # Forward file_patterns in SearchQuery
src/
├── constants/
│   └── index.ts                           # Storage keys, IDs, default values
├── types/
│   └── search.ts                          # Frontend SearchQuery & search state types
├── utils/
│   └── filePatternHistory.ts              # NEW: LRU history storage for file patterns
├── settings-core.ts                       # Add defaultFilePatterns to config
└── components/
    ├── search/
    │   └── SearchBar.tsx                  # Dedicated File Search Patterns row
    └── settings/
        └── SavedDefaultsTab.tsx           # Default File Search Patterns field
```

**Structure Decision**:
Follows the established multi-crate repository structure: shared core domain logic in `crates/core`, CLI interface in `crates/cli`, MCP tool server in `crates/mcp`, Tauri command bridge in `src-tauri`, and React components in `src/`.

---

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| *None* | All constitution principles satisfied | No architectural violations or exceptions required |
