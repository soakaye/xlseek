# Bug Fix: Settings Dialog History Limit Input Layout Row Wrapping

- **Slug**: history-limit-layout-wrap
- **Fixed**: 2026-10-02T01:08:00+09:00
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

Updated the Settings Dialog ("設定" > "検索設定 · 保存して反映") so that the search history limit label ("検索履歴の保存件数") and its number input field are rendered on the same row inside a horizontal flex container (`flex items-center justify-between gap-4`) with an appropriately sized compact field (`w-24`), matching the design prototype in `design/mainui/index.html`.

## Changes

| File | Change | Notes |
|---|---|---|
| `src/components/settings/SettingsDialog.tsx` | modified | Wrapped label and input in flex container (`flex items-center justify-between gap-4`) and replaced `w-full` with `w-24 focus:border-emerald-500 focus:outline-none`. |
| `tests/settings.test.tsx` | modified | Added unit test asserting that the history limit label and input render inside a horizontal flex row with `w-24`. |

## Diff Highlights

```diff
-              <label htmlFor={SEARCH_HISTORY_CONSTANTS.INPUT_ID} className="mb-2 block text-sm font-medium">{t("ui.HISTORY_LIMIT_LABEL")}</label>
-              <input id={SEARCH_HISTORY_CONSTANTS.INPUT_ID} type="number" min={SEARCH_HISTORY_CONSTANTS.MIN_ENTRIES} max={SEARCH_HISTORY_CONSTANTS.MAX_ENTRIES} step={SEARCH_HISTORY_CONSTANTS.STEP} value={historyInput} disabled={saving} aria-invalid={!historyValid} aria-describedby={`${SETTINGS_DIALOG_IDS.HISTORY_HELP} ${SETTINGS_DIALOG_IDS.HISTORY_ERROR}`} onChange={(event) => setHistoryInput(event.target.value)} className="w-full rounded border border-zinc-700 bg-zinc-800 px-3 py-2 text-sm" />
-              <p id={SETTINGS_DIALOG_IDS.HISTORY_HELP} className="mt-1 text-xs text-zinc-400">{t("ui.HISTORY_LIMIT_DESCRIPTION")}</p>
-              {!historyValid && <p id={SETTINGS_DIALOG_IDS.HISTORY_ERROR} role="alert" className="mt-1 text-xs text-amber-400">{t(SETTINGS_TRANSLATION_KEYS.HISTORY_RANGE_ERROR)}</p>}
+              <div className="mb-5">
+                <div className="flex items-center justify-between gap-4">
+                  <label htmlFor={SEARCH_HISTORY_CONSTANTS.INPUT_ID} className="text-sm font-medium">{t("ui.HISTORY_LIMIT_LABEL")}</label>
+                  <input id={SEARCH_HISTORY_CONSTANTS.INPUT_ID} type="number" min={SEARCH_HISTORY_CONSTANTS.MIN_ENTRIES} max={SEARCH_HISTORY_CONSTANTS.MAX_ENTRIES} step={SEARCH_HISTORY_CONSTANTS.STEP} value={historyInput} disabled={saving} aria-invalid={!historyValid} aria-describedby={`${SETTINGS_DIALOG_IDS.HISTORY_HELP} ${SETTINGS_DIALOG_IDS.HISTORY_ERROR}`} onChange={(event) => setHistoryInput(event.target.value)} className="w-24 rounded border border-zinc-700 bg-zinc-800 px-3 py-2 text-sm focus:border-emerald-500 focus:outline-none" />
+                </div>
+                <p id={SETTINGS_DIALOG_IDS.HISTORY_HELP} className="mt-2 text-xs text-zinc-400">{t("ui.HISTORY_LIMIT_DESCRIPTION")}</p>
+                {!historyValid && <p id={SETTINGS_DIALOG_IDS.HISTORY_ERROR} role="alert" className="mt-2 text-xs text-amber-400">{t(SETTINGS_TRANSLATION_KEYS.HISTORY_RANGE_ERROR)}</p>}
+              </div>
```

## Tests Added or Updated

- `tests/settings.test.tsx` (`renders history limit label and input within a horizontal flex row`): Verifies that the input element has `w-24` and its parent element uses `flex items-center justify-between`.

## Local Verification

- `npm test`: 18/18 test files passed (114/114 tests passed).
- `npm run build`: TypeScript compilation (`tsc`) and Vite production bundle passed with exit code 0.
- `npm run lint`: Zero ESLint warnings or errors.
- `cargo test --workspace`: 66/66 Rust tests passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: Zero warnings.
- `cargo fmt --check`: Clean formatting.

## Deviations from Assessment

None. The remediation followed the proposed preferred change.

## Follow-ups

None.
