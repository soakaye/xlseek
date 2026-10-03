# Tasks: Multiple Search Target Paths with Quoted Delimitation

**Feature**: `022-multiple-search-paths`  
**Plan**: [specs/022-multiple-search-paths/plan.md](file:///Users/soakaye/Develop/Projects/exgrep/specs/022-multiple-search-paths/plan.md)  
**Spec**: [specs/022-multiple-search-paths/spec.md](file:///Users/soakaye/Develop/Projects/exgrep/specs/022-multiple-search-paths/spec.md)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Define centralized constants and error definitions across backend and frontend layers.

- [X] T001 Define multi-path delimiters, quotes, and error string constants in `crates/core/src/constants.rs`
- [X] T002 [P] Define GUI multi-path error codes and event constants in `src-tauri/src/constants.rs`
- [X] T003 [P] Define frontend delimiter and quote constants in `src/constants/index.ts`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core tokenizer and path resolution functions that all user stories depend upon.

- [X] T004 Implement `PathParseError` enum (`EmptyInput`, `UnclosedQuote`) and `parse_search_paths(input: &str) -> Result<Vec<String>, PathParseError>` in `crates/core/src/search/path.rs`
- [X] T005 Implement unit tests for `parse_search_paths` covering commas, whitespace trimming, and quotes in `crates/core/src/search/path.rs`
- [X] T006 Implement `PathResolutionResult` and `resolve_and_partition_paths(paths: &[String], home_dir: Option<&Path>) -> PathResolutionResult` with path deduplication in `crates/core/src/search/path.rs`
- [X] T007 Implement unit tests for `resolve_and_partition_paths` in `crates/core/src/search/path.rs`

**Checkpoint**: Foundation ready - Core tokenizer and multi-path resolution tested and ready.

---

## Phase 3: User Story 1 - Multiple Comma-Separated Search Paths (Priority: P1) 🎯 MVP

**Goal**: Allow searching across multiple directories specified as a comma-separated list in both CLI and backend search engine.

**Independent Test**: Provide two valid directory paths separated by a comma (e.g., `/path/dirA, /path/dirB`) and run a search; results must combine matching items from both locations.

### Implementation for User Story 1

- [X] T008 [US1] Update `SearchEngine::execute_search_with_issues` in `crates/core/src/search/engine.rs` to parse multiple paths via `parse_search_paths`, validate roots, and pass all valid roots to `discover_workbooks`
- [X] T009 [US1] Add integration test for multi-directory search scanning in `crates/core/tests/multi_path_search.rs`
- [X] T010 [US1] Update CLI argument parser in `crates/cli/src/args.rs` to parse comma-delimited strings in `-p`/`--path` using `parse_search_paths`
- [X] T011 [US1] Add integration tests in `crates/cli/tests/cli_search.rs` verifying search execution with multiple comma-separated paths in `-p` and positional arguments

**Checkpoint**: At this point, User Story 1 (MVP) is fully functional and testable via Core and CLI.

---

## Phase 4: User Story 2 - Quoted Paths with Spaces and Commas (Priority: P2)

**Goal**: Support paths containing commas or whitespace enclosed in double quotation marks without splitting on internal delimiters.

**Independent Test**: Enter a path containing a comma wrapped in double quotes alongside an unquoted path (e.g. `"/tmp/test, dir", /tmp/docs`) and verify that exactly two distinct paths are resolved.

### Implementation for User Story 2

- [X] T012 [US2] Enhance `parse_search_paths` in `crates/core/src/search/path.rs` to handle escaped quotes (`""` and `\"`) and preserve internal commas and spaces
- [X] T013 [US2] Add unit test cases for escaped and nested quotation marks in `crates/core/src/search/path.rs`
- [X] T014 [US2] Create frontend tokenizer utility `src/utils/pathTokenizer.ts` with `splitSearchPaths`, `appendSearchPath`, and `getActivePathSegment`
- [X] T015 [US2] Add unit tests for `src/utils/pathTokenizer.ts` in `src/utils/__tests__/pathTokenizer.test.ts`
- [X] T016 [US2] Update folder selection dialog and drag-and-drop drop handler in `src/components/search/SearchBar.tsx` using `appendSearchPath` so newly selected or dropped folders append with quotes when needed

**Checkpoint**: Quoted paths with spaces and commas are supported across backend and GUI.

---

## Phase 5: User Story 3 - Graceful Handling of Mixed Valid and Invalid Paths (Priority: P3)

**Goal**: When multiple paths are specified, report non-fatal diagnostics for unreachable paths and proceed scanning valid paths, aborting only if all paths are invalid.

**Independent Test**: Provide one valid path and one non-existent path separated by comma (e.g. `/valid/path, /does_not_exist`). Valid path is searched and a non-fatal warning is displayed for the invalid path.

### Implementation for User Story 3

- [X] T017 [US3] Update `start_search` in `src-tauri/src/commands/search_cmd.rs` to partition paths via `resolve_and_partition_paths`, emit `search-issue` events for invalid paths, and proceed if at least one valid path exists
- [X] T018 [US3] Update `useSearch.ts` in `src/hooks/useSearch.ts` to listen for non-fatal path warning issues and display toast notifications
- [X] T019 [US3] Update inline directory auto-completion in `src/components/search/SearchBar.tsx` and `src-tauri/src/commands/search_cmd.rs` to complete only the active path segment under cursor via `getActivePathSegment`
- [X] T020 [US3] Update search history in `src/components/search/SearchBar.tsx` to preserve the complete multi-path query string

**Checkpoint**: All user stories are implemented with robust error handling and feedback.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Quality gates, formatting, and end-to-end verification.

- [X] T021 [P] Ensure all new and modified functions have comprehensive header comments per Constitution Principle III
- [X] T022 [P] Verify zero hardcoded string or number constants across all touched files per Constitution Principle II
- [X] T023 Run `cargo fmt --check` and `cargo clippy --workspace --all-targets -- -D warnings`
- [X] T024 Run `npm run build` to verify zero TypeScript errors and successful Vite build
- [X] T025 Run full workspace test suite `cargo test --workspace`
- [X] T026 Execute quickstart validation scenarios defined in `specs/022-multiple-search-paths/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies
- **Setup (Phase 1)**: Independent, can start immediately.
- **Foundational (Phase 2)**: Depends on Phase 1; blocks User Stories.
- **User Story 1 (Phase 3 - P1)**: Depends on Phase 2. Core multi-path engine & CLI.
- **User Story 2 (Phase 4 - P2)**: Depends on Phase 2 & 3. Quoting support & GUI folder appending.
- **User Story 3 (Phase 5 - P3)**: Depends on Phase 3 & 4. Partial failure tolerance & cursor auto-complete.
- **Polish (Phase 6)**: Depends on all user stories completed.

### Parallel Opportunities
- Setup tasks T002 and T003 can run in parallel with T001.
- Unit tests in T005 can be developed alongside T004.
- Frontend tokenizer utility T014 and tests T015 can be developed in parallel with backend tasks.
- Polish tasks T021 and T022 can run in parallel.

---

## Implementation Strategy

### MVP First (User Story 1 Only)
1. Complete Phase 1 (Setup) and Phase 2 (Foundational).
2. Complete Phase 3 (User Story 1: Core engine and CLI multi-path search).
3. Validate: `cargo test -p xlseek-core` and `cargo run -p xlseek-cli -- "test" -p "dirA, dirB"`.
4. Result: Functional MVP for multi-directory search.

### Incremental Delivery
1. Foundation + US1 -> Deliver core multi-path capability (MVP).
2. Add US2 -> Deliver quoted path support with spaces/commas and GUI append mechanics.
3. Add US3 -> Deliver non-fatal partial path handling, segment autocomplete, and warning toasts.
4. Polish -> Verify all constitution quality gates (`cargo clippy`, `cargo fmt`, `npm run build`).
