# Data Model & Storage Migration: Rename to xlseek

**Feature**: `021-rename-to-xlseek` | **Date**: 2026-10-02  
**Spec Reference**: [spec.md](file:///Users/soakaye/Develop/Projects/workspaces/exgrep/cuskeel/specs/021-rename-to-xlseek/spec.md)

## Entities & Configuration Identifiers

### 1. Cargo Package Entity

Represents the Rust workspace crate metadata defining package names, library crate names, and inter-crate dependency paths.

| Crate Directory | Previous Package Name | Previous Crate Lib Name | New Package Name | New Crate Lib Name |
|---|---|---|---|---|
| `crates/core` | `exlgrep-core` | `exlgrep_core` | `xlseek-core` | `xlseek_core` |
| `crates/cli` | `xlseek-cli` | `xlseek_cli` | `xlseek-cli` | `xlseek_cli` |
| `src-tauri` | `xlseek` | `xlseek_lib` | `xlseek` | `xlseek_lib` |

#### Dependency Declaration
```toml
# crates/cli/Cargo.toml
[dependencies]
xlseek-core = { path = "../core" }

# src-tauri/Cargo.toml
[dependencies]
xlseek-core = { path = "../crates/core" }
```

---

### 2. Storage Key Entity & Migration Scheme

Represents persistent configuration items stored in the browser's `localStorage` or desktop preferences.

| Configuration Item | Legacy Key (Read Fallback) | Canonical New Key (Read / Write) | Value Type / Schema |
|---|---|---|---|
| Language Preference | `exlgrep.language` | `xlseek.language` | `"default" \| "ja" \| "en"` |
| Search History | `exlgrep.searchHistory`, `exlgrep.search_history` | `xlseek.searchHistory` | `string[]` (JSON serialized) |
| Max History Entries | `exlgrep.search-history.maxEntries` | `xlseek.searchHistoryLimit` (or unified) | `number` |
| Default Search Options | `exlgrep.default_search_options` | `xlseek.default_search_options` | `SearchOptions` object (JSON) |

#### Migration State Transition & Resolution Rule

1. Check if canonical key (`xlseek.*`) exists.
   - If present: use its value directly.
2. If canonical key does NOT exist, check for legacy key (`exlgrep.*`).
   - If legacy key exists:
     - Read value and parse into domain model.
     - Write value to canonical `xlseek.*` key.
     - Optionally delete or retain legacy key (retain to prevent regressions on downgrade).
   - If neither key exists: use project standard default values.

---

### 3. File System & Temporary Artifact Entity

Runtime identifiers used for temporary export directories, scratch files, and testing fixtures.

| Item | Previous Identifier | Canonical New Identifier | Scope |
|---|---|---|---|
| CLI Export Temp Dir | `.exlgrep-export-` | `.xlseek-export-` | `crates/core/src/constants.rs` (`CLI_TEMP_DIRECTORY_PREFIX`) |
| CLI Test Output | `exlgrep-cli-args-test.csv` | `xlseek-cli-args-test.csv` | `crates/core/src/constants.rs` (`CLI_TEST_OUTPUT_NAME`) |
| i18n Test Prefix | `exlgrep-i18n-test` | `xlseek-i18n-test` | `crates/core/src/export/` |
| Shape Export Prefix | `exlgrep-shape-export` | `xlseek-shape-export` | `crates/core/tests/shape_export.rs` |
| Path Discovery Temp | `exlgrep-path-tests-` | `xlseek-path-tests-` | `crates/core/src/search/path.rs` |
