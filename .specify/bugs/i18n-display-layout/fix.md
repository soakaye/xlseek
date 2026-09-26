# Bug Fix: 英語モードで検索結果タイプ表示が折り返される

- **Slug**: i18n-display-layout
- **Fixed**: 2026-09-27
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

検索結果の一致種別バッジを1行に制限し、列幅を超えるラベルは省略表示するようにしました。完全なラベルはツールチップとアクセシブルな名前から確認できます。

## Changes

| File | Change | Notes |
|------|--------|-------|
| `src/components/results/ResultTable.tsx` | modified | バッジとヘッダーに折返し防止・省略表示を適用し、ラベルを `title` と `aria-label` に設定。ヘッダーと行の Match type 列を同じ幅・縮小条件に統一。 |
| `tests/locale-ui.test.tsx` | modified | 英語・日本語のバッジ表示、折返し防止クラス、完全なラベル、ヘッダーと行の幅を検証。 |
| `design/mainui/index.html` | modified | 検索結果タイプ表示のモックをアプリと同期。 |

## Tests Added or Updated

- `tests/locale-ui.test.tsx` — 英語・日本語それぞれで一致種別バッジが1行表示になり、省略時も完全なラベルを取得でき、ヘッダーと行の列幅が一致することを確認。

## Local Verification

- Commands run: `npm test -- --run` → 成功（6 files, 25 tests）。
- Commands run: `npm run lint` → 成功。
- Commands run: `npm run build` → 成功。
- Commands run: `cargo test --manifest-path src-tauri/Cargo.toml` → 成功（20 tests）。
- Commands run: `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings` → 成功。
- Commands run: `cargo fmt --manifest-path src-tauri/Cargo.toml --check` → 成功。
- Commands run: `git diff --check` → 成功。
- Commands run: HTML parser check for `design/mainui/index.html` → 成功。
- Manual checks: なし。

## Deviations from Assessment

なし。

## Follow-ups

- `$speckit-bug-test slug=i18n-display-layout` で修正検証を記録する。
