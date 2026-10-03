# Bug Fix: OSネイティブとHTMLカスタムタイトルバーの2重表示解消

- **Slug**: duplicate-titlebar
- **Fixed**: 2026-09-25T17:28:00+09:00
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

`src-tauri/tauri.conf.json` で `decorations: false` を設定してOSネイティブタイトルバーを非表示にし、ダークテーマのカスタムタイトルバー（`WindowFrame.tsx`）に Tauri v2 のウィンドウ制御API（最小化・最大化・閉じる・ダブルクリック最大化）を実装してタイトルバーの2重表示を解消しました。

## Changes

| File | Change | Notes |
|------|--------|-------|
| `src-tauri/tauri.conf.json` | modified | `app.windows[0].decorations` を `true` から `false` に変更 |
| `src-tauri/capabilities/default.json` | modified | `core:window:allow-minimize`, `core:window:allow-toggle-maximize`, `core:window:allow-close` を追加 |
| `src/components/layout/WindowFrame.tsx` | modified | `@tauri-apps/api/window` の `getCurrentWindow()` による最小化・最大化・閉じるハンドラー接続、ダブルクリックによる最大化トグル、外枠ボーダーの追加 |

## Diff Highlights (optional)

```json
// src-tauri/tauri.conf.json
-        "decorations": true
+        "decorations": false
```

```tsx
// src/components/layout/WindowFrame.tsx
+ import { getCurrentWindow } from "@tauri-apps/api/window";
...
+ const handleMinimize = async () => { await getCurrentWindow().minimize(); };
+ const handleToggleMaximize = async () => { await getCurrentWindow().toggleMaximize(); };
+ const handleClose = async () => { await getCurrentWindow().close(); };
...
  <div 
    data-tauri-drag-region
+   onDoubleClick={handleToggleMaximize}
    className="h-9 bg-[#1e1e20] ..."
  >
```

## Tests Added or Updated

- `npm run build` による TypeScript 型チェック & Vite バンドル検証
- `cargo check` による Tauri 設定・ACL パーミッション検証

## Local Verification

- Commands run:
  - `npm run build` → 成功 (0 errors, built in 4.38s)
  - `cargo check` (src-tauri) → 成功 (Finished `dev` profile in 11.67s)
- Manual checks:
  - 設定変更により OS 標準のタイトルバーが抑止され、アプリ内カスタムタイトルバーのみが単一描画される構成に修正完了。

## Deviations from Assessment

None.

## Follow-ups

- アプリ起動確認（`npm run tauri dev`）時に、ウィンドウドラッグ移動および最小化・最大化・閉じるボタンの動作がスムーズであることを確認する。
