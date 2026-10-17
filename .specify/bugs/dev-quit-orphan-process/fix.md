# Bug Fix: アプリ終了後も Dock に「xlseek バックグランドで実行中」アイコンが残存する

- **Slug**: dev-quit-orphan-process
- **Fixed**: 2026-10-17
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

macOS において、システムメニューからの Quit や `Cmd+Q`（`RunEvent::ExitRequested`）が発生した際に、メインウィンドウのクローズ要求（`WindowEvent::CloseRequested`）が `api.prevent_close()` / `window.hide()` で遮断されてアプリケーションが完全終了できずバックグラウンドプロセスとして Dock に残り続ける不具合を解消した。終了中フラグ `is_quitting` を導入し、通常の閉じる操作時のみ非表示化し、終了要求時は正常にウィンドウ破棄・プロセス終了へ進むよう修正した。

## Changes

| File | Change | Notes |
|------|--------|-------|
| `src-tauri/src/constants.rs` | modified | メニュー終了 ID 定数 `MENU_ITEM_QUIT_ID` および `MENU_KEY_QUIT` を定義し、テストを更新 |
| `src-tauri/src/lib.rs` | modified | アプリ終了中フラグ `is_quitting` (`Arc<AtomicBool>`) を追加。`on_window_event` で `!is_quitting` の場合のみ `prevent_close()` と `hide()` を呼び、`RunEvent::ExitRequested` 時にフラグをセットして正常終了を許可 |

## Diff Highlights (optional)

```rust
// src-tauri/src/lib.rs
#[cfg(target_os = "macos")]
let is_quitting = Arc::new(std::sync::atomic::AtomicBool::new(false));
#[cfg(target_os = "macos")]
let is_quitting_for_window = Arc::clone(&is_quitting);

// on_window_event
if window.label() == constants::MAIN_WINDOW_LABEL {
    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
        if !is_quitting_for_window.load(std::sync::atomic::Ordering::SeqCst) {
            api.prevent_close();
            let _ = window.hide();
        }
    }
}

// app.run
match event {
    tauri::RunEvent::ExitRequested { .. } => {
        is_quitting.store(true, std::sync::atomic::Ordering::SeqCst);
    }
    tauri::RunEvent::Reopen { .. } => {
        let _ = restore_main_window(app_handle);
    }
    _ => {}
}
```

## Tests Added or Updated

- `src-tauri/src/constants.rs::tests::test_menu_and_event_constants` — `MENU_ITEM_QUIT_ID` ("quit") および `MENU_KEY_QUIT` ("menu.quit") の定数定義を検証。

## Local Verification

- Commands run:
  - `cargo fmt --check` → passed
  - `cargo clippy --workspace --all-targets -- -D warnings` → passed (0 warnings)
  - `cargo test --workspace -- --test-threads=1` → passed (all 84+ tests in workspace passed)
  - `npm run build` → passed (`tsc && vite build` succeeded)
  - `npx vitest run src/hooks/__tests__/useLocale.test.tsx` → passed (2 tests passed)

## Deviations from Assessment

なし。

## Follow-ups

- 実機（macOS GUI）上での `npm run tauri dev` 起動後、Quit 操作でターミナルがシェルプロンプトへ復帰し Dock アイコンが消えることを確認する。
