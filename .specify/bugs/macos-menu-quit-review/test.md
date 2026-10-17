# Bug Verification: macOS メニューおよび終了・ライフサイクル処理のコードレビュー指摘解消

- **Slug**: macos-menu-quit-review
- **Tested**: 2026-10-17
- **Assessment**: ./assessment.md
- **Fix**: ./fix.md
- **Result**: partial

## Summary

`code-review`（Standards / Spec の2軸）で検出されたすべての指摘事項の解消、全21個の macOS メニュー翻訳キー（`ALL_MENU_KEYS`）の英日カタログ解決テスト、フロントエンド・バックエンドのリグレッションテストおよび静的解析（ESLint, TypeScript, Vite Build, `cargo fmt`, `cargo clippy`）がすべて成功することを確認した。ただし、macOS 実機 GUI 上での対話操作（`npm run tauri dev` 起動後のメニュー表示確認および Quit / ウィンドウクローズ時の Dock アイコン消滅確認）は CLI テスト環境から直接実行していないため、ガードレールに従い `partial` と判定する。

## Checks Performed

| Check | Command / Action | Result | Notes |
|-------|------------------|--------|-------|
| Reproduction (post-fix) | `npm run tauri dev` での macOS メニュー操作・終了確認 | not-run | 対話的 macOS GUI / Dock 操作を伴うため CLI からは未実行（コードパス上の `CloseRequested` インターセプト撤去と `MENU_ITEM_QUIT_ID` -> `app_handle.exit(0)` 接続は確認済み） |
| New / updated tests | `cargo test -p xlseek constants::tests && npx vitest run src/hooks/__tests__/useLocale.test.tsx tests/app-locale.test.tsx tests/locale.test.ts tests/catalog.test.ts` | pass | Rust 9 tests (`test_menu_and_event_constants` での全21メニューキー英日解決を含む) + Vitest 4 files (21 tests) すべて pass |
| Regression suite | `cargo test --workspace -- --test-threads=1` | pass | ワークスペース全体の単体・統合テストがすべて pass (0 failed) |
| Lint / type-check | `npm run lint && npm run build && cargo fmt --check && cargo clippy --workspace --all-targets -- -D warnings` | pass | ESLint 警告 0、`tsc && vite build` 成功、`rustfmt` 差分 0、`clippy` 警告 0 |

## Output Excerpts

```text
running 9 tests
test constants::tests::test_pipeline_constants ... ok
test constants::tests::test_error_messages_non_empty ... ok
test constants::tests::test_default_extensions_validity ... ok
test constants::tests::test_linux_desktop_constants ... ok
test constants::tests::test_csv_export_headers ... ok
test constants::tests::test_tauri_linux_config_product_name ... ok
test constants::tests::test_tauri_config_product_name ... ok
test constants::tests::test_tauri_config_main_window_label ... ok
test constants::tests::test_menu_and_event_constants ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.01s
```

```text
 ✓ tests/locale.test.ts (6 tests) 2ms
 ✓ tests/catalog.test.ts (4 tests) 2ms
 ✓ src/hooks/__tests__/useLocale.test.tsx (2 tests) 8ms
 ✓ tests/app-locale.test.tsx (9 tests) 384ms

 Test Files  4 passed (4)
      Tests  21 passed (21)
```

## Residual Risks

- CLI のヘッドレス検証環境では実際の macOS WindowServer / Dock 上でのメニュー描画およびアプリケーション終了を自動実行できないため、実機での `npm run tauri dev` による最終目視確認が推奨される。

## Recommendation

Hold for quick manual GUI confirmation — コードレビュー指摘事項（Standards / Spec）と自動テスト・品質ゲートはすべて通過しているため、実機の `npm run tauri dev` でウィンドウクローズおよびシステムメニューの「Excel Seek を終了 (`Cmd+Q`)」を実行し、Dock にバックグラウンドアイコンが残らず終了することを確認の上クローズしてください。
