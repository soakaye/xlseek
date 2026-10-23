# Tasks: Explicit Sheet Tab Name in AI Copy Target Cell Metadata

**Input**: Design documents from `/specs/030-ai-copy-tab-name/` (`spec.md`, `plan.md`, `research.md`, `data-model.md`, `contracts/llm-context-contracts.md`, `quickstart.md`)  
**Prerequisites**: `plan.md` (required), `spec.md` (required), `research.md`, `data-model.md`, `contracts/`  
**Tests**: Automated unit tests in `crates/core/src/search/llm_context.rs` and `crates/core/src/constants.rs`  
**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `- [ ] [ID] [P?] [Story?] Description with file path`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: Which user story this task belongs to (`[US1]`, `[US2]`)
- Include exact file paths in descriptions

---

## Phase 1: Setup & Constants (Shared Infrastructure)

**Purpose**: Define centralized formatting template constants and unit test assertions per Constitution Principle II.

- [X] T001 [P] Define `LLM_TARGET_CELL_WITH_SHEET_FMT` (`"- Target Cell: [{}] {} (Column: {}, Row: {})"`) and `LLM_TARGET_CELL_WITHOUT_SHEET_FMT` (`"- Target Cell: {} (Column: {}, Row: {})"`) in `crates/core/src/constants.rs`
- [X] T002 [P] Add unit assertions for `LLM_TARGET_CELL_WITH_SHEET_FMT` and `LLM_TARGET_CELL_WITHOUT_SHEET_FMT` in `crates/core/src/constants.rs` test module

---

## Phase 2: Foundational (Formatting Helper)

**Purpose**: Core formatting helper function in `xlseek-core` that MUST be complete before user story integration.

**⚠️ CRITICAL**: Foundational helper function is shared by both Table and Header-Only context modes.

- [X] T003 Implement `format_target_cell_line(sheet_name: &str, cell_address: &str, column_header: &str, row_key: &str) -> String` with 3-part header comments, constant reference annotations, and empty/whitespace sheet name fallback in `crates/core/src/search/llm_context.rs`

**Checkpoint**: Helper function implemented and ready for integration into context builders.

---

## Phase 3: User Story 1 - Explicit Sheet Tab Name in AI Copy Output (Priority: P1) 🎯 MVP

**Goal**: Ensure `Target Cell` line in AI Copy table context explicitly identifies the worksheet tab name enclosed in brackets `[SheetName] CellAddress` (e.g. `- Target Cell: [Sheet1] B2 (Column: Status, Row: Item 1)`).

**Independent Test**: Generate table-mode LLM context for an Excel match with sheet name `Sheet1`, cell `B2`, and verify the output contains `- Target Cell: [Sheet1] B2 (Column: Status, Row: Item 1)`.

### Implementation & Tests for User Story 1

- [X] T004 [US1] Update `format_table_excel_context` to format target cell lines using `format_target_cell_line` in `crates/core/src/search/llm_context.rs`
- [X] T005 [US1] Add unit tests in `crates/core/src/search/llm_context.rs` verifying Table context mode produces `[SheetName] CellAddress` for standard sheets, correctly formats Japanese/special characters (e.g. `[FY2026 Q3 (Final)]`), and falls back to coordinate-only if sheet name is empty

**Checkpoint**: User Story 1 is fully functional and verifiable via `cargo test --package xlseek-core search::llm_context`.

---

## Phase 4: User Story 2 - Consistent Metadata Formatting Across Context Modes (Priority: P2)

**Goal**: Ensure Header-only copy mode produces identical `[SheetName] CellAddress` formatting while preserving existing non-spreadsheet (Word) document context.

**Independent Test**: Generate header-only LLM context and verify the `Target Cell` format matches Table mode, while Word document context continues to output section/paragraph metadata without sheet prefixes.

### Implementation & Tests for User Story 2

- [X] T006 [US2] Update `format_header_only_excel_context` to format target cell lines using `format_target_cell_line` in `crates/core/src/search/llm_context.rs`
- [X] T007 [US2] Add unit tests in `crates/core/src/search/llm_context.rs` verifying Header-only mode matches Table mode output format and confirming Word document LLM context generation remains unaffected

**Checkpoint**: Both Table and Header-only modes format Target Cell metadata consistently across all Excel match types.

---

## Phase 5: Polish & Quality Gates Verification

**Purpose**: Execute full automated test suite, static analysis, formatting checks, and frontend build validation across the workspace.

- [X] T008 [P] Run Rust unit tests for `xlseek-core` via `cargo test --package xlseek-core`
- [X] T009 [P] Run workspace-wide test suite via `cargo test --workspace`
- [X] T010 [P] Run Clippy linter with zero warnings via `cargo clippy --workspace --all-targets -- -D warnings`
- [X] T011 [P] Verify official Rust formatting compliance via `cargo fmt --check`
- [X] T012 [P] Verify frontend TypeScript compilation and Vite bundle via `npm run build`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup & Constants (Phase 1)**: Independent, can start immediately.
- **Foundational (Phase 2)**: Depends on Phase 1 constants; blocks all user stories.
- **User Story 1 (Phase 3 - P1)**: Depends on Phase 2; delivers MVP for Table context mode.
- **User Story 2 (Phase 4 - P2)**: Depends on Phase 2 & Phase 3; delivers uniform Header-only mode and Word preservation.
- **Polish & Quality Gates (Phase 5)**: Depends on completion of all user stories.

### User Story Dependencies

- **User Story 1 (P1)**: Requires `format_target_cell_line` from Foundational.
- **User Story 2 (P2)**: Integrates with `format_target_cell_line` and shares formatting invariants established in US1.

### Parallel Opportunities

- T001 and T002 can be implemented and validated together in `crates/core/src/constants.rs`.
- In Phase 5, verification gates T008, T009, T010, T011, and T012 can run independently.

---

## Parallel Example: Setup & Constants

```bash
# Define constants and test assertions in crates/core/src/constants.rs:
Task: T001 Define LLM_TARGET_CELL_WITH_SHEET_FMT and LLM_TARGET_CELL_WITHOUT_SHEET_FMT
Task: T002 Add unit assertions in constants test module
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)
1. Complete Phase 1 (Constants) and Phase 2 (Foundational helper).
2. Implement Phase 3 (User Story 1 - Table mode formatting & tests).
3. Validate independently with `cargo test --package xlseek-core search::llm_context`.

### Incremental Delivery
1. Foundation & Constants ready.
2. User Story 1 delivers explicit sheet tab names in default Table context copy.
3. User Story 2 extends explicit sheet tab names to Header-only copy and verifies Word document non-regression.
4. Phase 5 verifies all workspace quality gates (tests, clippy, fmt, build).
