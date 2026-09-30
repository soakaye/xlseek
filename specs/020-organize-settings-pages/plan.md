<!--
Description: Defines the implementation approach, constitution gates, scope, and verification for reorganized settings.
Arguments & Returns: Input is the approved feature specification and repository research; output is a Markdown planning artifact.
Errors: No runtime execution; unresolved design requirements must be addressed before implementation.
-->
# Implementation Plan: Separate Settings Pages and Unify Saving

**Branch**: `020-organize-settings-pages` | **Date**: 2026-10-01 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/020-organize-settings-pages/spec.md`

## Summary

Reuse the existing settings dialog with two local pages: immediate language selection and draft search/history settings committed through one save action. Extend the existing history storage record with search defaults and remembered manual Burst concurrency. A single storage replacement precedes memory updates, preventing partial application when saving fails. Language retains its independent immediate persistence. See [research](research.md), [data model](data-model.md), and [UI contract](contracts/settings-ui-contract.md).

## Technical Context

**Language/Version**: TypeScript 5.5, React 18.3; existing Rust 2021 backend unchanged.

**Primary Dependencies**: Existing Tailwind CSS 3.4, Lucide React, Tauri v2 API, React hooks. No new dependencies.

**Storage**: Existing localStorage history key extended additively; legacy defaults key read as fallback for legacy records. Existing language key remains independent.

**Testing**: Existing Vitest, Testing Library, jsdom; frontend build/lint and workspace Rust quality gates; desktop and standalone mock visual/keyboard checks.

**Target Platform**: Existing Windows, macOS, and Linux desktop application; Japanese and English at 960×640 and larger.

**Project Type**: Tauri desktop application with React frontend and standalone HTML mock.

**Performance Goals**: Language text updates within one second; no new search scans, backend calls, or network requests when editing settings.

**Constraints**: One save button and one persistence write per explicit commit; failures retain prior settings/history; cancellation discards drafts; ongoing searches retain submitted conditions; header/footer remain visible and keyboard focus stays in the dialog.

**Scale/Scope**: Two pages in the existing dialog, history limit 0–50, existing search options and four file formats, manual concurrency 2–32. No additional languages, CLI changes, separate window, or router.

## Constitution Check

| Principle / gate | Before research | After design |
|---|---|---|
| I. Language-directed quality | Pass: preserve English artifacts and Japanese/English UI | Pass: translation keys cover navigation, actions, errors, and accessible names; check unwanted tokens |
| II. Central constants | Pass: reuse existing keys, ranges, and defaults | Pass: new page IDs and translation keys belong in `src/constants/index.ts`, with constant reference comments |
| III. Header comments | Pass: required for new/changed code | Pass: describe behavior, argument/return types, and error handling; no per-file history |
| IV. Modular design and standards | Pass: existing frontend architecture retained | Pass: one small persistence module, existing hook/dialog ownership, no generic store or new dependency; build and lint required |
| V. Error handling and tests | Pass: save failure and cancellation require coverage | Pass: storage-first commit, validated drafts, migration and integration tests; workspace test/clippy/fmt gates required |
| Local-only processing and responsive search | Pass: no external communication | Pass: settings never restart a running scan or alter its captured query |

No constitutional exceptions or unresolved clarifications remain. This phase changes planning documents only; runtime quality gates are required after implementation.

## Project Structure

### Documentation (this feature)

```text
specs/020-organize-settings-pages/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/settings-ui-contract.md
└── checklists/requirements.md
```

`tasks.md` is produced later by `/speckit-tasks`.

### Source Code (repository root)

```text
src/
├── App.tsx
├── settings-core.ts                   # New combined record loading/validation/persistence
├── default-options-core.ts            # Retain pure option validation/normalization/reset
├── constants/index.ts
├── types/defaultOptions.ts
├── hooks/useSearchHistory.ts          # Own combined committed record and latest ref
├── hooks/useSearch.ts                 # Load same defaults; preserve submitted searches
├── hooks/useLocale.tsx                # Reuse immediate language behavior
└── components/settings/SettingsDialog.tsx
src-tauri/locales/{en,ja}.yml
tests/                                # Extend existing suites; add storage-boundary coverage
design/mainui/index.html
```

**Structure Decision**: Keep the current application layout. Introduce only `settings-core.ts` to join persistence that currently lives in separate history/default-option paths. Declare the combined record type there; reuse `DefaultSearchOptions`. Remove separate default-option persistence entry points and update every caller rather than retaining redundant wrappers. Pure option helpers do not import settings persistence, preventing a dependency cycle.

## Implementation Sequence

1. Add combined record loading and validation with backward-compatible reads and no startup writes. Preserve the existing history fields and legacy option normalization; embedded defaults become authoritative after the first successful write.
2. Extend `useSearchHistory` to expose committed defaults and one settings-save operation. Build candidates from its latest history ref. Explicit save writes first and publishes state only on success; ordinary accepted-search recording retains its existing memory fallback and always preserves committed preferences.
3. Replace App's independent history/default saves and duplicate default state with the combined operation. Apply defaults only after persistence succeeds. Initialize `useSearch` from the same loader; preserve keyword, directory, results, selection, and in-flight scan conditions.
4. Add dialog-local page/draft state, active-only validation, remembered worker count, draft-only reset, one save action, and all discard exits. Initialize drafts only on opening, preserve them across language/page changes, and synchronously guard saving against duplicate events.
5. Add localized labels, visible fixed header/footer with scrolling content, semantic page navigation, focus containment and focus restoration. Synchronize the existing standalone mock using `syncing-mainui-mock`, including startup restoration and failure behavior.
6. Run the automated and manual checks in [quickstart](quickstart.md), then review constants, headers, translations, and the complete acceptance matrix.

## Validation and Material Risks

Extend existing option, history, settings, App integration, and mock suites. Add storage-boundary tests for legacy/extended loading and rejected writes. Verify no partial state updates, latest accepted history retained during a draft, no dual writes, manual count retention, all exit paths, localized errors, and next-search-only application. Avoid testing implementation details when observable behavior proves the requirement.

The existing history updater publishes memory before persistence; it must not be reused unchanged for explicit settings saves. Both App and `useSearch` currently load defaults independently and must move to the same loader. Ordinary history writes must retain the added fields. A storage replacement provides a single-key commit; power-loss durability and independent-window coordination are outside this feature's scope.
