# Bug Verification: 大量マッチ時のイベント過負荷解消および Shape プレビュー表示の改善

- **Slug**: preview-not-showing-emit-failed
- **Tested**: 2026-09-28
- **Assessment**: ./assessment.md
- **Fix**: ./fix.md
- **Result**: partial

## Summary

大量マッチ発生時の WebView2 メッセージキュー飽和防止（50件/25ms間隔のバッチ送信）および Shape（図形）選択時のプレビュー体験改善（アンカー付き Shape のセルプレビュー取得・アンカーなし Shape のガイダンス表示）について、全自動テスト（フロントエンド 65 件、Rust 43 件）、TypeScript 型検査、Vite バンドル、Clippy（警告ゼロ）、およびフォーマット検査がすべて合格することを確認しました。実機デスクトップ環境における GUI 上での最終目視確認（実ファイルに対する手動操作）が残されているため、ガードレール規定に則り結果を `partial` と記録します。

## Checks Performed

| Check | Command / Action | Result | Notes |
|-------|------------------|--------|-------|
| Reproduction (post-fix) | ヘッドレス環境での自動化テスト検証 | pass | バッチ emit 制御および Shape プレビュー分岐ロジックの動作を確認 |
| Reproduction (post-fix GUI) | 実機デスクトップ上での GUI 手動操作 | not-run | ヘッドレス CLI 環境のため GUI 描画操作は未実施（ユーザー実機確認推奨） |
| New / updated tests | `npm test tests/shape-results-ui.test.tsx` | pass | アンカー付き Shape で `get_cell_preview` 呼び出し、アンカーなし Shape で抑止を検証 |
| New / updated tests | `cargo test constants::tests::test_pipeline_constants` | pass | バッチサイズ (50) およびインターバル (25ms) の定数検証 |
| Regression suite | `npm test` | pass | 11 テストファイル、全 65 件すべて合格 (exit 0) |
| Regression suite | `cargo test` | pass | 全 43 件（単体・結合テスト）すべて合格 (exit 0) |
| Lint / type-check | `npm run build` | pass | `tsc` 型検査および Vite プロダクションビルド合格 (exit 0) |
| Lint / type-check | `cargo clippy --all-targets -- -D warnings` | pass | コンパイラ・Clippy 警告 0 件 (exit 0) |
| Lint / type-check | `cargo fmt --check` | pass | Rust 公式フォーマット規約完全適合 (exit 0) |

## Output Excerpts

### 1. `npm test`
```text
 Test Files  11 passed (11)
      Tests  65 passed (65)
   Duration  15.69s
```

### 2. `cargo test`
```text
test result: ok. 39 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.66s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s (shape_contract)
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.49s (shape_export)
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.48s (shape_extract)
```

### 3. `npm run build`
```text
vite v6.4.3 building for production...
✓ 1672 modules transformed.
dist/index.html                          0.57 kB │ gzip:   0.37 kB
dist/assets/index-CcI_YL_i.css          27.95 kB │ gzip:   5.78 kB
dist/assets/index-72ZRlI7_.js          367.23 kB │ gzip: 111.88 kB
dist/assets/AboutDialog-ycZ5YRRI.js  3,147.65 kB │ gzip: 132.56 kB
✓ built in 8.88s
```

### 4. `cargo clippy` & `cargo fmt`
```text
Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.24s (0 warnings)
cargo fmt --check (exit 0)
```

## Residual Risks

- 実機デスクトップ環境で対象ファイル（`01_８次着順制御装置_通信系制御ＰＣ_ＰＴ項目_.xlsx`）を実際に検索・プレビューした際の体感描画パフォーマンスや、700件超のレンダリングにおける Virtualizer のスクロール追従性は、手動での目視確認が必要です。

## Recommendation

自動テスト・型チェック・リント・フォーマットはすべてクリアしており、コード修正は完全に適用されています。開発サーバー（`npm run tauri dev`）にて対象ファイルの実機検索・プレビュー動作を目視確認後、問題がなければ本バグをクローズし、変更をコミットしてください。
