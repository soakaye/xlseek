# Bug Verification: 検索結果の一致種別ラベルを短縮する

- **Slug**: match-type-labels
- **Tested**: 2026-09-27
- **Assessment**: ./assessment.md
- **Fix**: ./fix.md
- **Result**: verified

## Summary

日英の検索結果一覧で CellValue / Formula が指定の短いラベルで表示され、一致詳細カードには従来のラベルが表示されることを確認しました。Comment / HiddenSheet のラベルも変更されておらず、回帰は見つかりませんでした。

## Checks Performed

| Check | Command / Action | Result | Notes |
|-------|------------------|--------|-------|
| Reproduction (post-fix) | `tests/locale-ui.test.tsx` の日英コンポーネント検証 | pass | 日本語・英語で一覧と詳細カードを同時に描画し、指定ラベルを確認。 |
| GUI manual check | ユーザーによる GUI 手動確認 | pass | ユーザーより「OK」と確認済み。 |
| New / updated tests | `npm test -- tests/locale-ui.test.tsx --run --reporter=dot` | pass | 1 file, 10 tests passed。 |
| Regression suite | `npm test -- --run` | pass | 6 files, 31 tests passed。 |
| Lint / type-check | `npm run lint` / `npm run build` | pass | ESLint と TypeScript/Vite build が成功。 |
| Mock HTML syntax | Python `html.parser` check for `design/mainui/index.html` | pass | HTML parser: OK。 |
| Diff whitespace | `git diff --check` | pass | whitespace errors なし。 |

## Output Excerpts

```text
tests/locale-ui.test.tsx: 10 tests passed
Test Files  6 passed (6)
Tests       31 passed (31)
✓ built in 977ms
HTML parser: OK
```

## Residual Risks

- 特記事項なし。

## Recommendation

不具合は検証済みとしてクローズ可能。
