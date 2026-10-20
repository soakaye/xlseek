# Bug Assessment: Missing Feature 028 Translation Keys in `src-tauri/locales`

- **Slug**: missing-locales-keys
- **Created**: 2026-10-20T12:20:00+09:00
- **Source**: pasted text
- **Verdict**: valid
- **Severity**: high

## Report (verbatim or summarized)

> 追加されたテキストがすべてSome text could not be translatedになっている

## Symptom

In the GUI desktop application, all newly added UI text elements associated with Feature 028 (toolbar button "AI向けにコピー", right-click context menu copy options, toast success notifications, and Settings dialog copy options) display the generic fallback error message:
`"Some text could not be translated."`
instead of their corresponding Japanese or English translated text strings.

## Reproduction

1. Launch the `xlseek` desktop application in either development mode (`npm run dev`) or production build (`npm run build`).
2. Run any search that returns results, or select a result from the results table.
3. Observe the preview header toolbar: the "Copy for LLM" action button text/tooltip displays `"Some text could not be translated."`.
4. Right-click any row in the search results table: the floating context menu items display `"Some text could not be translated."`.
5. Open the Settings dialog (`設定`): navigate to the Saved settings page. Under the newly added AI context copy section, the section title, adjacent column window size label/help text, and dropdown options all display `"Some text could not be translated."`.

## Suspected Code Paths

- `src-tauri/locales/ja.yml` — Missing all 11 translation keys introduced in Feature 028.
- `src-tauri/locales/en.yml` — Missing all 11 translation keys introduced in Feature 028.
- `src/i18n.ts:16` — Loads the bundled fallback English catalog via `import englishCatalogSource from "../src-tauri/locales/en.yml?raw";`.
- `src/i18n.ts:82-86` — `resolveTranslation` prioritizes the active plugin catalog (`tauri-plugin-i18n`), falls back to `fallbackCatalog[key]`, and when absent in both, falls back to `I18N_CONSTANTS.TRANSLATION_UNAVAILABLE_TEXT` (`"Some text could not be translated."`).
- `tests/catalog.test.ts` — Validates catalog completeness using `src-tauri/locales/*.yml`, but lacks assertion coverage for the newly added Feature 028 LLM copy keys.

## Root Cause Hypothesis

During the implementation of Feature 028 (Phase 1, Task T003), translation keys were added to `crates/core/locales/ja.yml` and `crates/core/locales/en.yml` (the catalogs utilized by the Rust core/CLI). However, the Tauri v2 GUI application relies on `tauri-plugin-i18n` which loads catalogs from `src-tauri/locales/ja.yml` and `src-tauri/locales/en.yml`, and `src/i18n.ts` bundles `src-tauri/locales/en.yml?raw` as its in-memory fallback.

Because the 11 new keys were not synchronized to `src-tauri/locales/ja.yml` and `src-tauri/locales/en.yml`:
1. `tauri-plugin-i18n` fails to find the key and returns the key string unmodified.
2. `src/i18n.ts` detects `pluginText === key` and attempts fallback to `englishCatalog[key]`.
3. `englishCatalog` (derived from `src-tauri/locales/en.yml`) also lacks the key.
4. `src/i18n.ts` falls back to `I18N_CONSTANTS.TRANSLATION_UNAVAILABLE_TEXT` (`"Some text could not be translated."`).

Confidence: High (100%).

## Proposed Remediation

**Preferred**:
1. Copy all 11 Feature 028 translation keys from `crates/core/locales/ja.yml` into `src-tauri/locales/ja.yml`:
   - `ui.COPY_FOR_LLM`
   - `ui.COPY_FOR_LLM_TABLE`
   - `ui.COPY_FOR_LLM_HEADER_ONLY`
   - `ui.COPY_FOR_LLM_WORD_HEADING`
   - `ui.COPY_FOR_LLM_WORD_PARAGRAPH`
   - `ui.COPIED_FOR_LLM_SUCCESS`
   - `ui.SETTINGS_LLM_SECTION`
   - `ui.SETTINGS_LLM_WINDOW_SIZE`
   - `ui.SETTINGS_LLM_WINDOW_SIZE_DESC`
   - `ui.SETTINGS_LLM_DEFAULT_COPY_TYPE`
   - `ui.SETTINGS_LLM_WORD_EXTRACTION_MODE`
2. Copy all 11 Feature 028 translation keys from `crates/core/locales/en.yml` into `src-tauri/locales/en.yml`.
3. Add a unit test to `tests/catalog.test.ts` verifying that all Feature 028 LLM copy localization keys exist in both `src-tauri/locales/ja.yml` and `src-tauri/locales/en.yml`.
4. Add a cross-catalog consistency check (or test) ensuring that `src-tauri/locales/` and `crates/core/locales/` maintain parity for shared keys.

**Files likely to change**:
- `src-tauri/locales/ja.yml`
- `src-tauri/locales/en.yml`
- `tests/catalog.test.ts`

**Tests to add or update**:
- `tests/catalog.test.ts`: Add `it("defines all LLM context copy localization keys in English and Japanese")` validating the presence and non-emptiness of all 11 keys in both languages.

## Risks & Considerations

- **Breaking Changes**: Zero. Adding missing keys to YAML catalogs is purely additive.
- **Side Effects**: Restores expected Japanese and English labels across the toolbar, context menu, and settings without altering any functional logic.

## Open Questions

- None.
