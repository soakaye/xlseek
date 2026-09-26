/**
 * @fileoverview 検索結果一覧テーブルコンポーネント (src/components/results/ResultTable.tsx)
 *
 * ## 処理内容
 * 検索に一致したセルアイテム一覧を TanStack Virtual を用いて高速仮想スクロール表示する。
 * 列ヘッダークリックによるソート（昇順/降順）、キーワードによる結果内絞り込み、
 * 上下キーによる選択行移動、および各セル一致バッジの描画を提供する。
 * 憲章原則II（定数参照）および原則III（ヘッダコメント）に準拠。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化（LAYOUT_CONSTANTS）およびJSDoc付与。
 */

import React, { useState, useRef, useMemo, useEffect } from "react";
import { useVirtualizer } from "@tanstack/react-virtual";
import { ListFilter, Filter, ArrowUp, ArrowDown } from "lucide-react";
import { SearchMatch, MatchType } from "../../types/search";
import { LAYOUT_CONSTANTS, UI_MESSAGES } from "../../constants";

interface ResultTableProps {
  items: SearchMatch[];
  selectedId: number | null;
  onSelectItem: (item: SearchMatch) => void;
}

type SortField = "file_name" | "sheet_name" | "cell_address" | "match_type";
type SortOrder = "asc" | "desc";

/**
 * ## 処理内容
 * 検索結果テーブルを表示し、選択や絞り込み、ソートを管理するUIコンポーネント。
 *
 * ## 引数
 * @param props - 検索結果リスト、選択中アイテムID、選択ハンドラ
 *
 * ## 戻り値
 * @returns レンダリング要素
 *
 * ## エラー / 例外発生条件
 * panicや例外は発生しない。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化。
 */
export const ResultTable: React.FC<ResultTableProps> = ({
  items,
  selectedId,
  onSelectItem,
}) => {
  const [filterText, setFilterText] = useState("");
  const [sortField, setSortField] = useState<SortField>("file_name");
  const [sortOrder, setSortOrder] = useState<SortOrder>("asc");

  const parentRef = useRef<HTMLDivElement>(null);

  // 絞り込み & ソート処理
  const filteredItems = useMemo(() => {
    let result = items;
    if (filterText.trim()) {
      const q = filterText.toLowerCase();
      result = items.filter(
        (it) =>
          it.file_name.toLowerCase().includes(q) ||
          it.sheet_name.toLowerCase().includes(q) ||
          it.cell_address.toLowerCase().includes(q) ||
          it.snippet.toLowerCase().includes(q) ||
          it.full_content.toLowerCase().includes(q)
      );
    }

    return [...result].sort((a, b) => {
      const valA = a[sortField];
      const valB = b[sortField];
      if (typeof valA === "string" && typeof valB === "string") {
        const cmp = valA.localeCompare(valB, undefined, {
          numeric: true,
          sensitivity: "base",
        });
        return sortOrder === "asc" ? cmp : -cmp;
      }
      return 0;
    });
  }, [items, filterText, sortField, sortOrder]);

  const rowVirtualizer = useVirtualizer({
    count: filteredItems.length,
    getScrollElement: () => parentRef.current,
    // 定数参照: LAYOUT_CONSTANTS.RESULT_ROW_HEIGHT_PX を使用
    estimateSize: () => LAYOUT_CONSTANTS.RESULT_ROW_HEIGHT_PX,
    // 定数参照: LAYOUT_CONSTANTS.VIRTUAL_OVERSCAN を使用
    overscan: LAYOUT_CONSTANTS.VIRTUAL_OVERSCAN,
  });

  const handleSort = (field: SortField) => {
    if (sortField === field) {
      setSortOrder(sortOrder === "asc" ? "desc" : "asc");
    } else {
      setSortField(field);
      setSortOrder("asc");
    }
  };

  // キーボード上下矢印キーでの選択移動
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (filteredItems.length === 0) return;
      if (e.key === "ArrowDown" || e.key === "ArrowUp") {
        e.preventDefault();
        const currentIndex = filteredItems.findIndex((it) => it.id === selectedId);
        let nextIndex = 0;
        if (currentIndex !== -1) {
          nextIndex =
            e.key === "ArrowDown"
              ? Math.min(filteredItems.length - 1, currentIndex + 1)
              : Math.max(0, currentIndex - 1);
        }
        const nextItem = filteredItems[nextIndex];
        if (nextItem) {
          onSelectItem(nextItem);
          rowVirtualizer.scrollToIndex(nextIndex, { align: "auto" });
        }
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [filteredItems, selectedId, onSelectItem, rowVirtualizer]);

  const renderBadge = (matchType: MatchType) => {
    switch (matchType) {
      case "CellValue":
        return (
          <span className="px-1.5 py-0.5 rounded text-[10px] font-medium bg-blue-950/80 text-blue-300 border border-blue-800/60">
            {UI_MESSAGES.MATCH_TYPE_CELL_VALUE}
          </span>
        );
      case "Formula":
        return (
          <span className="px-1.5 py-0.5 rounded text-[10px] font-medium bg-purple-950/80 text-purple-300 border border-purple-800/60">
            {UI_MESSAGES.MATCH_TYPE_FORMULA}
          </span>
        );
      case "Comment":
        return (
          <span className="px-1.5 py-0.5 rounded text-[10px] font-medium bg-amber-950/80 text-amber-300 border border-amber-800/60">
            {UI_MESSAGES.MATCH_TYPE_COMMENT}
          </span>
        );
      case "HiddenSheet":
        return (
          <span className="px-1.5 py-0.5 rounded text-[10px] font-medium bg-zinc-800 text-zinc-400 border border-zinc-700">
            {UI_MESSAGES.MATCH_TYPE_HIDDEN_SHEET}
          </span>
        );
    }
  };

  return (
    <div className="w-[58%] border-r border-zinc-800 flex flex-col bg-[#141416]">
      {/* テーブル操作ヘッダー */}
      <div className="p-2.5 bg-[#18181b] border-b border-zinc-800 flex items-center justify-between text-xs flex-shrink-0">
        <div className="flex items-center gap-2 text-zinc-400">
          <ListFilter className="w-4 h-4 text-zinc-400" />
          <span className="font-medium text-zinc-200">{UI_MESSAGES.RESULTS_TITLE}</span>
          <span className="bg-zinc-800 text-emerald-400 px-2 py-0.2 rounded-full font-mono text-[11px] border border-zinc-700">
            {filteredItems.length} {UI_MESSAGES.RESULT_COUNT_SUFFIX}
          </span>
        </div>
        <div className="flex items-center gap-2">
          {/* 簡易フィルタ */}
          <div className="relative">
            <input
              type="text"
              value={filterText}
              onChange={(e) => setFilterText(e.target.value)}
              placeholder={UI_MESSAGES.FILTER_RESULTS_PLACEHOLDER}
              className="w-44 pl-7 pr-2 py-1 bg-zinc-900 border border-zinc-700 rounded text-xs text-zinc-200 placeholder-zinc-500 focus:outline-none focus:border-zinc-500"
            />
            <Filter className="w-3.5 h-3.5 text-zinc-500 absolute left-2 top-1.5" />
          </div>
        </div>
      </div>

      {/* 固定テーブルヘッダー */}
      <div className="bg-[#1e1e22] text-zinc-400 font-semibold border-b border-zinc-800 shadow-sm z-10 text-xs flex-shrink-0">
        <div className="flex items-center">
          <button
            onClick={() => handleSort("file_name")}
            className="w-[30%] py-2.5 px-3 flex items-center gap-1 hover:text-zinc-200 transition text-left"
          >
            <span>{UI_MESSAGES.COLUMN_FILE}</span>
            {sortField === "file_name" &&
              (sortOrder === "asc" ? (
                <ArrowUp className="w-3 h-3 text-emerald-400" />
              ) : (
                <ArrowDown className="w-3 h-3 text-emerald-400" />
              ))}
          </button>
          <button
            onClick={() => handleSort("sheet_name")}
            className="w-[18%] py-2.5 px-3 flex items-center gap-1 hover:text-zinc-200 transition text-left"
          >
            <span>{UI_MESSAGES.COLUMN_SHEET}</span>
            {sortField === "sheet_name" &&
              (sortOrder === "asc" ? (
                <ArrowUp className="w-3 h-3 text-emerald-400" />
              ) : (
                <ArrowDown className="w-3 h-3 text-emerald-400" />
              ))}
          </button>
          <button
            onClick={() => handleSort("cell_address")}
            className="w-[12%] py-2.5 px-3 flex items-center gap-1 hover:text-zinc-200 transition text-left"
          >
            <span>{UI_MESSAGES.COLUMN_CELL}</span>
            {sortField === "cell_address" &&
              (sortOrder === "asc" ? (
                <ArrowUp className="w-3 h-3 text-emerald-400" />
              ) : (
                <ArrowDown className="w-3 h-3 text-emerald-400" />
              ))}
          </button>
          <button
            onClick={() => handleSort("match_type")}
            className="w-[15%] py-2.5 px-3 flex items-center gap-1 hover:text-zinc-200 transition text-left"
          >
            <span>{UI_MESSAGES.COLUMN_MATCH_TYPE}</span>
            {sortField === "match_type" &&
              (sortOrder === "asc" ? (
                <ArrowUp className="w-3 h-3 text-emerald-400" />
              ) : (
                <ArrowDown className="w-3 h-3 text-emerald-400" />
              ))}
          </button>
          <div className="w-[25%] py-2.5 px-3 text-left">{UI_MESSAGES.COLUMN_PREVIEW}</div>
        </div>
      </div>

      {/* 仮想スクロールコンテナ */}
      <div ref={parentRef} className="flex-1 overflow-auto">
        {filteredItems.length === 0 ? (
          <div className="h-full flex items-center justify-center text-xs text-zinc-500">
            {UI_MESSAGES.NO_FILTERED_RESULTS}
          </div>
        ) : (
          <div
            style={{
              height: `${rowVirtualizer.getTotalSize()}px`,
              width: "100%",
              position: "relative",
            }}
          >
            {rowVirtualizer.getVirtualItems().map((virtualRow) => {
              const item = filteredItems[virtualRow.index];
              const isSelected = item.id === selectedId;

              return (
                <div
                  key={virtualRow.key}
                  onClick={() => onSelectItem(item)}
                  style={{
                    position: "absolute",
                    top: 0,
                    left: 0,
                    width: "100%",
                    height: `${virtualRow.size}px`,
                    transform: `translateY(${virtualRow.start}px)`,
                  }}
                  className={`flex items-center text-xs border-b border-zinc-800/80 cursor-pointer select-none transition-colors overflow-hidden ${
                    isSelected
                      ? "bg-emerald-950/70 text-emerald-200 font-medium"
                      : "hover:bg-zinc-800/60 text-zinc-300"
                  }`}
                >
                  <div className="w-[30%] px-3 truncate font-mono text-[11px]" title={item.file_name}>
                    {item.file_name}
                  </div>
                  <div className="w-[18%] px-3 truncate" title={item.sheet_name}>
                    {item.sheet_name}
                  </div>
                  <div className="w-[12%] px-3 font-mono font-bold text-emerald-400">
                    {item.cell_address}
                  </div>
                  <div className="w-[15%] px-3">
                    {renderBadge(item.match_type)}
                  </div>
                  <div
                    className="w-[25%] px-3 truncate text-zinc-300 font-mono text-[11px]"
                    title={item.full_content}
                    dangerouslySetInnerHTML={{ __html: item.snippet }}
                  />
                </div>
              );
            })}
          </div>
        )}
      </div>
    </div>
  );
};
