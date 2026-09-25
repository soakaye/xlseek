/**
 * @fileoverview Excel風数式バーコンポーネント (src/components/preview/FormulaBar.tsx)
 *
 * ## 処理内容
 * 選択中のセル番地（例: `A1`）およびそのセルの生データ（数式または値文字列）を、
 * Excelスプレッドシートの数式入力バーに模した形式でリアルタイムに表示する。
 * 憲章原則I（自然かつ正確な日本語）、原則III（網羅的なヘッダコメント）に準拠。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 初版策定。
 * - v1.1.0 (2026-09-26, AI Agent): 憲章原則に準拠し、4要素ヘッダコメントを付与。
 */

import React from "react";

/**
 * 数式バーコンポーネントのプロパティ定義
 *
 * ## プロパティ一覧
 * - `cellAddress`: string - 表示対象のセル番地文字列（例: "B5"）
 * - `formulaOrValue`: string - セルに設定されている数式または値文字列
 */
interface FormulaBarProps {
  cellAddress: string;
  formulaOrValue: string;
}

/**
 * Excel風数式バーコンポーネント
 *
 * ## 処理詳細
 * セル番地ボックスと、数式/テキスト値表示エリアをインライン描画する。
 *
 * ## 引数
 * - `props`: FormulaBarProps - コンポーネントプロパティ
 *
 * ## 戻り値
 * - `React.ReactElement`: 数式バーUI要素
 *
 * ## エラー・例外条件
 * - セル番地が未指定の場合はデフォルト記号 ("--") を安全にフォールバック表示する。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 初版作成。
 * - v1.1.0 (2026-09-26, AI Agent): 憲章原則に準拠し、4要素コメントを追加。
 */
export const FormulaBar: React.FC<FormulaBarProps> = ({
  cellAddress,
  formulaOrValue,
}) => {
  return (
    <div className="bg-[#202024] border-b border-zinc-800 px-3.5 py-1.5 flex items-center gap-2 text-xs flex-shrink-0">
      {/* 名前ボックス (セル番地) */}
      <div className="w-16 bg-[#18181b] border border-zinc-700 rounded px-2 py-1 text-center font-mono font-bold text-emerald-400">
        {cellAddress || "--"}
      </div>

      <div className="h-4 w-px bg-zinc-700"></div>

      <div className="flex items-center gap-1 text-zinc-500 font-mono text-[11px] select-none">
        <span>✕</span>
        <span>✓</span>
        <span className="font-serif italic font-bold text-zinc-400">fx</span>
      </div>

      {/* 数式バー内容表示 */}
      <div className="flex-1 bg-[#18181b] border border-zinc-700 rounded px-2.5 py-1 font-mono text-zinc-200 overflow-x-auto whitespace-nowrap">
        {formulaOrValue || ""}
      </div>
    </div>
  );
};
