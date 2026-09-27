# Bug Fix: ステータスバーの進捗表示と詳細文の重なり

- **Slug**: status-bar-overlap
- **Fixed**: 2026-09-27
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

総ファイル数が未確定の間、固定幅の進捗欄には走査済み件数だけを表示し、進捗文字列には縮小・省略の CSS 制約を付けた。隣の詳細文には従来どおり状態と件数を表示する。

## Changes

| File | Change | Notes |
|------|--------|-------|
| `src/components/common/StatusBar.tsx` | modified | 件数を短く表示し、すべての進捗状態で文字を欄内に省略する |
| `design/mainui/index.html` | modified | 進捗欄の構造・表示とモック動作を同期する |
| `tests/locale-ui.test.tsx` | updated test | 英語・日本語で総数未確定、確定、待機状態を確認する |
| `src-tauri/src/commands/system_cmd.rs` | modified | 既存の macOS 未使用変数警告を解消する |

## Tests Added or Updated

- `tests/locale-ui.test.tsx` の `keeps %s progress inside its fixed slot` — 未確定時は短い件数、各状態で進捗文字列に縮小・省略の制約があることを確認する。修正前に英語・日本語の 2 ケースが失敗し、修正後に通過した。

## Local Verification

- `npm test` → 33 件成功、失敗 0 件。
- `npm run build` → 成功。
- `npm run lint` → 成功、警告 0 件。
- `cargo test` (`src-tauri`) → 20 件成功、失敗・警告 0 件。
- `cargo clippy --all-targets -- -D warnings` (`src-tauri`) → 成功、警告 0 件。
- `cargo fmt --check` (`src-tauri`) → 成功。
- `design/mainui/index.html` → Python の HTML パーサーで読み込み成功。
- 実画面での要素境界の比較は未実施。利用可能な描画ブラウザーがなかったため、DOM テストではピクセル単位の重なりを検証していない。

## Deviations from Assessment

- `src-tauri/src/commands/system_cmd.rs` を追加修正した。修正前から存在した macOS の `ext_normalized` 未使用警告で必須の Clippy ゲートが失敗したため、値を明示的に破棄した。アプリの動作は変更していない。

## Follow-ups

- `/speckit-bug-test slug=status-bar-overlap` で、英語・日本語の検出中表示と幅 960px の実画面を確認する。
