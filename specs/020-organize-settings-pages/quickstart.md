<!--
Description: Provides runnable checks and end-to-end acceptance scenarios for the implemented feature.
Arguments & Returns: Input is the approved feature specification and repository research; output is a Markdown planning artifact.
Errors: No runtime execution; unresolved design requirements must be addressed before implementation.
-->
# Quickstart: Validate Settings Pages

This guide validates the feature after implementation and records checks that were actually run. See the [UI contract](contracts/settings-ui-contract.md) and [data model](data-model.md).

## Implementation Verification Log

Run on 2026-10-01 on branch `020-organize-settings-pages`.

| Check | Result |
|---|---|
| `NODE_OPTIONS=--no-experimental-webstorage npm test` | Passed: 18 files, 113 tests. |
| `npm run lint` | Passed with no warnings. |
| `npm run build` | Passed: TypeScript and Vite production build. |
| `cargo test --workspace` | Passed: all workspace unit, integration, and doc tests. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed with no warnings. |
| `cargo fmt --check` | Passed. |
| CLI sidecar | Already present at `src-tauri/binaries/xlseek-cli-aarch64-apple-darwin`; no rebuild needed. |
| Mock interaction | At 1087×973, Japanese immediate/saved pages, one save action, combined storage record, history limit zero, and legacy-key absence verified in a standalone browser tab. |
| Tauri interaction | At 960×640 and 1087×940, Japanese and English immediate/saved pages, fixed header/footer with scrollable content, arrow-key page navigation, and Escape close verified. Before and after restarting the dev app, saved settings showed history limit 20 and Burst/manual 32. English was selected for visual verification, then the original Default language preference was restored. |

English and Japanese labels, immediate language switching, language persistence failures, and draft-preserving translations are covered by Vitest; the standalone mock also ran in both locales. App integration tests inject an active scan and selected result, save new defaults, and verify the scan, query, result, and selection remain while the new preference persists for remount.

## Prerequisites and Setup

Use the existing supported Node/npm and Rust toolchains and platform Tauri development dependencies. Work on `020-organize-settings-pages` with dependencies installed. Build the CLI sidecar before workspace checks when it is absent.

```sh
npm ci
npm run build:cli
```

## Automated Quality Gates

```sh
npm test
npm run lint
npm run build
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --check
```

All commands must exit successfully without errors or warnings. If the local Node version emits experimental web-storage warnings, run frontend tests with `NODE_OPTIONS=--no-experimental-webstorage npm test` where supported.

Automated acceptance coverage must include legacy and extended records, no startup/dual writes, storage rejection with unchanged state, latest histories retained when saving a draft, ordinary history write preserving preferences, every cancellation path, validation boundaries, reset scope, manual count memory, duplicate-event rejection, locale changes preserving edits, and active-search conditions remaining unchanged.

## Run the Application and Mock

```sh
npm run tauri dev
```

Open `design/mainui/index.html` separately in a browser for mock validation. Repeat applicable screen and keyboard scenarios in both the application and mock. Use application restart, rather than opening the mock, to verify desktop persistence.

## End-to-End Scenarios

| Scenario | Steps | Expected outcome |
|---|---|---|
| Immediate language | Open settings; select Japanese/English/Default; close and reopen | Immediate page first; zero save buttons; labels update within one second; selection persists |
| Unified save | Record histories; edit limit and multiple search defaults; save once; reopen and restart | One save action applies every preference; histories trim only after success; values survive restart |
| Draft preservation | Edit saved page; switch page; change language; return | Draft values and errors retained; labels/errors use current language |
| Discard exits | Set limit to zero and change options; repeat Cancel, header close, Escape, backdrop, and immediate-page Close | Saved settings/history unchanged; language change retained; reopening restores saved values |
| Validation | Try blank/fractional/negative/51 history; Burst/manual blank/1/33/fractional; deselect all extensions | Relevant localized errors prevent saves; limits 0/50 and workers 2/32 accepted; inactive invalid workers do not block saving |
| Worker memory | Save Burst/manual count; save sequential and then automatic; reopen and select Burst/manual | Last valid manual count remains available; sequential search does not use it |
| Reset | Change history/language and defaults; restore defaults; cancel, then repeat and save | Only draft search defaults reset; history/language untouched; changes apply only on save |
| Failure and retry | In automated integration, reject the storage write; attempt combined save; retry after restoring storage | Prior record, current conditions, and histories unchanged; draft stays open; no success; retry applies all values |
| Search preservation | Start a scan, then save defaults; finish scan and start another | Running scan conditions/results stay intact; next scan uses new defaults; input and selection remain intact |
| Layout and keyboard | Repeat in Japanese/English at 960×640 and 1087×940; use keyboard only | Header/footer visible; content reachable by scroll; selected page/focus visible; focus trapped and restored; language/save/cancel flows complete |

Use integration-test storage mocks for failure injection; do not make the real application storage inaccessible. Preserve existing saved records while testing legacy loading through isolated test storage.

## Delivery Review

Confirm all FR-001–FR-017 and SC-001–SC-006 acceptance cases, inspect new constants and reference comments, verify comprehensive headers, check translation catalogs and artifacts for garbled text, and confirm the mock remains consistent. Record actual commands and results during implementation; planning alone does not satisfy runtime gates.
