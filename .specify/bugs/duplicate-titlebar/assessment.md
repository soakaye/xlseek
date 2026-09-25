# Bug Assessment: OSネイティブとHTMLカスタムタイトルバーの2重表示

- **Slug**: duplicate-titlebar
- **Created**: 2026-09-25T17:25:00+09:00
- **Source**: pasted text
- **Verdict**: valid
- **Severity**: medium

## Report (verbatim or summarized)

アプリケーションタイトルバーがシステムのタイトルバーと２重に表示されている。

## Symptom

- アプリケーション起動時、ウィンドウ上部に Windows OS 標準のタイトルバー（「Excel Grep」およびOS標準の最小化/最大化/閉じるボタン）が表示される。
- その直下（Web描画領域の最上部）に、ダークテーマのカスタムタイトルバー（Excel風「X」アイコン、タイトル「Excel Grep v0.1.0 (Rust & Tauri)」、Calamine Engine バッジ、カスタム最小化/最大化/閉じるボタン）が重なって表示され、タイトルバーが上下に2重化している。

## Reproduction

1. `npm run tauri dev` でアプリケーションを起動する。
2. 起動したウィンドウの最上部を確認する。
3. Windows標準のネイティブタイトルバーと、アプリケーション内部のWeb描画タイトルバー（`h-9 bg-[#1e1e20]`）が両方とも表示されている。

## Suspected Code Paths

- `src-tauri/tauri.conf.json:22`: `"decorations": true`
  - Tauriの設定でOSネイティブのウィンドウ装飾（タイトルバー・枠線）が有効化されている。
- `src/components/layout/WindowFrame.tsx:11-51`: カスタムタイトルバーコンポーネント
  - `data-tauri-drag-region` 属性付きで、独自のタイトルバーおよびウィンドウ操作ボタン（最小化・最大化・閉じる）を描画している。

## Root Cause Hypothesis

**確信度: high (極めて高い)**

`src-tauri/tauri.conf.json` の `app.windows[0].decorations` が `true` に設定されており、OSネイティブのタイトルバーが有効になっています。一方で、フロントエンドのレイアウトコンポーネント `src/components/layout/WindowFrame.tsx` でも独自のカスタムタイトルバー（HTML/CSS）を描画しているため、ネイティブヘッダーとカスタムヘッダーが上下に並んで2重表示されています。

## Proposed Remediation

**Preferred**:
ダークテーマに統一されたカスタムタイトルバーのUIデザインを生かすため、以下の改修を実施します：
1. `src-tauri/tauri.conf.json` の `decorations` を `false` に変更し、OSネイティブのタイトルバーを非表示にする。
2. `src/components/layout/WindowFrame.tsx` 内の最小化・最大化・閉じるボタンに、`@tauri-apps/api/window` の `getCurrentWindow()` を用いて実際のウィンドウ操作イベントハンドラー（`minimize()`, `toggleMaximize()`, `close()`）を接続する。
3. 必要に応じて `src-tauri/capabilities/default.json` にウィンドウ制御権限（`core:window:allow-minimize`, `core:window:allow-maximize`, `core:window:allow-close`, `core:window:allow-toggle-maximize` 等）を付与する。

**Alternatives**:
- OSネイティブのタイトルバーを採用し、`WindowFrame.tsx` からカスタムタイトルバー領域（11〜51行目）を削除して、ネイティブウィンドウ枠に統一する。
  - トレードオフ: ダークテーマとOS標準枠との統一感が失われ、デザイン仕様（ダークモダン調）から外れるが、OS標準のウィンドウ挙動（スナップやアニメーション）を完全にOSに委ねられる。

**Files likely to change**:
- `src-tauri/tauri.conf.json`
- `src/components/layout/WindowFrame.tsx`
- `src-tauri/capabilities/default.json`

**Tests to add or update**:
- アプリ起動時のビジュアル確認（OSタイトルバーがなく、カスタムタイトルバーのみが単一で表示されること）。
- カスタムタイトルバーのドラッグ移動（`data-tauri-drag-region`）が正常に機能すること。
- 最小化・最大化/元に戻す・閉じるボタンが正常に動作すること。

## Risks & Considerations

- `decorations: false`（フレームレスウィンドウ）にした場合、Windowsにおいてウィンドウのリサイズ境界の掴みやすさや、ウィンドウのスナップ機能（Aero Snap）の挙動に注意が必要。Tauri v2 では `resizable: true` が指定されていれば基本的にリサイズ境界は維持される。
- フロントエンドからウィンドウ操作を行うため、Tauri v2のパーミッション設定（`capabilities/default.json`）でウィンドウ操作APIが適切に許可されている必要がある。

## Open Questions

- なし（原因・方針ともに明確に特定完了）。
