<!--
Purpose: Defines the desktop settings, search request, and discovery issue contracts for Burst directory search.
Inputs: The feature specification and data model (Markdown); output: interface rules for the React UI and Tauri command boundary.
Errors: This document has no runtime behavior; invalid requests and discovery failures are described as contract outcomes.
-->
# Contract: Desktop Directory Search Mode

## Settings

The existing settings dialog provides one labeled choice with two values: **Sequential** and **Burst**. Burst mode also provides an **Automatic** or **Custom** parallel count. Custom accepts a whole number from 2 through 32; the numeric control is enabled only while Burst and Custom are selected. Sequential is the default mode, and Automatic is the default count. Switching to Sequential retains a saved custom count for later Burst searches. Saving applies both choices to searches started afterward and persists them across restarts. These settings are not controls on the main search bar. The dialog and the standalone mock in `design/mainui/index.html` must show and save the same behavior.

The existing `xlseek.default_search_options` object gains `directory_mode: "sequential" | "burst"` and `burst_workers: null | number`. `null` means Automatic; a number must be an integer from 2 through 32. Reading a legacy object that omits either field keeps all valid old options and supplies `sequential` and `null`. An invalid saved mode or count normalizes only that field without resetting other valid options. Saving writes the canonical object. The settings screen rejects invalid custom input before save, including fractions and values outside the range. UI labels and warning messages use the English/Japanese locale catalogs; mode tokens remain language-neutral.

## Tauri `start_search` request

The existing command still receives `{ "query": SearchQuery }`. It gains two optional fields inside `query`:

```json
{
  "query": {
    "keyword": "example",
    "target_dir": "<selected directory>",
    "directory_mode": "burst",
    "burst_workers": 6
  }
}
```

All existing query fields remain supported and unchanged. Omission of `directory_mode` means `sequential`; omission or `null` for `burst_workers` means Automatic. Automatic resolves to a bounded count from 2 through 8 based on available parallelism. An unknown explicit mode or invalid explicit count is rejected as an invalid search request. The command snapshots the mode and count when a search starts; later settings changes do not change an active search. A saved custom count has no effect while Sequential mode is selected.

## Events

- Existing `search-match` and `scan-progress` event names and payload shapes remain compatible. Match order and generated IDs may vary. Final `Completed` progress follows completion of directory discovery and workbook processing; `Cancelled` follows cancellation.
- A new `search-issue` event reports each inaccessible descendant folder, without terminating the search. Its payload is:

```json
{
  "path": "<local folder path>",
  "stage": "discovery",
  "code": "permission_denied"
}
```

`code` is either `permission_denied` or `read_failed`. The UI resolves a localized message that contains `path`, exposes each issue in a search warnings view, and clears warnings when a new search starts. The event contains no workbook contents or search term. A missing or unreadable root continues to use the existing fatal command error path.

## Compatibility and acceptance

- Existing desktop searches and saved settings use sequential mode by default.
- Settings migration preserves previous match flags and extension choices while defaulting the new count to Automatic.
- A custom count from 2 through 32 survives restart and is never exceeded by concurrent directory visits; Reset returns the count to Automatic.
- Desktop results match semantically between modes for the same tree and query; duplicate paths and duplicate matches are absent.
- Cancellation stops pending discovery and file delivery, preserves already surfaced matches according to existing behavior, and never emits a false `Completed` state.
