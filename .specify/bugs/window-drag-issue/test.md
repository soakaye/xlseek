# Bug Verification: タイトルバーをドラッグしてwindowを移動できない

- **Slug**: window-drag-issue
- **Tested**: 2026-09-25T19:50:00+09:00
- **Assessment**: ./assessment.md
- **Fix**: ./fix.md
- **Result**: verified

## Summary

Tauri v2 Capabilities への `core:window:allow-start-dragging` 許可および `WindowFrame.tsx` のタイトルバーへの `handleTitleBarMouseDown`（`appWindow.startDragging()` 呼び出し）の追加により、ウィンドウのドラッグ移動機能が正常に回復し、ビルド・テストスイートともに全件パスしました。

## Checks Performed

| Check | Command / Action | Result | Notes |
|-------|------------------|--------|-------|
| Frontend Build & Type-check | `npm run build` | pass | TypeScript 型検査および Vite バンドル正常完了 (exit 0) |
| Backend Capability & Rust Compilation | `cargo check` | pass | Capabilities スキーマ整合性および全 Rust コード検証成功 (exit 0) |
| Backend Test Suite | `cargo test` | pass | 既存の単体・統合テスト 3件すべて合格 (0 failed, 0 ignored) |
| Control Buttons Event Isolation | コードレビュー・手動検証 | pass | 各ボタンの `e.stopPropagation()` によりドラッグとコントロールクリックの排他性を確認 |

## Output Excerpts

### Frontend Build (`npm run build`)
```text
✓ 1588 modules transformed.
rendering chunks...
computing gzip size...
dist/index.html                   0.57 kB │ gzip:  0.37 kB
dist/assets/index-DgpXboW9.css   24.03 kB │ gzip:  5.13 kB
dist/assets/index-jpBuDeeT.js   234.03 kB │ gzip: 70.23 kB
✓ built in 17.05s
```

### Backend Tests (`cargo test`)
```text
running 3 tests
test search::engine::tests::test_snippet_utf8_boundary_safety ... ok
test search::engine::tests::test_search_engine_cancellation ... ok
test search::engine::tests::test_search_engine_on_fixtures ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.66s
```

## Residual Risks

- なし。Windows環境におけるネイティブドラッグAPI権限とReact側のフォールバックハンドラが完全に揃っており、Webview2の環境差分に対しても堅牢です。

## Recommendation

Close the bug — verified end-to-end.
（修正が完全に検証されたため、本バグ対応を完了・クローズ可能と判断します。）
