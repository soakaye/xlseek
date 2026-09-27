# Bug Fix: フォーカス離脱後に検索履歴・パス候補が残る

- **Slug**: history-popup-focus-loss
- **Fixed**: 2026-09-27
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

検索文字列または検索パスの操作領域からフォーカスが外れたとき、対応する履歴・候補を閉じる。フォーカス離脱時に進行中のパス補完応答を無効化し、離脱後の再表示も防ぐ。

## Changes

| File | Change | Notes |
|------|--------|-------|
| `src/components/search/SearchBar.tsx` | modified | 操作領域のフォーカス離脱と領域外クリックで一覧を閉じ、パス入力にフォーカスがある間だけ補完する。 |
| `tests/search-history-ui.test.tsx` | modified | 履歴・候補の離脱、領域内移動、領域外クリック、遅延応答を検証する。 |
| `design/mainui/index.html` | modified | 試作画面の履歴表示領域と非表示条件を同期する。 |

## Tests Added or Updated

- `tests/search-history-ui.test.tsx` — 検索文字列・検索パスの履歴と表示済み候補がフォーカス離脱で閉じること。
- `tests/search-history-ui.test.tsx` — 履歴項目へフォーカスを移して選択でき、フォーカスの移らない領域へのクリックでも閉じること。
- `tests/search-history-ui.test.tsx` — 離脱後に返った補完応答が候補を再表示しないこと。既存の IME 確定時の Enter 検証も継続する。

## Local Verification

- `npm test` → 9 ファイル、51 テスト成功。
- `npm run lint` → 成功、警告なし。
- `npm run build` → 成功。
- `cargo test` → 31 テスト成功。
- `cargo clippy --all-targets -- -D warnings` → 成功。
- `cargo fmt --check` → 成功。
- `design/mainui/index.html` の埋め込みスクリプト 2 件に `node --check` → 成功。
- `git diff --check` → 成功。

## Deviations from Assessment

なし。

## Follow-ups

- `/speckit-bug-test slug=history-popup-focus-loss` で実アプリの操作も含めて検証する。

## 追加修正（2026-09-27）

フォルダ履歴の表示中にブラウズボタンを押すと、ボタンが同じフォーカス領域内にあるため履歴が残ることが判明した。`SearchBar.tsx` でダイアログを開く直前に一覧を閉じ、保留中の補完を無効化した。`design/mainui/index.html` のブラウズ操作も同じ動作に更新した。

`tests/search-history-ui.test.tsx` に、ダイアログの応答待ち中でも履歴が閉じる回帰テストを追加した。修正前は失敗、修正後は成功。再検証では `npm test` 52 件、`cargo test` 31 件が成功し、`npm run lint`、`npm run build`、`cargo clippy --all-targets -- -D warnings`、`cargo fmt --check`、モックの `node --check` も成功した。

実アプリでは、ユーザーがフォルダ履歴の表示中にブラウズボタンを押し、ダイアログ表示時に履歴が閉じることを確認した。
