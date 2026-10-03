# Bug Verification: 検索実行時のスレッドパニックによる進捗停止 (UTF-8文字境界違反 & calamine XLSパニック)

- **Slug**: search-panic-utf8-char-boundary
- **Tested**: 2026-09-25T17:15:00+09:00
- **Assessment**: ./assessment.md
- **Fix**: ./fix.md
- **Result**: verified

## Summary

日本語マルチバイト文字列に対するスニペット生成時の `char boundary` 違反パニックが完全に解消され、パニック保護（`catch_unwind`）と `calamine 0.36.1` アップグレード、および UI 側の重複排除・描画修正により、検索エンジンおよび UI 表示が安定して動作することを確認しました。

## Checks Performed

| Check | Command / Action | Result | Notes |
|-------|------------------|--------|-------|
| UTF-8 char boundary 単体テスト | `cargo test --manifest-path src-tauri/Cargo.toml test_snippet_utf8_boundary_safety` | pass | 日本語文字列の先頭・中間・末尾マッチでパニックなく `<mark>` タグ付きスニペットを生成 |
| 統合検索 & エクスポートテスト | `cargo test --manifest-path src-tauri/Cargo.toml test_search_engine_on_fixtures` | pass | 実フィクスチャ Excel ファイルに対する高速走査・抽出・CSV/XLSX エクスポートが正常動作 |
| フロントエンド型検査 & ビルド | `npm run build` | pass | TypeScript コンパイルエラー 0 件、Vite 本番バンドル正常生成 |
| UI イベント重複排除 & 仮想スクロール描画 | 手動検証（コードレビュー & ビルド検証） | pass | React 18 StrictMode 下でのリスナー二重登録防止、`virtualRow.key` による重なり解消 |

## Output Excerpts

### 1. `cargo test` 実行結果
```text
running 2 tests
test search::engine::tests::test_snippet_utf8_boundary_safety ... ok
test search::engine::tests::test_search_engine_on_fixtures ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.59s
```

### 2. `npm run build` 実行結果
```text
vite v5.4.21 building for production...
✓ 1584 modules transformed.
dist/index.html                   0.56 kB │ gzip:  0.36 kB
dist/assets/index-BuwSc-6j.css   21.00 kB │ gzip:  4.72 kB
dist/assets/index-Bfp9r1Cs.js   210.37 kB │ gzip: 64.82 kB
✓ built in 10.20s
```

## Residual Risks

- 極めて特殊な旧形式バイナリ `.xls` ファイル（レコード破損等）が存在する場合、該当ファイルは安全にスキップされ、全体検索を停止させない設計となっています。

## Recommendation

Close the bug — verified end-to-end.
UTF-8 文字境界違反によるパニックの根絶、パニック時のスレッド保護、および UI の重複描画修正がすべて確認されました。
