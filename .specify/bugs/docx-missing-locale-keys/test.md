# Bug Verification: docx オプションおよび検索結果バッジの翻訳カタログ欠落

- **Slug**: docx-missing-locale-keys
- **Tested**: 2026-10-15T19:53:30+09:00
- **Assessment**: ./assessment.md
- **Fix**: ./fix.md
- **Result**: verified

## Summary

Word 文書検索関連キー（`ui.OPTION_ENABLE_WORD_SEARCH`、`ui.OPTION_ENABLE_WORD_SEARCH_DESC`、`ui.MATCH_TYPE_WORD_TEXT`、`ui.RESULT_MATCH_TYPE_WORD_TEXT`、`export.match.wordText`）が `src-tauri/locales/{en,ja}.yml` に正しく定義され、フォールバックメッセージ（"Some text could not be translated."）の再現が発生しなくなったことを確認しました。フロントエンド・バックエンドの全テストおよび品質ゲートもすべて成功しています。

## Checks Performed

| Check | Command / Action | Result | Notes |
|-------|------------------|--------|-------|
| Reproduction (post-fix) | `node` スクリプトによるカタログ直接検証およびキー存在チェック | pass | フォールバック文字列は返されず、期待通りの日英翻訳文字列が取得できることを確認 |
| New / updated tests | `npx vitest run tests/catalog.test.ts` | pass | Word 文書検索キー 5 種の日英定義検証を含む全 3 テスト合格 |
| Regression suite | `npm test` | pass | 19 テストスイート（全 126 テスト）すべて合格 |
| Regression suite | `cargo test --workspace` | pass | 全クレート（xlseek-core, xlseek, xlseek-cli）の全テスト合格 |
| Lint / type-check | `npm run build` | pass | `tsc` 型チェックおよび Vite バンドル成功（エラー 0 件） |
| Lint / type-check | `cargo clippy --workspace --all-targets -- -D warnings` | pass | Clippy 警告 0 件 |
| Formatting check | `cargo fmt --check` | pass | Rust 公式フォーマット規約遵守確認 |

## Output Excerpts

### カタログテスト
```text
 RUN  v4.1.11 F:/source/orca/workspaces/xlseek/.orca/workspaces/xlseek/ponyfish
 ✓ tests/catalog.test.ts (3 tests) 5ms

 Test Files  1 passed (1)
      Tests  3 passed (3)
```

### フロントエンド全テスト
```text
 Test Files  19 passed (19)
      Tests  126 passed (126)
   Duration  8.14s
```

### バックエンド全テスト
```text
test result: ok. 58 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s
test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.34s
```

### 静的解析・型チェック
```text
✓ 1677 modules transformed.
✓ built in 4.20s
Finished `dev` profile [unoptimized + debuginfo] target(s) in 11.23s
```

## Residual Risks

- 現状は `crates/core/locales/` と `src-tauri/locales/` の辞書が手動同期となっているため、今後新しい UI キーや一致種別を追加する際には双方の YAML ファイルへ登録を行う必要があります（`tests/catalog.test.ts` により検知可能）。

## Recommendation

バグの修正が完全に確認され、全テストスイートおよびビルド・Clippy 検証をパスしたため、本バグのステータスを解消済（Verified）としてクローズすることを推奨します。
