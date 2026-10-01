# Tasks: Unify Project and Package Naming to xlseek

**Branch**: `021-rename-to-xlseek` | **Date**: 2026-10-02 | **Spec**: [spec.md](file:///Users/soakaye/Develop/Projects/workspaces/exgrep/cuskeel/specs/021-rename-to-xlseek/spec.md) | **Plan**: [plan.md](file:///Users/soakaye/Develop/Projects/workspaces/exgrep/cuskeel/specs/021-rename-to-xlseek/plan.md)

## Phase 1: Setup (Package & Workspace Alignment)

**Purpose**: Update workspace manifests and package names to establish `xlseek-core` / `xlseek_core`.

- [X] T001 Rename core package and library from `exlgrep-core` / `exlgrep_core` to `xlseek-core` / `xlseek_core` in `crates/core/Cargo.toml`
- [X] T002 [P] Update CLI crate dependency from `exlgrep-core` to `xlseek-core` in `crates/cli/Cargo.toml`
- [X] T003 [P] Update desktop application dependency from `exlgrep-core` to `xlseek-core` in `src-tauri/Cargo.toml`
- [X] T004 [P] Update root Cargo workspace comments and description in `Cargo.toml`

---

## Phase 2: Foundational (Core Library Refactoring)

**Purpose**: Update core library imports, documentation headers, and internal constants.

- [X] T005 Update library module header and documentation in `crates/core/src/lib.rs` from `exlgrep_core` to `xlseek_core`
- [X] T006 Update temporary directory prefix constant `CLI_TEMP_DIRECTORY_PREFIX` to `".xlseek-export-"` and test output name `CLI_TEST_OUTPUT_NAME` to `"xlseek-cli-args-test.csv"` in `crates/core/src/constants.rs`
- [X] T007 [P] Update test prefix constants in `crates/core/src/export/csv_export.rs` and `crates/core/src/export/xlsx_export.rs` from `exlgrep-i18n-test` to `xlseek-i18n-test`
- [X] T008 [P] Update temporary directory path prefix in `crates/core/src/search/path.rs` from `exlgrep-path-tests-` to `xlseek-path-tests-`

---

## Phase 3: User Story 1 - Consistent Branding and Binary Identification (Priority: P1) 🎯 MVP

**Goal**: Update all active crate code, re-exports, and tests to import from `xlseek_core` and use `xlseek` identifiers.

**Independent Test**: Run `cargo test --workspace` and verify that all crates compile and all integration tests pass with zero `exlgrep` references.

### Implementation for User Story 1

- [X] T009 [P] [US1] Update imports and constants re-export from `exlgrep_core` to `xlseek_core` in `crates/cli/src/constants.rs`
- [X] T010 [P] [US1] Update imports from `exlgrep_core` to `xlseek_core` in `crates/cli/src/args.rs`, `crates/cli/src/lib.rs`, and `crates/cli/src/output.rs`
- [X] T011 [P] [US1] Update re-exports and command imports from `exlgrep_core` to `xlseek_core` in `src-tauri/src/lib.rs` and `src-tauri/src/commands/search_cmd.rs`
- [X] T012 [P] [US1] Update integration test imports and fixture references in `crates/core/tests/burst_discovery.rs`, `crates/core/tests/shape_contract.rs`, `crates/core/tests/shape_export.rs`, and `crates/core/tests/shape_extract.rs`
- [X] T013 [US1] Run `cargo test --workspace` to verify end-to-end compilation and all tests pass with the new crate name

**Checkpoint**: At this point, the backend workspace builds cleanly and passes all unit and integration tests using `xlseek_core`.

---

## Phase 4: User Story 2 - Smooth Migration for Existing User Configurations (Priority: P2)

**Goal**: Ensure frontend storage keys default to `xlseek.*` and gracefully migrate any legacy `exlgrep.*` keys found in local storage.

**Independent Test**: Load frontend with mock legacy `exlgrep.*` keys and verify they are read and migrated into `xlseek.*` keys without loss of data.

### Implementation for User Story 2

- [X] T014 [US2] Review and verify `src/constants/index.ts` constants and add backward-compatible legacy key fallbacks where needed in `src/constants/index.ts`
- [X] T015 [US2] Ensure search history loader/hook in `src/hooks/useSearch.ts` (or relevant history/options manager) reads legacy `exlgrep` keys and saves to canonical `xlseek` keys
- [X] T016 [US2] Update legacy key comment and mock constants in `design/mainui/index.html` to reflect migration behavior

**Checkpoint**: Existing stored configurations under `exlgrep` keys continue to load seamlessly and are persisted under `xlseek` keys.

---

## Phase 5: User Story 3 - Coherent Documentation and Developer References (Priority: P3)

**Goal**: Update READMEs, developer guidelines, and utility scripts to consistently refer to `xlseek` and the new clone URLs.

**Independent Test**: Review `README.md`, `README_ja.md`, `AGENTS.md`, and execute `python3 scripts/generate-licenses.py` without error.

### Implementation for User Story 3

- [X] T017 [P] [US3] Update package filter in `scripts/generate-licenses.py` to target `xlseek` and `xlseek-cli`
- [X] T018 [P] [US3] Update repository clone URLs to `https://github.com/soakaye/xlseek.git`, `cd xlseek`, and crate directory descriptions in `README.md`
- [X] T019 [P] [US3] Update repository clone URLs to `https://github.com/soakaye/xlseek.git`, `cd xlseek`, and crate directory descriptions in `README_ja.md`
- [X] T020 [P] [US3] Update project directory structure and crate references from `exlgrep` to `xlseek` in `AGENTS.md`

---

## Phase 6: Polish & Verification

**Purpose**: Verify all quality gates, linters, formatting, and zero residual references in active files.

- [X] T021 Run `git grep -i -E "exlgrep|exgrep" -- :!specs` to verify zero residual occurrences remain in active code or docs
- [X] T022 [P] Run `cargo fmt --check` and `cargo clippy --workspace --all-targets -- -D warnings` to verify complete Rust standard compliance
- [X] T023 [P] Run `npm run build` and `npm test` to verify zero frontend type errors or regressions
- [X] T024 Execute `python3 scripts/generate-licenses.py` to regenerate `src/constants/licenses.json` with updated package metadata

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: Can start immediately.
- **Phase 2 (Foundational)**: Depends on Phase 1 completion.
- **Phase 3 (User Story 1)**: Depends on Phase 2 completion (MVP).
- **Phase 4 (User Story 2)**: Can run independently after Phase 1.
- **Phase 5 (User Story 3)**: Can run in parallel with Phase 3/4.
- **Phase 6 (Polish)**: Depends on all implementation phases (1-5) complete.

### Parallel Opportunities

- Tasks marked `[P]` touch disjoint files and can be executed concurrently.
- `T002`, `T003`, `T004` can run in parallel during Setup.
- `T007`, `T008` can run in parallel during Foundational.
- `T009`, `T010`, `T011`, `T012` can run concurrently during User Story 1.
- `T017`, `T018`, `T019`, `T020` can run concurrently during User Story 3.
- `T022`, `T023` can run concurrently in Polish.

---

## Implementation Strategy

### MVP First (User Story 1 Only)
1. Complete Phase 1 (Setup: manifests renamed).
2. Complete Phase 2 (Foundational: core library renamed).
3. Complete Phase 3 (User Story 1: imports and integration tests updated).
4. Run `cargo test --workspace` to validate that the backend builds and passes all tests under `xlseek_core`.

### Incremental Delivery
1. Setup + Foundational + US1 → Working Rust workspace under `xlseek` (MVP).
2. US2 → Frontend migration and backwards compatibility.
3. US3 → Documentation, license generator script, and guidelines updated.
4. Polish → Zero residual references verified, `cargo clippy`, `cargo fmt`, `npm run build`.
