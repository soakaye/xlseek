# Research & Architectural Decisions: Rename exgrep/exlgrep to xlseek

**Feature**: `021-rename-to-xlseek` | **Date**: 2026-10-02  
**Spec Reference**: [spec.md](file:///Users/soakaye/Develop/Projects/workspaces/exgrep/cuskeel/specs/021-rename-to-xlseek/spec.md)

## Summary of Research

This document details the architectural decisions and mapping rules for systematically migrating all active occurrences of `exgrep` and `exlgrep` to `xlseek` across crates, manifests, code, constants, storage keys, and documentation.

---

### Decision 1: Rust Crate and Library Package Names

- **Decision**: 
  - `crates/core/Cargo.toml`: Package name changed from `exlgrep-core` to `xlseek-core`, library name changed from `exlgrep_core` to `xlseek_core`.
  - `crates/cli/Cargo.toml`: Package name remains `xlseek-cli` (or verify matches `xlseek_cli`), dependency updated from `exlgrep-core` to `xlseek-core`.
  - `src-tauri/Cargo.toml`: Dependency updated from `exlgrep-core = { path = "../crates/core" }` to `xlseek-core = { path = "../crates/core" }`.
  - `Cargo.toml` (root workspace): Comments and package descriptions updated from `exlgrep` to `xlseek`.
- **Rationale**: Rust Cargo requires matching package and library names for clear dependency resolution. Using `xlseek-core` / `xlseek_core` directly aligns with `xlseek` and `xlseek-cli`.
- **Alternatives considered**:
  - Aliasing `exlgrep-core` under `xlseek-core` or retaining old crate names: Rejected because it leaves lingering legacy names in `Cargo.lock`, crate metadata, and developer tooling.

---

### Decision 2: Rust Source Code and Import Refactoring

- **Decision**:
  - Replace all `use exlgrep_core::` with `use xlseek_core::` across `crates/cli`, `crates/core`, `crates/core/tests/`, and `src-tauri/`.
  - Update `crates/core/src/lib.rs` header documentation and module docs from `exlgrep_core` to `xlseek_core`.
  - Update `crates/cli/src/constants.rs` re-export from `exlgrep_core::constants::*` to `xlseek_core::constants::*`.
  - Update `src-tauri/src/lib.rs` re-export from `exlgrep_core::{export, i18n, models, search}` to `xlseek_core::{export, i18n, models, search}`.
  - Update test fixture identifiers in `crates/core/src/constants.rs` and `crates/core/tests/`:
    - `CLI_TEMP_DIRECTORY_PREFIX`: change `".exlgrep-export-"` to `".xlseek-export-"`.
    - `CLI_TEST_OUTPUT_NAME`: change `"exlgrep-cli-args-test.csv"` to `"xlseek-cli-args-test.csv"`.
    - Test prefixes: `"exlgrep-i18n-test"` -> `"xlseek-i18n-test"`, `"exlgrep-shape-export"` -> `"xlseek-shape-export"`, `"exlgrep-path-tests-"` -> `"xlseek-path-tests-"`.
- **Rationale**: Eliminates hidden references in temp files, error messages, and integration tests, ensuring clean isolation during testing and runtime.
- **Alternatives considered**:
  - Leaving test fixture names untouched: Rejected because it violates SC-001 (zero occurrences in active codebase).

---

### Decision 3: Storage Keys & Backward-Compatible Migration

- **Decision**:
  - Primary frontend constants (`src/constants/index.ts`):
    - `LANGUAGE_STORAGE_KEY`: already `"xlseek.language"`.
    - `SEARCH_HISTORY_CONSTANTS.STORAGE_KEY`: already `"xlseek.searchHistory"`.
    - `DEFAULT_OPTIONS_STORAGE_KEY`: already `"xlseek.default_search_options"`.
  - Migration / Fallback:
    - In search history hook (`useSearchHistory` or equivalent) and options loader, verify if legacy keys (`exlgrep.searchHistory`, `exlgrep.search_history`, `exlgrep.search-history.maxEntries`, `exlgrep.default_search_options`, `exlgrep.language`) exist. If present and the corresponding `xlseek.*` key does not exist, migrate the value over to the `xlseek.*` key.
  - `design/mainui/index.html`: Update legacy mock references (e.g. `LEGACY_HISTORY_KEY`) to properly document migration behavior.
- **Rationale**: Ensures existing users upgrading from previous versions don't lose their customized settings or history, fulfilling User Story 2.
- **Alternatives considered**:
  - Immediately dropping legacy keys without fallback: Rejected because users would have their settings and history reset.

---

### Decision 4: Build Scripts, Packaging & License Tools

- **Decision**:
  - `scripts/generate-licenses.py`:
    - Update workspace package filters: keep `xlseek` and `xlseek-cli`, remove or adapt `exgrep` and `exlgrep`.
  - `README.md` & `README_ja.md`:
    - Update clone URLs: `https://github.com/soakaye/xlseek.git` and `cd xlseek` (as clarified in Session 2026-10-02).
    - Update project directory diagrams: `xlseek/` root structure and `xlseek-core: shared engine...`.
  - `AGENTS.md`:
    - Update directory structure diagram from `exlgrep/` to `xlseek/`.
    - Update crate references from `exlgrep-core` to `xlseek-core`.
- **Rationale**: Complete consistency across documentation and automated tools.

---

### Decision 5: Historical Specification Preservation

- **Decision**:
  - `specs/001-` through `specs/020-` and old `.specify/bugs/` reports will NOT be edited.
- **Rationale**: Conforming to FR-007 and spec Edge Cases: historical design documents reflect the state at the time they were written and must remain immutable revision records.
