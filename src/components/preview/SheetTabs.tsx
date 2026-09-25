/**
 * @fileoverview スプレッドシート内ワークシート切替タブバーコンポーネント (src/components/preview/SheetTabs.tsx)
 *
 * ## 処理内容
 * プレビュー中のブックに含まれる全ワークシート一覧をタブ形式で表示し、横スクロールナビゲーションおよび
 * クリックによるアクティブシート切り替え操作を提供する。
 * 憲章原則I（自然かつ正確な日本語）、原則II（定数の外部抽出とハードコード禁止）、原則III（網羅的なヘッダコメント）に準拠。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 初版策定。
 * - v1.1.0 (2026-09-26, AI Agent): 憲章原則に準拠し、スクロール量およびUI文言の定数参照化、4要素ヘッダコメントを追加。
 */

import React, { useRef } from "react";
import { ChevronLeft, ChevronRight, FileSpreadsheet } from "lucide-react";
import { LAYOUT_CONSTANTS, UI_MESSAGES } from "../../constants";

/**
 * シートタブコンポーネントのプロパティ定義
 *
 * ## プロパティ一覧
 * - `sheets`: string[] - ブック内に存在するシート名の配列
 * - `activeSheet`: string - 現在アクティブ表示されているシート名
 * - `onSelectSheet`: (sheetName: string) => void - シート選択時コールバック関数
 */
interface SheetTabsProps {
  sheets: string[];
  activeSheet: string;
  onSelectSheet: (sheetName: string) => void;
}

/**
 * ワークシート切替タブバーコンポーネント
 *
 * ## 処理詳細
 * 左右スクロールボタンおよびシート名ボタングループを描画し、タブ選択イベントを発火する。
 *
 * ## 引数
 * - `props`: SheetTabsProps - コンポーネントプロパティ
 *
 * ## 戻り値
 * - `React.ReactElement | null`: シートタブバーUI要素。シートが存在しない場合は null。
 *
 * ## エラー・例外条件
 * - `sheets` が空配列または未定義の場合は何も描画せず安全に終了する。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 初版作成。
 * - v1.1.0 (2026-09-26, AI Agent): 憲章原則に準拠し、定数参照と4要素コメントを追加。
 */
export const SheetTabs: React.FC<SheetTabsProps> = ({
  sheets,
  activeSheet,
  onSelectSheet,
}) => {
  const scrollContainerRef = useRef<HTMLDivElement>(null);

  const scrollLeft = () => {
    if (scrollContainerRef.current) {
      // 定数参照: LAYOUT_CONSTANTS.SHEET_SCROLL_OFFSET_PX
      scrollContainerRef.current.scrollBy({ left: -LAYOUT_CONSTANTS.SHEET_SCROLL_OFFSET_PX, behavior: "smooth" });
    }
  };

  const scrollRight = () => {
    if (scrollContainerRef.current) {
      // 定数参照: LAYOUT_CONSTANTS.SHEET_SCROLL_OFFSET_PX
      scrollContainerRef.current.scrollBy({ left: LAYOUT_CONSTANTS.SHEET_SCROLL_OFFSET_PX, behavior: "smooth" });
    }
  };

  if (!sheets || sheets.length === 0) {
    return null;
  }

  return (
    <div className="mt-2 bg-[#1a1a1d] rounded border border-zinc-800 px-2 py-1 flex items-center justify-between text-xs select-none flex-shrink-0 mr-2 shadow-sm">
      <div className="flex items-center gap-1 overflow-x-auto min-w-0">
        {/* シート移動ナビゲーション */}
        <div className="flex items-center text-zinc-500 border-r border-zinc-800 pr-1.5 mr-1 gap-0.5 flex-shrink-0">
          <button
            type="button"
            onClick={scrollLeft}
            className="p-1 hover:text-zinc-300 hover:bg-zinc-800 rounded transition"
            /* 定数参照: UI_MESSAGES.PREV_SHEET_TOOLTIP */
            title={UI_MESSAGES.PREV_SHEET_TOOLTIP}
          >
            <ChevronLeft className="w-3.5 h-3.5" />
          </button>
          <button
            type="button"
            onClick={scrollRight}
            className="p-1 hover:text-zinc-300 hover:bg-zinc-800 rounded transition"
            /* 定数参照: UI_MESSAGES.NEXT_SHEET_TOOLTIP */
            title={UI_MESSAGES.NEXT_SHEET_TOOLTIP}
          >
            <ChevronRight className="w-3.5 h-3.5" />
          </button>
        </div>

        {/* シートタブ一覧 */}
        <div
          ref={scrollContainerRef}
          className="flex items-center gap-1 overflow-x-auto no-scrollbar min-w-0"
        >
          {sheets.map((sheet) => {
            const isActive = sheet === activeSheet;
            return (
              <button
                key={sheet}
                type="button"
                onClick={() => onSelectSheet(sheet)}
                className={`px-3 py-1 rounded text-xs font-medium flex items-center gap-1.5 whitespace-nowrap transition flex-shrink-0 ${
                  isActive
                    ? "bg-[#27272a] text-emerald-400 border-b-2 border-emerald-500 font-semibold shadow-sm"
                    : "text-zinc-400 hover:text-zinc-200 hover:bg-zinc-800/60"
                }`}
              >
                <FileSpreadsheet className="w-3 h-3" />
                <span>{sheet}</span>
              </button>
            );
          })}
        </div>
      </div>

      <div className="text-[10px] text-zinc-500 flex items-center gap-1 pl-2 flex-shrink-0">
        <span className="bg-zinc-800 px-1.5 py-0.5 rounded border border-zinc-700 font-mono text-[10px] text-zinc-400">
          {/* 定数参照: UI_MESSAGES.SHEET_TABS_BADGE */}
          {UI_MESSAGES.SHEET_TABS_BADGE}
        </span>
      </div>
    </div>
  );
};
