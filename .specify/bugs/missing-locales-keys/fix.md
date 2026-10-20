# Bug Fix: Missing Feature 028 Translation Keys in `src-tauri/locales`

- **Slug**: missing-locales-keys
- **Fixed**: 2026-10-20T12:26:00+09:00
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

Synchronized all 11 missing Feature 028 translation keys into `src-tauri/locales/ja.yml` and `src-tauri/locales/en.yml`, and added unit test assertions to `tests/catalog.test.ts` to pin down all LLM context copy localization keys.

## Changes

| File | Change | Notes |
|------|--------|-------|
| `src-tauri/locales/ja.yml` | modified | Added 11 Japanese translation keys for Feature 028 |
| `src-tauri/locales/en.yml` | modified | Added 11 English translation keys for Feature 028 |
| `tests/catalog.test.ts` | modified | Added unit test asserting all 11 LLM keys exist in both locales |

## Diff Highlights (optional)

```yaml
ui.COPY_FOR_LLM: AI向けに文脈付きでコピー
ui.COPY_FOR_LLM_TABLE: AI向けに文脈コピー (表形式)
ui.COPY_FOR_LLM_HEADER_ONLY: AI向けに文脈コピー (見出し・セルのみ)
ui.COPY_FOR_LLM_WORD_HEADING: AI向けに文脈コピー (章・節見出し付き)
ui.COPY_FOR_LLM_WORD_PARAGRAPH: AI向けに文脈コピー (段落情報のみ)
ui.COPIED_FOR_LLM_SUCCESS: AI向けコンテキストをクリップボードにコピーしました
ui.SETTINGS_LLM_SECTION: AIコンテキストコピー設定
ui.SETTINGS_LLM_WINDOW_SIZE: 多列シート時の前後切り出し列数
ui.SETTINGS_LLM_WINDOW_SIZE_DESC: 横長シートにおいてヒットセルの前後に抽出する列数を指定します
ui.SETTINGS_LLM_DEFAULT_COPY_TYPE: 既定のコピー形式
ui.SETTINGS_LLM_WORD_EXTRACTION_MODE: Word文書の既定抽出モード
```

## Tests Added or Updated

- `tests/catalog.test.ts::"shared localization catalogs > defines all LLM context copy localization keys in English and Japanese"` — Validates that all 11 keys exist and are non-empty in both catalogs.

## Local Verification

- Commands run:
  - `npm run test tests/catalog.test.ts` → 5 passed (100%)
  - `npm run test` → 25 test files / 173 tests passed (100%)
  - `npm run build` → TypeScript type checks and Vite build passed
  - `cargo test --workspace` → All unit and integration tests passed
  - `cargo clippy --workspace --all-targets -- -D warnings` → 0 warnings
  - `cargo fmt --check` → Clean formatting

## Deviations from Assessment

None.

## Follow-ups

- Run `/speckit-bug-test slug=missing-locales-keys` to record formal verification.
