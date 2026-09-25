/**
 * @fileoverview ウィンドウフレームレイアウトコンポーネント (src/components/layout/WindowFrame.tsx)
 *
 * ## 処理内容
 * アプリケーション全体のルート外枠フレームを提供し、ダークテーマスタイル、
 * 全画面高さ固定、テキスト選択抑制、およびメイン領域のオーバーフロー制御を行う。
 * 憲章原則III（ヘッダコメント）に準拠。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 初版策定。JSDocドキュメントの付与。
 */

import React from "react";

interface WindowFrameProps {
  children: React.ReactNode;
}

/**
 * ## 処理内容
 * アプリケーションの最上位コンテナレイアウトコンポーネント。
 *
 * ## 引数
 * @param props - 子要素ノードを含むプロパティ
 *
 * ## 戻り値
 * @returns レンダリング要素
 *
 * ## エラー / 例外発生条件
 * panicや例外は発生しない。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 初版策定 / 憲章準拠。
 */
export const WindowFrame: React.FC<WindowFrameProps> = ({ children }) => {
  return (
    <div className="bg-[#121214] text-zinc-100 min-h-screen flex flex-col font-sans select-none overflow-hidden h-screen">
      {/* メインコンテンツ */}
      <div className="flex-1 flex flex-col overflow-hidden">
        {children}
      </div>
    </div>
  );
};
