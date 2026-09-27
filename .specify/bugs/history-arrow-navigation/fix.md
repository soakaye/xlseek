# Bug Fix: 履歴一覧へ下矢印で移動できない

- **Slug**: history-arrow-navigation
- **Fixed**: 2026-09-27
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

履歴ボタンをクリックしても WebView 上のフォーカスがボタンへ移らない場合があった。そのため、クリック処理で先に履歴ボタンへフォーカスを設定し、その後で一覧を開くようにした。これにより、続けて押した下矢印がボタンのキー操作へ届き、先頭項目へ移動する。

## Changes

| File | Change | Notes |
|------|--------|-------|
| `src/components/search/SearchBar.tsx` | modified | 両履歴ボタンのクリック時に明示的にフォーカスし、一覧表示中の下矢印処理へつなぐ。 |
| `tests/search-history-ui.test.tsx` | modified | 入力欄がフォーカス中にボタンをクリックし、ボタンにフォーカスが移った後、下矢印で先頭項目へ移動する経路を両履歴で検証。 |
| `design/mainui/index.html` | modified | モックでも履歴ボタンへフォーカスしてから履歴一覧を開く。 |

## Tests Added or Updated

- `tests/search-history-ui.test.tsx` — クリック前に入力欄へフォーカスし、クリック後に履歴ボタンがフォーカスを受け取り、下矢印で先頭項目へ移ることを検証。

## Local Verification

- Commands run: `npm test -- --run tests/search-history-ui.test.tsx` → 17 tests passed.
- Commands run: `npm test` → 9 files / 58 tests passed.
- Commands run: `npm run lint` → passed.
- Commands run: `npm run build` → passed.
- Commands run: `cd src-tauri && cargo test` → 31 tests passed.
- Commands run: `cd src-tauri && cargo clippy --all-targets -- -D warnings` → passed with no warnings.
- Commands run: `cd src-tauri && cargo fmt --check` → passed.
- Commands run: HTML parser, inline-script `node --check`, and `git diff --check` → passed.
- Manual checks: Tauri アプリで履歴ボタンをクリック後に Down を送り、検索テキスト履歴では先頭の `Test`、ディレクトリ一覧では先頭の `/Users/soakaye` が選択状態になったことをアクセシビリティ状態で確認。

## Deviations from Assessment

- 評価書では入力欄側の下矢印処理を優先案としたが、再確認でボタンクリック後にフォーカスがボタンへ移らないことを確認した。根本原因に合わせてクリック時にボタンへ明示的にフォーカスする修正を加えた。入力欄側で履歴表示中の下矢印を受ける処理も残し、ボタン・入力欄どちらにフォーカスがある場合にも対応する。

## Follow-ups

- `/speckit-bug-test slug=history-arrow-navigation` で独立した検証レポートを作成する。
