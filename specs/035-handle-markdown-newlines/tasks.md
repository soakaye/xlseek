---
description: "Task list for multiline text handling in Markdown and CSV search results export"
---

# Tasks: Handle Multiline Text in Markdown and CSV Search Results Export

**Input**: Design documents from `/specs/035-handle-markdown-newlines/`  
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/  

## Format: `[ID] [P?] [Story] Description with file path`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (`[US1]`, `[US2]`, `[US3]`, `[US4]`)
- Every task includes explicit file paths

---

## Phase 1: Setup (Shared Constants & Infrastructure)

**Purpose**: Define centralized constants and unit tests conforming to Constitution Principle II

- [ ] T001 Define Markdown line break constant MD_LINE_BREAK and string constants in crates/core/src/constants.rs
- [ ] T002 [P] Add unit tests for new export constants in crates/core/src/constants.rs

---

## Phase 2: Foundational (Core Sanitization Functions)

**Purpose**: Core sanitization primitives that MUST be complete before user story integration

**⚠️ CRITICAL**: No user story integration work can begin until this phase is complete

- [ ] T003 Implement escape_markdown_cell with newline-to-break conversion, pipe escaping, and outer trimming in crates/core/src/search/llm_context.rs
- [ ] T004 [P] Implement sanitize_csv_field with newline-to-space conversion and outer trimming in crates/core/src/export/csv_export.rs

**Checkpoint**: Core string sanitization primitives ready - story integration can begin

---

## Phase 3: User Story 1 - Preserve Multiline Content in Markdown Table Export (Priority: P1) 🎯 MVP

**Goal**: Preserve multiline cell content as `<br>` line breaks inside Markdown tables without breaking table rows or column alignment.

**Independent Test**: Search a workbook containing multiline cells (`"Line 1\nLine 2"`), export to Markdown, and verify that the table row is intact on a single line with `Line 1<br>Line 2`.

- [ ] T005 [P] [US1] Add unit tests for escape_markdown_cell verifying newline conversion, pipe escaping, and multiline preservation in crates/core/src/search/llm_context.rs
- [ ] T006 [US1] Integrate escape_markdown_cell into format_table_excel_context in crates/core/src/search/llm_context.rs
- [ ] T007 [P] [US1] Add integration test for multiline cell table export in crates/core/tests/export_md_test.rs

**Checkpoint**: User Story 1 complete and independently testable. Markdown table exports retain multiline structure.

---

## Phase 4: User Story 2 - Single-Line Guarantee in CSV Export (Priority: P2)

**Goal**: Replace newlines in CSV export fields with single spaces so that each search match strictly occupies exactly 1 physical line (`total_lines = 1 + match_count`).

**Independent Test**: Search a workbook containing multiline cells, export to CSV or stream via CLI stdout, and verify that `wc -l` outputs exactly `match_count + 1`.

- [ ] T008 [P] [US2] Add unit tests for sanitize_csv_field verifying newline-to-space conversion and outer trimming in crates/core/src/export/csv_export.rs
- [ ] T009 [US2] Apply sanitize_csv_field to full_content, formula, and shape_name in write_csv_to_writer in crates/core/src/export/csv_export.rs
- [ ] T010 [P] [US2] Add integration test verifying N matches yield exactly N+1 lines in crates/core/tests/export_csv_test.rs
- [ ] T011 [P] [US2] Add CLI integration test verifying stdout CSV streaming with multiline cells in crates/cli/tests/cli_search.rs

**Checkpoint**: User Stories 1 and 2 complete. CSV exports strictly enforce 1 record per line.

---

## Phase 5: User Story 3 - Clean Multiline Formatting in Header & Cell Only / List Mode (Priority: P3)

**Goal**: Sanitize multiline values in Header & Cell Only list mode so that bullet list indentation and bold markup tags are not broken across physical lines.

**Independent Test**: Export or copy in Header & Cell Only mode with multiline values and verify `- Matched Value: **Line 1<br>Line 2**` maintains bullet list structure.

- [ ] T012 [US3] Sanitize display_val using escape_markdown_cell in format_header_only_excel_context in crates/core/src/search/llm_context.rs
- [ ] T013 [P] [US3] Add integration test for multiline values in Header & Cell Only mode in crates/core/tests/export_md_test.rs

**Checkpoint**: User Stories 1, 2, and 3 complete. Both Markdown table and list exports format multiline text cleanly.

---

## Phase 6: User Story 4 - Line Ending Normalization and Whitespace Hygiene (Priority: P4)

**Goal**: Normalize CRLF, CR, and LF line endings and trim outer whitespace across all document formats (Excel, Shape, Word) while preserving internal consecutive newlines in Markdown.

**Independent Test**: Search a workbook with cells containing `"\n\nHeader\n\nValue\n\n"` and verify output becomes `"Header<br><br>Value"` in Markdown and `"Header  Value"` in CSV.

- [ ] T014 [P] [US4] Add edge case unit tests for CRLF, CR, LF, consecutive newlines, and whitespace-only cells in crates/core/src/search/llm_context.rs
- [ ] T015 [P] [US4] Add edge case unit tests for CRLF, CR, LF, and whitespace trimming in crates/core/src/export/csv_export.rs

**Checkpoint**: All user stories complete and edge cases verified.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Quality gate verification, static analysis, and regression testing across workspace

- [ ] T016 Run cargo clippy and fix any warnings across workspace in crates/core and crates/cli
- [ ] T017 [P] Run cargo fmt check across workspace
- [ ] T018 [P] Run npm test and npm run build to verify frontend and CLI integration
- [ ] T019 Run quickstart verification scenarios per specs/035-handle-markdown-newlines/quickstart.md

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately.
- **Foundational (Phase 2)**: Depends on Setup (Phase 1) - BLOCKS all user stories.
- **User Stories (Phase 3+)**: All depend on Foundational phase completion.
  - User Story 1 (P1, Markdown Tables) and User Story 2 (P2, CSV Single-Line) can be implemented independently or sequentially.
  - User Story 3 (P3, Header & Cell Only) builds on User Story 1 Markdown primitives.
  - User Story 4 (P4, Edge cases) verifies line-ending combinations across US1 and US2.
- **Polish (Phase 7)**: Depends on all user stories being complete.

### User Story Dependencies

- **User Story 1 (P1)**: Depends on T001, T003. No dependency on other user stories.
- **User Story 2 (P2)**: Depends on T004. No dependency on Markdown stories.
- **User Story 3 (P3)**: Depends on T003, T006. Reuses Markdown sanitization in list mode.
- **User Story 4 (P4)**: Depends on T003, T004. Verifies mixed line endings across both pipelines.

---

## Parallel Opportunities

- T001 and T002 can proceed in parallel.
- T003 (Markdown) and T004 (CSV) can proceed in parallel once constants are defined.
- Phase 3 (US1) and Phase 4 (US2) operate on completely separate files (`llm_context.rs` vs `csv_export.rs`) and can be implemented in parallel.
- Integration tests T007, T010, T011, T013, T014, T015 can run in parallel.
- Polish tasks T016, T017, T018 can run in parallel.

---

## Implementation Strategy

### MVP First (User Story 1 Only)
1. Complete Phase 1: Setup (`MD_LINE_BREAK` constant).
2. Complete Phase 2: Foundational (`escape_markdown_cell` core).
3. Complete Phase 3: User Story 1 (Markdown Table multiline support).
4. Run `cargo test -p xlseek-core --test export_md_test` to validate MVP independently.

### Incremental Delivery
1. Setup + Foundational -> Foundation ready.
2. US1 -> Markdown Table multiline support delivered (MVP).
3. US2 -> CSV 1-line guarantee delivered.
4. US3 -> Header & Cell Only list mode delivered.
5. US4 -> Edge cases (CRLF/CR/LF, consecutive newlines, trimming) verified.
6. Polish -> Workspace clippy, fmt, and full test suite verification.
