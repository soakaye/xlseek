<!--
Description: Records source-backed decisions for the settings-page implementation plan.
Arguments & Returns: Inputs are the feature specification, existing source/tests, and the storage standard; output is resolved design decisions.
Errors: This document runs no code; the proposed behavior requires implementation and verification.
-->
# Research: Settings Pages and Unified Saving

## 1. Keep the existing dialog and language flow

**Decision**: Implement two pages inside `SettingsDialog`, retaining the existing locale hook and its independent storage key. Use the preceding standalone mock as the layout reference.

**Rationale**: The current dialog already owns drafts, localization, and close callbacks. Page state and a fixed header/footer are enough; no router, second window, component framework, or new dependency is necessary. Locale changes already apply for the session even if persistence fails.

**Alternatives considered**: Separate windows would duplicate lifecycle and focus management. Saving language with search settings would contradict FR-002 and FR-008.

**Evidence**: `src/components/settings/SettingsDialog.tsx`, `src/hooks/useLocale.tsx`, `src/locale-core.ts`, `design/mainui/index.html`.

## 2. Commit settings and history with one write

**Decision**: Extend the existing `xlseek.searchHistory` record with `defaultOptions` and `rememberedBurstWorkers`. Persist this complete record in one `localStorage.setItem` operation. For explicit settings saves, publish state only after the write succeeds.

**Rationale**: `useSearchHistory.commit` currently publishes memory before writing, and `setMaxEntries` returns success on write failure. Combining that operation with `saveDefaultSearchOptions` would permit partial settings and irreversible history deletion. A single record includes the reduced history and new defaults in the same commit.

The standard checks whether the value can be stored before replacing the key. The design therefore uses one replacement rather than multiple writes and rollback; this is an inference from the specified algorithm. It does not promise power-loss durability or coordination between independent application windows. [WHATWG Storage interface](https://html.spec.whatwg.org/multipage/webstorage.html#the-storage-interface)

**Alternatives considered**: Two-key rollback can itself fail. A new storage backend, transaction journal, or new combined key would add mechanisms unnecessary for one window and at most 50 entries per history.

**Evidence**: `src/hooks/useSearchHistory.ts`, `src/default-options-core.ts`, `src/App.tsx`; delegated read-only review independently confirmed the persistence failure path.

## 3. Read existing preferences without destructive migration

**Decision**: Preserve the history key and its existing fields. When the record has no embedded defaults, normalize defaults from `xlseek.default_search_options` using existing compatibility rules. The first successful settings save or accepted-search history write materializes the extended record. Do not delete or dual-write the old options key.

**Rationale**: Existing installations remain usable without a startup write. Once embedded defaults exist, they are authoritative, so an old options key cannot overwrite later preferences. App and initial search loading must use the same loader.

**Alternatives considered**: Eager migration can fail during startup. Permanent dual writes recreate the partial-commit problem. Keeping a separate defaults loader reading only the old key would make the first search inconsistent with settings.

**Evidence**: Both `App.tsx` and `useSearch.ts` currently call `loadDefaultSearchOptions`; existing options tests cover preferences predating directory-mode fields.

## 4. One hook owns the latest committed record

**Decision**: Extend the existing `useSearchHistory` hook to own the combined record and expose one settings-save operation. Add a small `settings-core.ts` for validated loading and single-record persistence. Keep `default-options-core.ts` for option validation, normalization, and reset; replace its standalone storage entry points and update their callers/tests.

**Rationale**: The settings dialog submits only preference values, not a history snapshot. The hook derives trimmed history from its latest ref at commit time. Normal history updates always carry committed defaults forward. Separating pure option rules from record persistence prevents import cycles.

**Alternatives considered**: A global store, event bus, generic storage interface, or test-only compatibility wrapper is unnecessary. Maintaining separate App defaults state creates a second authority.

**Failure distinction**: Failed explicit settings saves retain the previous memory and storage record. A failed ordinary accepted-search history write may retain the newly recorded history in memory and report failure, preserving existing behavior. Both paths retain the last successfully saved preference values.

## 5. Remember manual Burst values separately

**Decision**: Keep `DefaultSearchOptions.burst_workers` as the selected worker configuration: a valid number for manual selection or `null` for automatic. Store the last valid manual number separately as `rememberedBurstWorkers`. Keep invalid/empty input strings only in the dialog draft.

**Rationale**: Automatic selection currently assigns `null` and loses the remembered value. Validation currently blocks invalid manual input even when Sequential is selected. Remembering a value independently fixes both without changing the Rust/CLI query schema.

**Alternatives considered**: A numeric fallback such as 4 invents a new initial preference. Persisting invalid input violates validation. Adding fields to backend commands is unnecessary.

**Normalization**: Inactive invalid text cannot overwrite a remembered valid count. Restoring search defaults resets the draft count memory to the existing automatic/no-custom-value initial state. A Sequential query ignores worker configuration; it can carry a valid saved manual value, as the existing backend already ignores it during sequential discovery.

## 6. Keep saves synchronous and submitted searches immutable

**Decision**: Validate and persist synchronously, with an immediate ref guard against reentry. Disable editing, page switching, reset, and exit while a commit is in progress. Apply successful defaults through the existing query updater; never resubmit or cancel the running search.

**Rationale**: The existing storage operation is synchronous, so a Promise-based save pipeline is unnecessary. `startSearch` passes the captured query to the backend; subsequent query-state changes affect the next invocation while preserving results and selection.

**Alternatives considered**: Waiting for an active search to finish would contradict immediate application to the search controls. Adding asynchronous persistence or a general transaction state machine is unnecessary.

## Research outcome

All technical decisions needed for Phase 1 are resolved. Existing dependencies suffice. The main validation risks are partial application on storage failure, stale history snapshots, legacy defaults overriding embedded defaults, forgotten manual counts, draft reset on language changes, and focus escaping the dialog.
