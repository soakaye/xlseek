# Tasks: Clarify Search Results Save Button in Status Bar

**Input**: Design documents from `/specs/034-clarify-save-results-button/`

**Prerequisites**: `plan.md` (required), `spec.md` (required for user stories), `research.md`, `data-model.md`, `contracts/`, `quickstart.md`

**Tests**: Unit tests in `src/components/common/__tests__/StatusBar.test.tsx` are required and will be updated to verify label rendering, format switching, and accessibility attributes.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure / Constants & Locales)

**Purpose**: Register new translation keys and centralized constants for the explicit save label across frontend and backend.

- [X] T001 Register `ui.EXPORT_BUTTON_LABEL_WITH_FORMAT: "結果を保存: {format}"` and `ui.EXPORT_BUTTON_TOOLTIP_WITH_FORMAT: "検索結果を保存 ({format})"` in `crates/core/locales/ja.yml`
- [X] T002 [P] Register `ui.EXPORT_BUTTON_LABEL_WITH_FORMAT: "Save Results: {format}"` and `ui.EXPORT_BUTTON_TOOLTIP_WITH_FORMAT: "Save search results ({format})"` in `crates/core/locales/en.yml`
- [X] T003 [P] Add `BUTTON_LABEL_WITH_FORMAT: "ui.EXPORT_BUTTON_LABEL_WITH_FORMAT"` and `BUTTON_TOOLTIP_WITH_FORMAT: "ui.EXPORT_BUTTON_TOOLTIP_WITH_FORMAT"` to `EXPORT_CONSTANTS` in `src/constants/index.ts`
- [X] T004 [P] Add `EXPORT_BUTTON_LABEL_WITH_FORMAT_KEY` and `EXPORT_BUTTON_TOOLTIP_WITH_FORMAT_KEY` constants in `crates/core/src/constants.rs` and re-export in `src-tauri/src/constants.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Verify that constants and localization assets compile cleanly before component updates.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [X] T005 Verify constants compilation and frontend type checking with `npm run build` and `cargo test --workspace`

**Checkpoint**: Foundation ready - user story implementation can now begin.

---

## Phase 3: User Story 1 - Explicit Save Action on Results Export Button (Priority: P1) 🎯 MVP

**Goal**: Make the primary export button in the status bar explicitly state that it saves search results by displaying `結果を保存: {Format}` (or `Save Results: {Format}` in English) alongside the format icon.

**Independent Test**: Render `StatusBar` in both enabled and disabled states; verify the primary button text displays "Save Results: Excel (.xlsx)" (or "結果を保存: Excel (.xlsx)" under Japanese locale) with the spreadsheet icon, and clicking it immediately triggers the export action for active format.

### Tests for User Story 1 ⚠️

> **NOTE: Write/update these tests FIRST, ensure they fail or assert the new requirements before implementation.**

- [X] T006 [P] [US1] Update unit tests in `src/components/common/__tests__/StatusBar.test.tsx` to assert explicit save button label ("Save Results: Excel (.xlsx)") in both enabled and disabled states

### Implementation for User Story 1

- [X] T007 [US1] Update primary button text in `src/components/common/StatusBar.tsx` to interpolate active format with `t(EXPORT_CONSTANTS.BUTTON_LABEL_WITH_FORMAT, { format: t(activeOption.labelKey) })`
- [X] T008 [US1] Verify User Story 1 unit tests pass with `npm test -- src/components/common/__tests__/StatusBar.test.tsx --run`

**Checkpoint**: At this point, User Story 1 is fully functional and delivers the core MVP increment.

---

## Phase 4: User Story 2 - Clear Format Switching and Accessible Descriptions (Priority: P2)

**Goal**: Ensure format switching via dropdown updates the explicit save label immediately and provides clear accessibility attributes (`title`, `aria-label`).

**Independent Test**: Open the format menu, select an alternative format (e.g. CSV), verify the primary button immediately updates its text to `Save Results: CSV (.csv)` (or `結果を保存: CSV (.csv)`), and verify `aria-label` / `title` reads `Save search results (CSV (.csv))` / `検索結果を保存 (CSV (.csv))`.

### Tests for User Story 2 ⚠️

- [X] T009 [P] [US2] Update format switching and accessibility test cases in `src/components/common/__tests__/StatusBar.test.tsx` to assert updated button label and `aria-label` after selecting alternative formats

### Implementation for User Story 2

- [X] T010 [US2] Update `title` and `aria-label` on the primary export button in `src/components/common/StatusBar.tsx` using `t(EXPORT_CONSTANTS.BUTTON_TOOLTIP_WITH_FORMAT, { format: t(activeOption.labelKey) })`
- [X] T011 [US2] Verify User Story 2 format switching and accessibility tests pass with `npm test -- src/components/common/__tests__/StatusBar.test.tsx --run`

**Checkpoint**: At this point, User Stories 1 AND 2 are both fully functional and tested.

---

## Phase 5: User Story 3 - Responsive Status Bar Layout & Mockup Synchronization (Priority: P3)

**Goal**: Ensure the updated button layout fits cleanly into the fixed 40px height status bar down to 800px viewport width, and synchronize the standalone HTML prototype.

**Independent Test**: Verify layout in `design/mainui/index.html` and check that the status bar maintains `h-10` with clean ellipsis truncation of status text.

### Implementation for User Story 3

- [X] T012 [P] [US3] Update standalone HTML prototype in `design/mainui/index.html` with explicit save button text (`結果を保存: Excel (.xlsx)`) and tooltip (`検索結果を保存 (Excel (.xlsx))`)
- [X] T013 [US3] Verify status bar fixed height (`h-10`), flex layout, and ellipsis truncation at 800px minimum viewport width in `src/components/common/StatusBar.tsx` and `design/mainui/index.html`

**Checkpoint**: All user stories and UI prototypes are complete and consistent.

---

## Phase 6: Polish & Cross-Cutting Verification

**Purpose**: Execute full verification suite across all repository crates and quality gates.

- [X] T014 Run full frontend build and type-check with `npm run build`
- [X] T015 [P] Run Rust workspace tests with `cargo test --workspace`
- [X] T016 [P] Run Rust workspace Clippy check with `cargo clippy --workspace --all-targets -- -D warnings`
- [X] T017 [P] Run Rust formatting check with `cargo fmt --check`
- [X] T018 Execute manual verification scenarios from `specs/034-clarify-save-results-button/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Can start immediately.
- **Foundational (Phase 2)**: Depends on Setup (Phase 1) - BLOCKS all user stories.
- **User Stories (Phase 3+)**: All depend on Foundational phase completion.
  - Phase 3 (US1 - MVP) can proceed first.
  - Phase 4 (US2) builds upon US1 button updates.
  - Phase 5 (US3) syncs mockups and verifies responsive layout.
- **Polish (Phase 6)**: Runs after all user stories are implemented.

### Parallel Opportunities

- In Phase 1: T002, T003, T004 can run in parallel with T001.
- In Phase 3: T006 test update can be written while reviewing T007.
- In Phase 4: T009 test update can be prepared in parallel with T010.
- In Phase 5: T012 mockup sync can run in parallel.
- In Phase 6: T015, T016, T017 can run in parallel.

---

## Parallel Example: User Story 1 & Setup

```bash
# Launch Phase 1 constant/locale tasks in parallel:
Task: "Register ui.EXPORT_BUTTON_LABEL_WITH_FORMAT in crates/core/locales/en.yml"
Task: "Add BUTTON_LABEL_WITH_FORMAT to EXPORT_CONSTANTS in src/constants/index.ts"
Task: "Add EXPORT_BUTTON_LABEL_WITH_FORMAT_KEY in crates/core/src/constants.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001-T004)
2. Complete Phase 2: Foundational validation (T005)
3. Complete Phase 3: User Story 1 (T006-T008)
4. **STOP and VALIDATE**: Verify that the button explicitly displays "Save Results: Excel (.xlsx)" and opens the file save dialog.
5. Proceed to User Story 2 (format switching & accessibility), User Story 3 (mockup sync), and final polish.
