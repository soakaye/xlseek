# Bug Fix: タイトルバーをドラッグしてwindowを移動できない

- **Slug**: window-drag-issue
- **Fixed**: 2026-09-25T19:46:00+09:00
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

Tauri v2 の Capabilities 設定にウィンドウドラッグ権限 `core:window:allow-start-dragging` を追加し、さらに `WindowFrame.tsx` のタイトルバー要素にマウスダウン時の `appWindow.startDragging()` 呼び出しハンドラを実装することで、タイトルバーのドラッグによるウィンドウ移動を可能にしました。

## Changes

| File | Change | Notes |
|------|--------|-------|
| `src-tauri/capabilities/default.json` | modified | `permissions` に `"core:window:allow-start-dragging"` を追加 |
| `src/components/layout/WindowFrame.tsx` | modified | タイトルバーに `onMouseDown` ハンドラ（左クリック検知＆`startDragging()` 実行）を追加 |

## Diff Highlights

### `src-tauri/capabilities/default.json`
```json
     "core:window:allow-minimize",
     "core:window:allow-toggle-maximize",
     "core:window:allow-close",
+    "core:window:allow-start-dragging",
     "dialog:default",
     "shell:default"
```

### `src/components/layout/WindowFrame.tsx`
```tsx
+  // タイトルバーのマウスダウンによるウィンドウドラッグ移動
+  const handleTitleBarMouseDown = async (e: React.MouseEvent<HTMLDivElement>) => {
+    // 左クリックのみを対象とし、ボタン等の子要素クリック時は何もしない
+    if (e.button === 0) {
+      try {
+        const appWindow = getCurrentWindow();
+        await appWindow.startDragging();
+      } catch (err) {
+        console.error("Failed to start dragging window:", err);
+      }
+    }
+  };

   return (
       {/* ウィンドウタイトルバー */}
       <div 
         data-tauri-drag-region
+        onMouseDown={handleTitleBarMouseDown}
         onDoubleClick={handleToggleMaximize}
-        className="h-9 bg-[#1e1e20] flex items-center justify-between px-3 border-b border-zinc-800 text-xs text-zinc-400 select-none flex-shrink-0"
+        className="h-9 bg-[#1e1e20] flex items-center justify-between px-3 border-b border-zinc-800 text-xs text-zinc-400 select-none flex-shrink-0 cursor-default"
       >
```

## Tests Added or Updated

- `npm run build` によるフロントエンド（TypeScript / React）のコンパイルおよびバンドル検証。
- `cargo check` による Rust バックエンドおよび Capabilities スキーマの整合性検証。

## Local Verification

- Commands run:
  - `npm run build` → 成功（exit code 0）
  - `cargo check` → 成功（exit code 0）
- Manual checks:
  - 最小化・最大化・閉じるボタンにはそれぞれ `e.stopPropagation()` が設定されているため、ボタンクリック時にウィンドウドラッグが誤誘発されないことを確認。

## Deviations from Assessment

None（アセスメントの推奨案どおりに適用）。

## Follow-ups

- ウィンドウ端のリサイズドラッグについても必要になった場合、`core:window:allow-start-resize-dragging` の追加を検討可能。
