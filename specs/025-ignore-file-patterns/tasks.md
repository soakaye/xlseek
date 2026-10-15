# Tasks: Ignore File Patterns with Gitignore-Style Matching and History

**Input**: Design documents from `specs/025-ignore-file-patterns/`  
**Prerequisites**: [plan.md](file:///F:/source/orca/workspaces/xlseek/.orca/workspaces/xlseek/ponyfish/specs/025-ignore-file-patterns/plan.md), [spec.md](file:///F:/source/orca/workspaces/xlseek/.orca/workspaces/xlseek/ponyfish/specs/025-ignore-file-patterns/spec.md), [data-model.md](file:///F:/source/orca/workspaces/xlseek/.orca/workspaces/xlseek/ponyfish/specs/025-ignore-file-patterns/data-model.md), [contracts/ignore-patterns-contract.md](file:///F:/source/orca/workspaces/xlseek/.orca/workspaces/xlseek/ponyfish/specs/025-ignore-file-patterns/contracts/ignore-patterns-contract.md), [research.md](file:///F:/source/orca/workspaces/xlseek/.orca/workspaces/xlseek/ponyfish/specs/025-ignore-file-patterns/research.md), [quickstart.md](file:///F:/source/orca/workspaces/xlseek/.orca/workspaces/xlseek/ponyfish/specs/025-ignore-file-patterns/quickstart.md)

**Tests**: Unit and integration tests are required per Constitution Principle V and Spec verification criteria.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `- [ ] [ID] [P?] [Story?] Description with exact file path`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: Which user story this task belongs to (e.g., [US1], [US2], [US3], [US4])

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization, dependency additions, centralized constants, and localization catalogs.

- [X] T001 Add `ignore = "0.4"` dependency to `crates/core/Cargo.toml`
- [X] T002 [P] Define centralized constants in `crates/core/src/constants.rs` (`DEFAULT_IGNORE_PATTERNS = "~$*"`, `SEARCH_QUERY_IGNORE_PATTERNS_FIELD`, `CLI_IGNORE_PATTERNS_FIELD = "ignore-patterns"`, `CLI_SHORT_IGNORE_PATTERNS = "-i"`)
- [X] T003 [P] Define centralized constants in `src/constants/index.ts` (`IGNORE_PATTERNS_CONSTANTS`, `STORAGE_KEY_HISTORY = "xlseek.ignorePatternHistory"`, `DEFAULT_PATTERNS = "~$*"`, `MAX_HISTORY_ENTRIES = 20`, `INPUT_ID`, `HISTORY_DROPDOWN_ID`, `SETTINGS_INPUT_ID`)
- [X] T004 [P] Add localized UI labels, tooltips, and placeholder strings in `crates/core/locales/ja.yml` and `crates/core/locales/en.yml` (`ui.IGNORE_PATTERNS_LABEL`, `ui.IGNORE_PATTERNS_PLACEHOLDER`, `ui.IGNORE_PATTERNS_CLEAR`, `ui.IGNORE_HISTORY_BUTTON`, `ui.SETTINGS_DEFAULT_IGNORE_PATTERNS_LABEL`, `ui.SETTINGS_DEFAULT_IGNORE_PATTERNS_HELP`)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core data model entities and gitignore evaluation domain engine that MUST be complete before user stories can proceed.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [X] T005 Extend `SearchQuery` in `crates/core/src/models/mod.rs` with `pub ignore_patterns: Option<String>` with serde default and doc comments
- [X] T006 [P] Extend `SearchQuery` interface in `src/types/search.ts` with `ignore_patterns?: string`
- [X] T007 Implement `IgnoreMatcher` domain struct and methods in `crates/core/src/search/ignore.rs` (wrapping `ignore::gitignore::GitignoreBuilder`, `from_patterns(root: &Path, raw_patterns: Option<&str>) -> Result<Self, CoreError>`, `is_ignored(&self, path: &Path, is_dir: bool) -> bool`, and `passthrough()`)
- [X] T008 [P] Add unit tests for `IgnoreMatcher` in `crates/core/src/search/ignore.rs` (testing `~$*` lock files, glob wildcards, trailing `/` directory pruning, `!` negation, and case-insensitive matching)

**Checkpoint**: Foundation ready — User Story 1 implementation can now begin.

---

## Phase 3: User Story 1 - Specify Comma-Separated Gitignore-Style Ignore Patterns (Priority: P1) 🎯 MVP

**Goal**: Allow users to supply comma-separated `.gitignore`-style wildcard expressions that prune directories early and filter out matching files before parsing in Core, Tauri IPC, and CLI.

**Independent Test**: Target a directory containing `Normal.xlsx`, `~$Normal.xlsx`, and `Archive/OldReport.xlsx`; run search with ignore pattern `~$*, Archive/`; verify `Normal.xlsx` matches while `~$Normal.xlsx` is skipped without error and `Archive/` directory is pruned early without descending.

### Implementation & Tests for User Story 1

- [X] T009 [P] [US1] Integrate `IgnoreMatcher` into directory traversal in `crates/core/src/search/discovery.rs` (early pruning of directories via `is_ignored(&entry_path, true)` and file skipping via `is_ignored(&entry_path, false)`)
- [X] T010 [P] [US1] Wire `ignore_patterns` from `SearchQuery` through search execution pipeline in `crates/core/src/search/engine.rs`
- [X] T011 [US1] Add `-i` and `--ignore-patterns` CLI argument parsing and validation in `crates/cli/src/args.rs` and forward to `SearchQuery`
- [X] T012 [P] [US1] Add integration tests for ignore patterns and directory pruning in `crates/core/tests/ignore_patterns_test.rs`
- [X] T013 [P] [US1] Add CLI integration tests for `-i` / `--ignore-patterns` option in `crates/cli/tests/cli_ignore_patterns.rs`
- [X] T014 [US1] Update Tauri IPC search command in `src-tauri/src/commands/search_cmd.rs` to accept and forward `ignore_patterns` to `SearchEngine`

**Checkpoint**: At this point, User Story 1 is fully functional and testable via CLI and Tauri backend commands.

---

## Phase 4: User Story 2 - UI Placement Adjacent to Search Directory (Priority: P2)

**Goal**: Provide a dedicated row for ignore patterns directly beneath the search directory input and above the file extension toggles in the search interface, with localized label, Filter icon, clear button, and Enter key search trigger.

**Independent Test**: Launch GUI; verify the dedicated ignore patterns row appears directly beneath the search directory input; enter `~$*` and press Enter to start search; verify clear button clears the field.

### Implementation & Tests for User Story 2

- [X] T015 [P] [US2] Implement comma-separated pattern tokenizer utility in `src/utils/ignoreTokenizer.ts` (with unit tests in `src/utils/__tests__/ignoreTokenizer.test.ts` for spacing, commas, and double quotes)
- [X] T016 [US2] Add ignore pattern state management and query dispatch in `src/hooks/useSearch.ts` (handling `ignorePatterns` input state, reset action, and clear button)
- [X] T017 [US2] Update `src/components/search/SearchBar.tsx` to render the dedicated ignore patterns row directly beneath the search directory row (with Filter icon, localized label, text input, clear button, and Enter key triggering search)
- [X] T018 [P] [US2] Update standalone HTML UI mockup in `design/mainui/index.html` to reflect the new ignore patterns row (keeping mockup in sync per AGENTS.md)

**Checkpoint**: At this point, User Stories 1 AND 2 work together seamlessly in the GUI.

---

## Phase 5: User Story 3 - Ignore Pattern History and Selection (Priority: P3)

**Goal**: Save successfully executed non-empty ignore pattern strings in `localStorage` (max 20 entries, MRU ordered, deduplicated) and provide an accessible dropdown for quick recall.

**Independent Test**: Enter `~$*, *.tmp`, execute search, verify item is saved to history; click history button or press ArrowDown to open dropdown, select item and confirm input field is populated.

### Implementation & Tests for User Story 3

- [X] T019 [P] [US3] Implement `IgnoreHistoryStore` in `src/utils/ignoreHistory.ts` (handling `localStorage` reading under `xlseek.ignorePatternHistory`, MRU deduplication, trimming, max 20 limit, and clearing)
- [X] T020 [P] [US3] Implement accessible dropdown component in `src/components/search/IgnoreHistoryDropdown.tsx` (supporting mouse clicks and keyboard navigation: ArrowUp, ArrowDown, Enter, Escape)
- [X] T021 [US3] Connect `IgnoreHistoryDropdown` and history recording to `src/components/search/SearchBar.tsx` and `src/hooks/useSearch.ts` (recording pattern on search start, toggling dropdown, selecting history items)
- [X] T022 [P] [US3] Add unit tests for `ignoreHistory.ts` in `src/utils/__tests__/ignoreHistory.test.ts` (testing MRU promotion, duplicate prevention, whitespace trimming, and 20-entry limit)

**Checkpoint**: At this point, User Stories 1, 2, and 3 are complete with persistent history recall.

---

## Phase 6: User Story 4 - Configure Default Ignore Patterns in Settings (Priority: P3)

**Goal**: Allow users to configure preferred default ignore patterns in the Settings dialog ("Saved Defaults" tab), persisting to `localStorage` and initializing new search sessions with the configured default.

**Independent Test**: Open Settings > Saved Defaults, modify default ignore patterns to `~$*, *.bak`, save settings and reload; verify search bar ignore pattern input initializes with `~$*, *.bak`.

### Implementation & Tests for User Story 4

- [X] T023 [P] [US4] Extend `DefaultSearchOptions` in `src/types/settings.ts` and `src/settings-core.ts` with `ignore_patterns: string` (default: `"~$*"`)
- [X] T024 [US4] Add "Default Ignore Patterns" input field in `src/components/settings/SettingsDialog.tsx` under the Saved Defaults tab with localized labels, help text, and validation
- [X] T025 [US4] Wire default ignore patterns initialization and reset from saved defaults in `src/App.tsx` and `src/hooks/useSearch.ts`
- [X] T026 [P] [US4] Add unit tests for settings persistence and default ignore pattern migration in `src/__tests__/settings-core.test.ts`

**Checkpoint**: All user stories (US1 through US4) are now fully implemented and testable.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Improvements, verification quality gates, formatting, and end-to-end scenario validation across all stories.

- [X] T027 [P] Verify zero compiler and Clippy warnings across workspace with `cargo clippy --workspace --all-targets -- -D warnings`
- [X] T028 [P] Verify code formatting across workspace with `cargo fmt --check`
- [X] T029 [P] Run frontend TypeScript type check and bundle build with `npm run build`
- [X] T030 Execute all workspace tests (`cargo test --workspace`) and validate end-to-end scenarios per `specs/025-ignore-file-patterns/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately.
- **Foundational (Phase 2)**: Depends on Setup completion — BLOCKS all user stories.
- **User Story 1 (Phase 3)**: Depends on Foundational completion. Delivers core MVP.
- **User Story 2 (Phase 4)**: Depends on Foundational and US1 core model completion.
- **User Story 3 (Phase 5)**: Depends on US2 search bar layout completion.
- **User Story 4 (Phase 6)**: Depends on US2 and US3 state binding.
- **Polish (Phase 7)**: Depends on all user stories being implemented.

### User Story Dependencies

- **User Story 1 (P1)**: Independent of frontend UI; can be validated via CLI and core unit tests.
- **User Story 2 (P2)**: Depends on `SearchQuery.ignore_patterns` from Foundational.
- **User Story 3 (P3)**: Builds on top of `SearchBar.tsx` created in US2.
- **User Story 4 (P3)**: Integrates with default search options and search bar initialization.

### Parallel Opportunities

- Within Phase 1: `T002`, `T003`, and `T004` can execute in parallel.
- Within Phase 2: `T006` and `T008` can execute in parallel with other foundational work.
- Within Phase 3 (US1): `T009`, `T010`, `T012`, and `T013` can be developed in parallel across different files.
- Within Phase 4 (US2): `T015` and `T018` can execute in parallel.
- Within Phase 5 (US3): `T019`, `T020`, and `T022` can execute in parallel.
- Within Phase 6 (US4): `T023` and `T026` can execute in parallel.
- Within Phase 7 (Polish): `T027`, `T028`, and `T029` can run concurrently.

---

## Parallel Example: User Story 1

```bash
# Launch core matcher traversal integration and CLI args in parallel:
Task: "Integrate IgnoreMatcher into directory traversal in crates/core/src/search/discovery.rs"
Task: "Add -i and --ignore-patterns CLI argument parsing and validation in crates/cli/src/args.rs"

# Launch core integration test and CLI integration test in parallel:
Task: "Add integration tests for ignore patterns and directory pruning in crates/core/tests/ignore_patterns_test.rs"
Task: "Add CLI integration tests for -i / --ignore-patterns option in crates/cli/tests/cli_ignore_patterns.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001 - T004).
2. Complete Phase 2: Foundational (T005 - T008) — ensures `IgnoreMatcher` is robust and unit-tested.
3. Complete Phase 3: User Story 1 (T009 - T014) — enables gitignore filtering in Core and CLI.
4. **STOP and VALIDATE**: Verify CLI `-i "~$*"` ignores temporary lock files and prunes directories.
5. Deliver/demonstrate MVP.

### Incremental Delivery

1. Setup + Foundational ready.
2. Deliver US1 (Core & CLI ignore filtering) -> MVP!
3. Deliver US2 (GUI dedicated ignore patterns input row directly beneath search directory).
4. Deliver US3 (Persistent ignore patterns history dropdown with keyboard navigation).
5. Deliver US4 (Settings dialog Saved Defaults integration for baseline ignore patterns).
6. Run Phase 7 Polish (zero Clippy warnings, clean fmt, passing `npm run build`, and `cargo test --workspace`).
