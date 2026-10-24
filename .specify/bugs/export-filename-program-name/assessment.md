# Bug Assessment: エクスポート保存ダイアログのデフォルトファイル名が旧プログラム名（ExcelGrep）になっている

- **Slug**: export-filename-program-name
- **Created**: 2026-10-24T22:50:00+09:00
- **Source**: pasted text
- **Verdict**: valid
- **Severity**: low

## Report (verbatim or summarized)

> 出力ファイル名が ExcelGrep.*になっている。プログラム名ベースに変更してほしい。

## Symptom

検索結果のエクスポート時に OS 標準のファイル保存ダイアログに表示されるデフォルトファイル名のプレフィックスが、旧プログラム名 `ExcelGrep_Results_` のままとなっており、現行のプログラム名 `Excel Seek`（または `ExcelSeek`）と不整合が生じている。
期待される動作は、現行プログラム名に基づいたプレフィックス（`ExcelSeek_Results_YYYY-MM-DD.<ext>`）が提示されること。

## Reproduction

1. アプリケーションを起動し検索を実行して検索結果を表示させる。
2. ステータスバーのエクスポートボタン（または形式選択メニューから各形式）をクリックして OS ファイル保存ダイアログを開く。
3. デフォルトファイル名として `ExcelGrep_Results_2026-10-24.<ext>` が提示される。

## Suspected Code Paths

- `crates/core/locales/en.yml:60` — `ui.EXPORT_DEFAULT_FILENAME_PREFIX: ExcelGrep_Results_`
- `crates/core/locales/ja.yml:60` — `ui.EXPORT_DEFAULT_FILENAME_PREFIX: ExcelGrep_Results_`
- `src-tauri/locales/en.yml:56` — `ui.EXPORT_DEFAULT_FILENAME_PREFIX: ExcelGrep_Results_`
- `src-tauri/locales/ja.yml:56` — `ui.EXPORT_DEFAULT_FILENAME_PREFIX: ExcelGrep_Results_`
- `design/mainui/index.html:9326-9344` — スタンドアロンモックアップ内のエクスポート完了トースト文字列
- `src/components/common/__tests__/StatusBar.test.tsx` — 保存ダイアログ引数の正規表現アサーション
- `tests/export-dialog.test.tsx` — 保存ダイアログ引数の正規表現アサーション

## Root Cause Hypothesis

**確信度: High（高）**

プロジェクト名称を "Excel Grep" から "Excel Seek" (`xlseek`) に刷新した際、エクスポート保存ダイアログで参照される言語カタログキー `ui.EXPORT_DEFAULT_FILENAME_PREFIX` が `ExcelGrep_Results_` のまま更新されていなかった。

## Proposed Remediation

**Preferred（推奨）**:
1. 日英言語カタログファイル（`crates/core/locales/{en,ja}.yml` および `src-tauri/locales/{en,ja}.yml`）の `ui.EXPORT_DEFAULT_FILENAME_PREFIX` を `ExcelSeek_Results_` に変更する。
2. `design/mainui/index.html` のトースト通知モック文字列を `ExcelSeek_Results_2026-09-26.*` に更新する。
3. フロントエンドテスト（`StatusBar.test.tsx`, `export-dialog.test.tsx`）の期待値を更新する。

**Files likely to change**:
- `crates/core/locales/en.yml`
- `crates/core/locales/ja.yml`
- `src-tauri/locales/en.yml`
- `src-tauri/locales/ja.yml`
- `design/mainui/index.html`
- `specs/013-default-options-config/quickstart.md`
- `src/components/common/__tests__/StatusBar.test.tsx`
- `tests/export-dialog.test.tsx`

**Tests to add or update**:
- `tests/export-dialog.test.tsx`: デフォルトパスのアサーションが `ExcelSeek_Results_` に一致することを確認。
- `src/components/common/__tests__/StatusBar.test.tsx`: Markdown / JSON 出力時のデフォルトパスが `ExcelSeek_Results_` に一致することを確認。

## Risks & Considerations

- 表示文字列およびファイル名プレフィックスのみの変更であり、検索・パース・出力処理ロジックへの影響やリグレッションリスクはない。
