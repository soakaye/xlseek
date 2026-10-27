# Tasks: File Search Patterns with Extension Complement and Sequential Filtering

**Input**: Design documents from `specs/037-file-search-patterns/`  
**Prerequisites**: [plan.md](file:///Users/soakaye/Develop/Projects/workspaces/exgrep/shrimpfish/specs/037-file-search-patterns/plan.md), [spec.md](file:///Users/soakaye/Develop/Projects/workspaces/exgrep/shrimpfish/specs/037-file-search-patterns/spec.md), [data-model.md](file:///Users/soakaye/Develop/Projects/workspaces/exgrep/shrimpfish/specs/037-file-search-patterns/data-model.md), [contracts/file-search-patterns-contract.md](file:///Users/soakaye/Develop/Projects/workspaces/exgrep/shrimpfish/specs/037-file-search-patterns/contracts/file-search-patterns-contract.md), [research.md](file:///Users/soakaye/Develop/Projects/workspaces/exgrep/shrimpfish/specs/037-file-search-patterns/research.md), [quickstart.md](file:///Users/soakaye/Develop/Projects/workspaces/exgrep/shrimpfish/specs/037-file-search-patterns/quickstart.md)

**Tests**: Unit and integration tests are required per Constitution Principle V and Spec verification criteria.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `- [ ] [ID] [P?] [Story?] Description with exact file path`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: Which user story this task belongs to (e.g., [US1], [US2], [US3], [US4], [US5])

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Centralized constants, localization catalogs, and shared project infrastructure.

- [ ] T001 Define centralized core constants in `crates/core/src/constants.rs` (`SEARCH_QUERY_FILE_PATTERNS_FIELD`, `CLI_FILE_PATTERNS_FIELD = "file_patterns"`, `CLI_SHORT_FILE_PATTERNS = "-g"`, `CLI_LONG_FILE_PATTERNS = "--file-patterns"`, `DEFAULT_FILE_PATTERNS = ""`, `STAGE_FILE_PATTERNS = "file_patterns"`, delimiter and quote constants)
- [ ] T002 [P] Define centralized frontend constants in `src/constants/index.ts` (`FILE_PATTERNS_CONSTANTS`, `STORAGE_KEY_FILE_PATTERN_HISTORY = "xlseek.filePatternHistory"`, `DEFAULT_FILE_PATTERNS = ""`, `MAX_FILE_PATTERN_HISTORY_ENTRIES = 20`, input IDs, dropdown IDs)
- [ ] T003 [P] Add localized UI labels, tooltips, and placeholder strings in `crates/core/locales/ja.yml` and `crates/core/locales/en.yml` (`ui.FILE_PATTERNS_LABEL`, `ui.FILE_PATTERNS_PLACEHOLDER`, `ui.FILE_PATTERNS_CLEAR`, `ui.FILE_PATTERN_HISTORY_BUTTON`, `ui.SETTINGS_DEFAULT_FILE_PATTERNS_LABEL`, `ui.SETTINGS_DEFAULT_FILE_PATTERNS_HELP`)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core data model entities and pattern matching domain engine that MUST be complete before user stories can proceed.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [ ] T004 Extend `SearchQuery` in `crates/core/src/models/mod.rs` with `pub file_patterns: Option<String>` with serde attributes and doc comments
- [ ] T005 [P] Extend `SearchQuery` interface in `src/types/search.ts` with `filePatterns?: string`
- [ ] T006 [P] Implement `FilePatternMatcher` domain module in `crates/core/src/search/pattern.rs` (`parse_file_patterns`, quote handling, `from_patterns`, `is_match`, `has_explicit_extension`, `complement_tokens`, and `passthrough`)
- [ ] T007 [P] Add unit tests for `FilePatternMatcher` in `crates/core/src/search/pattern.rs` (testing tokenization, extensionless complement, explicit extensions, case-insensitive matching, and malformed globs)

**Checkpoint**: Foundation ready — User Story 1 implementation can now begin.

---

## Phase 3: User Story 1 - Specify Target File Search Patterns with Multiple Inclusion Matching (Priority: P1) 🎯 MVP

**Goal**: Allow users to specify one or more comma-separated file search patterns (e.g. `*budget*, annual_*`) with logical OR inclusion matching. If empty or whitespace, defaults to matching all enabled extensions.

**Independent Test**: Target a directory containing `sample_budget.xlsx`, `sample_report.xlsx`, and `sample_inventory.xlsx`; run search with pattern `*budget*, *report*`; verify `sample_budget.xlsx` and `sample_report.xlsx` are scanned while `sample_inventory.xlsx` is skipped.

### Implementation & Tests for User Story 1

- [ ] T008 [P] [US1] Integrate `FilePatternMatcher` into directory traversal in `crates/core/src/search/discovery.rs` to filter candidate files against complemented inclusion patterns before file delivery
- [ ] T009 [P] [US1] Wire `file_patterns` from `SearchQuery` through search execution pipeline in `crates/core/src/search/engine.rs` into `discover_files_with_ignore`
- [ ] T010 [P] [US1] Update Tauri IPC search command in `src-tauri/src/commands/search_cmd.rs` to forward `file_patterns` from IPC payload into `SearchEngine`
- [ ] T011 [P] [US1] Add integration tests in `crates/core/tests/file_patterns_test.rs` validating multiple comma-separated inclusion patterns and default empty fallback

**Checkpoint**: At this point, User Story 1 is functional and testable via Tauri backend commands and unit tests.

---

## Phase 4: User Story 2 - Sequential Filtering: Apply Search Patterns First, Then Ignore Patterns (Priority: P1)

**Goal**: Ensure directory traversal evaluates candidate files in strict sequential order: candidate file is checked against `FilePatternMatcher` first, and only matching files are evaluated against `IgnoreMatcher`.

**Independent Test**: Target a directory containing `sample_budget.xlsx`, temporary lock file `~$sample_budget.xlsx`, and archive copy `archive/sample_budget.xlsx`; set search patterns to `*budget*` and ignore patterns to `~$*, archive/*`; verify only `sample_budget.xlsx` is searched.

### Implementation & Tests for User Story 2

- [ ] T012 [US2] Update directory traversal loop in `crates/core/src/search/discovery.rs` to enforce strict sequential order: evaluate file against `FilePatternMatcher` first, and only matching candidates are tested against `IgnoreMatcher`
- [ ] T013 [P] [US2] Add integration tests in `crates/core/tests/sequential_pattern_filter_test.rs` verifying that candidate files matching include patterns but also matching ignore patterns (e.g. `~$*` or `archive/*`) are excluded

**Checkpoint**: At this point, User Stories 1 AND 2 are complete with verified sequential filtering.

---

## Phase 5: User Story 3 - Automatic Extension Complement for Patterns Lacking Extensions (Priority: P2)

**Goal**: Automatically complement input pattern tokens lacking extensions with all currently enabled file extension options, while preserving explicit extensions and granting explicit extensions precedence over unchecked toggle options for supported formats.

**Independent Test**: With `.xlsx` and `.xlsm` enabled, enter `budget*` (no extension); verify `budget_2024.xlsx` and `budget_archive.xlsm` match; enter explicit `*.xls` when `.xls` toggle is unchecked; verify `legacy.xls` matches.

### Implementation & Tests for User Story 3

- [ ] T014 [US3] Implement `has_explicit_extension` and `complement_tokens` in `crates/core/src/search/pattern.rs` supporting `.xlsx`, `.xlsm`, `.xls`, `.xlsb`, `.docx`, `.docm`, `.doc`, preserving explicit extensions and expanding extensionless tokens with active options
- [ ] T015 [US3] Enforce explicit extension precedence in `crates/core/src/search/pattern.rs` and `crates/core/src/search/discovery.rs` so explicit extensions in search patterns match supported formats even if unchecked in options toggles
- [ ] T016 [P] [US3] Add unit and integration tests in `crates/core/tests/extension_complement_test.rs` verifying extensionless pattern complement, explicit extension preservation, and override behavior

**Checkpoint**: At this point, User Stories 1, 2, and 3 provide robust pattern matching and extension complement.

---

## Phase 6: User Story 4 - Search Interface Integration, History Selection, and CLI/MCP Support (Priority: P2)

**Goal**: Provide a dedicated row for File Search Patterns between Search Directory and Ignore Patterns (`Directory` → `File Search Patterns` → `Ignore Patterns`), LRU history dropdown (up to 20 entries), CLI flags (`-g, --file-patterns`), and MCP tool `search` with `file_patterns` parameter.

**Independent Test**: Launch GUI; verify dedicated File Search Patterns row appears between Directory and Ignore Patterns; enter pattern, execute search, verify item is added to history; verify CLI search works with `-g "*budget*"`; verify MCP tool works with `file_patterns`.

### Implementation & Tests for User Story 4

- [ ] T017 [P] [US4] Implement `FilePatternHistoryStore` in `src/utils/filePatternHistory.ts` with `localStorage` persistence under `xlseek.filePatternHistory`, LRU deduplication, trimming, and 20-entry limit
- [ ] T018 [P] [US4] Implement `FilePatternHistoryDropdown.tsx` in `src/components/search/FilePatternHistoryDropdown.tsx` with mouse and keyboard navigation (ArrowUp, ArrowDown, Enter, Escape)
- [ ] T019 [US4] Update search state and query dispatch in `src/hooks/useSearch.ts` and `src/App.tsx` to manage `filePatterns` state, history recording on search execution, and clear/reset actions
- [ ] T020 [US4] Update `src/components/search/SearchBar.tsx` to render the dedicated File Search Patterns row directly between Search Directory and Ignore Patterns (`Directory` → `File Search Patterns` → `Ignore Patterns`), including icon, placeholder, clear button, and history dropdown
- [ ] T021 [P] [US4] Add `-g` and `--file-patterns` CLI argument parsing in `crates/cli/src/args.rs` and forward to `SearchQuery`
- [ ] T022 [P] [US4] Add CLI integration tests in `crates/cli/tests/cli_file_patterns.rs` verifying `-g` and `--file-patterns` execution
- [ ] T023 [P] [US4] Add `file_patterns` parameter to MCP `search` tool JSON schema and argument extraction in `crates/mcp/src/tools/search.rs`
- [ ] T024 [P] [US4] Update standalone HTML prototype in `design/mainui/index.html` to reflect the new File Search Patterns row per AGENTS.md rule

**Checkpoint**: At this point, User Stories 1 through 4 are complete across GUI, CLI, and MCP.

---

## Phase 7: User Story 5 - Configure Default File Search Patterns in Settings (Priority: P3)

**Goal**: Allow users to configure preferred default file search patterns in the Settings dialog (Saved Defaults tab), persisting to `localStorage` and initializing new search sessions with the configured default.

**Independent Test**: Open Settings > Saved Defaults, enter `*budget*` in Default File Search Patterns, save settings and reset search bar; verify File Search Patterns input field initializes with `*budget*`.

### Implementation & Tests for User Story 5

- [ ] T025 [US5] Add `defaultFilePatterns: string` to `SearchOptionsConfig` in `src/settings-core.ts` with default value `""`
- [ ] T026 [US5] Add "Default File Search Patterns" text input field to `src/components/settings/SavedDefaultsTab.tsx` with localized label and help text
- [ ] T027 [US5] Wire default file search patterns initialization on app startup and search bar reset in `src/hooks/useSearch.ts`
- [ ] T028 [P] [US5] Add frontend unit tests in `src/utils/__tests__/filePatternHistory.test.ts` for history management and settings default loading

**Checkpoint**: All 5 user stories are complete and integrated.

---

## Phase 8: Polish & Cross-Cutting Verification

**Purpose**: Documentation updates, quality gate verification, and end-to-end testing across all crates.

- [ ] T029 [P] Update documentation in `crates/cli/README.md` and `README.md` describing file search patterns, `-g`, `--file-patterns`, and sequential filtering
- [ ] T030 Execute synthetic quickstart validation scenarios from `specs/037-file-search-patterns/quickstart.md`
- [ ] T031 [P] Run Rust test suite: `cargo test --workspace`
- [ ] T032 [P] Run Rust Clippy static analysis: `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] T033 [P] Run Rust format check: `cargo fmt --check`
- [ ] T034 [P] Run Frontend build & typecheck: `npm run build`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately.
- **Foundational (Phase 2)**: Depends on Phase 1 completion — BLOCKS all user stories.
- **User Story 1 (Phase 3)**: Depends on Phase 2 completion — delivers core MVP search pattern filtering.
- **User Story 2 (Phase 4)**: Depends on Phase 3 completion — enforces sequential evaluation pipeline.
- **User Story 3 (Phase 5)**: Depends on Phase 3 and Phase 4 completion — implements extension complement.
- **User Story 4 (Phase 6)**: Depends on Phase 3, 4, and 5 completion — provides UI, CLI, and MCP surfaces.
- **User Story 5 (Phase 7)**: Depends on Phase 6 completion — adds settings persistence for default patterns.
- **Polish (Phase 8)**: Depends on all user stories being complete.

### Parallel Opportunities

- Phase 1: `T002`, `T003` can run in parallel with `T001`.
- Phase 2: `T005`, `T006`, `T007` can run in parallel once `T004` is defined.
- Phase 3: `T008`, `T009`, `T010`, `T011` can be implemented and tested concurrently across their respective files.
- Phase 6: Frontend history (`T017`, `T018`), CLI parsing (`T021`, `T022`), MCP tooling (`T023`), and design mockup (`T024`) can run completely in parallel.
- Phase 8: All quality gates (`T031`, `T032`, `T033`, `T034`) can run in parallel.

---

## Implementation Strategy

### MVP First (Phases 1, 2, and 3)
1. Complete Phase 1 (Setup: constants and i18n catalogs).
2. Complete Phase 2 (Foundational: `SearchQuery.file_patterns` and `FilePatternMatcher`).
3. Complete Phase 3 (User Story 1: integrate into `discovery.rs` and `engine.rs`).
4. **Checkpoint**: Verify core include filtering with `cargo test -p xlseek_core`.

### Incremental Delivery
1. Add Sequential Filtering (Phase 4, US2) → Verify include-then-ignore ordering.
2. Add Extension Complement (Phase 5, US3) → Verify extensionless and explicit extension handling.
3. Add UI, CLI, and MCP (Phase 6, US4) → Complete end-to-end interface integration.
4. Add Settings Defaults (Phase 7, US5) → Complete user customization experience.
5. Polish & Verification (Phase 8) → Verify all 4 quality gates pass cleanly.
