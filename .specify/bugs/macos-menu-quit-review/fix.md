# Bug Fix: macOS メニューおよび終了・ライフサイクル処理のコードレビュー指摘解消

- **Slug**: macos-menu-quit-review
- **Fixed**: 2026-10-17
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

`code-review`（Standards / Spec の2軸レビュー）で特定された macOS メニュー構築およびアプリケーション終了・ライフサイクル処理の不整合と規約違反をすべて解消した。
1. **ウィンドウクローズ / Quit 時のバックグラウンド残留解消**: Tauri v2 (`tauri-runtime-wry`) では全ウィンドウが破棄 (`Destroyed`) されて初めて `RunEvent::ExitRequested` が発火するため、`WindowEvent::CloseRequested` での `api.prevent_close()` + `window.hide()` インターセプトおよび `is_quitting` フラグを廃止した。さらに Quit メニュー項目をカスタム `MenuItem::with_id` (`MENU_ITEM_QUIT_ID`, ショートカット `CmdOrCtrl+Q`) として構築し、`on_menu_event` で `app_handle.exit(EXIT_CODE_SUCCESS)` を呼び出すよう統一した。
2. **macOS システムメニューの完全ローカライズと `Show All` 補完**: `create_app_menu` 内のすべての `PredefinedMenuItem`（Services, Hide, Hide Others, Show All, Close Window, Undo, Redo, Cut, Copy, Paste, Select All, Fullscreen, Minimize, Maximize）および Quit メニュー項目に対し、`crates/core/locales/{en,ja}.yml` と `src-tauri/locales/{en,ja}.yml` の双方から翻訳テキストを渡すよう更新し、欠落していた `PredefinedMenuItem::show_all` を追加した。
3. **コーディング規約（Constitution Principle II / IV / V）の完全準拠**: ハードコード文字列（`ERR_TAURI_BUILD_FAILED` 等）の定数化、全定数参照箇所への `// Constant reference:` コメント付与、テストコード内の重複設定読込ヘルパー（`load_tauri_config`）抽出および不要な Middle Man ラッパー関数の削除を実施した。

## Changes

| File | Change | Notes |
|------|--------|-------|
| `crates/core/locales/en.yml` | modified | macOS メニュー項目用の英語翻訳キー（`menu.services`, `menu.hide`, `menu.hideOthers`, `menu.showAll`, `menu.quit`, `menu.closeWindow`, `menu.undo`, `menu.redo`, `menu.cut`, `menu.copy`, `menu.paste`, `menu.selectAll`, `menu.fullscreen`, `menu.minimize`, `menu.maximize`）を追加 |
| `crates/core/locales/ja.yml` | modified | macOS メニュー項目用の日本語翻訳キー一式を追加 |
| `src-tauri/locales/en.yml` | modified | フロントエンド/Tauri プラグイン参照用の英語翻訳カタログに同一の `menu.*` キー一式を追加 |
| `src-tauri/locales/ja.yml` | modified | フロントエンド/Tauri プラグイン参照用の日本語翻訳カタログに同一の `menu.*` キー一式を追加 |
| `src-tauri/src/constants.rs` | modified | 全メニュー翻訳キー定数、`ALL_MENU_KEYS`、`MENU_ACCELERATOR_QUIT`、`EXIT_CODE_SUCCESS`、`ERR_TAURI_BUILD_FAILED` を定義。単体テストに全メニューキーの英日カタログ解決検証と共通ヘルパー `load_tauri_config` を追加 |
| `src-tauri/src/lib.rs` | modified | `create_app_menu` で全メニュー項目をローカライズし `show_all` と `quit_item` (`MENU_ITEM_QUIT_ID`) を追加。`on_menu_event` で `app_handle.exit(EXIT_CODE_SUCCESS)` を呼び出し、`CloseRequested` の `prevent_close()` インターセプトを撤去。`RunEvent::Reopen` の `restore_main_window` は維持 |
| `src/hooks/useLocale.tsx` | modified | Principle II に従い定数参照箇所に `// Constant reference:` コメントを追加 |
| `src/hooks/__tests__/useLocale.test.tsx` | modified | 冗長な `mockSetI18nLocaleWrapper` を削除し、定数参照箇所に `// Constant reference:` コメントを追加 |

## Diff Highlights (optional)

```rust
// src-tauri/src/lib.rs
// Constant reference: constants::MENU_ITEM_QUIT_ID, constants::MENU_KEY_QUIT, constants::MENU_ACCELERATOR_QUIT
let quit_item = MenuItem::with_id(
    app_handle,
    MENU_ITEM_QUIT_ID,
    menu_text(MENU_KEY_QUIT)?,
    true,
    Some(MENU_ACCELERATOR_QUIT),
)?;

// on_menu_event
if event.id() == MENU_ITEM_ABOUT_ID {
    let _ = app_handle.emit(EVENT_OPEN_ABOUT_DIALOG, ());
} else if event.id() == MENU_ITEM_QUIT_ID {
    app_handle.exit(EXIT_CODE_SUCCESS);
}

// app.run
app.run(|app_handle, event| {
    #[cfg(target_os = "macos")]
    if let tauri::RunEvent::Reopen { .. } = event {
        let _ = restore_main_window(app_handle);
    }
    #[cfg(not(target_os = "macos"))]
    let _ = (app_handle, event);
});
```

## Tests Added or Updated

- `src-tauri/src/constants.rs::tests::test_menu_and_event_constants` — 全21個の macOS メニュー翻訳キー (`ALL_MENU_KEYS`) が英語 (`en`)・日本語 (`ja`) の埋め込みカタログ双方で空でない文字列に解決されることを検証。
- `src-tauri/src/constants.rs::tests::load_tauri_config` — Tauri 設定ファイル検証テストの共通読込ヘルパーを抽出。
- `tests/catalog.test.ts` — `src-tauri/locales/en.yml` と `src-tauri/locales/ja.yml` のキー一致・翻訳検証を通過。
- `src/hooks/__tests__/useLocale.test.tsx` — IPC タイムアウトフォールバックおよび通常初期化テストを通過。

## Local Verification

- Commands run:
  - `npm run lint` → passed (0 warnings)
  - `npm run build` → passed (`tsc && vite build` succeeded)
  - `npx vitest run src/hooks/__tests__/useLocale.test.tsx tests/app-locale.test.tsx tests/locale.test.ts tests/catalog.test.ts` → passed (4 test files, 21 tests passed)
  - `cargo fmt --check` → passed
  - `cargo clippy --workspace --all-targets -- -D warnings` → passed (0 warnings)
  - `cargo test --workspace -- --test-threads=1` → passed (all workspace unit and integration tests passed)

## Deviations from Assessment

なし。`assessment.md` の全指摘事項（Spec / Lifecycle 指摘および Standards 指摘）をすべて反映した。

## Follow-ups

- `/speckit-bug-test slug=macos-menu-quit-review` を実行して修正内容の検証レポートを記録する。
