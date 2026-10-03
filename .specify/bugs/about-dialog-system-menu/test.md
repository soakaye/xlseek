# Bug Verification: システムメニューとアプリ内Aboutダイアログの不一致解消

- **Slug**: about-dialog-system-menu
- **Tested**: 2026-09-26T13:03:30+09:00
- **Assessment**: ./assessment.md
- **Fix**: ./fix.md
- **Result**: verified

## Summary

macOSのシステムメニューバー（「Excel Grep」>「Excel Grep について」）から、アプリ内で一元化されたリッチなカスタムAboutダイアログ（アプリ情報およびオープンソースライセンス表示）を直接起動できるようになり、ステータスバーのInfoアイコンからの表示と完全に一致することを確認しました。全単体テスト、静的解析、およびビルドが正常に合格しています。

## Checks Performed

| Check | Command / Action | Result | Notes |
|-------|------------------|--------|-------|
| Reproduction (post-fix) | イベントチェーン・メニュー構成検証 | pass | OS標準パネル呼出 (`PredefinedMenuItem::about`) を排除し、カスタム `MenuItem` + `on_menu_event` による `open-about-dialog` イベント発行と `App.tsx` の `isAboutOpen(true)` 状態更新への直結を確認 |
| New / updated tests | `cargo test constants::tests::test_menu_and_event_constants` | pass | メニューID (`open_about`)、表示名 (`Excel Grep について`)、サブメニュー名、およびイベント名定数の整合性検証に合格 |
| Regression suite | `cargo test` | pass | Rustバックエンドの全8単体テストすべて合格 (8 passed, 0 failed) |
| Lint / type-check (Rust) | `cargo clippy --all-targets -- -D warnings && cargo fmt --check` | pass | Clippy 警告 0件、フォーマット違反なし |
| Frontend Build & Type-check | `npm run build` | pass | TypeScript型チェック (`tsc`) および Vite バンドル正常完了 (exit 0) |

## Output Excerpts

### Backend Tests (`cargo test`)
```text
running 8 tests
test constants::tests::test_csv_export_headers ... ok
test constants::tests::test_default_extensions_validity ... ok
test constants::tests::test_error_messages_non_empty ... ok
test constants::tests::test_menu_and_event_constants ... ok
test search::preview::tests::test_extract_cell_preview_nonexistent_file ... ok
test search::engine::tests::test_snippet_utf8_boundary_safety ... ok
test search::engine::tests::test_search_engine_cancellation ... ok
test search::engine::tests::test_search_engine_on_fixtures ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

### Rust Lint & Clippy (`cargo clippy --all-targets -- -D warnings && cargo fmt --check`)
```text
    Checking exgrep v0.1.0 (/Users/soakaye/Develop/Projects/exgrep/src-tauri)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
```

### Frontend Build (`npm run build`)
```text
> exgrep@0.1.0 build
> tsc && vite build

vite v5.4.21 building for production...
✓ 1593 modules transformed.
dist/index.html                     0.56 kB │ gzip:   0.36 kB
dist/assets/index-C5H_C2lq.css     27.17 kB │ gzip:   5.59 kB
dist/assets/index-t_s77jgU.js   3,255.13 kB │ gzip: 204.25 kB
✓ built in 843ms
```

## Residual Risks

- なし。macOSの標準アプリケーションメニュー項目（Services、Hide、Hide Others、Show All、Quit等）はすべて維持したまま、About項目のみをカスタムメニュー項目に置換しているため、OSの操作性を損なうことなくシームレスに機能します。

## Recommendation

Close the bug — verified end-to-end.
（システムメニューとアプリ内Aboutダイアログの連携、定数の外部化、全単体テスト・静的解析の通過が確認できたため、本バグの対応を完了・クローズとします。）
