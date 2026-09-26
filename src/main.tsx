/**
 * @fileoverview フロントエンドアプリケーションエントリポイント (src/main.tsx)
 *
 * ## 処理内容
 * React 18のルートレンダラーを初期化し、DOMツリーのルート要素（`#root`）に対して
 * `App` コンポーネントを StrictMode 下でマウントする。グローバルスタイルシート `index.css` を適用する。
 * 憲章原則I（自然かつ正確な日本語）、原則II（定数の外部抽出とハードコード禁止）、原則III（網羅的なヘッダコメント）に準拠。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 初版策定。
 * - v1.1.0 (2026-09-26, AI Agent): 憲章原則に準拠し、ルート要素IDの定数参照化および4要素ヘッダコメントを追加。
 */

import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./index.css";
import { APP_CONSTANTS } from "./constants";

// 定数参照: APP_CONSTANTS.ROOT_ELEMENT_ID ("root")
const rootElement = document.getElementById(APP_CONSTANTS.ROOT_ELEMENT_ID);

if (rootElement) {
  ReactDOM.createRoot(rootElement).render(
    <React.StrictMode>
      <App />
    </React.StrictMode>
  );
}
