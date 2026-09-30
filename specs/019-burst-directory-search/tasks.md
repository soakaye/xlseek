<!--
Purpose: Defines dependency-ordered implementation and validation tasks for Burst directory discovery in the desktop app and CLI.
Inputs: Feature specification, implementation plan, research, data model, contracts, and constitution (Markdown); output: an executable task checklist.
Errors: This document has no runtime behavior; failed prerequisites or quality gates block the dependent implementation or completion report.
-->
# Tasks: Burst Directory Search

**Input**: Design documents in `specs/019-burst-directory-search/`.

**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md), [data-model.md](./data-model.md), [desktop contract](./contracts/desktop-search-mode.md), [CLI contract](./contracts/cli-directory-mode.md).

**Organization**: Shared foundations precede US1 (desktop, P1) and US2 (CLI, P2). Tests accompany every changed module as required by constitution principle V; this checklist does not execute them.

**Format**: `- [ ] Tnnn [P?] [USn?] Description with exact file paths`. `[P]` indicates disjoint work that can proceed after the stated prerequisites. Paths are relative to the repository root; explicitly identified new files must be created.

**Implementation rules**: Every new or changed code element needs a header explaining purpose, typed inputs/outputs, and failure behavior. Extract fixed values other than `0` and `""` into the appropriate constants module and annotate references. English is the default; maintain approved Japanese locale resources. Preserve workbook parsing, filtering, output formats, and local-only processing.

## Phase 1: Setup

**Purpose**: Establish traceability and reusable validation fixtures in the existing repository.

- [ ] T001 Record the FR-001–FR-015 and SC-001–SC-009 validation matrix, linking each requirement to the tasks below, in `specs/019-burst-directory-search/quickstart.md`; retain separate desktop and CLI acceptance checkpoints.
- [ ] T002 Create reusable nested, wide, empty, overlapping-root, and controlled-error fixture helpers in new `crates/core/tests/common/burst_fixtures.rs` and new `crates/cli/tests/common/burst_fixtures.rs`, reusing `tests/fixtures/sample_report.xlsx`; use deterministic error injection where OS permissions cannot reproduce failures reliably.

## Phase 2: Foundational — Shared Discovery

**Purpose**: Deliver mode/count models and safe traversal used by both stories.

- [x] T003 Define shared mode tokens, Sequential visitor count, Automatic minimum/maximum, custom bounds, queue capacity, and retry scheduling constants in `crates/core/src/constants.rs`; implement model validation and a testable resolver in `crates/core/src/models/mod.rs`: mode is "`sequential` or `burst`", Automatic is "`null` or omitted" with "a minimum of 2 and a maximum of 8", Custom is "integer from 2 through 32", and fractional/out-of-range input is invalid; default older query fields to Sequential and Automatic.
- [ ] T004 Add deterministic discovery tests in new `crates/core/tests/burst_discovery.rs` using T002 fixtures: nested coverage, no duplicate files, semantic equivalence, Sequential maximum one operation, Burst overlap with available work, effective count boundaries 2 and 32, Automatic resolution, filters, and directory symlinks; test observed concurrency with barriers/counters rather than elapsed-time thresholds (depends on T003).
- [ ] T005 Add fault and shutdown tests in new `crates/core/tests/burst_shutdown.rs` for full pending/file queues, locally suspended work, deep iteration, child registration before parent retirement, root versus descendant failures, worker failure, and cancellation; specify no false Completed outcome (depends on T003).
- [x] T006 Implement streaming sequential traversal and the shared discovery interface in new `crates/core/src/search/discovery.rs`, exporting it through `crates/core/src/search/mod.rs`; accept roots, mode/effective count, cancellation, and file/issue sinks; preserve filtering and do not follow directory symlinks; report fatal roots and nonfatal descendant errors with path, stage "discovery", code "permission denied or read failed", and "optional diagnostic text" cause (depends on T004, T005).
- [x] T007 Implement bounded Burst scheduling in `crates/core/src/search/discovery.rs`: register children before retiring parents, synchronize queue/accounting, wake waiters on work/completion/cancellation/failure, and finish only at zero outstanding work; model state as "queued, visiting, completed, failed, or cancelled"; use iterative streaming continuation frames on frontier overflow, counting simultaneous directory operations rather than suspended frames; document depth-dependent memory/open iterators and report handle exhaustion (depends on T006).
- [x] T008 Implement cancellation-aware `try_send` delivery retaining unsent paths, receiver-disconnect handling, worker join failure propagation, and cooperative checks between filesystem operations in `crates/core/src/search/discovery.rs`; validate all T004/T005 cases and avoid publishing success after cancellation or fatal failure (depends on T007).
- [ ] T009 Integrate shared discovery into `crates/cli/src/lib.rs` with existing default Sequential behavior, preserving same-file deduplication across overlapping roots, workbook consumers, diagnostics, output, and exit policies; update `crates/cli/tests/cli_search.rs` for default-path regressions so CLI option wiring can proceed independently of desktop integration (depends on T008).

**Checkpoint**: Shared models/discovery and default CLI traversal pass focused tests. Both stories can now start; no foundation task requires desktop settings or new CLI options.

## Phase 3: User Story 1 — Desktop Settings (P1, MVP)

**Goal**: Save Sequential/Burst and Automatic/custom count choices, apply them to future searches, and display discovery failures.

**Independent test**: Search the same nested fixture in both modes and compare semantic match multisets; verify actual visitor bounds, saved custom 2–32 count across restart, legacy migration, reset, count retention in Sequential, warnings, and cancellation.

### Tests

- [ ] T010 [P] [US1] Extend `tests/default-options.test.ts` and `tests/settings-default-options.test.tsx` to cover legacy objects, invalid new fields without preference loss, Custom boundaries, fractions/nonnumeric input, mode switching, save/reopen, and reset; assert new searches snapshot choices and active searches retain their original choices.
- [ ] T011 [P] [US1] Add desktop request/event tests in new `tests/burst-search.test.tsx` for query defaults, mode/count payloads, issue display/clearing, cancellation, and compatible match/progress events; extend `tests/app-default-options.test.tsx` for persistence and application of both settings.
- [ ] T012 [P] [US1] Add engine integration tests in new `crates/core/tests/burst_engine.rs` for mode parity, nested/empty trees, exact-once processing, descendant issues, count limits, final total accounting, and shutdown with a full file channel; add IPC default/invalid-field tests alongside handlers in `src-tauri/src/commands/search_cmd.rs`.

### Implementation

- [x] T013 [US1] Add frontend mode/count defaults, limits, locale keys, and event tokens to `src/constants/index.ts`; extend `src/types/search.ts` and `src/types/defaultOptions.ts`: query count is "integer from 2 through 32, `null`, or omitted"; saved count is "integer from 2 through 32 or `null`", saved mode is "Required in the canonical saved object; defaults to `sequential` when absent or invalid on load" (depends on T010, T011).
- [x] T014 [US1] Implement backward-compatible normalization and canonical save/reset in `src/default-options-core.ts`: "Missing or invalid saved values normalize to `null`" for count; "Preserve values when either new field is missing or invalid. Existing validation remains in place for other fields"; retain custom count when switching to Sequential (depends on T013).
- [ ] T015 [P] [US1] Add English/Japanese settings, Automatic/Custom labels, range validation, and discovery-warning translations in `src-tauri/locales/en.yml` and `src-tauri/locales/ja.yml`; extend `tests/catalog.test.ts` for catalog coverage (depends on T013).
- [x] T016 [US1] Add labeled Sequential/Burst and Automatic/Custom controls to `src/components/settings/SettingsDialog.tsx`, enable numeric input only for Burst+Custom, reject invalid values before save, preserve custom selection across mode switches, and support keyboard interaction and associated error text (depends on T014, T015).
- [x] T017 [US1] Wire persistence and future-search snapshots through `src/App.tsx` and `src/hooks/useSearch.ts`; omitted query mode defaults to Sequential, omitted/null count means Automatic, and changing settings must not mutate an active search (depends on T016).
- [x] T018 [US1] Replace desktop serial producer traversal with shared discovery in `crates/core/src/search/engine.rs`, preserve parallel workbook consumers and match semantics, finalize totals only after discovery, release receivers before producer joins when consumers stop, and propagate failure without false completion (depends on T012).
- [x] T019 [US1] Bridge validated mode/count and discovery issues through `src-tauri/src/commands/search_cmd.rs`, defining stable `search-issue` tokens in `src-tauri/src/constants.rs`; reject unknown explicit mode/invalid count, emit path/stage/code only, and keep `search-match`/`scan-progress` shapes compatible (depends on T018).
- [x] T020 [US1] Extend issue state/subscriptions in `src/hooks/useSearch.ts` and issue types in `src/types/search.ts`; clear warnings at the next search, dispose listeners safely, and resolve localized messages containing the failed path (depends on T017, T019).
- [x] T021 [US1] Create a warnings view in new `src/components/results/SearchWarnings.tsx` and integrate it in `src/App.tsx`, showing accessible results alongside failed-folder paths without blocking search (depends on T020).
- [x] T022 [US1] Apply the `syncing-mainui-mock` skill to synchronize settings controls, validation, save/reset, count retention, and warnings in `design/mainui/index.html` with T016/T021; preserve the standalone prototype's operation (depends on T021).
- [ ] T023 [US1] Run the desktop independent checks and focused frontend/core/IPC tests from T010–T012; record results and any platform limitations in `specs/019-burst-directory-search/quickstart.md`, including settings restart, invalid input, nested parity, issue paths, and cancellation (depends on T022).

**Checkpoint**: US1 is an independently demonstrable MVP with default CLI behavior preserved.

## Phase 4: User Story 2 — CLI Options (P2)

**Goal**: Choose discovery mode/count for one invocation without reading desktop preferences.

**Independent test**: Compare Sequential, Burst Automatic, explicit custom Burst, count-only Burst, and omitted-options semantic results on nested/overlapping roots. Confirm precedence, boundaries, invalid input, exit status, and unchanged output formats.

### Tests

- [ ] T024 [P] [US2] Extend parser tests in `crates/cli/src/args.rs` for separate/equals syntax, no short aliases, mode "`sequential` or `burst`", count "integer from 2 through 32 or absent", both omissions defaulting to Sequential, count-only inference, explicit Sequential conflict regardless of option order, duplicate/missing/unknown values, fractions, and boundaries 1/2/32/33.
- [ ] T025 [P] [US2] Extend `crates/cli/tests/cli_search.rs` and `crates/cli/tests/cli_output.rs` for mode/count parity, default independence from GUI settings, overlapping roots, single-file acceptance, descendant/fatal errors, CSV stdout/file and XLSX compatibility, and exit statuses 0/1/2.

### Implementation

- [x] T026 [US2] Define long option names and CLI diagnostic/help constants in `crates/cli/src/constants.rs`, then extend `CliOptions` and parsing in `crates/cli/src/args.rs`: "`--directory-mode` value when present; otherwise `burst` if `--burst-workers` is present, or `sequential` if neither option is present"; count applies once to all roots, absent means Automatic, and explicit Sequential/count conflicts are usage errors (depends on T024).
- [x] T027 [P] [US2] Add English/Japanese help and validation diagnostics in `crates/core/locales/en.yml` and `crates/core/locales/ja.yml` for both long options, defaults, 2–32 range, and conflicts; preserve the existing `--help` language-only companion rule (depends on T024).
- [x] T028 [US2] Pass parsed mode/count into shared discovery in `crates/cli/src/lib.rs`, resolve count once per invocation, retain existing root/file-identity issue policy and outputs, accept mode/count for single files without scheduling directory work, and avoid desktop settings access (depends on T025, T026, T027).
- [ ] T029 [US2] Run parser/search/output tests and the five valid syntax cases in `specs/019-burst-directory-search/contracts/cli-directory-mode.md`; exercise invalid counts/conflicts and both localized help catalogs, recording results in `specs/019-burst-directory-search/quickstart.md` (depends on T028).
- [ ] T030 [US2] Validate CLI and desktop use the same effective-count semantics and semantic match multiset in `crates/cli/tests/cli_search.rs` and `crates/core/tests/burst_engine.rs`; retain independent entry-point defaults and output order tolerance (depends on T029; when running alongside US1, wait for T018 before the engine comparison).

**Checkpoint**: US2 works independently after the shared foundation; both entry points expose the confirmed count behavior.

## Phase 5: Polish and Cross-Cutting Validation

- [x] T031 Document Settings and CLI mode/count usage, Sequential defaults, Automatic/custom limits, and directory-only scope in `README.md`, referencing `specs/019-burst-directory-search/contracts/` (depends on T023, T030).
- [ ] T032 Review all changed files listed in `specs/019-burst-directory-search/plan.md` for three-part headers, constant extraction/reference comments, localization, unsupported tokens, bounded resources, failure propagation, and unrelated changes; resolve findings and record the review in `specs/019-burst-directory-search/quickstart.md` (depends on T031).
- [x] T033 Run `npm test`, `npm run lint`, `npm run build`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo fmt --check` from the root containing `package.json` and `Cargo.toml`; fix feature-related failures and record command results in `specs/019-burst-directory-search/quickstart.md` (depends on T032).
- [ ] T034 Execute the complete desktop/CLI end-to-end guide in `specs/019-burst-directory-search/quickstart.md`, including nested matches, count boundaries, warnings, cancellation, migration, and `design/mainui/index.html` parity; record observed results and any unexecuted platform cases without claiming they passed (depends on T033).
- [ ] T035 Reconcile requirement coverage and implementation evidence in `specs/019-burst-directory-search/quickstart.md` against `specs/019-burst-directory-search/spec.md` and `specs/019-burst-directory-search/checklists/requirements.md`; confirm every FR/SC has evidence and update completion markers in `specs/019-burst-directory-search/tasks.md` only for completed work (depends on T034).

## Dependencies and Execution Order

```text
Setup T001–T002
  → Shared foundation T003 → {T004, T005} → T006 → T007 → T008 → T009
    → US1 T010–T023 → desktop MVP
    → US2 T024–T030 → CLI increment
      → Both story checkpoints → T031 → T032 → T033 → T034 → T035
```

- US1 and US2 depend on the complete foundation. US2 option implementation does not depend on desktop settings; T030's shared-engine comparison waits for T018.
- Within US1: test tasks T010/T011/T012 use disjoint files; T013→T014→T016→T017 controls UI wiring, T015 can proceed alongside T014; T018→T019 controls the backend. Both paths join at T020→T021→T022→T023.
- Within US2: T024/T025 use disjoint files; after parser tests, T026 and T027 can proceed together; integration T028 waits for both and T025.
- T023 and T029 write the same quickstart artifact, so serialize their evidence updates. T030 also shares test files with T012/T025 and must follow their edits. Final quality commands run after integration to avoid build/test resource contention.
- Where tests are specified, establish the expected failure before implementing the changed behavior, then rerun the focused cases. Do not mark future work complete from design review alone.

## Parallel Examples

### US1

After T009, T010 (settings/storage tests), T011 (request/event tests), and T012 (engine/IPC tests) can proceed concurrently. After these tests exist, the UI branch T013–T017 and engine branch T018–T019 can proceed independently; coordinate shared interface definitions before T019. T015 catalog work is disjoint from T014 storage normalization.

### US2

After T009, T024 parser tests and T025 search/output tests can proceed concurrently. After T024, implement T026 options/constants alongside T027 locale catalogs; both join at T028. This story may run alongside US1 except for the explicit T030 engine-test dependency and shared evidence updates.

## Implementation Strategy

1. Complete setup and shared discovery first, validating count resolution, progress, error handling, and full-queue shutdown.
2. Deliver US1 as the MVP: persistent desktop controls, warnings, mock synchronization, and independent acceptance evidence.
3. Deliver US2 as an incremental CLI feature: invocation-only mode/count, inference/conflict rules, help, and output parity.
4. Complete repository quality gates and end-to-end evidence before reporting implementation complete. No fixed speedup target applies; use deterministic concurrency observations rather than a timing threshold.

## Requirement Coverage

| Requirements / outcomes | Primary tasks |
|---|---|
| FR-001, FR-004, FR-005; SC-003, SC-005 | T003–T008, T018, T028 |
| FR-002, FR-003, FR-009, FR-013; SC-001, SC-008 | T010, T013–T017, T023 |
| FR-006, FR-007; SC-002, SC-006 | T004, T009, T012, T018, T025, T030 |
| FR-008; SC-004 | T005–T006, T019–T021, T025, T028 |
| FR-010 | T005, T008, T012, T018, T023 |
| FR-011, FR-012, FR-014; SC-007, SC-009 | T024–T029 |
| FR-015 | T003, T010, T014, T016, T019, T024, T026 |
| Constitution, UI mock, complete acceptance evidence | T022, T031–T035 |
