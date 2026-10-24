# Bug Verification: エクスポート保存ダイアログのデフォルトファイル名が旧プログラム名（ExcelGrep）になっている

- **Slug**: export-filename-program-name
- **Tested**: 2026-10-24T22:55:00+09:00
- **Assessment**: ./assessment.md
- **Fix**: ./fix.md
- **Result**: verified

## Summary

ファイル保存ダイアログのデフォルトファイル名プレフィックスが `ExcelSeek_Results_` に更新され、全形式（CSV, XLSX, MD, JSON）で一貫して現行プログラム名ベースのファイル名が提示されることをユニットテストおよび統合テストスイートで検証しました。

## Checks Performed

| Check | Command / Action | Result | Notes |
|-------|------------------|--------|-------|
| Reproduction (post-fix) | `npx vitest run tests/export-dialog.test.tsx src/components/common/__tests__/StatusBar.test.tsx` | pass | 保存ダイアログ引数のデフォルトファイル名が `ExcelSeek_Results_*.{csv,xlsx,md,json}` に更新されていることを確認 |
| New / updated tests | `npx vitest run tests/export-dialog.test.tsx` | pass | 全 3 件のダイアログテスト通過 |
| Regression suite (Frontend) | `npm test` | pass | 全 27 テストファイル、187 件のテストすべて合格 (100%) |
| Regression suite (Rust backend) | `cargo test --workspace` | pass | `xlseek-core`, `xlseek-cli`, `xlseek-mcp`, `xlseek` の全テスト合格 |
| Lint / type-check (Frontend) | `npm run build` | pass | TypeScript (`tsc`) 型チェックおよび Vite バンドル正常完了 (Exit code 0) |
| Lint / format (Rust) | `cargo clippy --workspace --all-targets -- -D warnings` & `cargo fmt --check` | pass | Clippy 警告 0 件、コードフォーマット準拠 (Exit code 0) |

## Output Excerpts

### `npx vitest run tests/export-dialog.test.tsx src/components/common/__tests__/StatusBar.test.tsx`
```text
 ✓ tests/export-dialog.test.tsx (3 tests) 160ms
 ✓ src/components/common/__tests__/StatusBar.test.tsx (6 tests) 175ms

 Test Files  2 passed (2)
      Tests  9 passed (9)
```

### `npm test`
```text
 Test Files  27 passed (27)
      Tests  187 passed (187)
   Start at  22:54:36
   Duration  3.24s
```

### `cargo test --workspace`
```text
test result: ok. 15 passed; 0 failed; finished in 0.02s (xlseek_lib)
test result: ok. 11 passed; 0 failed; finished in 0.02s (xlseek_cli)
test result: ok. 10 passed; 0 failed; finished in 0.06s (cli_search)
test result: ok. 77 passed; 0 failed; finished in 0.04s (xlseek_core)
test result: ok. 3 passed; 0 failed; finished in 0.01s (export_json_test)
test result: ok. 3 passed; 0 failed; finished in 0.01s (export_md_test)
test result: ok. 12 passed; 0 failed; finished in 0.00s (xlseek_mcp)
```

## Residual Risks

なし (None)

## Recommendation

Close the bug — verified end-to-end.
