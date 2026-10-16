# Bug Fix: 放置後のバックグラウンド移行および再表示時の空白画面表示

- **Slug**: background-blank-window
- **Fixed**: 2026-10-17
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

macOS においてメインウィンドウを閉じた際に WebView インスタンスが破棄されないよう `WindowEvent::CloseRequested` で `prevent_close()` と `hide()` を行うとともに、Dock アイコンがクリックされた際（`RunEvent::Reopen`）にメインウィンドウを確実に復元（`unminimize` / `show` / `set_focus`）するライフサイクル制御を実装した。あわせて、WebView の復帰や再ロード時に Tauri IPC（OS ロケール取得・翻訳カタログ初期化）が遅延・停滞した場合でも画面が空白（`return null`）で止まらないよう、`useLocale` にタイムアウト付きフォールバックを導入した。

## Changes

| File | Change | Notes |
|------|--------|-------|
| `src-tauri/src/constants.rs` | modified | メインウィンドウラベル定数 `MAIN_WINDOW_LABEL` (`"main"`) および設定整合性テストを追加 |
| `src-tauri/src/lib.rs` | modified | macOS 向けの `restore_main_window`、`on_window_event` (`CloseRequested` 時の非表示化)、`app.run` (`RunEvent::Reopen` 時のウィンドウ再表示・フォーカス) を追加 |
| `src/constants/index.ts` | modified | 初期ロケール・i18n IPC の安全タイムアウト定数 `TIMING_CONSTANTS.I18N_INIT_TIMEOUT_MS` (`1000`) を追加 |
| `src/hooks/useLocale.tsx` | modified | `withInitTimeout` ヘルパーを追加し、`locale()` と `initializeI18n()` が停滞しても `ready=true` へ遷移するよう改善 |
| `src/hooks/__tests__/useLocale.test.tsx` | added test | 通常解決時および IPC ハング時のタイムアウトフォールバックを検証するユニットテストを追加 |

## Diff Highlights (optional)

```rust
// src-tauri/src/lib.rs
#[cfg(target_os = "macos")]
fn restore_main_window<R: tauri::Runtime>(app_handle: &tauri::AppHandle<R>) -> bool {
    use tauri::Manager;

    // Constant reference: constants::MAIN_WINDOW_LABEL
    if let Some(window) = app_handle.get_webview_window(constants::MAIN_WINDOW_LABEL) {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
        true
    } else {
        false
    }
}
```

```tsx
// src/hooks/useLocale.tsx
const browserLocale = typeof navigator !== "undefined" && navigator.language ? navigator.language : DISPLAY_LANGUAGES.EN;
const systemLanguage = (await withInitTimeout(locale(), browserLocale)) ?? browserLocale;
const resolvedLanguage = stored === LANGUAGE_PREFERENCES.DEFAULT ? resolveLanguage(systemLanguage) : stored;
await withInitTimeout(initializeI18n(resolvedLanguage), false);
```

## Tests Added or Updated

- `src-tauri/src/constants.rs::tests::test_menu_and_event_constants` — `MAIN_WINDOW_LABEL` が `"main"` であることを検証
- `src-tauri/src/constants.rs::tests::test_tauri_config_main_window_label` — `tauri.conf.json` のプライマリウィンドウ `label` が `MAIN_WINDOW_LABEL` と一致することを検証
- `src/hooks/__tests__/useLocale.test.tsx::useLocale` — 通常時および Tauri IPC 無応答（ハング）時に `TIMING_CONSTANTS.I18N_INIT_TIMEOUT_MS` 経過後 `ready=true` となって UI 描画をブロックしないことを検証

## Local Verification

- Commands run:
  - `npm run lint` → passed (0 warnings / 0 errors)
  - `npm run build` → passed (`tsc && vite build` exited with code 0)
  - `npx vitest run src/hooks/__tests__/useLocale.test.tsx tests/app-locale.test.tsx tests/locale.test.ts` → passed (3 files, 17 tests passed)
  - `cargo fmt --check` → passed
  - `cargo clippy --workspace --all-targets -- -D warnings` → passed
  - `cargo test --workspace` → passed (all unit and integration tests across `xlseek`, `xlseek_cli`, `xlseek_core` passed)
- Manual checks: `dist/` ビルド成果物の生成および `tauri.conf.json` のウィンドウ定義との整合性を確認。

## Deviations from Assessment

なし（評価レポートの推奨修正方針どおりに適用）。

## Follow-ups

- Finder 等から開発サーバー (`npm run dev`) なしで単体起動する場合は、`target/debug/xlseek`（`devUrl: http://localhost:1420` 参照）ではなく、`npm run build:gui` で生成されるバンドル版（または `npm run tauri dev`）を使用するよう運用する。
