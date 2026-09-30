<!--
Purpose: Defines feature data, validation rules, and state transitions for directory search modes and discovery issues.
Inputs: The approved specification and research decisions (Markdown); output: a data model for implementation and tests.
Errors: This document has no runtime behavior; invalid values and failure handling are specified for the implementing components.
-->
# Data Model: Burst Directory Search

## DirectorySearchMode

| Field | Type | Meaning |
|---|---|---|
| Value | `sequential` or `burst` | Determines how directory entries are discovered. |

- The canonical default is `sequential`.
- The choice changes directory traversal only. Existing workbook parsing may remain concurrent in either mode.
- Rust serialization and TypeScript values use the same stable tokens. New fixed strings are owned by the appropriate constants modules.
- An omitted mode in an older desktop search request resolves to `sequential`. An invalid explicit IPC value is rejected as a search request error. An invalid saved desktop value resolves to `sequential` while other valid options are preserved. An invalid CLI value is a usage error.

## BurstParallelCount

| Value | Type | Meaning |
|---|---|---|
| Automatic | `null` or omitted | Resolve a count from available parallelism, with a minimum of 2 and a maximum of 8. |
| Custom | integer from 2 through 32 | Use exactly this many directory visitors as the upper bound. |

The shared core resolves the effective count once when a Burst search starts. The count bounds directory visits, not workbook parsing. Sequential mode uses one directory visitor and ignores a saved custom Burst count without deleting it. New numeric bounds are named constants in the code. A custom value outside the range or with a fractional component is invalid.

## DesktopSearchQuery

Extends the existing `SearchQuery` model in `crates/core/src/models/mod.rs` and `src/types/search.ts`.

| Field | Type | Rule |
|---|---|---|
| `directory_mode` | `DirectorySearchMode` | Optional in older incoming requests; defaults to `sequential`. A new search snapshots the current saved setting. |
| `burst_workers` | integer from 2 through 32, `null`, or omitted | Custom count or Automatic. A new search snapshots the current choice; Sequential mode ignores it. |
| Existing search fields | Existing types | No change to keyword, path, extensions, or matching flags. |

The mode and parallel count are fixed for an active search. Saving new settings during a search affects the next search, not work already running. An invalid explicit count in an IPC request is rejected as an invalid search request.

## DefaultSearchOptions

Extends the existing desktop settings object in `src/types/defaultOptions.ts`, stored under the existing key in local storage.

| Field | Type | Rule |
|---|---|---|
| `directory_mode` | `DirectorySearchMode` | Required in the canonical saved object; defaults to `sequential` when absent or invalid on load. |
| `burst_workers` | integer from 2 through 32 or `null` | Required in the canonical saved object; `null` means Automatic. Missing or invalid saved values normalize to `null`. |
| Existing options | Existing types | Preserve values when either new field is missing or invalid. Existing validation remains in place for other fields. |

**State transition**: Load saved object → validate existing fields → merge or normalize mode and count → show settings → save canonical object → apply both to subsequent search queries. Reset to defaults selects `sequential` and Automatic. Switching to Sequential retains a custom Burst count for later reuse.

## CliOptions

Extends the existing parsed invocation in `crates/cli/src/args.rs`.

| Field | Type | Rule |
|---|---|---|
| `directory_mode` | `DirectorySearchMode` | `--directory-mode` value when present; otherwise `burst` if `--burst-workers` is present, or `sequential` if neither option is present. Applies to all directory roots in one invocation. |
| `burst_workers` | integer from 2 through 32 or absent | `--burst-workers` value for one invocation; absent means Automatic when Burst is selected. |
| Existing options | Existing types | Retain query, paths, output format, language, and search flags. |

The CLI does not read or write the desktop setting. If `--burst-workers` is present without `--directory-mode`, the effective mode is Burst. Combining it with explicit Sequential mode is a usage error. A single-file input accepts the option but has no subfolder discovery to schedule.

## SubfolderDiscoveryTask

An internal, short-lived work item representing one directory path. It is not persisted or sent to the UI.

| Field | Type | Rule |
|---|---|---|
| `path` | local path | One directory at any depth; do not follow directory symlinks. |
| `state` | queued, visiting, completed, failed, or cancelled | A task visits direct entries once, then leaves the active set. |

**Transitions**: queued → visiting → completed or failed; queued or visiting → cancelled after a cancellation request. A discovered child creates one outstanding task, queued when space is available or continued locally with its parent suspended. File entries go to the existing bounded workbook pipeline after extension and temporary-file filters. Simultaneous directory operations never exceed the effective Burst parallel count, the pending queue remains bounded, and total job accounting reaches zero only after every accepted task finishes or is cancelled.

## DiscoveryIssue

Scheduler accounting includes queued work, active work, and locally suspended continuations. Register children before retiring their parent; only zero outstanding work signals completion. The parallel count bounds simultaneous directory operations, excluding suspended frames. Overflow uses iterative streaming traversal; continuation memory and open iterators scale with depth. Overlapping CLI roots may revisit directories, but same-file filtering ensures each workbook is processed once.

A nonfatal descendant-directory failure. The shared core reports it once per failed directory; each entry point presents it using its existing conventions.

| Field | Type | Rule |
|---|---|---|
| `path` | local path | Identifies the failed subfolder. |
| `stage` | discovery | Stable stage identifier. |
| `code` | permission denied or read failed | Stable error category; the UI resolves the user-facing message from locale resources. |
| `cause` | optional diagnostic text | May be used by CLI diagnostics; is not required in the desktop event payload. |

An invalid or unreadable search root is a fatal request error rather than a `DiscoveryIssue`. Descendant issues do not stop accessible work. The desktop uses a separate issue event; CLI keeps its issue list, diagnostics, and exit-status policy.

## DiscoverySummary and ScanProgress

Discovery returns or internally tracks visited-directory count, discovered-file count, issue count, and cancellation/completion state. Counters never decrease. The existing `ScanProgress` payload remains compatible: `total_files` becomes final only after discovery finishes, and the final completed state is emitted only after directory workers and workbook consumers finish. Cancellation never reports completion. Match IDs and delivery order may differ between modes; semantic match fields must be identical as a multiset.
