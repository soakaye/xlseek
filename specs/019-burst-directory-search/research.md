<!--
Purpose: Records local-code research and design decisions for Burst directory discovery before implementation planning.
Inputs: The approved feature specification, repository source files, and constitution (text); output: resolved technical decisions in Markdown.
Errors: This document performs no runtime work; an unresolved decision blocks the implementation plan until researched or clarified.
-->
# Research: Burst Directory Search

## Decision 1: Define the mode around directory discovery

**Decision**: Sequential and Burst modes control how subfolders are visited and eligible file paths are discovered. Both modes retain the existing concurrent workbook parsing pipeline.

**Rationale**: The active GUI path uses one serial `WalkDir` producer and Rayon file consumers (`crates/core/src/search/engine.rs`). The CLI has the same separation (`crates/cli/src/lib.rs`). Serializing workbook parsing would change established behavior and slow the existing mode. The specification now names file discovery explicitly.

**Alternatives considered**: Make each folder worker parse all workbooks before visiting another folder. Rejected because it changes current search semantics and weakens the existing pipeline.

## Decision 2: Share a bounded discovery service

**Decision**: Add a focused discovery module in `crates/core/src/search/` that accepts one or more roots, a `DirectorySearchMode`, an effective Burst parallel count, a cancellation signal, and file/issue sinks. The GUI and CLI retain their own workbook consumers and output flows but use the same directory discovery behavior.

**Rationale**: Both entry points currently duplicate recursive `WalkDir` traversal. A shared service prevents diverging mode behavior and keeps CLI output and desktop progress contracts intact. Sequential mode uses one directory visitor; Burst mode uses a bounded number of visitors with a bounded pending frontier. Each directory is assigned once, and workers inspect only its direct entries, so nested files are not rediscovered by ancestor jobs. When the pending frontier is full, a worker visits a discovered child in its own local work loop rather than blocking every worker on an enqueue; this preserves progress without unbounded global queuing.

**Alternatives considered**: Duplicate a Burst traversal in both entry points. Rejected because cancellation, symlink handling, and deduplication would drift. Recursively spawn one operating-system thread per folder. Rejected because the user selected a concurrency limit and the constitution requires bounded resource use.

## Decision 3: Keep cancellation and completion accounting explicit

**Decision**: Track queued and active directory jobs until the count reaches zero. Publish discovery completion only after all jobs have finished and all discovered files have been handed to the workbook pipeline. Check cancellation before visiting entries, enqueuing children, and delivering files. Use cancellation-aware delivery to the bounded file channel; release the receiver before joining producers when consumers stop.

**Rationale**: The current GUI producer can block on `sync_channel::send` while cancellation stops consumers (`crates/core/src/search/engine.rs`). Its join result is also ignored. The new design must prevent a full queue from hanging cancellation and must not report `Completed` after a worker fails.

**Alternatives considered**: Reuse blocking `send` and ignore worker joins. Rejected because cancellation can hang or report false success. Use an unbounded file channel. Rejected because large directory trees could consume excessive memory.

## Decision 4: Preserve file identity and error policy

**Decision**: Do not follow directory symlinks, preserve existing extension and Excel lock-file filtering, and deliver each discovered file once per search. An invalid root is fatal. A descendant directory read failure is nonfatal and carries its path and a stable error code. The GUI exposes discovery issues to users; the CLI preserves its existing partial-issue exit behavior and diagnostics.

**Rationale**: Current walkers use `follow_links(false)`. CLI already records issues and deduplicates overlapping roots with `same_file::Handle`; GUI currently skips walker errors silently. The specification requires visible failed locations and match parity in both modes.

**Alternatives considered**: Ignore descendant errors in both modes. Rejected because failed locations would be hidden. Follow symlinks. Rejected because it changes existing coverage and can create cycles or duplicate files.

## Decision 5: Add a backward-compatible mode value to each entry point

**Decision**: Use a shared Rust `DirectorySearchMode` with `sequential` and `burst` serialized values. Add defaulted mode and optional `burst_workers` fields to the shared search query. The desktop settings persist both choices and send them in each search request. Add CLI `--directory-mode sequential|burst` and `--burst-workers N`, with no short aliases. The parallel count applies to one invocation and all its directory inputs; a single-file CLI input accepts the option but has no directory traversal.

**Rationale**: The GUI query and CLI query use the same core model (`crates/core/src/models/mod.rs`). Serde defaulting preserves older IPC callers. The CLI parser already supports long value options with separate or `=` values (`crates/cli/src/args.rs`), and default sequential preserves existing commands. `--burst-workers N` implies Burst mode when no mode is supplied; explicit Sequential mode with a count is a usage error, so the count is never silently ignored.

**Alternatives considered**: Read the desktop setting from the CLI. Rejected by clarification: CLI defaults independently. Add a separate `--burst` flag. Rejected because an explicit two-value option is clearer and easier to extend without conflicting flags.

## Decision 6: Migrate saved desktop options without losing preferences

**Decision**: Add `directory_mode` and `burst_workers` to desktop default options. `burst_workers` is `null` for Automatic or an integer from 2 through 32 for Custom. When loading an old saved object that lacks either field, merge in `sequential` and Automatic while preserving every existing option. Normalize an invalid mode to `sequential` and an invalid count to Automatic without discarding other valid preferences. Saving writes the canonical object. The settings dialog applies the saved choices to subsequent searches in the current session and after restart.

**Rationale**: The current validator in `src/default-options-core.ts` treats schema mismatch as a full reset. A required new field would erase existing saved choices. The mode belongs in the existing settings flow (`src/components/settings/SettingsDialog.tsx`, `src/App.tsx`, `src/hooks/useSearch.ts`).

**Alternatives considered**: Reset all options on upgrade. Rejected because it discards user preferences. Add a separate storage key. Rejected because it complicates settings persistence and reset behavior.

## Decision 7: Resolve and validate the parallel count centrally

**Decision**: One core resolver converts Automatic to a bounded count based on available parallelism, with a minimum of 2 and an automatic maximum of 8. Custom values are whole numbers from 2 through 32. Both limits and the automatic cap are named constants. The resolved count limits active directory visitors only; Sequential mode always uses one visitor and retains any saved custom Burst choice for later use.

**Rationale**: Clarification confirmed the custom range of 2 through 32 and that a CLI count alone selects Burst, with explicit Sequential/count conflicts rejected. A user-specified count must produce the same bound in GUI and CLI. Automatic keeps existing usage simple and bounded. A documented upper bound protects memory and filesystem resources. The desktop rejects invalid input before save; CLI rejects it as a usage error; old or invalid saved values normalize to Automatic.

**Alternatives considered**: Permit any positive integer. Rejected because excessively high values conflict with the constitution's resource limit. Force every user to provide a number. Rejected because existing searches and saved settings need a safe default. Silently clamp invalid custom values. Rejected because it hides input mistakes.

## Decision 8: Keep user-facing contracts and verification focused

**Decision**: Keep existing `search-match` and `scan-progress` payloads compatible; add a separate discovery issue event for path-aware warnings. Update English and Japanese locale catalogs and the standalone UI mock when the settings screen changes. Compare semantic match multisets, not row order or generated IDs, across modes. Do not set a fixed elapsed-time improvement threshold.

**Rationale**: Burst mode changes discovery order, so match order can vary. The specification requires identical matches and bounded overlap, not identical output bytes or a fixed speedup. A separate issue event avoids adding large issue lists to every progress notification.

**Alternatives considered**: Append all issues to each progress event. Rejected because progress events are frequent and repeated issue payloads would grow. Compare CSV files byte-for-byte. Rejected because parallel completion order can differ while results remain equivalent.

## Decision 9: Define scheduler accounting and overflow continuations

**Decision**: Register each child as outstanding before retiring its parent, including local overflow work. Queue mutation and outstanding accounting share one synchronization protocol. Queue emptiness alone never means completion. Wake waiting workers after enqueue, zero outstanding work, cancellation, or fatal failure. When the frontier is full, use iterative depth-first continuation frames and streaming entry iteration, without recursive function calls or eagerly collecting siblings. Suspended frames do not consume additional worker slots; the count limits simultaneous directory operations.

**Rationale**: These invariants prevent early completion and deadlocks. Local continuation memory and open directory iterators scale with depth; handle exhaustion follows the descendant read-failure policy. The bounded frontier does not imply constant total traversal memory. Overlapping CLI roots may revisit directories as today, but synchronized same-file filtering prevents duplicate workbook processing. Identity tracking scales with discovered files, and lookup failures retain the existing CLI issue policy.

**Alternatives considered**: Treat an empty queue as completion, recursively traverse overflow children, or eagerly collect local siblings. Rejected because they can lose work, overflow the call stack, or grow memory with tree width. Add global directory identity tracking. Deferred because existing file identity filtering already preserves result uniqueness across overlapping roots.

## Decision 10: Make shutdown cooperative and failure-aware

**Decision**: Use cancellation-aware `try_send` delivery, retaining the unsent path between retries. Release receiver ownership before joining producers when consumers stop. Propagate worker join failures and suppress successful completion after fatal discovery failure. Check cancellation between filesystem operations; an OS filesystem call already blocked cannot be interrupted by the cancellation flag.

**Rationale**: This gives concrete shutdown behavior for bounded channels and avoids false completion after producer failure.

**Alternatives considered**: Blocking delivery without cancellation checks or ignoring join errors. Rejected because they can hang shutdown or report incomplete work as successful.
