# Tasks: Horizontal Side-by-Side Pattern Inputs Layout

**Input**: Design documents from `/specs/038-horizontal-pattern-inputs/`  
**Prerequisites**: [plan.md](./plan.md) (required), [spec.md](./spec.md) (required), [research.md](./research.md), [data-model.md](./data-model.md), [contracts/ui-layout-contract.md](./contracts/ui-layout-contract.md), [quickstart.md](./quickstart.md)

**Tests**: Test tasks are included per Constitution Principle V and Spec Kit requirements.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Baseline review of existing pattern controls and layout structures.

- [ ] T001 Review existing SearchBar pattern controls and layout styles in src/components/search/SearchBar.tsx

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Validate centralized constants and IDs required across pattern filter components.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [ ] T002 Verify constants and element IDs for file search and ignore patterns in src/constants/index.ts

**Checkpoint**: Foundation verified - user story implementation can now begin.

---

## Phase 3: User Story 1 - Side-by-Side Alignment of File Search and Ignore Patterns (Priority: P1) 🎯 MVP

**Goal**: Merge the two separate pattern rows into a single horizontal row container with balanced 2-column distribution (`grid grid-cols-1 md:grid-cols-2 gap-3 items-center`).

**Independent Test**: Render `SearchBar` in Vitest; assert that the File Search Patterns input and Ignore Patterns input are rendered inside the same parent grid container with `md:grid-cols-2`.

### Tests for User Story 1

- [ ] T003 [P] [US1] Create automated layout test asserting side-by-side pattern grid in tests/pattern-layout-ui.test.tsx

### Implementation for User Story 1

- [ ] T004 [US1] Refactor SearchBar pattern rows into a unified side-by-side grid in src/components/search/SearchBar.tsx

**Checkpoint**: At this point, User Story 1 is fully functional and delivers a compact side-by-side pattern layout.

---

## Phase 4: User Story 2 - Full Functionality and History Dropdown Usability (Priority: P1)

**Goal**: Ensure independent typing, clear (`X`) button actions, Enter shortcuts, and history dropdown overlays function flawlessly in the side-by-side columns without visual clipping or mutual interference.

**Independent Test**: Click clear buttons to verify only target input clears; toggle history dropdowns to verify only one dropdown is open at a time with correct z-index layering.

### Tests for User Story 2

- [ ] T005 [P] [US2] Add test cases for independent pattern clearing and history dropdown exclusivity in tests/pattern-layout-ui.test.tsx

### Implementation for User Story 2

- [ ] T006 [US2] Ensure dropdown overlay positioning and z-index layering in src/components/search/SearchBar.tsx

**Checkpoint**: User Stories 1 and 2 are functional; history dropdowns and clear buttons operate without conflict.

---

## Phase 5: User Story 3 - Responsive Layout on Narrow Windows (Priority: P2)

**Goal**: Support graceful responsive 1-column fallback on narrow screens (< 768px) and synchronize the standalone HTML prototype in `design/mainui/index.html`.

**Independent Test**: Verify `design/mainui/index.html` contains matching side-by-side grid classes and `tests/pattern-layout-ui.test.tsx` verifies mockup structure.

### Implementation for User Story 3

- [ ] T007 [P] [US3] Update standalone HTML prototype in design/mainui/index.html with side-by-side pattern row
- [ ] T008 [P] [US3] Add automated layout check for design/mainui/index.html in tests/pattern-layout-ui.test.tsx

**Checkpoint**: All user stories functional, responsive fallback active, and UI prototype fully synchronized.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Verification across all automated quality gates and project constitution compliance.

- [ ] T009 [P] Execute frontend test suite and build validation via npm test and npm run build
- [ ] T010 [P] Execute Rust workspace test suite and clippy linting via cargo test --workspace and cargo clippy --workspace --all-targets -- -D warnings
- [ ] T011 Perform manual verification of scenarios in specs/038-horizontal-pattern-inputs/quickstart.md

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately.
- **Foundational (Phase 2)**: Depends on Setup completion - blocks user stories.
- **User Story 1 (Phase 3)**: Depends on Phase 2 - delivers core MVP layout.
- **User Story 2 (Phase 4)**: Depends on Phase 3 - refines dropdown and clear interactivity.
- **User Story 3 (Phase 5)**: Depends on Phase 3/4 - syncs HTML prototype and adds responsive verification.
- **Polish (Phase 6)**: Depends on all user stories being complete.

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories.
- **User Story 2 (P1)**: Builds upon the unified grid container from US1.
- **User Story 3 (P2)**: Syncs HTML mockup and verifies responsive grid behavior.

### Parallel Opportunities

- Within Phase 3: T003 (test creation) can be written before or alongside T004.
- Within Phase 4: T005 test cases can be prepared while verifying T006.
- Within Phase 5: T007 (HTML prototype) and T008 (mockup test) can be executed in parallel.
- Within Phase 6: T009 and T010 can execute independently.

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1 (T001) & Phase 2 (T002).
2. Complete Phase 3 (T003, T004): Merge the two rows into a single `grid-cols-1 md:grid-cols-2` row in `SearchBar.tsx`.
3. **STOP and VALIDATE**: Test that both inputs render side-by-side in `npm run tauri dev`.

### Incremental Delivery

1. Step 1: MVP Side-by-side Layout (Phase 3).
2. Step 2: Dropdown layering & independent clearing verification (Phase 4).
3. Step 3: Prototype synchronization & responsive validation (Phase 5).
4. Step 4: Full test suite & linting quality gates (Phase 6).
