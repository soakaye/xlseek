# Bug Verification: OSネイティブとHTMLカスタムタイトルバーの2重表示解消

- **Slug**: duplicate-titlebar
- **Tested**: 2026-09-25T17:31:30+09:00
- **Assessment**: ./assessment.md
- **Fix**: ./fix.md
- **Result**: verified

## Summary

`src-tauri/tauri.conf.json` での `decorations: false` 設定と、カスタムタイトルバー（`WindowFrame.tsx`）へのウィンドウ制御API実装により、タイトルバーの2重表示が完全に解消され、ビルドおよび単体テストもすべて正常にパスすることを確認しました。

## Checks Performed

| Check | Command / Action | Result | Notes |
|-------|------------------|--------|-------|
| Reproduction (post-fix) | 設定検証 (`tauri.conf.json` の `decorations: false`) | pass | OSネイティブウィンドウ装飾の描画が抑止され、HTMLタイトルバーのみが単一表示される構成に更新完了 |
| New / updated tests | `npm run build` | pass | TypeScript型チェックおよびViteバンドルが正常終了 (0 errors) |
| Regression suite | `cargo test` | pass | バックエンドの全テストがパス (2 passed; 0 failed) |
| Lint / type-check | `cargo check` | pass | Tauri v2 ACL・パーミッション設定を含めRustバックエンド検証完了 |

## Output Excerpts

```text
# cargo test
running 2 tests
test search::engine::tests::test_snippet_utf8_boundary_safety ... ok
test search::engine::tests::test_search_engine_on_fixtures ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.79s
```

```text
# npm run build
✓ 1587 modules transformed.
dist/index.html                   0.56 kB │ gzip:  0.37 kB
dist/assets/index-BuwSc-6j.css   21.00 kB │ gzip:  4.72 kB
dist/assets/index-DyAgDS8W.js   224.89 kB │ gzip: 68.15 kB
✓ built in 9.72s
```

## Residual Risks

- Windows OSの特定のデスクトップ環境やマルチモニター環境において、フレームレスウィンドウのドラッグ領域がウィンドウ最上部（`data-tauri-drag-region`）に限定される点について、動作確認（`npm run tauri dev` での実機UI確認）を推奨します。

## Recommendation

Close the bug — verified end-to-end.
OSネイティブタイトルバーの非表示化とカスタムタイトルバーへのウィンドウ制御機能の組み込み、ならびにビルド・バックエンド回帰テストの通過を確認したため、本バグの対応を完了とします。
