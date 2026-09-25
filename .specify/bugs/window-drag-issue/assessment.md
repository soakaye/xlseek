# Bug Assessment: タイトルバーをドラッグしてwindowを移動できない

- **Slug**: window-drag-issue
- **Created**: 2026-09-25T19:44:30+09:00
- **Source**: pasted text
- **Verdict**: valid
- **Severity**: high

## Report (verbatim or summarized)

> タイトルバーをドラッグしてwindowを移動できない

## Symptom

カスタムタイトルバー（WindowFrameコンポーネント）をマウスでドラッグしても、ウィンドウを移動することができない。
期待される動作としては、タイトルバー上の非対話的領域（ボタンや操作UI以外の部分）をドラッグした際、ネイティブウィンドウが追従して移動すること。

## Reproduction

1. アプリケーションを起動する（`decorations: false` のカスタムタイトルバー環境）。
2. ウィンドウ上部のカスタムタイトルバー領域（Excel Grep ロゴや「Calamine Engine」バッジ付近の何もない領域）をマウスの左ボタンでクリックし、そのままドラッグする。
3. ウィンドウが移動せず、画面上に留まったままとなる。

## Suspected Code Paths

- `src-tauri/capabilities/default.json:6-14` — ウィンドウ権限に `core:window:allow-start-dragging` が許可されていない。
- `src/components/layout/WindowFrame.tsx:43-47` — タイトルバー要素に `data-tauri-drag-region` 属性が付与されているものの、Tauri v2の環境または子要素のポインターイベント伝播・権限不足によりネイティブドラッグが発火していない（また、`onMouseDown` による `appWindow.startDragging()` のフォールバック呼び出しも実装されていない）。

## Root Cause Hypothesis

**信頼度: High（高）**

原因は2点あります：
1. **Tauri v2 機能権限（Capabilities）の不足**:
   `src-tauri/capabilities/default.json` において、`core:window:allow-minimize`, `core:window:allow-toggle-maximize`, `core:window:allow-close` は許可されているものの、ウィンドウドラッグ移動に必要な `core:window:allow-start-dragging`（および必要に応じて `core:window:allow-start-resize-dragging`）が登録されていません。
2. **タイトルバーのドラッグハンドリングの堅牢性不足**:
   `src/components/layout/WindowFrame.tsx` では HTML 属性 `data-tauri-drag-region` にのみ依存していますが、Tauri v2 ではマウスダウンイベントで `await getCurrentWindow().startDragging()` を呼び出す明示的なイベントハンドラを設けることで、確実かつ安全にウィンドウ移動を開始できます。また、ボタン等の対話的要素以外のクリック時に `startDragging()` をトリガーする構造にすることで、Webview2の既定ドラッグ挙動の差異を吸収できます。

## Proposed Remediation

**Preferred（推奨）**:
1. `src-tauri/capabilities/default.json` の `permissions` に `core:window:allow-start-dragging` を追加する。
2. `src/components/layout/WindowFrame.tsx` のタイトルバー要素に `onMouseDown` ハンドラを追加し、左クリック（`e.button === 0`）かつボタン等のコントロールクリックでない場合に `await getCurrentWindow().startDragging()` を実行する処理を実装する（`data-tauri-drag-region` 属性も併せて維持）。

**Alternatives（代替案）**:
- `data-tauri-drag-region` 属性のみに任せる方法：権限追加だけで動作する可能性もあるが、Webviewのバージョンや環境によって `startDragging()` 直接呼び出しの方がより確実に動作する。

**Files likely to change**:
- `src-tauri/capabilities/default.json`
- `src/components/layout/WindowFrame.tsx`

**Tests to add or update**:
- `npm run build` および `cargo check` の疎通検証。
- タイトルバーをドラッグした際にウィンドウが移動し、ボタン（最小化・最大化・閉じる）押下時にはドラッグが誤動作せずボタン機能が実行されることの手動確認。

## Risks & Considerations

- コントロールボタン（最小化・最大化・閉じる）上でドラッグが開始されないよう、ボタン側の `e.stopPropagation()` やマウスイベントのターゲット判定を確実に行う必要がある（現行の WindowFrame でも `e.stopPropagation()` は配置済み）。

## Open Questions

- なし（原因と修正方法は明確に特定）。
