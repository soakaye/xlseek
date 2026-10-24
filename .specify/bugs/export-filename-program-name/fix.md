# Bug Fix: エクスポート保存ダイアログのデフォルトファイル名が旧プログラム名（ExcelGrep）になっている

- **Slug**: export-filename-program-name
- **Fixed**: 2026-10-24T22:54:00+09:00
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

言語カタログ内の `ui.EXPORT_DEFAULT_FILENAME_PREFIX` を `ExcelGrep_Results_` から現行プログラム名ベースの `ExcelSeek_Results_` に変更し、UIモックアップおよびフロントエンドテストを同期しました。

## Changes

| File | Change | Notes |
|------|--------|-------|
| `crates/core/locales/en.yml` | modified | `ui.EXPORT_DEFAULT_FILENAME_PREFIX` を `ExcelSeek_Results_` に更新 |
| `crates/core/locales/ja.yml` | modified | `ui.EXPORT_DEFAULT_FILENAME_PREFIX` を `ExcelSeek_Results_` に更新 |
| `src-tauri/locales/en.yml` | modified | `ui.EXPORT_DEFAULT_FILENAME_PREFIX` を `ExcelSeek_Results_` に更新 |
| `src-tauri/locales/ja.yml` | modified | `ui.EXPORT_DEFAULT_FILENAME_PREFIX` を `ExcelSeek_Results_` に更新 |
| `design/mainui/index.html` | modified | トースト通知のダミーファイル名を `ExcelSeek_Results_2026-09-26.*` に更新 |
| `specs/013-default-options-config/quickstart.md` | modified | シナリオ例示のファイル名を `ExcelSeek_Results_` に更新 |
| `src/components/common/__tests__/StatusBar.test.tsx` | modified | MD / JSON 出力時のファイル名正規表現アサーションを更新 |
| `tests/export-dialog.test.tsx` | modified | CSV / XLSX 出力時のファイル名正規表現アサーションを更新 |

## Diff Highlights (optional)

```yaml
# crates/core/locales/{en,ja}.yml, src-tauri/locales/{en,ja}.yml
ui.EXPORT_XLSX_FILTER_NAME: Excel workbook
-ui.EXPORT_DEFAULT_FILENAME_PREFIX: ExcelGrep_Results_
+ui.EXPORT_DEFAULT_FILENAME_PREFIX: ExcelSeek_Results_
ui.COMPLETED: Search completed
```

## Tests Added or Updated

- `src/components/common/__tests__/StatusBar.test.tsx`:
  - `persists selected format to localStorage and triggers export with chosen format`
  - `restores previously stored format preference on mount`
- `tests/export-dialog.test.tsx`:
  - `opens save dialog and triggers export command for CSV when selected from format menu`
  - `handles user cancellation of save dialog gracefully without errors`

## Local Verification

- `npm run build` → 型チェックおよび Vite バンドル正常完了 (Exit code 0)
- `npm test` → 全 27 テストファイル、187 件のテストすべて合格 (100%)
- `cargo fmt --check` → フォーマット準拠 (Exit code 0)
- `cargo clippy --workspace --all-targets -- -D warnings` → 警告 0 件 (Exit code 0)
- `cargo test --workspace` → 全クレートのテスト合格 (Exit code 0)

## Deviations from Assessment

なし (None)

## Follow-ups

- `/speckit-bug-test slug=export-filename-program-name` による自動検証およびレポート作成
