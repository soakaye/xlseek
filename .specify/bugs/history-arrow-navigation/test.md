# Bug Verification: 履歴一覧へ下矢印で移動できない

- **Slug**: history-arrow-navigation
- **Tested**: 2026-09-27
- **Assessment**: ./assessment.md
- **Fix**: ./fix.md
- **Result**: verified

## Summary

修正後の Tauri アプリで検索テキスト履歴とディレクトリ一覧を開き、Tab を挟まず Down を押すと、それぞれ先頭項目が選択・フォーカスされた。追加した回帰テストとフロントエンドの回帰検証も通過し、関連する不具合は再現しなかった。

## Checks Performed

| Check | Command / Action | Result | Notes |
|-------|------------------|--------|-------|
| Reproduction (post-fix) | Tauri 起動後に履歴ボタンをクリックし、続けて Down を押す | pass | 検索テキスト履歴で `Test`、ディレクトリ一覧で `/Users/soakaye` が選択・フォーカスされたことを後続のアクセシビリティ状態で確認。 |
| New / updated tests | `npm test -- --run tests/search-history-ui.test.tsx` | pass | 1 ファイル、17 tests passed。両履歴でクリック後のフォーカスと下矢印移動を検証。 |
| Regression suite | `npm test` | pass | 9 files、58 tests passed。 |
| Lint / type-check | `npm run lint` | pass | ESLint が成功。 |
| Lint / type-check | `npm run build` | pass | `tsc` と Vite production build が成功。 |

## Output Excerpts

```text
Test Files  9 passed (9)
Tests       58 passed (58)
✓ built in 969ms
```

Tauri の状態確認では、Down 操作後に次のフォーカス状態を確認した。

```text
text (selected) Test
The focused UI element is 9 text (selected) Test.
text (selected) /Users/soakaye
The focused UI element is 13 text (selected) /Users/soakaye.
```

## Residual Risks

- 確認した履歴内容は実行環境に保存済みのもの。0 件の一覧で下矢印が空項目へ移動しないことは既存の UI テストで担保している。
- Rust 側のコード変更はなく、Rust の検証コマンドはこの検証では再実行していない。

## Recommendation

Close the bug — 修正後のデスクトップ画面で報告された「クリック後に Tab なしで下矢印を押す」操作を両一覧で確認し、回帰テストも通過した。
