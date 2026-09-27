# Bug Verification: フォーカス離脱後に検索履歴・パス候補が残る

- **Slug**: history-popup-focus-loss
- **Tested**: 2026-09-27
- **Assessment**: ./assessment.md
- **Fix**: ./fix.md
- **Result**: verified

## Summary

検索文字列・検索パスの履歴とパス候補を開いてフォーカスを移す元の再現手順を、自動テストで確認した。一覧は閉じ、遅延した補完応答でも再表示されなかった。フォルダ選択ダイアログを開く追加経路は、ユーザーが実アプリでも履歴が閉じることを確認した。回帰テストと品質チェックに失敗はない。

## Checks Performed

| Check | Command / Action | Result | Notes |
|-------|------------------|--------|-------|
| Reproduction (post-fix) | `npm test -- tests/search-history-ui.test.tsx` | pass | 履歴・候補の表示後、別欄や検索ボタンへのフォーカス移動、領域外クリック、遅延補完応答を検証。 |
| Folder dialog (manual) | フォルダ履歴を開き、ブラウズボタンからダイアログを表示 | pass | ユーザーが実アプリで履歴が閉じることを確認。 |
| New / updated tests | `npm test -- tests/search-history-ui.test.tsx` | pass | 11 テスト成功。履歴項目の選択と IME 確定時の Enter も含む。 |
| Regression suite | `npm test` | pass | 9 ファイル、52 テスト成功。 |
| Lint | `npm run lint` | pass | ESLint 警告・エラーなし。 |
| Type-check / build | `npm run build` | pass | TypeScript 型チェックと Vite ビルドが成功。 |
| Rust regression | `cargo test` | pass | 31 テスト成功。 |
| Rust lint | `cargo clippy --all-targets -- -D warnings` | pass | 警告なし。 |
| Rust format | `cargo fmt --check` | pass | 差分なし。 |
| UI mock script | `node --check`（埋め込みスクリプト 2 件） | pass | 両方とも構文エラーなし。 |
| Patch whitespace | `git diff --check` | pass | エラーなし。 |

## Output Excerpts

```text
Test Files  1 passed (1)
Tests       11 passed (11)
Test Files  9 passed (9)
Tests       52 passed (52)
test result: ok. 31 passed; 0 failed
✓ built in 1.06s
```

## Residual Risks

- 元の一般的なフォーカス移動手順は実アプリでは再操作していない。対応する操作はコンポーネントテストで確認した。

## Recommendation

自動テストで元の症状が再現せず、追加のダイアログ経路も実アプリで確認されたため、本バグをクローズしてよい。
