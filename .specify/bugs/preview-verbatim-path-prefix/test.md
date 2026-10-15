# Bug Verification: プレビュー画面の場所表示における Windows Verbatim パスプレフィックス除去

- **Slug**: preview-verbatim-path-prefix
- **Tested**: 2026-10-15T23:36:30+09:00
- **Assessment**: ./assessment.md
- **Fix**: ./fix.md
- **Result**: verified

## Summary

Windows 環境において検索結果プレビューの「場所:」行およびクリップボードコピー処理に付加されていた Verbatim プレフィックス（`\\?\` および `//?/`）が正常に除去され、標準的なローカルパス（`C:\...`）および UNC パス（`\\server\share\...`）として表示されることを確認しました。新規単体テスト・UI描画テスト・回帰テストスイート・ビルド・Clippy のすべてが合格しています。

## Checks Performed

| Check | Command / Action | Result | Notes |
|-------|------------------|--------|-------|
| Reproduction (post-fix) | `npx vitest run tests/preview-header.test.tsx` | pass | `\\?\` または `//?/` を持つパスが `PreviewHeader` の描画時に除去され、再発しないことを検証 |
| New / updated tests | `npx vitest run tests/path-display.test.ts` | pass | `normalizeDisplayPath` の全 12 テストケース（ローカル・UNC・POSIX・相対パス）合格 |
| Regression suite (frontend) | `npm test` (`vitest run`) | pass | 全 21 テストスイート（142 テスト）すべて合格 |
| Regression suite (backend) | `cargo test --workspace` | pass | 全クレート（xlseek-core, xlseek, xlseek-cli）の全テスト合格 |
| Lint / type-check (frontend) | `npm run build` | pass | `tsc` 型エラー 0 件、Vite プロダクションバンドル成功 |
| Lint / type-check (backend) | `cargo clippy --workspace --all-targets -- -D warnings` | pass | Clippy 警告 0 件 |
| Formatting check | `cargo fmt --check` | pass | 公式 Rust フォーマット規約適合確認 |

## Output Excerpts

### パス正規化単体テスト & プレビューヘッダー表示テスト
```text
 RUN  v4.1.11 F:/source/orca/workspaces/xlseek/.orca/workspaces/xlseek/ponyfish

 ✓ tests/path-display.test.ts (12 tests) 5ms
 ✓ tests/preview-header.test.tsx (4 tests) 66ms

 Test Files  2 passed (2)
      Tests  16 passed (16)
   Duration  1.72s
```

### フロントエンド全テスト
```text
 Test Files  21 passed (21)
      Tests  142 passed (142)
   Duration  9.32s
```

### バックエンド全テスト
```text
test result: ok. 58 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.14s
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.15s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.34s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.73s
```

### ビルド & Clippy
```text
✓ 1678 modules transformed.
✓ built in 4.75s
Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.19s
```

## Residual Risks

- 探索エンジン内部およびファイルアクセス処理では Windows の 260 文字制限（MAX_PATH）回避のため内部的に canonicalize された Verbatim パスが保持されますが、ユーザー向け表示・UI操作レイヤー（`PreviewHeader`、クリップボード、フォルダ起動）のみを安全に正規化しているため、ファイルアクセスへの副作用リスクはありません。

## Recommendation

バグの修正が完全に確認され、フロントエンドおよびバックエンドの全テストスイート、型チェック、Clippy、フォーマット検証をすべてパスしたため、本バグのステータスを解決済（Verified）としてクローズし、`/speckit-git-commit` による変更のコミットを推奨します。
