<!--
Purpose: Plans implementation of bounded Burst subfolder discovery across the desktop app and standalone CLI.
Inputs: The approved feature specification, constitution, and Phase 0 research (Markdown); output: a Phase 1 implementation design and artifact map.
Errors: This document has no runtime behavior; an unresolved requirement or unjustified constitution violation blocks implementation planning.
-->
# Implementation Plan: Burst Directory Search

**Branch**: `019-burst-directory-search` | **Date**: 2026-09-30 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/019-burst-directory-search/spec.md`

## Summary

Add persistent desktop settings and command-line options to choose sequential or Burst directory discovery and set the Burst parallel count. A shared core discovery service assigns each nested subfolder once per traversal root, preserving file deduplication across overlapping CLI roots. Sequential mode uses one directory visitor; Burst mode uses a bounded worker pool sized by Automatic or a validated custom count and a bounded pending frontier. Both modes feed the existing workbook search pipeline, preserve match semantics, cancellation, symlink policy, and CLI output behavior, and report inaccessible subfolders.

The confirmed custom range is 2 through 32. A CLI count without a mode selects Burst; explicit Sequential with a count is a usage error. Omitting both options selects Sequential independently of desktop settings.

The choice controls directory traversal and eligible-file discovery. Existing concurrent workbook parsing is retained in both modes. Phase 0 decisions are recorded in [research.md](./research.md); Phase 1 interfaces and validation are in [data-model.md](./data-model.md), [contracts/](./contracts/), and [quickstart.md](./quickstart.md).

## Technical Context

**Language/Version**: Rust 2021; TypeScript 5.5; React 18; Tauri v2

**Primary Dependencies**: Existing `walkdir`, `rayon`, `same-file`, `serde`, React, and Tauri APIs; standard-library synchronization for bounded directory scheduling. No new package is planned.

**Storage**: Desktop mode and Automatic/custom Burst count in local storage (`xlseek.default_search_options`); CLI mode and count are invocation-local and are not persisted.

**Testing**: Rust unit/integration tests for discovery, cancellation, progress, effective count and bounds, CLI parsing/output, and IPC defaults; Vitest settings, custom-count validation, and migration tests; `npm run build`, `npm run lint`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo fmt --check` during implementation.

**Target Platform**: Windows, macOS, and Linux desktop app plus standalone CLI on those platforms.

**Project Type**: Shared Rust search library, standalone CLI, Tauri desktop application, and React UI.

**Performance Goals**: Automatic Burst mode uses 2 to 8 directory visitors based on available parallelism; a custom setting uses 2 to 32. Active directory visitors never exceed the effective count, and queued directory jobs remain bounded. No fixed elapsed-time reduction target is required.

**Constraints**: Keep all search data local; do not follow directory symlinks; preserve existing workbook search semantics and CLI output/exit behavior; reject invalid custom counts and mode/count conflicts; avoid blocking cancellation on a full queue; keep desktop UI responsive. Extract new constants into the established constants modules and annotate references. Keep the standalone HTML UI mock synchronized with settings changes.

**Scale/Scope**: Deep and wide directory trees, including nested folders and multiple CLI roots, without creating one operating-system thread per folder. Workbook and file extension support remains unchanged.

## Constitution Check

*GATE: Pass before Phase 0 research; re-evaluated after Phase 1 design.*

| Principle or constraint | Pre-research gate | Design evidence |
|---|---|---|
| I. Language-directed quality | PASS | New documentation, comments, and default UI text use English; Japanese locale resources remain Japanese. Generated artifacts are checked for garbled text and placeholder tokens. |
| II. No hardcoded constants | PASS | Mode values, CLI option names, event names, labels, automatic/custom count bounds, and queue bounds belong in `crates/core/src/constants.rs`, `crates/cli/src/constants.rs`, `src-tauri/src/constants.rs`, or `src/constants/index.ts` as appropriate; reference comments are required at use sites. |
| III. Comprehensive headers | PASS | New and changed files, types, and functions require purpose, typed inputs/outputs, and error behavior in header comments. No in-file change histories are added. |
| IV. Modular design and standards | PASS | One core discovery module owns traversal; GUI and CLI adapt its output. `rustfmt`, Clippy, TypeScript, and ESLint gates are planned. |
| V. Error handling and tests | PASS | Root errors are fatal; descendant errors are reported without losing accessible results. Invalid counts and mode conflicts are rejected, while legacy saved settings are migrated. Cancellation and worker failure paths are explicit, with targeted unit and integration tests. |
| Resource, privacy, and UI constraints | PASS | Active workers and queued jobs are bounded, file delivery remains bounded, no file data leaves the device, and UI work stays off the UI thread. |

No unresolved technical context item or unjustified gate violation remains after [research.md](./research.md).

### Post-design re-check

| Principle or constraint | Result | Phase 1 confirmation |
|---|---|---|
| I. Language-directed quality | PASS | Contracts, model, and quickstart use English and reserve translated text for locale catalogs. |
| II. No hardcoded constants | PASS | The contract identifies every new stable token and numeric count bound and places them in the existing constants ownership scheme. |
| III. Comprehensive headers | PASS | All generated artifacts have three-part file headers; implementation tasks must extend that rule to code elements. |
| IV. Modular design and standards | PASS | The model separates settings, invocation mode, directory jobs, and issue delivery; both entry points share traversal. |
| V. Error handling and tests | PASS | Contracts define invalid mode/count combinations, unreadable folders, cancellation, and completion behavior; quickstart covers end-to-end and quality gates. |
| Resource, privacy, and UI constraints | PASS | Bounded scheduling, local-only processing, and mock synchronization are included in the design. |

## Project Structure

### Documentation (this feature)

```text
specs/019-burst-directory-search/
├── spec.md
├── checklists/requirements.md
├── plan.md
├── research.md
├── data-model.md
├── contracts/
│   ├── desktop-search-mode.md
│   └── cli-directory-mode.md
├── quickstart.md
└── tasks.md                    # Created by /speckit-tasks, not this phase
```

### Source Code (repository root)

```text
crates/core/src/
├── constants.rs                # Shared mode tokens, worker-count bounds, and scheduler bounds
├── models/mod.rs               # DirectorySearchMode, count, query, and issue models
└── search/
    ├── discovery.rs            # Shared sequential/Burst directory discovery
    └── engine.rs               # Existing workbook pipeline integration
crates/cli/src/
├── args.rs                     # --directory-mode and --burst-workers parsing
├── constants.rs                # CLI-owned text and error constants
└── lib.rs                      # Shared discovery adapter; existing output/exit policy
crates/core/locales/{en,ja}.yml # CLI help and diagnostics
src-tauri/src/
├── constants.rs                # Desktop event names and other GUI constants
├── commands/search_cmd.rs      # Query and discovery-issue event bridge
└── locales/{en,ja}.yml        # Desktop settings and warning text
src/
├── constants/index.ts          # UI mode/count tokens, bounds, defaults, event names
├── types/{search,defaultOptions}.ts
├── default-options-core.ts     # Backward-compatible saved-setting migration
├── hooks/useSearch.ts          # Search request and issue event state
├── App.tsx                     # Settings persistence and immediate application
└── components/settings/SettingsDialog.tsx
design/mainui/index.html       # Standalone settings prototype sync
tests/                         # Frontend option and settings tests
crates/core/tests/              # Shared discovery integration cases
crates/cli/tests/               # CLI mode, parity, and exit behavior cases
```

**Structure Decision**: The shared core owns directory traversal, mode semantics, and effective-count resolution. The desktop and CLI retain their existing workbook consumers and presentation layers. The desktop settings and CLI options expose both mode and Burst parallel count.

## Complexity Tracking

No constitution violations or exceptions are planned.

The worker count bounds simultaneous directory operations, and the pending frontier is bounded. Iterative suspended continuations use memory and open directory iterators proportional to depth; same-file identity tracking scales with discovered files. Overlapping CLI roots may revisit directories while file deduplication prevents duplicate workbook processing. Scheduler accounting and shutdown invariants are defined in research decisions 9 and 10.
