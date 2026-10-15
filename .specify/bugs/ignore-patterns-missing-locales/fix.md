# Bug Fix: 無視パターン入力領域および設定ダイアログのラベル類の翻訳テキストが表示されない

- **Slug**: ignore-patterns-missing-locales
- **Fixed**: 2026-10-16T01:05:00+09:00
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

GUIフロントエンド（`tauri-plugin-i18n` および `src/i18n.ts`）が参照・バンドルする `src-tauri/locales/ja.yml` および `src-tauri/locales/en.yml` に、不足していた無視パターン関連のUI翻訳キー（全7件）を追加しました。また `tests/catalog.test.ts` にキー存在確認テストを追加して再発防止を施しました。

## Changes

| File | Change | Notes |
|------|--------|-------|
| `src-tauri/locales/ja.yml` | modified | 無視パターンおよび設定ダイアログ用キー7件を追加 |
| `src-tauri/locales/en.yml` | modified | 無視パターンおよび設定ダイアログ用キー7件を追加 |
| `tests/catalog.test.ts` | modified | 無視パターンキーが日英カタログに定義されていることを確認するテストを追加 |

## Diff Highlights (optional)

```yaml
ui.IGNORE_PATTERNS_LABEL: "除外パターン:"
ui.IGNORE_PATTERNS_PLACEHOLDER: "~$*, *.tmp, backup/ (カンマ区切り)"
ui.IGNORE_PATTERNS_TOOLTIP: "除外するファイルまたはフォルダのパターン (.gitignore構文)"
ui.IGNORE_PATTERNS_CLEAR: "除外パターンをクリア"
ui.IGNORE_HISTORY_BUTTON: "除外パターンの履歴"
ui.SETTINGS_DEFAULT_IGNORE_PATTERNS_LABEL: "デフォルトの除外パターン"
ui.SETTINGS_DEFAULT_IGNORE_PATTERNS_HELP: "新しい検索セッションで初期設定される除外パターン（カンマ区切り）"
```

## Tests Added or Updated

- `tests/catalog.test.ts` — `defines all ignore pattern localization keys in English and Japanese`: 日英双方のカタログに全7キー（`ui.IGNORE_PATTERNS_LABEL`, `ui.IGNORE_PATTERNS_PLACEHOLDER`, `ui.IGNORE_PATTERNS_TOOLTIP`, `ui.IGNORE_PATTERNS_CLEAR`, `ui.IGNORE_HISTORY_BUTTON`, `ui.SETTINGS_DEFAULT_IGNORE_PATTERNS_LABEL`, `ui.SETTINGS_DEFAULT_IGNORE_PATTERNS_HELP`）が存在し、空でないことを検証。

## Local Verification

- `npx vitest run tests/catalog.test.ts` → 4 passed (100%)
- `npm test` → 全23テストファイル、166件のテストすべて合格 (100%)
- `npm run build` → 型チェックおよびViteバンドル正常完了 (Exit code 0)
- `cargo test --workspace` → 全クレートのテスト合格 (Exit code 0)
- `cargo clippy --workspace --all-targets -- -D warnings` → 警告 0 件 (Exit code 0)
- `cargo fmt --check` → フォーマット準拠 (Exit code 0)

## Deviations from Assessment

なし (None)

## Follow-ups

- `/speckit-bug-test slug=ignore-patterns-missing-locales` による自動検証およびレポート作成
