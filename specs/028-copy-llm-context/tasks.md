# Tasks: Context-Aware Header Extraction and Prompt Copy for LLM

**Feature Branch**: `028-copy-llm-context` | **Date**: 2026-10-19 | **Spec**: [spec.md](./spec.md) | **Plan**: [plan.md](./plan.md)

This document defines the actionable, dependency-ordered tasks required to implement Feature 028, structured by user story to enable independent implementation and testing.

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Centralize constants, command identifiers, and localization resources across all crates and frontend layers.

- [X] T001 [P] Define centralized constants for LLM context copy (`CMD_GENERATE_LLM_CONTEXT`, `DEFAULT_LLM_COLUMN_WINDOW_SIZE`, `LLM_ELLIPSIS_COL`) in `crates/core/src/constants.rs` and `src-tauri/src/constants.rs`
- [X] T002 [P] Define centralized frontend constants (`COMMAND_NAMES.GENERATE_LLM_CONTEXT`, `LLM_DEFAULTS`) in `src/constants/index.ts`
- [X] T003 [P] Add bilingual localization keys (`ui.COPY_FOR_LLM`, `ui.COPY_FOR_LLM_TABLE`, `ui.COPY_FOR_LLM_HEADER_ONLY`, `ui.COPY_FOR_LLM_WORD_HEADING`, `ui.COPY_FOR_LLM_WORD_PARAGRAPH`, `ui.COPIED_FOR_LLM_SUCCESS`, `ui.SETTINGS_LLM_SECTION`, `ui.SETTINGS_LLM_WINDOW_SIZE`) in `crates/core/locales/ja.yml` and `crates/core/locales/en.yml`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core data models, TypeScript types, and IPC communication services required by all user stories.

**⚠️ CRITICAL**: Must complete before implementing any user story.

- [X] T004 [P] Create LLM data models (`LlmCopyOptions`, `LlmCopyType`, `WordExtractionMode`, `HeaderContext`, `PromptSnippet`) in `crates/core/src/models/llm.rs` and re-export in `crates/core/src/models/mod.rs`
- [X] T005 [P] Create TypeScript interfaces for LLM copy options and prompt snippets in `src/types/llm.ts` and re-export in `src/types/index.ts`
- [X] T006 Extend `DefaultSearchOptions` with `llm_column_window_size` (number, default: 5), `llm_default_copy_type` ("table" | "header_cell_only"), and `llm_word_extraction_mode` ("headings_and_paragraph" | "paragraph_only") in `src/types/defaultOptions.ts`
- [X] T007 Implement frontend IPC client service `generateLlmContext` in `src/services/llm.ts`

**Checkpoint**: Foundation ready — user story implementation can begin.

---

## Phase 3: User Story 1 - One-Click "Copy for LLM" with Header Context (Priority: P1) 🎯 MVP

**Goal**: Enable users to copy a single matched Excel cell with its identified column header, row key, cell value, and bolded Markdown table representation directly to the system clipboard via the preview toolbar button.

**Independent Test**: Search for an Excel cell (e.g. `G42` with `150,000`), click "Copy for LLM" on the preview toolbar, paste into a text editor, and verify that the column header, row key, cell coordinate, and bolded cell value (`**150,000**`) are cleanly formatted.

### Tests for User Story 1

- [X] T008 [P] [US1] Create unit tests for single-cell header extraction and Markdown table formatting with bolded hit values in `crates/core/src/search/llm_context.rs`

### Implementation for User Story 1

- [X] T009 [US1] Implement single-cell header extraction, row data retrieval, and bolded Markdown table generation in `crates/core/src/search/llm_context.rs`
- [X] T010 [US1] Implement Tauri IPC command handler `generate_llm_context` in `src-tauri/src/commands/preview_cmd.rs` and register the command in `src-tauri/src/lib.rs`
- [X] T011 [US1] Add "Copy for LLM" action button with `navigator.clipboard.writeText` and toast feedback in `src/components/preview/PreviewHeader.tsx`

**Checkpoint**: User Story 1 (MVP) is fully functional and testable independently.

---

## Phase 4: User Story 2 - Markdown Table Row Context Extraction and Multi-Selection Bundling (Priority: P2)

**Goal**: Support consolidating multiple selected search results from the same sheet into a single unified Markdown table sharing column headers, and grouping results cleanly when multiple sheets or workbooks are selected.

**Independent Test**: Select multiple search results from the same sheet, trigger "Copy for LLM", and verify that all selected rows appear in a single unified Markdown table with their respective hit cells bolded.

### Tests for User Story 2

- [X] T012 [P] [US2] Create unit tests for multi-hit bundling in the same sheet and multi-sheet grouping in `crates/core/src/search/llm_context.rs`

### Implementation for User Story 2

- [X] T013 [US2] Implement multi-hit deduplication, row consolidation, and sheet/workbook grouping in `crates/core/src/search/llm_context.rs`
- [X] T014 [US2] Update selection state handling in `src/components/preview/PreviewHeader.tsx` and `src/App.tsx` to pass all currently selected matches to `generateLlmContext`

**Checkpoint**: User Stories 1 and 2 are functional and testable independently.

---

## Phase 5: User Story 3 - Robust Header & Key Column Heuristics (Priority: P3)

**Goal**: Intelligently identify true table header rows when sheets have title banners or blank margin rows (starting below row 1), handle merged category headers, and locate leftmost descriptive key columns.

**Independent Test**: Perform search on an Excel sheet with title banners on rows 1–2 and column headers on row 3; verify that row 3 is recognized as the header row rather than row 1.

### Tests for User Story 3

- [X] T015 [P] [US3] Create unit tests for banner row offsets, merged header cells, and fallback coordinates in `crates/core/src/search/llm_context.rs`

### Implementation for User Story 3

- [X] T016 [US3] Implement upward-scan heuristic for header row discovery and leftward-scan heuristic for descriptive key column in `crates/core/src/search/llm_context.rs`
- [X] T017 [US3] Implement merged header cell text resolution for horizontal merged header ranges in `crates/core/src/search/llm_context.rs`

**Checkpoint**: User Stories 1, 2, and 3 are functional and resilient against complex sheet layouts.

---

## Phase 6: User Story 4 - Context Menu Selection & Customization Options (Priority: P2)

**Goal**: Allow users to right-click search result rows to select copy types directly (Table Context, Header & Cell Only, or Word document modes), configure column window limits with `...` ellipsis notation, and persist options in settings.

**Independent Test**: Right-click an Excel result and select "Header & Cell Only"; right-click a Word result and select "With Headings & Context"; adjust column window size in Settings and verify that output tables adhere to the new column limit.

### Tests for User Story 4

- [X] T018 [P] [US4] Create unit tests for column windowing with ellipsis (`...`) notation and Word document context extraction modes in `crates/core/src/search/llm_context.rs`

### Implementation for User Story 4

- [X] T019 [US4] Implement column windowing logic (`[hit_col - W, hit_col + W]` preserving leading key column and inserting `...` ellipsis markers) in `crates/core/src/search/llm_context.rs`
- [X] T020 [US4] Implement Word document context extraction (`headings_and_paragraph` with `### Heading` + `> quote`, and `paragraph_only`) in `crates/core/src/search/llm_context.rs`
- [X] T021 [P] [US4] Create custom React context menu component `src/components/results/ContextMenu.tsx` with copy type actions and viewport boundary clamping
- [X] T022 [US4] Integrate context menu event handlers (`onContextMenu`) and copy action dispatching in `src/components/results/ResultTable.tsx`
- [X] T023 [US4] Add LLM copy preferences (column window size slider, default copy type, Word extraction mode) to Settings dialog in `src/components/settings/`

**Checkpoint**: All user stories are complete, independently testable, and customizable.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Documentation, UI prototype synchronization, and full quality gate verification across all crates.

- [X] T024 [P] Update UI prototype in `design/mainui/index.html` to reflect the "Copy for LLM" toolbar button and right-click context menu
- [X] T025 Execute Rust verification suite: `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo fmt --check`
- [X] T026 Execute frontend verification suite: `npm run build` confirming zero TypeScript errors and bundle success
- [X] T027 Validate all 5 end-to-end scenarios documented in `specs/028-copy-llm-context/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies
- **Setup (Phase 1)**: No dependencies — can start immediately.
- **Foundational (Phase 2)**: Depends on Setup (Phase 1) — BLOCKS all user stories.
- **User Story 1 (Phase 3)**: Depends on Foundational (Phase 2). Delivers core MVP.
- **User Story 2 (Phase 4)**: Depends on User Story 1. Expands to multi-hit bundling.
- **User Story 3 (Phase 5)**: Depends on User Story 1. Enhances extraction heuristics.
- **User Story 4 (Phase 6)**: Depends on User Story 1 & 2. Adds context menu, Word extraction, and settings.
- **Polish (Phase 7)**: Depends on completion of all desired user stories.

### Parallel Opportunities
- In Setup (Phase 1): `T001`, `T002`, `T003` can execute concurrently in parallel.
- In Foundational (Phase 2): `T004` (Rust) and `T005` (TS) can execute concurrently in parallel.
- Across stories: After Foundational (Phase 2) is complete, `T008` (US1 test), `T012` (US2 test), `T015` (US3 test), and `T018` (US4 test) can be developed concurrently.
- In Polish (Phase 7): `T024` (UI prototype) can execute in parallel with test verification.

---

## Implementation Strategy

### MVP First (User Story 1)
1. Complete Setup (Phase 1: `T001`–`T003`)
2. Complete Foundational (Phase 2: `T004`–`T007`)
3. Complete User Story 1 (Phase 3: `T008`–`T011`)
4. **STOP and VALIDATE**: Verify single-cell context copy end-to-end (MVP achieved!)

### Incremental Delivery
1. Add User Story 2 (`T012`–`T014`) → Multi-selection bundling enabled.
2. Add User Story 3 (`T015`–`T017`) → Resilient banner & merged header heuristics enabled.
3. Add User Story 4 (`T018`–`T023`) → Context menu, Word extraction, and settings enabled.
4. Execute Polish (`T024`–`T027`) → All quality gates confirmed.
