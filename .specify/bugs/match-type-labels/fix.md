# Bug Fix: 検索結果の一致種別ラベルを短縮する

- **Slug**: match-type-labels
- **Fixed**: 2026-09-27
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

CellValue と Formula の検索結果ラベルを日英それぞれ「値 / Value」「数式 / Formula」にしました。一致詳細カードでは従来の翻訳キーを維持し、既存ラベルを変えていません。

## Changes

| File | Change | Notes |
|------|--------|-------|
| `src/components/results/ResultTable.tsx` | modified | CellValue と Formula の一覧バッジを一覧専用翻訳キーへ切り替え。 |
| `src-tauri/locales/ja.yml` | modified | 一覧用の値 `値`、数式 `数式` を追加。既存の詳細ラベルは維持。 |
| `src-tauri/locales/en.yml` | modified | 一覧用の値 `Value`、数式 `Formula` を追加。既存の詳細ラベルは維持。 |
| `tests/locale-ui.test.tsx` | modified | 日英の4種別について、一覧ラベルと詳細ラベルを検証。 |
| `design/mainui/index.html` | modified | 日本語の短縮表示と詳細表示は既に実装と一致していたため、同期確認を履歴へ記録。 |

## Tests Added or Updated

- `tests/locale-ui.test.tsx` — 日英それぞれの CellValue / Formula 一覧ラベル、詳細カードの既存ラベル、Comment / HiddenSheet の一覧および詳細ラベルを検証。

## Local Verification

- Commands run: `npm test -- tests/locale-ui.test.tsx --run --reporter=dot` → 成功（10 tests）。
- Commands run: `npm test -- --run` → 成功（6 files, 31 tests）。
- Commands run: `npm run lint` → 成功。
- Commands run: `npm run build` → 成功。
- Commands run: `cargo test --manifest-path src-tauri/Cargo.toml` → 成功（20 tests）。
- Commands run: `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings` → 成功。
- Commands run: `cargo fmt --manifest-path src-tauri/Cargo.toml --check` → 成功。
- Commands run: HTML parser check for `design/mainui/index.html` → 成功。
- Commands run: `git diff --check` → 成功。
- Manual checks: なし。

## Deviations from Assessment

なし。

## Follow-ups

- `$speckit-bug-test slug=match-type-labels` で修正検証を記録する。
