# Tasks: Search Path Normalization and Subpath Deduplication

**Input**: Design documents from `/specs/026-dedup-search-paths/`
**Prerequisites**: [plan.md](./plan.md) (required), [spec.md](./spec.md) (required for user stories), [research.md](./research.md), [data-model.md](./data-model.md), [contracts/path-normalization.md](./contracts/path-normalization.md), [quickstart.md](./quickstart.md)

**Tests**: Included per Project Constitution Principle V (all new or modified modules/functions MUST include unit and integration tests).

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Define shared constants in `crates/core/src/constants.rs` required for path normalization and home directory expansion without hardcoded literals (Constitution Principle II).

- [x] T001 Define exact home shorthand constant (`HOME_PATH_SHORTHAND_EXACT = "~"`) and any path normalization helper constants in `crates/core/src/constants.rs` with unit test verification in `crates/core/src/constants.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core path normalization and subpath deduplication primitives in `crates/core/src/search/path.rs` that all user stories depend on.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [x] T002 Implement `normalize_canonical_path(path: &Path) -> PathBuf` (resolving `std::fs::canonicalize` with lexical `.` / `..` fallback and stripping Windows verbatim prefixes `\\?\` and `\\?\UNC\` via `crate::export::normalize_export_path`) and OS home directory fallback helper in `crates/core/src/search/path.rs`
- [x] T003 Implement `deduplicate_and_prune_subpaths(paths: Vec<PathBuf>) -> Vec<PathBuf>` in `crates/core/src/search/path.rs` enforcing exact duplicate removal and order-independent component-boundary subpath pruning (`!child.starts_with(ancestor)` when `ancestor.is_dir()`)

**Checkpoint**: Foundation ready - user story implementation and verification can now proceed.

---

## Phase 3: User Story 1 - Prevent Redundant Scanning of Nested Subdirectories (Priority: P1) 🎯 MVP

**Goal**: Normalize specified target directories and exclude any directory that is a subpath of another specified directory (in any input order) so overlapping directory trees are traversed and scanned only once without false-positive pruning on sibling prefix collisions.

**Independent Test**: Specify both a parent directory and a nested subdirectory (e.g., `/data, /data/sub` or `/data/sub, /data`) in `resolve_and_partition_paths` and `SearchEngine::execute_search`, verifying that only `/data` is retained in `valid_roots` and each file under `/data/sub` is scanned once.

### Tests for User Story 1

- [x] T004 [P] [US1] Add unit tests in `crates/core/src/search/path.rs` covering parent-then-child, child-then-parent, deeply nested (`/a`, `/a/b`, `/a/b/c`), sibling prefix collision (`/data/report` vs `/data/report_archive`), and invalid subpath warning preservation (`/valid/parent, /valid/parent/missing`)
- [x] T005 [P] [US1] Add integration test in `crates/core/src/search/engine.rs` verifying that `SearchEngine::execute_search` with overlapping parent and subdirectory paths (`<fixtures>, <fixtures>/subdir`) scans each eligible file once (`scanned_files` and `matches_found` equal single-root scan) and emits clean paths without `\\?\`

### Implementation for User Story 1

- [x] T006 [US1] Update `resolve_and_partition_paths` in `crates/core/src/search/path.rs` to validate existence/readability first (preserving `invalid_paths` diagnostics), normalize valid roots with `normalize_canonical_path`, and prune duplicates and subpaths via `deduplicate_and_prune_subpaths`

**Checkpoint**: User Story 1 is fully functional and independently testable.

---

## Phase 4: User Story 2 - Normalize Equivalent and Non-Canonical Path Representations (Priority: P2)

**Goal**: Normalize paths written in different syntactic forms (relative `.` / `..` segments, trailing slashes, `~` home directory shorthand, mixed `/` and `\` separators, and symlinks) before comparing them for equality and subpath pruning across both `path.rs` and `engine.rs`.

**Independent Test**: Provide equivalent or parent/child paths using relative segments (`/dir/sub/..` and `/dir/sub`), trailing slashes, and `~/...` shorthand alongside explicit home paths; verify they normalize and deduplicate to a single clean canonical root.

### Tests for User Story 2

- [x] T007 [P] [US2] Add unit tests in `crates/core/src/search/path.rs` for relative segments (`.`, `..`), trailing separators, home shorthand (`~`, `~/sub`, `~\sub`) vs explicit home subdirectory paths, and clean human-readable output formatting (no `\\?\` prefix)

### Implementation for User Story 2

- [x] T008 [US2] Update `resolve_search_path` and `resolve_and_partition_paths` in `crates/core/src/search/path.rs` and `execute_search_with_issues` in `crates/core/src/search/engine.rs` to support exact `~` and OS environment home directory fallback (`HOME` / `USERPROFILE`) when `home_dir` is `None`

**Checkpoint**: User Stories 1 and 2 work independently and together.

---

## Phase 5: User Story 3 - Consistent Deduplication Across CLI and Individual File Targets (Priority: P3)

**Goal**: Apply the same canonical normalization and subpath deduplication to CLI searches (whether specified via `-p`/`--path` comma-separated tokens, multiple positional arguments, or mixed directory and file paths).

**Independent Test**: Run `parse_args` and CLI integration tests with overlapping positional directory arguments (`xlseek query /dir /dir/sub`) and mixed directory + descendant file arguments (`xlseek query /dir /dir/sub/file.xlsx`); verify that redundant subpaths/files are pruned from `CliOptions.input_paths` and each file is scanned only once.

### Tests for User Story 3

- [x] T009 [P] [US3] Add unit tests in `crates/cli/src/args.rs` verifying that overlapping positional directories, comma-separated `--path` directories, and a directory paired with a descendant file path are normalized (without `\\?\`) and pruned to top-level roots in `CliOptions.input_paths`
- [x] T010 [P] [US3] Add CLI integration test in `crates/cli/tests/cli_search.rs` verifying that running the CLI with overlapping parent and child paths produces zero duplicate matches and accurate `scanned_files` counts

### Implementation for User Story 3

- [x] T011 [US3] Update `resolve_input_path` and `parse_args` in `crates/cli/src/args.rs` to normalize input paths via `xlseek_core::search::path::normalize_canonical_path`, support comma-separated tokens in positional path arguments via `parse_search_paths`, and prune duplicates and descendant subpaths/files via `xlseek_core::search::path::deduplicate_and_prune_subpaths`

**Checkpoint**: All user stories (US1, US2, US3) are independently functional and verified.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final quality gate verification across the entire workspace in accordance with `AGENTS.md` and the Project Constitution.

- [x] T012 Run all required workspace verification commands (`npm run build`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`) and confirm zero errors or warnings

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - starts immediately (`T001`).
- **Foundational (Phase 2)**: Depends on Setup (`T001`) - BLOCKS all user stories (`T002`, `T003`).
- **User Stories (Phase 3–5)**:
  - **US1 (Phase 3)**: Depends on Phase 2 (`T002`, `T003`).
  - **US2 (Phase 4)**: Depends on Phase 2 (`T002`, `T003`) and integrates with `resolve_and_partition_paths` (`T006`).
  - **US3 (Phase 5)**: Depends on Phase 2 (`T002`, `T003`) — can run in parallel with US1/US2 since it modifies `crates/cli/`.
- **Polish (Phase 6)**: Depends on completion of Phases 3, 4, and 5 (`T012`).

### Parallel Opportunities

- `T004` (`path.rs` unit tests) and `T005` (`engine.rs` integration test) can be developed in parallel.
- Phase 5 (`US3` in `crates/cli/src/args.rs` and `crates/cli/tests/cli_search.rs`) touches separate files from `crates/core/` and can be executed in parallel once Phase 2 (`T002`, `T003`) is complete.

---

## Parallel Example: User Story 1 & User Story 3

```bash
# After Phase 2 (T002, T003) completes:
Task T005: "Add integration test in crates/core/src/search/engine.rs"
Task T009: "Add unit tests in crates/cli/src/args.rs"
Task T010: "Add CLI integration test in crates/cli/tests/cli_search.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (`T001`)
2. Complete Phase 2: Foundational (`T002`, `T003`)
3. Complete Phase 3: User Story 1 (`T004`, `T005`, `T006`)
4. **STOP and VALIDATE**: Run `cargo test -p xlseek-core` to confirm nested directories are pruned and never scanned twice.

### Incremental Delivery

1. Setup + Foundational → Core normalization & pruning helpers ready.
2. Add User Story 1 → Parent/child subpath pruning and clean path formatting in Core/GUI.
3. Add User Story 2 → Home shorthand fallback (`~`) and relative segment normalization across Core/Engine.
4. Add User Story 3 → Unified normalization and subpath pruning in CLI (`args.rs` + `cli_search.rs`).
5. Run Phase 6 Quality Gates → Full workspace verification.
