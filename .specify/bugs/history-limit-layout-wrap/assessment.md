# Bug Assessment: Settings Dialog History Limit Input Layout Row Wrapping

- **Slug**: history-limit-layout-wrap
- **Created**: 2026-10-02T01:05:00+09:00
- **Source**: pasted text and local screenshot (`/var/folders/pl/7w4kq9mj72zg76cjpl14dcg00000gn/T/orca-paste-1790870696440-d5e29e61-e77b-46b2-92aa-ce17d855e470.png`)
- **Verdict**: valid
- **Severity**: low

## Report (verbatim or summarized)

> 検索画面の保存件数の入力欄が次の行になっている。/var/folders/pl/7w4kq9mj72zg76cjpl14dcg00000gn/T/orca-paste-1790870696440-d5e29e61-e77b-46b2-92aa-ce17d855e470.png

The screenshot displays the "検索履歴の保存件数" label placed on its own line as a block element, with a wide, full-width `input[type="number"]` (`w-full`) displaying `20` placed directly underneath on the next line.

## Symptom

In the Settings Dialog ("設定" > "検索設定 · 保存して反映" / Saved Settings tab), the search history limit input (`#search-history-limit`) is rendered below the label ("検索履歴の保存件数") occupying the full width (`w-full`) on a separate line.

Expected design layout (matching `design/mainui/index.html` prototype and standard modal dialog ergonomics): The label "検索履歴の保存件数" and its compact number input field (`w-24`) should be aligned horizontally on the same row (`flex items-center justify-between gap-4`), with the input right-aligned and sized appropriately for a small integer (0–50), followed by the description hint below.

## Reproduction

1. Open the application.
2. Click the Settings gear/dialog button in the header bar.
3. Switch to the second tab: "検索設定 · 保存して反映" (`settings-saved-panel`).
4. Inspect the "検索履歴の保存件数" control.
5. Notice that the label is on the top line and the number input box spans the entire width on the line below it.

## Suspected Code Paths

- `src/components/settings/SettingsDialog.tsx:240-244`:
  ```tsx
  <label htmlFor={SEARCH_HISTORY_CONSTANTS.INPUT_ID} className="mb-2 block text-sm font-medium">{t("ui.HISTORY_LIMIT_LABEL")}</label>
  <input id={SEARCH_HISTORY_CONSTANTS.INPUT_ID} type="number" ... className="w-full rounded border border-zinc-700 bg-zinc-800 px-3 py-2 text-sm" />
  <p id={SETTINGS_DIALOG_IDS.HISTORY_HELP} className="mt-1 text-xs text-zinc-400">{t("ui.HISTORY_LIMIT_DESCRIPTION")}</p>
  ```
  `label` is set to `block` and `input` is set to `w-full` instead of being wrapped in a flex container `flex items-center justify-between gap-4` with `w-24`.

- `design/mainui/index.html:604-611`:
  Reference layout in UI mockup:
  ```html
  <div class="mb-5">
    <div class="flex items-center justify-between gap-4">
      <label id="historyLimitLabel" for="historyLimit" class="text-sm font-medium">検索履歴の保存件数</label>
      <input id="historyLimit" type="number" min="0" max="50" step="1" value="20" class="w-24 rounded border border-zinc-700 bg-zinc-800 px-3 py-2 text-sm focus:border-emerald-500 focus:outline-none">
    </div>
    <p id="historyLimitHint" class="mt-2 text-xs text-zinc-500">0～50 件。0 件にすると履歴を保存しません。</p>
    <p id="historyLimitError" class="mt-2 text-xs text-amber-400" role="alert" hidden>0 から 50 の整数を入力してください。</p>
  </div>
  ```

## Root Cause Hypothesis

High confidence. In `src/components/settings/SettingsDialog.tsx`, the search history limit section was implemented with `<label className="mb-2 block text-sm font-medium">` and `<input className="w-full ...">`, forcing the input into a full-width block on the following line. In contrast, the design mockup in `design/mainui/index.html` groups the label and input inside a horizontal flex container (`<div className="flex items-center justify-between gap-4">`) with `<input className="w-24 ...">`, keeping the label and input on the same row.

## Proposed Remediation

**Preferred**:
Update `src/components/settings/SettingsDialog.tsx`:
1. Wrap the label and input in `<div className="flex items-center justify-between gap-4">`.
2. Change the label className from `mb-2 block text-sm font-medium` to `text-sm font-medium`.
3. Change the input className from `w-full rounded border border-zinc-700 bg-zinc-800 px-3 py-2 text-sm` to `w-24 rounded border border-zinc-700 bg-zinc-800 px-3 py-2 text-sm focus:border-emerald-500 focus:outline-none`.
4. Keep the hint paragraph (`SETTINGS_DIALOG_IDS.HISTORY_HELP`) and validation error (`SETTINGS_DIALOG_IDS.HISTORY_ERROR`) below the flex row with `mt-2 text-xs text-zinc-400`.
5. Wrap the block in an outer `<div className="mb-5">` to preserve clean vertical spacing before the default options fieldset.

**Files likely to change**:
- `src/components/settings/SettingsDialog.tsx`
- `tests/settings-default-options.test.tsx` or `tests/settings.test.tsx` (verify test assertions)

**Tests to add or update**:
- Verify in `tests/settings.test.tsx` / `tests/locale-ui.test.tsx` that the history limit input and label remain accessible and render in the expected layout container.

## Risks & Considerations

- Low risk: Purely presentation and CSS layout adjustment within the Settings modal.
- No impact on state management, validation bounds (0-50), persistence keys, or IPC.

## Open Questions

- None. The design specification in `design/mainui/index.html` provides the exact intended HTML/Tailwind classes.
