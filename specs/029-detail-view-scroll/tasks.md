---
description: "Task list for Scrollable Match Detail Area in Search Preview implementation"
---

# Tasks: Scrollable Match Detail Area in Search Preview

**Input**: Design documents from `specs/029-detail-view-scroll/`  
**Prerequisites**: [plan.md](./plan.md) (tech stack, project structure), [spec.md](./spec.md) (user stories and priorities), [research.md](./research.md), [data-model.md](./data-model.md), [contracts/detail-scroll-contracts.md](./contracts/detail-scroll-contracts.md), [quickstart.md](./quickstart.md)  
**Tests**: Unit tests created in `tests/meta-info-card.test.tsx` to verify component scrolling, newline preservation, and visual styling.  
**Organization**: Tasks are structured by user story to enable independent implementation, testing, and verification.

## Format: `- [ ] [TaskID] [P?] [Story?] Description with file path`

- **[P]**: Can run in parallel (different files, no blocking dependencies)
- **[Story]**: Target user story (`[US1]`, `[US2]`, `[US3]`)
- Each task description contains exact file paths.

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Centralize layout constants in compliance with Constitution Principle II.

- [X] T001 Define centralized layout constants `META_DETAIL_MAX_HEIGHT_CLASS: "max-h-36"`, `META_DETAIL_MAX_HEIGHT_PX: 144`, and test identifier `META_DETAIL_TEST_ID: "meta-info-content"` in `LAYOUT_CONSTANTS` in `src/constants/index.ts`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Establish component unit testing infrastructure required by all user stories.

- [X] T002 Create test scaffold with translation mocks, `SearchMatch` factory helper, and initial test suite in `tests/meta-info-card.test.tsx`

**Checkpoint**: Foundation ready — user story implementation and testing can begin.

---

## Phase 3: User Story 1 - Long Cell Content Scrolling and Sheet Preview Preservation (Priority: P1) 🎯 MVP

**Goal**: Cap the match detail container to a maximum height (approx. 144px) with automatic vertical scrolling, preventing long cell content from squashing the surrounding cell preview grid (`SpreadsheetGrid`).

**Independent Test**: Render a `SearchMatch` containing 10+ lines of text and verify that `max-h-36` and `overflow-y-auto` are applied to the detail container, preserving the spreadsheet preview grid display.

### Tests for User Story 1

- [X] T003 [P] [US1] Add unit test in `tests/meta-info-card.test.tsx` verifying that long cell content renders with `max-h-36` and `overflow-y-auto` classes

### Implementation for User Story 1

- [X] T004 [US1] Apply `LAYOUT_CONSTANTS.META_DETAIL_MAX_HEIGHT_CLASS`, `overflow-y-auto`, and `data-testid={LAYOUT_CONSTANTS.META_DETAIL_TEST_ID}` to the content viewport in `src/components/preview/MetaInfoCard.tsx` with Principle II constant comments and Principle III header documentation
- [X] T005 [P] [US1] Synchronize HTML prototype by adding `max-h-36 overflow-y-auto` to the `#metaFullContent` element in `design/mainui/index.html`

**Checkpoint**: User Story 1 (MVP) is fully functional and independently verifiable.

---

## Phase 4: User Story 2 - Keyboard Navigation, Newline Preservation, and Text Selection (Priority: P2)

**Goal**: Preserve newline formatting (`\n`, `\r\n`) within cell content and ensure smooth keyboard navigation and text selection/copying.

**Independent Test**: Render multi-line cell text containing explicit newlines and verify that `whitespace-pre-wrap` and `select-text` wrap lines cleanly without stripping newlines.

### Tests for User Story 2

- [X] T006 [P] [US2] Add unit test in `tests/meta-info-card.test.tsx` verifying that text containing explicit newline characters preserves formatting with `whitespace-pre-wrap`

### Implementation for User Story 2

- [X] T007 [US2] Add `whitespace-pre-wrap` utility class to the content viewport element in `src/components/preview/MetaInfoCard.tsx`
- [X] T008 [P] [US2] Add `whitespace-pre-wrap` class to `#metaFullContent` in `design/mainui/index.html` to keep the HTML prototype in sync

**Checkpoint**: User Story 2 is complete, ensuring readability and selectable formatting for multi-line cells.

---

## Phase 5: User Story 3 - Scroll Chaining Prevention and Visual Indicators (Priority: P3)

**Goal**: Prevent mouse wheel scrolling within the detail container from chaining to parent panes and ensure natural sizing for short text without scrollbars.

**Independent Test**: Verify `overscroll-contain` class application, natural compact sizing for short text, and empty content fallback display.

### Tests for User Story 3

- [X] T009 [P] [US3] Add unit tests in `tests/meta-info-card.test.tsx` verifying `overscroll-contain` class presence, short content rendering without scrollbars, and empty string fallback
### Implementation for User Story 3

- [X] T010 [US3] Add `overscroll-contain` utility class to the content viewport element in `src/components/preview/MetaInfoCard.tsx`
- [X] T011 [P] [US3] Add `overscroll-contain` class to `#metaFullContent` in `design/mainui/index.html` to keep the HTML prototype in sync

**Checkpoint**: All user stories implemented and covered by unit tests.

---

## Phase 6: Polish & Cross-Cutting Concerns (Quality Gates)

**Purpose**: Execute and verify complete workspace quality gates (Constitution Principles I–V).

- [X] T012 [P] Execute Vitest unit test suite via `npm test -- tests/meta-info-card.test.tsx` and confirm 100% pass rate
- [X] T013 [P] Execute TypeScript compilation and Vite bundle check via `npm run build` with zero type errors and zero warnings
- [X] T014 Execute Rust workspace quality gates (`cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`) to verify zero regressions
- [X] T015 Perform end-to-end manual verification against test scenarios defined in `specs/029-detail-view-scroll/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can execute immediately.
- **Foundational (Phase 2)**: Depends on Phase 1 completion; establishes test harness for user stories.
- **User Story 1 (Phase 3: P1 🎯 MVP)**: Depends on Phase 2; provides the core fix for grid squashing.
- **User Story 2 (Phase 4: P2)**: Depends on Phase 3.
- **User Story 3 (Phase 5: P3)**: Depends on Phase 4.
- **Polish (Phase 6)**: Depends on all user stories being completed.

### Within Each User Story

1. Write / expand unit tests (`T003`, `T006`, `T009`)
2. Update component implementation (`T004`, `T007`, `T010`)
3. Synchronize HTML prototype (`T005`, `T008`, `T011`)

---

## Parallel Opportunities

- **Phase 3 (US1)**: Test task `T003` and HTML prototype sync `T005` can run in parallel.
- **Phase 4 (US2)**: Test task `T006` and HTML prototype sync `T008` can run in parallel.
- **Phase 5 (US3)**: Test task `T009` and HTML prototype sync `T011` can run in parallel.
- **Phase 6 (Polish)**: `T012` (Vitest) and `T013` (npm run build) can run in parallel.

---

## Implementation Strategy

### MVP First (User Story 1 Only)
1. Complete Phase 1 (`T001`)
2. Complete Phase 2 (`T002`)
3. Complete Phase 3 (`T003`–`T005`)
4. **Pause and Validate**: Verify that long cells no longer squash `SpreadsheetGrid` and scrolling works (MVP delivered).

### Incremental Delivery
1. Deliver MVP (User Story 1)
2. Add User Story 2 (`T006`–`T008`) to preserve explicit line breaks and formatting
3. Add User Story 3 (`T009`–`T011`) to prevent scroll chaining
4. Execute Phase 6 (`T012`–`T015`) across all verification gates
