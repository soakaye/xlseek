# Bug Verification: 英語モードで検索結果タイプ表示が折り返される

- **Slug**: i18n-display-layout
- **Tested**: 2026-09-27
- **Assessment**: ./assessment.md
- **Fix**: ./fix.md
- **Result**: verified

## Summary

英語・日本語の検索結果タイプバッジが1行表示用のクラスを持ち、全文をツールチップとアクセシブルな名前から取得できることを確認しました。Match type ヘッダーと行の列幅も一致し、回帰は見つかりませんでした。

## Checks Performed

| Check | Command / Action | Result | Notes |
|-------|------------------|--------|-------|
| Reproduction (post-fix) | `tests/locale-ui.test.tsx` の日英コンポーネント検証 | pass | 実際の `ResultTable` を描画し、折返し防止、省略表示、全文ラベル、ヘッダーと行の幅を確認。 |
| New / updated tests | `npm test -- tests/locale-ui.test.tsx --run --reporter=dot` | pass | 1 file, 10 tests passed。 |
| Regression suite | `npm test -- --run` | pass | 6 files, 31 tests passed。 |
| Lint / type-check | `npm run lint` / `npm run build` | pass | ESLint と TypeScript/Vite build が成功。 |
| Mock HTML syntax | Python `html.parser` check for `design/mainui/index.html` | pass | HTML parser: OK。 |
| Diff whitespace | `git diff --check` | pass | whitespace errors なし。 |
| GUI manual check | 手動操作 | not-run | 今回の検証では GUI 操作を行っていない。 |

## Output Excerpts

```text
tests/locale-ui.test.tsx: 10 tests passed
Test Files  6 passed (6)
Tests       31 passed (31)
✓ built in 966ms
HTML parser: OK
```

## Residual Risks

- コンポーネントテストはブラウザーの実ピクセル幅を測定しないため、極端に狭いウィンドウでの見た目は GUI で確認していない。

## Recommendation

折り返しを防ぐ CSS 条件、列幅の整合性、完全ラベルの提供は検証済み。極端に狭い画面での目視確認が必要な場合は GUI で追加確認する。
