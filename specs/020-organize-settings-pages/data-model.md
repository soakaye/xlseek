<!--
Description: Defines persisted settings, dialog drafts, validation, and state transitions.
Arguments & Returns: Input is the feature specification and research decisions; output is the data model used by planning and implementation.
Errors: Invalid persisted values require safe normalization; invalid active drafts must not be committed.
-->
# Data Model: Settings Pages

## SavedSettings

A single JSON record stored under the existing `SEARCH_HISTORY_CONSTANTS.STORAGE_KEY` (`xlseek.searchHistory`). One record is owned by the existing history hook.

| Field | Type | Meaning and validation |
|---|---|---|
| `maxEntries` | integer | Existing shared history limit, 0–50; initial value 20 |
| `keywords` | string[] | Existing newest-first, exact-unique, nonblank search text history; length ≤ maxEntries |
| `directories` | string[] | Existing newest-first, exact-unique, nonblank directory history; length ≤ maxEntries |
| `defaultOptions` | DefaultSearchOptions | Existing search preferences described below |
| `rememberedBurstWorkers` | integer or null | Last valid manual count, 2–32; null before a custom count exists or after restoring defaults |

The history field names stay compatible. A `defaultOptions` property distinguishes an extended record from legacy history. No schema-version field is needed for this additive change.

## DefaultSearchOptions

Retain the current type and option defaults rather than defining another query schema.

| Field | Type | Existing initial value / validation |
|---|---|---|
| `match_case` | boolean | false |
| `use_regex` | boolean | false |
| `include_formula` | boolean | true |
| `include_shape` | boolean | false |
| `include_comment` | boolean | true |
| `include_hidden` | boolean | false |
| `extensions` | string[] | `.xlsx`, `.xlsm`, `.xlsb`, `.xls`; at least one supported extension, no duplicates |
| `directory_mode` | sequential or burst | sequential |
| `burst_workers` | integer or null | null means automatic; a number means manual and must be 2–32 |

A manual count may remain saved with Sequential mode, but Sequential discovery does not use it. Automatic mode always has `burst_workers = null` while retaining `rememberedBurstWorkers` for later manual selection.

## SettingsDraft

The dialog owns this non-persisted editing state, initialized on each closed-to-open transition.

| Field | Type | Meaning |
|---|---|---|
| `page` | immediate or saved | Initially immediate; switching pages preserves the draft |
| `historyLimitInput` | string | Raw input, including empty/invalid edits |
| `defaultOptions` | DefaultSearchOptions | Copy of saved search preferences; extension array copied |
| `workerChoice` | automatic or custom | Initially derived from saved `burst_workers` |
| `workerInput` | string | Saved manual value or remembered count; empty when neither exists |
| `rememberedBurstWorkers` | integer or null | Draft count memory, reset along with search defaults |
| `saving` | boolean plus ref guard | Disables interactions and rejects duplicate save/exit attempts |

History entries are not copied into the dialog. Language is not part of the draft. Locale changes and history updates must not reinitialize an open dialog.

## Validation and draft conversion

- History input must represent an integer from 0 to 50. Blank, fractional, negative, and out-of-range values are invalid.
- At least one supported extension must remain selected. Worker errors and extension errors must be reported independently.
- Worker input is required and validated only when both Burst and custom selection are active.
- A valid new custom input becomes the draft remembered count. Inactive invalid input does not replace it.
- Automatic selection produces `burst_workers = null` and retains the remembered number.
- Sequential with custom selection uses the draft's valid remembered number if present; if no valid number exists, save automatic/null rather than persisting invalid text. The unavailable manual field cannot block Sequential saving.
- Reset restores existing option defaults and clears the draft manual count memory. It preserves the history input and immediate language preference.

## Commit lifecycle

1. Opening settings copies saved preference values into a draft and selects the immediate page.
2. Editing or page switching changes only the draft; language selection independently updates the locale.
3. Saving validates the draft and guards against reentry.
4. The hook derives a candidate record using its latest current history, trimming only the candidate arrays to the proposed limit.
5. Serialize the complete candidate and replace the history key once. On failure, keep current settings/history and the draft unchanged.
6. On success, replace the hook's ref/state, apply defaults to the search controls, show success, and close.
7. Closing before a commit discards the draft; reopening starts from saved values. Language changes remain effective.

Ordinary accepted-search recording uses the same current record and preserves all settings fields. Its existing memory-only fallback on write failure remains distinct from explicit settings saving.

## Loading and compatibility

- Missing history record: initialize empty histories at limit 20; read legacy defaults when available.
- Valid legacy history without `defaultOptions`: preserve its histories/limit and normalize legacy default options. Derive remembered count from a valid manual legacy value.
- Extended record: use embedded defaults; do not fall back to stale legacy defaults when embedded values are invalid. Normalize invalid embedded options to existing defaults, preserve valid history independently, and repair invalid count memory using a valid selected manual value or null.
- Malformed JSON or unreadable storage: use safe defaults, report read failure through the existing notification path, and perform no startup writes. Recovering valid individual fields from a parsed record must never trigger immediate history deletion.
- First successful ordinary history write or settings save writes the normalized extended record. The old options key remains untouched and becomes read-only fallback for legacy records only.
- No API/backend entities, extra windows, or cross-process synchronization are added.
