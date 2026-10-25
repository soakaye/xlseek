# Tasks: Minimal Version Bump Files and Consolidated Display Copyright

**Input**: Design documents from `specs/033-minimize-version-files/`
**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md), [data-model.md](./data-model.md), [contracts/version-sync-contract.md](./contracts/version-sync-contract.md), [quickstart.md](./quickstart.md)

## Format: `[ID] [P?] [Story?] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Exact file paths are specified in each description

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Build-time environment preparation and Vite configuration for dynamic version injection

- [X] T001 Inspect current Git status and stage baseline dependencies in `package.json`
- [X] T002 [P] Configure Vite `define` for `__APP_VERSION__` from `package.json.version` in `vite.config.ts`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Establish Cargo workspace package inheritance and Tauri package version delegation across all crates

**⚠️ CRITICAL**: Must complete before user story implementation to ensure crate manifests inherit workspace metadata without version drift

- [X] T003 Configure `[workspace.package]` table with canonical version, edition, authors, and license in root `Cargo.toml`
- [X] T004 [P] Update `crates/core/Cargo.toml` to inherit `version.workspace = true`, `edition.workspace = true`, and `authors.workspace = true`
- [X] T005 [P] Update `crates/cli/Cargo.toml` to inherit `version.workspace = true`, `edition.workspace = true`, and `authors.workspace = true`
- [X] T006 [P] Update `crates/mcp/Cargo.toml` to inherit `version.workspace = true`, `edition.workspace = true`, and `authors.workspace = true`
- [X] T007 [P] Update `src-tauri/Cargo.toml` to inherit `version.workspace = true`, `edition.workspace = true`, and `authors.workspace = true`
- [X] T008 Update `src-tauri/tauri.conf.json` to delegate application version to `"version": "../package.json"`

**Checkpoint**: Foundation ready — all member crates and Tauri configuration now resolve versioning from root workspace manifests.

---

## Phase 3: User Story 1 - Streamlined Version Update via npm version (Priority: P1) 🎯 MVP

**Goal**: Minimize modified files during `npm version` to just 2 primary manifests (`package.json`, root `Cargo.toml`) plus lock files, and guarantee `Cargo.lock` is updated and staged in the release commit.

**Independent Test**: Run `npm version patch --no-git-tag-version`. Verify with `git status -s` that only `package.json`, `package-lock.json`, `Cargo.toml`, and `Cargo.lock` are modified, both `Cargo.toml` and `Cargo.lock` are staged in the Git index, and no dirty lockfile diff remains.

### Implementation for User Story 1

- [X] T009 [P] [US1] Remove static `about.APP_VERSION` entry from `crates/core/locales/en.yml` and `crates/core/locales/ja.yml`
- [X] T010 [P] [US1] Remove static `about.APP_VERSION` entry from `src-tauri/locales/en.yml` and `src-tauri/locales/ja.yml`
- [X] T011 [US1] Update `src/constants/index.ts` to declare `__APP_VERSION__` global and set `APP_CONSTANTS.APP_VERSION` dynamically without hardcoding version strings
- [X] T012 [US1] Refactor `scripts/sync-version-constants.js` to retain only `Cargo.toml` and `Cargo.lock` targets, removing obsolete locale, mockup, and subcrate paths and regexes
- [X] T013 [US1] Refactor `scripts/sync-version-utils.js` to update root `Cargo.toml` `[workspace.package].version`, trigger `cargo check --workspace`, verify `Cargo.lock` update, stage with `git add Cargo.toml Cargo.lock`, and verify Git index staging status
- [X] T014 [US1] Refactor `scripts/sync-version.js` entrypoint to execute streamlined sync and abort with exit code 1 if `Cargo.lock` update or staging fails
- [X] T015 [US1] Update unit tests in `scripts/sync-version.test.js` to validate root `Cargo.toml` transformation, `Cargo.lock` staging verification, and rejection of invalid semver

**Checkpoint**: User Story 1 is fully functional. `npm version` now modifies only the minimal authoritative files and guarantees `Cargo.lock` staging.

---

## Phase 4: User Story 2 - Consolidated Display Copyright Information (Priority: P2)

**Goal**: Eliminate static copyright string duplication across localization files and dynamically generate `Copyright © {year} {author}` in the About dialog while preserving locale labels.

**Independent Test**: Launch the application or view the About dialog in both English and Japanese locales. Verify that the label displays correctly ("Copyright:" in English, "著作権:" in Japanese) and the attribution displays `Copyright © <current_year> soakaye` without any static copyright strings in `en.yml` or `ja.yml`.

### Implementation for User Story 2

- [X] T016 [US2] Centralize `APP_AUTHOR: "soakaye"` in `src/constants/index.ts` in compliance with Constitution Principle II
- [X] T017 [P] [US2] Remove static `about.COPYRIGHT` string from `crates/core/locales/en.yml` and `crates/core/locales/ja.yml` while retaining `about.COPYRIGHT_LABEL`
- [X] T018 [P] [US2] Remove static `about.COPYRIGHT` string from `src-tauri/locales/en.yml` and `src-tauri/locales/ja.yml` while retaining `about.COPYRIGHT_LABEL`
- [X] T019 [US2] Update `src/components/about/AboutDialog.tsx` to dynamically format `Copyright © ${new Date().getFullYear()} ${APP_CONSTANTS.APP_AUTHOR}` using `t("about.COPYRIGHT_LABEL")`
- [X] T020 [US2] Update frontend unit tests in `tests/meta-info-card.test.tsx` and related test files to verify dynamic copyright rendering and localized label presentation

**Checkpoint**: User Stories 1 AND 2 are complete. Display copyright is dynamic and single-sourced, with zero duplicate strings in locale YAMLs.

---

## Phase 5: User Story 3 - Preservation of Source File Header Copyrights (Priority: P3)

**Goal**: Guarantee that existing copyright notices and header comments across all existing source files remain 100% intact and unaffected by version/copyright minimization.

**Independent Test**: Run Git diff audit across all existing `.rs`, `.ts`, `.tsx`, and `.js` source files to verify that zero file header comments have been altered or deleted.

### Implementation for User Story 3

- [X] T021 [US3] Add automated test assertion in `scripts/sync-version.test.js` verifying that source file headers are untouched during version operations
- [X] T022 [US3] Perform automated header integrity check across all `.rs`, `.ts`, `.tsx`, and `.js` files in `crates/`, `src-tauri/`, `src/`, and `scripts/` to confirm zero header regressions

**Checkpoint**: User Stories 1, 2, and 3 are all complete and verified. Source file header integrity is confirmed.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: End-to-end validation, compilation check, linting, and quality gate verification

- [X] T023 Run Rust workspace build and tests via `cargo test --workspace` and verify zero warnings via `cargo clippy --workspace --all-targets -- -D warnings`
- [X] T024 Run frontend type checking and bundle build via `npm run build`
- [X] T025 Execute end-to-end dry-run simulation via `npm version patch --no-git-tag-version` to confirm exactly 4 files (`package.json`, `package-lock.json`, `Cargo.toml`, `Cargo.lock`) are modified and staged, then clean up simulation state

---

## Dependencies & Execution Order

### Phase Dependencies
- **Setup (Phase 1)**: Independent — can start immediately.
- **Foundational (Phase 2)**: Depends on Setup (Phase 1) — BLOCKS all user stories.
- **User Story 1 (Phase 3)**: Depends on Foundational completion.
- **User Story 2 (Phase 4)**: Depends on Foundational completion (can run in parallel with US1).
- **User Story 3 (Phase 5)**: Depends on US1 and US2 completion (validates header integrity across all modified files).
- **Polish (Phase 6)**: Depends on all user stories being complete.

### Parallel Opportunities
- In Phase 2: T004, T005, T006, T007 can all run in parallel (editing different `Cargo.toml` files).
- In Phase 3: T009 and T010 can run in parallel (editing different locale catalogs).
- In Phase 4: T017 and T018 can run in parallel (editing different locale catalogs).
- US1 and US2 can proceed in parallel once Phase 2 (Foundational) is complete.

---

## Parallel Example: User Story 1 & 2 Locales

```bash
# Clean up locale YAML files in parallel across crates/core and src-tauri:
Task: "Remove static about.APP_VERSION and about.COPYRIGHT from crates/core/locales/{en,ja}.yml"
Task: "Remove static about.APP_VERSION and about.COPYRIGHT from src-tauri/locales/{en,ja}.yml"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)
1. Complete Phase 1 (Setup) and Phase 2 (Foundational).
2. Complete Phase 3 (User Story 1 - Version minimization & Cargo.lock staging guarantee).
3. **STOP and VALIDATE**: Run `npm run test:sync-version` and simulate `npm version patch --no-git-tag-version`.
4. Verify MVP delivery.

### Incremental Delivery
1. Foundation: Cargo workspace inheritance + Tauri package reference (T001–T008).
2. Increment 1 (MVP): Minimal version sync script + `Cargo.lock` staging guarantee (T009–T015).
3. Increment 2: Dynamic display copyright consolidation (T016–T020).
4. Increment 3: Source header integrity verification (T021–T022).
5. Quality Gate: Workspace clippy, test, and build validation (T023–T025).
