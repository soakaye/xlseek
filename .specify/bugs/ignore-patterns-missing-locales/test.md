# Bug Verification: 無視パターン入力領域および設定ダイアログのラベル類の翻訳テキストが表示されない

- **Slug**: ignore-patterns-missing-locales
- **Tested**: 2026-10-16T01:10:00+09:00
- **Assessment**: ./assessment.md
- **Fix**: ./fix.md
- **Result**: verified

## Summary

GUIフロントエンドが直接参照する辞書カタログ `src-tauri/locales/{ja,en}.yml` に不足していた7件の無視パターン関連キーを追加したことで、未翻訳フォールバック（`common.translationUnavailable`: "Some text could not be translated."）が解消され、日英双方で正しいラベル・プレースホルダー・ツールチップが表示されることを自動テストで実証・検証しました。既存テストスイートおよび型チェック・ビルドにもリグレッションは発生していません。

## Checks Performed

| Check | Command / Action | Result | Notes |
|-------|------------------|--------|-------|
| Reproduction (post-fix) | `npx vitest run tests/catalog.test.ts tests/app-locale.test.tsx` | pass | カタログ検証およびロケール切り替えテストが通過し、未翻訳フォールバックが発生しないことを確認 |
| New / updated tests | `npx vitest run tests/catalog.test.ts` | pass | `defines all ignore pattern localization keys in English and Japanese` を含む全4テスト通過 |
| Regression suite (Frontend) | `npm test` | pass | 全23テストファイル、166件のテストすべて合格 (100%) |
| Regression suite (Rust backend) | `cargo test --workspace` | pass | `xlseek-core`, `xlseek-cli`, `xlseek` の全テスト（107件）合格 |
| Lint / type-check (Frontend) | `npm run build` | pass | TypeScript (`tsc`) 型チェックおよび Vite バンドル正常完了 (Exit code 0) |
| Lint / format (Rust) | `cargo clippy --workspace --all-targets -- -D warnings` & `cargo fmt --check` | pass | Clippy 警告 0 件、コードフォーマット準拠 (Exit code 0) |

## Output Excerpts

### `npx vitest run tests/catalog.test.ts`
```text
 ✓ tests/catalog.test.ts (4 tests) 408ms
   ✓ src-tauri catalog verification > matches keys between en.yml and ja.yml
   ✓ src-tauri catalog verification > does not contain untranslated template values
   ✓ src-tauri catalog verification > defines all preview localization keys in English and Japanese
   ✓ src-tauri catalog verification > defines all ignore pattern localization keys in English and Japanese

 Test Files  1 passed (1)
      Tests  4 passed (4)
   Start at  01:06:50
   Duration  915ms (transform 48ms, setup 0ms, collect 52ms, tests 408ms, environment 0ms, prepare 79ms)
```

### `npm test`
```text
 Test Files  23 passed (23)
      Tests  166 passed (166)
   Start at  01:07:05
   Duration  3.57s (transform 340ms, setup 1.55s, collect 875ms, tests 1.48s, environment 17.76s, prepare 2.37s)
```

### `npm run build`
```text
> xlseek@0.1.0 build
> tsc && vite build

vite v5.4.14 building for production...
transforming...
✓ 1908 modules transformed.
rendering chunks...
computing gzip size...
dist/index.html                   0.89 kB │ gzip:  0.46 kB
dist/assets/index-D7UqQe_k.css   33.27 kB │ gzip:  6.46 kB
dist/assets/index-B-QhI7T_.js   450.41 kB │ gzip: 139.77 kB
✓ built in 549ms
```

### `cargo test --workspace`
```text
test result: ok. 107 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.13s
```

## Residual Risks

- なし。静的カタログのキー同期および単体テストによる恒久的な回帰防止が施されており、他のUIコンポーネントへの波及影響や機能破壊のリスクはありません。

## Recommendation

Close the bug — verified end-to-end. 不足していたUIローカライズキーが正しく定義され、フロントエンドおよびバックエンドの全テスト・品質ゲート（TypeScript型チェック、Viteビルド、Cargoテスト、Clippy、rustfmt）が完全にパスすることを確認したため、本バグをクローズすることを推奨します。
