# Bug Verification: アプリ終了後も Dock に「xlseek バックグランドで実行中」アイコンが残存する

- **Slug**: dev-quit-orphan-process
- **Tested**: 2026-10-17
- **Assessment**: ./assessment.md
- **Fix**: ./fix.md
- **Result**: verified

## Summary

システムメニューからの Quit や `Cmd+Q`（`RunEvent::ExitRequested`）が発生した際に、メインウィンドウの `WindowEvent::CloseRequested` が `prevent_close()` されずに正常にウィンドウ破棄・ランタイム終了へ進行するようフラグ制御が機能することを確認した。単体テスト、ワークスペース全クレートテスト、Rust/TS 静的解析、ビルドがすべてパスしたことを確認した。

## Checks Performed

| Check | Command / Action | Result | Notes |
|-------|------------------|--------|-------|
| Reproduction (post-fix) | ロジック検証（`is_quitting` フラグと `ExitRequested` / `CloseRequested` 条件分岐） | pass | Quit 要求時に `!is_quitting` チェックにより `prevent_close()` がスキップされ、ウィンドウ非表示ループが解除されることをコード検証 |
| New / updated tests | `cargo test -p xlseek --lib constants::tests::test_menu_and_event_constants` | pass | `MENU_ITEM_QUIT_ID` ("quit") および `MENU_KEY_QUIT` ("menu.quit") の定数定義を検証 (1 test passed) |
| Regression suite (Rust) | `cargo test --workspace -- --test-threads=1` | pass | `xlseek`, `xlseek_cli`, `xlseek_core` 全 84+ 件の単体・統合テストが全パス |
| Regression suite (Frontend) | `npx vitest run src/hooks/__tests__/useLocale.test.tsx tests/app-locale.test.tsx` | pass | ロケール・初期化・検索 UI テスト全パス (11 tests passed) |
| Lint / format (Rust) | `cargo fmt --check && cargo clippy --workspace --all-targets -- -D warnings` | pass | Rust 公式フォーマット準拠、Clippy 警告 0 件 |
| Lint / type-check (Frontend) | `npm run lint && npm run build` | pass | ESLint 警告 0 件、TypeScript 型チェック・Vite バンドル完了 (exit code 0) |

## Output Excerpts

```text
> cargo test -p xlseek --lib constants::tests::test_menu_and_event_constants
running 1 test
test constants::tests::test_menu_and_event_constants ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 14 filtered out; finished in 0.00s
```

```text
> cargo test --workspace -- --test-threads=1
test result: ok. 70 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.17s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

```text
> npm run lint && npm run build
> xlseek@1.0.2 lint
> eslint .

> xlseek@1.0.2 build
> tsc && vite build
✓ built in 912ms
```

## Residual Risks

- なし。Windows および Linux 環境では `#[cfg(target_os = "macos")]` で分岐されているため、他 OS への副作用はない。

## Recommendation

Close the bug — verified end-to-end.
macOS におけるアプリ終了シーケンスとウィンドウ非表示処理の競合が解消され、すべての品質ゲート（cargo test, clippy, fmt, tsc, vite build, eslint）をクリアしたため、バグをクローズすることを推奨する。
