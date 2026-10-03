# Bug Fix: システムメニューとアプリ内Aboutダイアログの不一致解消

- **Slug**: about-dialog-system-menu
- **Fixed**: 2026-09-26T12:58:00+09:00
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

macOSのシステムメニューバー「Excel Grep について」をクリックした際に、OS標準の簡素なAboutパネルではなく、アプリケーション内で一元管理されているリッチなカスタムAboutダイアログ（アプリ情報・ライセンス一覧）が開くように連携を実装しました。

## Changes

| File | Change | Notes |
|------|--------|-------|
| `src-tauri/src/constants.rs` | modified | メニューID、表示名、サブメニュー名、およびイベント名定数と単体テストを追加 |
| `src-tauri/src/lib.rs` | modified | macOS向けカスタムメニュー構築 (`create_app_menu`) とメニューイベントハンドラ (`on_menu_event`) を実装 |
| `src/constants/index.ts` | modified | フロントエンド側イベント名定数 `OPEN_ABOUT_DIALOG` を追加 |
| `src/App.tsx` | modified | `open-about-dialog` イベントを購読し `isAboutOpen(true)` を設定する `useEffect` フックを追加 |

## Diff Highlights (optional)

```rust
// src-tauri/src/lib.rs
let about_item = MenuItem::with_id(
    app_handle,
    MENU_ITEM_ABOUT_ID,
    MENU_ITEM_ABOUT_TEXT,
    true,
    None::<&str>,
)?;

app.on_menu_event(|app_handle, event| {
    if event.id() == MENU_ITEM_ABOUT_ID {
        let _ = app_handle.emit(EVENT_OPEN_ABOUT_DIALOG, ());
    }
});
```

```typescript
// src/App.tsx
useEffect(() => {
  const setupListener = async () => {
    const u = await listen(EVENT_NAMES.OPEN_ABOUT_DIALOG, () => {
      setIsAboutOpen(true);
    });
    // ...
  };
  setupListener();
  // ...
}, []);
```

## Tests Added or Updated

- `src-tauri/src/constants.rs::tests::test_menu_and_event_constants` — Aboutダイアログ表示イベント名 (`open-about-dialog`)、メニュー項目ID (`open_about`)、メニュー表示名、各サブメニュー定数が正確に定義されていることを固定検証。

## Local Verification

- Commands run:
  - `cargo test` → 全8テスト合格 (`test constants::tests::test_menu_and_event_constants ... ok`)
  - `cargo clippy --all-targets -- -D warnings` → 警告・エラー 0 件で合格
  - `cargo fmt --check` → フォーマット違反なし
  - `npm run build` → TypeScript型チェック (`tsc`) および Vite プロダクションバンドルがエラーなく成功
- Manual checks:
  - コードベース全体において憲章原則I（自然な日本語コメント・表示名）、原則II（定数の外部化と参照コメント）、原則III（網羅的なヘッダコメント）、原則IV（モジュール設計）、原則V（堅牢なエラーハンドリング）への完全準拠を確認。

## Deviations from Assessment

なし。評価レポート（`assessment.md`）の推奨方針に完全に従って実装しました。

## Follow-ups

- macOS環境において、実際にアプリを起動しメニューバー「Excel Grep」>「Excel Grep について」からダイアログが正しくポップアップし、ステータスバーのInfoアイコン押下時と同一のダイアログが表示されることを `/speckit-bug-test` で検証する。
