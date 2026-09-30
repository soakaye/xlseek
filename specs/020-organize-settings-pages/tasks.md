<!--
Description: Lists dependency-ordered implementation and verification work for reorganizing settings into immediate and saved pages.
Arguments & Returns: Inputs are spec.md, plan.md, research.md, data-model.md, and contracts; output is an executable Markdown task checklist.
Errors: No runtime execution. Failed tests or quality gates block completion; unresolved requirements must be addressed before delivery.
-->
# Tasks: Separate Settings Pages and Unify the Save Action

**Input**: Design documents from `/specs/020-organize-settings-pages/`

**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md), [data-model.md](data-model.md), [UI contract](contracts/settings-ui-contract.md), [quickstart.md](quickstart.md).

**Tests**: Required by the project constitution and user-supplied AGENTS.md. Extend existing Vitest suites, write new behavior checks before implementation, and confirm they fail for the intended reason. No new testing framework is needed.

**Organization**: Setup → shared persistence foundation → four user stories → delivery checks. Every implementation task must preserve comprehensive headers, centralized constants and reference comments, localized labels, and handled errors. `[P]` indicates independent files within the stated phase, not authorization to spawn agents.

## Phase 1: Setup

**Purpose**: Confirm the existing project and feature context; do not scaffold another application.

- [x] T001 Review `specs/020-organize-settings-pages/plan.md`, `specs/020-organize-settings-pages/contracts/settings-ui-contract.md`, and `AGENTS.md`; inspect `git status` on branch `020-organize-settings-pages`, preserve unrelated changes, and use existing tools from `package.json` without adding dependencies.

## Phase 2: Foundational — Combined Persistence

**Purpose**: Establish backward-compatible, storage-first settings commits shared by the saved-page stories.

- [x] T002 [P] Add failing boundary tests in `tests/settings-core.test.ts` for legacy/extended loading, missing/corrupt/unreadable storage, embedded defaults taking precedence, no startup writes, one-key replacement, rejected serialization/storage writes, and remembered-worker recovery; move persistence expectations from `tests/default-options.test.ts` while retaining pure option tests there.
- [x] T003 [P] Add failing combined-save tests in `tests/search-history.test.ts` for latest accepted histories retained while editing, successful-only trimming/zero clearing, no preference/state changes on write failure, and ordinary history writes preserving defaults with the existing memory-only failure fallback.
- [x] T004 Implement `SavedSettings` loading/validation and single-key saving in `src/settings-core.ts`, reusing pure helpers in `src/default-options-core.ts` and `src/types/defaultOptions.ts`: `maxEntries` is "Existing shared history limit, 0–50; initial value 20"; both history arrays are "Existing newest-first, exact-unique, nonblank" entries with "length ≤ maxEntries"; remembered workers are "Last valid manual count, 2–32; null before a custom count exists or after restoring defaults". Preserve existing boolean defaults and supported extensions ".xlsx, .xlsm, .xlsb, .xls; at least one supported extension, no duplicates"; `directory_mode` is "sequential or burst" and `burst_workers` is "integer or null" with "null means automatic; a number means manual and must be 2–32". Extend the existing history key, use legacy default storage only for records without embedded defaults, normalize safely without startup writes or destructive migration, and leave the legacy key untouched.
- [x] T005 Extend `src/hooks/useSearchHistory.ts` to own the combined committed record/ref and expose one preference-save operation: validate, derive a trimmed candidate from latest histories, persist once, then publish ref/state only on success; retain accepted-search memory fallback and preserve all preference fields on ordinary writes. Keep existing UI usable until unified App/dialog wiring replaces the separate callbacks.

**Checkpoint**: T002–T003 pass; failed explicit saves leave persisted and committed data unchanged. All stories depend on this phase.

## Phase 3: User Story 1 — Immediate Language Page (Priority: P1)

**Goal**: Clearly distinguish application timing with two pages, initially displaying immediate language selection.

**Independent Test**: Open settings, change language, close/reopen; selection persists without a save action. Immediate page has zero save buttons, saved page remains reachable, and language persistence failure retains the session language.

- [x] T006 [P] [US1] Extend `tests/settings.test.tsx` with failing tests for the initial immediate page, two distinguishable page controls, immediate language selection, no immediate-page save button, and Close/header actions; retain existing language persistence behavior checks.
- [x] T007 [P] [US1] Extend `tests/app-locale.test.tsx` with failing tests proving page/language changes preserve search input, results, and selection and that language persistence errors retain the selected session language.
- [x] T008 [US1] Add page identifiers and translation keys in `src/constants/index.ts` and Japanese/English page names, timing descriptions, Close/Cancel/Save Settings labels, validation messages, and accessible names in `src-tauri/locales/ja.yml` and `src-tauri/locales/en.yml`; reuse existing labels and ranges where suitable.
- [x] T009 [US1] Split the existing layout in `src/components/settings/SettingsDialog.tsx` into local immediate/saved pages with immediate selected on opening, language-only immediate content and Close action; reuse `src/hooks/useLocale.tsx` behavior and keep language state independent of saved preferences.

**Checkpoint**: US1 independently passes. This is a demonstrable first increment, not the complete requested feature.

## Phase 4: User Story 2 — One Save for Search Settings (Priority: P1)

**Goal**: Commit history and all existing search defaults together; retain drafts until successful saving.

**Independent Test**: Edit history and multiple defaults; verify no current changes before saving, then one save persists/applies everything and closes. Reopening/restarting restores settings; rejected writes preserve the draft and prior settings/history.

- [x] T010 [P] [US2] Add failing cases in `tests/settings-default-options.test.tsx` for one save action, every existing option, drafts retained across pages/language changes, history "integer from 0 to 50", workers "required and validated only when both Burst and custom selection are active" and "integer from 2 to 32", at least one supported extension, and inactive invalid worker text preserving the last valid count; cover lower/upper bounds, blank/fractional input, automatic/manual/sequential transitions, and fresh-open count memory.
- [x] T011 [P] [US2] Extend `tests/app-default-options.test.tsx` with failing integration cases for one combined persistence write, success notification/close, failed save retaining draft/state without success, reopen/remount restoration, and saving during a scan applying only to the next search while preserving input/results/selection. Reenter the save event during the synchronous save operation through a test-controlled storage write and verify that the save callback and storage write each execute only once. After a rejected write, verify that editing, canceling, and saving become available again; restore storage and retry, then verify that the retained draft is committed together, success is notified, and the dialog closes. Do not add delays or asynchronous persistence solely to exercise these tests.
- [x] T012 [US2] Implement saved-page draft state and validation in `src/components/settings/SettingsDialog.tsx`: raw history/worker strings, copied options/extensions, selected worker mode, remembered count, and synchronous saving guard. Initialize only on closed-to-open transition; preserve drafts on locale/page/history updates; ignore invalid inactive worker text without replacing valid memory; prevent edits, page switches, exits, and duplicate commits during saving; show localized field errors and keep the draft open after callback failure.
- [x] T013 [US2] Replace separate history/default save callbacks and duplicate default state in `src/App.tsx` with the hook's combined preference-save callback; apply options and show success/close only after persistence succeeds, report failures in the current language, and submit preferences rather than draft history arrays.
- [x] T014 [US2] Initialize defaults through the same combined loader in `src/hooks/useSearch.ts`, remove superseded standalone persistence entry points from `src/default-options-core.ts`, and update their remaining callers/tests in `tests/default-options.test.ts`; preserve pure option helpers and ensure `applyDefaultOptions` changes only future search controls, leaving the submitted scan and current input/results/selection intact.

**Checkpoint**: US2, foundation, and US1 tests pass; one commit includes all saved-page preferences, and failures cannot partially apply them.

## Phase 5: User Story 3 — Discard Unsaved Changes (Priority: P1)

**Goal**: Every exit safely discards drafts; restoring defaults remains draft-only.

**Independent Test**: Draft a zero history limit and changed search options, exit through every supported route, and reopen. Saved histories/preferences stay intact, language changes remain, and draft errors disappear. Reset changes only draft search defaults until save.

- [x] T015 [P] [US3] Extend `tests/settings.test.tsx` with failing cases for Cancel, header close, Escape, backdrop, and immediate-page Close after editing the saved page; verify reopening restores committed values and clears errors, and exit events during saving cannot discard the candidate.
- [x] T016 [P] [US3] Extend `tests/settings-default-options.test.tsx` with failing reset/cancel cases proving "Reset restores existing option defaults and clears the draft manual count memory" and "It preserves the history input and immediate language preference", with no persistence until save.
- [x] T017 [US3] Route every discard exit through a shared guarded close operation in `src/components/settings/SettingsDialog.tsx`; implement Cancel, Close, header close, Escape, and backdrop behavior without a confirmation dialog, and reset only draft search options/count memory. Keep saved history unchanged until successful save and restore clean drafts on reopening.

**Checkpoint**: All exit paths and reset behavior pass independently, including preservation of already-applied language changes.

## Phase 6: User Story 4 — Small-Screen and Keyboard Operation (Priority: P2)

**Goal**: Keep actions visible and make the complete settings workflow keyboard accessible in Japanese and English.

**Independent Test**: At 960×640 and 1087×940, complete language selection, save, and cancel using only the keyboard; header/footer stay visible, content scrolls, selected page/focus is identifiable, and closing restores launcher focus.

- [x] T018 [P] [US4] Extend `tests/settings.test.tsx` with failing keyboard cases for semantic tabs/panels, page navigation, visible initial focus, focus containment, Escape, launcher-focus restoration, localized accessible names, and errors associated with fields.
- [x] T019 [P] [US4] Extend `tests/locale-ui.test.tsx` with failing coverage for translated page/action/error labels after a language change while valid and invalid saved-page drafts remain intact.
- [x] T020 [US4] Constrain the modal layout in `src/components/settings/SettingsDialog.tsx` with fixed visible header/navigation/footer and a scrollable content area; implement selected tab/panel semantics, keyboard navigation, focus containment/restoration, localized accessible names, and field error associations without adding a router or modal library.

**Checkpoint**: Keyboard/localization suites pass; actual viewport behavior is verified in Phase 7 because jsdom cannot establish visual layout.

## Phase 7: Polish and Cross-Cutting Verification

**Purpose**: Synchronize the reference mock and verify the whole feature against the specification.

- [x] T021 Extend `tests/mainui-settings.test.ts` with failing behavior checks, then use the `syncing-mainui-mock` skill to align `design/mainui/index.html` with the implemented pages, draft/cancel/reset behavior, combined persistence/startup restoration, remembered workers, failure handling, and keyboard controls; preserve standalone browser use.
- [x] T022 Review changed code in `src/settings-core.ts`, `src/default-options-core.ts`, `src/hooks/useSearchHistory.ts`, `src/hooks/useSearch.ts`, `src/App.tsx`, `src/components/settings/SettingsDialog.tsx`, `src/constants/index.ts`, and changed tests for comprehensive headers, constant reference comments, handled exceptions, redundant persistence paths, and unintended changes; check `src-tauri/locales/ja.yml`, `src-tauri/locales/en.yml`, and feature documents for garbled text.
- [x] T023 Run every automated quality gate from `specs/020-organize-settings-pages/quickstart.md` using `package.json` and `Cargo.toml`: frontend tests/lint/build, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo fmt --check`; build the CLI sidecar through `scripts/build-cli.js` if absent, resolve failures/warnings, and record actual results in `specs/020-organize-settings-pages/quickstart.md`.
- [x] T024 Run the Japanese/English 960×640 and 1087×940 layout and keyboard scenarios plus desktop restart/search-preservation scenarios in `specs/020-organize-settings-pages/quickstart.md` for the application and applicable mock flows in `design/mainui/index.html`; record evidence or material blockers, verify FR-001–FR-017 and SC-001–SC-006, and do not report completion while required checks remain unresolved.

## Dependencies and Execution Order

```text
T001 → T002/T003 → T004 → T005
                         ↓
             US1: T006/T007 → T008 → T009
                         ↓
             US2: T010/T011 → T012 → T013 → T014
                         ↓
             US3: T015/T016 → T017
                         ↓
             US4: T018/T019 → T020
                         ↓
                    T021 → T022 → T023 → T024
```

Setup blocks foundation. Foundation blocks all story work. US2 integrates the page structure from US1; US3 integrates US2's draft/save lifecycle; US4 completes keyboard/layout behavior on the stabilized pages. Stories share `SettingsDialog.tsx`, so implement them sequentially rather than editing that file concurrently. Their acceptance checks remain independently runnable after dependencies are ready.

Within each phase, write tests first, confirm their expected failures, implement, and run the relevant suites. Final gates run after all implementation/mock changes; repeat broader checks only when subsequent changes require them. Complete IDs in dependency order and mark checkboxes only when work is verified.

## Parallel Execution Examples

| Phase/story | Independent work permitted after its prerequisites |
|---|---|
| Foundation | T002 (`tests/settings-core.test.ts`, `tests/default-options.test.ts`) alongside T003 (`tests/search-history.test.ts`) |
| US1 | T006 (`tests/settings.test.tsx`) alongside T007 (`tests/app-locale.test.tsx`) |
| US2 | T010 (`tests/settings-default-options.test.tsx`) alongside T011 (`tests/app-default-options.test.tsx`) |
| US3 | T015 (`tests/settings.test.tsx`) alongside T016 (`tests/settings-default-options.test.tsx`) |
| US4 | T018 (`tests/settings.test.tsx`) alongside T019 (`tests/locale-ui.test.tsx`) |

These examples describe scheduling opportunities; no delegation is required. Do not run separate phases' edits to the same test or source files concurrently.

## Implementation Strategy

The first demonstrable MVP is foundation plus US1: two clearly named pages and immediate language selection. It does not satisfy the user's unified-save request by itself. Complete US2 and US3 for the functional requested scope, then US4 and delivery checks before claiming the feature complete. Reuse the existing dialog/hooks, one small persistence module, and existing tests; no new window, state framework, dependencies, or backend protocol changes.

## Requirement Traceability

| Requirements | Tasks |
|---|---|
| FR-001–FR-002 | T006–T009 |
| FR-003–FR-006 | T002–T005, T010–T014 |
| FR-007–FR-008 | T015, T017 |
| FR-009–FR-010 | T002–T005, T010, T012 |
| FR-011 | T016–T017 |
| FR-012–FR-013 | T002–T005, T011–T013, T015, T017 |
| FR-014–FR-015 | T018, T020, T024 |
| FR-016 | T008–T010, T019–T020 |
| FR-017 | T007, T011, T013–T014 |
| SC-001–SC-006 | Story acceptance tests and T021–T024 |
