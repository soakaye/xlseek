# Bug Verification: ステータスバーの進捗表示と詳細文の重なり

- **Slug**: status-bar-overlap
- **Tested**: 2026-09-27
- **Assessment**: ./assessment.md
- **Fix**: ./fix.md
- **Result**: verified

## Summary

ユーザーが修正後の実アプリを起動し、英語・日本語のどちらでもステータスバーに重なりがないことを確認した。自動テストと必須品質ゲートもすべて通過し、回帰は検出されなかった。

## Checks Performed

| Check | Command / Action | Result | Notes |
|-------|------------------|--------|-------|
| 修正後の症状確認 | ユーザーが実アプリを起動し、英語・日本語の表示を目視確認 | pass | ユーザー報告: 「英語日本語とも重なりがない」 |
| 追加した回帰テスト | `npm test` | pass | 進捗欄の英語・日本語ケースを含む全 33 件が成功 |
| フロントエンドビルド | `npm run build` | pass | TypeScript と Vite のビルド成功 |
| フロントエンド静的検査 | `npm run lint` | pass | ESLint のエラー・警告なし |
| Rust 回帰テスト | `cargo test` (`src-tauri`) | pass | 20 件成功、失敗・警告なし |
| Rust 静的検査 | `cargo clippy --all-targets -- -D warnings` (`src-tauri`) | pass | 警告なし |
| Rust フォーマット | `cargo fmt --check` (`src-tauri`) | pass | 差分なし |
| 差分検査 | `git diff --check` | pass | 空白エラーなし |

## Output Excerpts

- Vitest: `Test Files 6 passed (6)`、`Tests 33 passed (33)`。
- Cargo test: `20 passed; 0 failed`。
- Vite: `built in 938ms`。
- Clippy: `Finished dev profile`、exit 0。

## Residual Risks

- 実画面確認時のウィンドウ幅、表示倍率、走査済みファイル数は記録されていない。幅 960px と極端に大きい件数での個別確認は未実施。
- DOM 回帰テストは文字列と省略制約を検証する。ピクセル単位の境界比較は行っていない。

## Recommendation

バグをクローズしてよい。元の表示重なりは修正後の実アプリで英語・日本語とも再現せず、自動検証も通過した。
