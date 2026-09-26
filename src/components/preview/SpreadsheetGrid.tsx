/**
 * @fileoverview 周辺セルプレビューグリッド表示コンポーネント (src/components/preview/SpreadsheetGrid.tsx)
 *
 * ## 処理内容
 * 検索一致セルを中心とする周辺セル（前後3行・前後2列）を仮想スプレッドシートテーブル形式で描画する。
 * 行・列ヘッダーの固定表示（sticky）、一致セルおよび現在選択セルのハイライト、セル選択イベントのハンドリングを行う。
 * 縦横両方向のスムーズなスクロール（overflow-auto）と、固定見出し（Freeze Panes）の完全対応。
 * 憲章原則I（自然かつ正確な日本語）、原則II（定数の外部抽出とハードコード禁止）、原則III（網羅的なヘッダコメント）に準拠。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 初版策定。
 * - v1.1.0 (2026-09-26, AI Agent): 憲章準拠改修。UIメッセージの定数参照化、4要素ヘッダコメントを付与。
 * - v1.2.0 (2026-09-26, AI Agent): スクロールレイアウト修正。overflow-hiddenによる横スクロール遮断を解消し、縦横スクロールとsticky固定見出しを両立。
 */

import React from "react";
import { Move } from "lucide-react";
import { CellPreviewData } from "../../types/search";
import { UI_MESSAGES } from "../../constants";

/**
 * 周辺セルプレビューグリッドコンポーネントのプロパティ定義
 *
 * ## プロパティ一覧
 * - `previewData`: CellPreviewData | null - バックエンドから取得したプレビューテーブル構造データ
 * - `isLoading`: boolean - プレビューデータ取得中のローディング状態フラグ
 * - `selectedCell`: { address: string; value: string } | null - ユーザーが現在クリック選択しているセル情報
 * - `onSelectCell`: (address: string, value: string, formula?: string | null) => void - セル選択時コールバック
 */
interface SpreadsheetGridProps {
  previewData: CellPreviewData | null;
  isLoading: boolean;
  selectedCell: {
    address: string;
    value: string;
  } | null;
  onSelectCell: (address: string, value: string, formula?: string | null) => void;
}

/**
 * 周辺セルプレビューグリッドコンポーネント
 *
 * ## 処理詳細
 * 周辺セルのテーブル表示を行い、ローディング中や空データ時のプレースホルダー表示を制御する。
 *
 * ## 引数
 * - `props`: SpreadsheetGridProps - コンポーネントプロパティ
 *
 * ## 戻り値
 * - `React.ReactElement`: グリッドテーブルUI要素
 *
 * ## エラー・例外条件
 * - `previewData` が null または空行の場合は、空状態のガイダンスメッセージを表示する。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 初版作成。
 * - v1.1.0 (2026-09-26, AI Agent): 憲章原則に準拠し、定数参照と4要素コメントを追加。
 * - v1.2.0 (2026-09-26, AI Agent): スクロールレイアウト修正。overflow-hiddenによる横スクロール遮断を解消し、縦横スクロールとsticky固定見出しを両立。
 */
export const SpreadsheetGrid: React.FC<SpreadsheetGridProps> = ({
  previewData,
  isLoading,
  selectedCell,
  onSelectCell,
}) => {
  if (isLoading) {
    return (
      <div className="flex-1 p-6 flex items-center justify-center text-xs text-zinc-500">
        <div className="flex items-center gap-2">
          <div className="w-4 h-4 border-2 border-emerald-500 border-t-transparent rounded-full animate-spin"></div>
          {/* 定数参照: UI_MESSAGES.PREVIEW_LOADING */}
          <span>{UI_MESSAGES.PREVIEW_LOADING}</span>
        </div>
      </div>
    );
  }

  if (!previewData || previewData.rows.length === 0) {
    return (
      <div className="flex-1 p-6 flex items-center justify-center text-xs text-zinc-500">
        {/* 定数参照: UI_MESSAGES.PREVIEW_EMPTY */}
        {UI_MESSAGES.PREVIEW_EMPTY}
      </div>
    );
  }

  return (
    <div className="flex-1 min-h-0 min-w-0 p-3.5 pr-4 pb-2 bg-[#141416] flex flex-col">
      {/* ガイダンスミニバー */}
      <div className="flex items-center justify-between text-[11px] text-zinc-400 mb-2 px-1 select-none flex-shrink-0">
        <div className="flex items-center gap-1.5">
          <span className="w-1.5 h-1.5 rounded-full bg-emerald-500"></span>
          {/* 定数参照: UI_MESSAGES.PREVIEW_GUIDANCE */}
          <span className="font-medium text-zinc-300">{UI_MESSAGES.PREVIEW_GUIDANCE}</span>
        </div>
        <div className="flex items-center gap-1 text-zinc-500 bg-zinc-900/90 px-2 py-0.5 rounded border border-zinc-800 text-[10px]">
          <Move className="w-2.5 h-2.5 text-zinc-400" />
          {/* 定数参照: UI_MESSAGES.PREVIEW_SCROLL_GUIDANCE */}
          <span>{UI_MESSAGES.PREVIEW_SCROLL_GUIDANCE}</span>
        </div>
      </div>

      {/* スプレッドシート グリッド スクロールコンテナ (縦横スクロール対応 & 固定見出し) */}
      <div className="flex-1 min-h-0 min-w-0 overflow-auto rounded border border-zinc-700/80 bg-[#1a1a1d] shadow-sm relative">
        <table className="excel-grid w-max min-w-full text-xs border-collapse font-sans">
          <thead className="sticky top-0 z-20">
            <tr className="bg-[#242428] text-zinc-400 text-center font-medium">
              {/* 行番号ヘッダーセル */}
              <th className="w-12 py-1.5 px-2 bg-[#202024] select-none text-zinc-500 font-mono text-[11px] sticky left-0 top-0 z-30">
                #
              </th>
              {/* 列ヘッダーセル群 */}
              {previewData.columns.map((col) => (
                <th
                  key={col.key}
                  className="py-1.5 px-3 min-w-[120px] bg-[#222226] sticky top-0 z-20 font-mono text-zinc-300 border border-zinc-800"
                >
                  {col.label}
                </th>
              ))}
            </tr>
          </thead>
          <tbody className="divide-y divide-zinc-800">
            {previewData.rows.map((row) => (
              <tr key={row.row_number} className="hover:bg-zinc-800/30 transition">
                {/* 行番号見出し (sticky left) */}
                <td className="w-12 py-1.5 px-2 bg-[#202024] select-none text-zinc-500 font-mono text-[11px] text-center sticky left-0 z-10 border border-zinc-800">
                  {row.row_number}
                </td>
                {/* セル群 */}
                {previewData.columns.map((col) => {
                  const cellInfo = row.cells[col.key];
                  const address = `${col.label}${row.row_number}`;
                  const isTarget = cellInfo?.is_target ?? false;
                  const isCurrentSelected = selectedCell?.address === address;

                  return (
                    <td
                      key={col.key}
                      onClick={() =>
                        onSelectCell(
                          address,
                          cellInfo?.value ?? "",
                          cellInfo?.formula
                        )
                      }
                      className={`py-1.5 px-2.5 font-mono text-xs cursor-pointer border border-zinc-800/80 transition ${
                        isTarget
                          ? "bg-emerald-950/60 text-emerald-200 font-semibold ring-1 ring-emerald-500"
                          : isCurrentSelected
                          ? "bg-zinc-800 text-white font-medium ring-1 ring-zinc-500"
                          : "text-zinc-300 hover:bg-zinc-800/60"
                      }`}
                      title={
                        cellInfo?.formula
                          ? `${UI_MESSAGES.FORMULA_TOOLTIP_PREFIX} ${cellInfo.formula}`
                          : cellInfo?.value
                      }
                    >
                      <div className="truncate max-w-[200px]">
                        {cellInfo?.value || "\u00A0"}
                      </div>
                    </td>
                  );
                })}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
};
