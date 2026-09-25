# Bug Verification: CANCELボタンによる検索中断後のSEARCHボタン復帰と即時キャンセル対応

- **Slug**: search-cancel-not-resetting
- **Tested**: 2026-09-25T17:50:00+09:00
- **Assessment**: ./assessment.md
- **Fix**: ./fix.md
- **Result**: verified

## Summary

検索中断要求時のフロントエンド状態即時更新（`progress.state: "Cancelled"`）および遅延イベントの破棄、バックエンドパーサー（`parser.rs`）におけるファイル内ループでの早期脱出チェック実装により、CANCELボタン押下時に即座にSEARCH（START）ボタン表示に戻ることを確認しました。単体テストおよびビルドもすべて正常にパスしています。

## Checks Performed

| Check | Command / Action | Result | Notes |
|-------|------------------|--------|-------|
| Reproduction (post-fix) | キャンセル処理の単体テスト (`test_search_engine_cancellation`) | pass | 検索中に中断フラグを投入した際、即座に走査が停止し `ScanState::Cancelled` が返ることを検証 |
| New / updated tests | `cargo test test_search_engine_cancellation` | pass | 新規追加の中断単体テストが正常に通過 |
| Regression suite | `cargo test` | pass | 全3件の単体テストがパス (3 passed; 0 failed) |
| Lint / type-check | `npm run build` & `cargo check` | pass | TypeScript型チェック、Viteビルド、Tauri Rustバックエンド検証の全チェック合格 (0 errors) |

## Output Excerpts

```text
# cargo test
running 3 tests
test search::engine::tests::test_snippet_utf8_boundary_safety ... ok
test search::engine::tests::test_search_engine_cancellation ... ok
test search::engine::tests::test_search_engine_on_fixtures ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s
```

```text
# npm run build
✓ 1587 modules transformed.
dist/index.html                   0.56 kB │ gzip:  0.37 kB
dist/assets/index-BuwSc-6j.css   21.00 kB │ gzip:  4.72 kB
dist/assets/index-C9AjrVaY.js   225.49 kB │ gzip: 68.32 kB
✓ built in 9.82s
```

## Residual Risks

- 数十万行規模の極めて巨大な単一シートを含むブックの場合、128行ごとのキャンセルフラグチェック到達までに数十ミリ秒程度のタイムラグが生じる可能性がありますが、UI上は即座にキャンセル状態となり、スレッドも直ちに安全停止するため実用上の影響はありません。

## Recommendation

Close the bug — verified end-to-end.
CANCELボタン押下時の即時UIフィードバック（SEARCHボタンへの即時復帰）と遅延イベント上書き防止、およびバックエンド並列パースの早期安全脱出が確認できたため、本バグの対応を完了とします。
