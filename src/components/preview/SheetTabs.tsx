import React, { useRef } from "react";
import { ChevronLeft, ChevronRight, FileSpreadsheet } from "lucide-react";

interface SheetTabsProps {
  sheets: string[];
  activeSheet: string;
  onSelectSheet: (sheetName: string) => void;
}

export const SheetTabs: React.FC<SheetTabsProps> = ({
  sheets,
  activeSheet,
  onSelectSheet,
}) => {
  const scrollContainerRef = useRef<HTMLDivElement>(null);

  const scrollLeft = () => {
    if (scrollContainerRef.current) {
      scrollContainerRef.current.scrollBy({ left: -100, behavior: "smooth" });
    }
  };

  const scrollRight = () => {
    if (scrollContainerRef.current) {
      scrollContainerRef.current.scrollBy({ left: 100, behavior: "smooth" });
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
            title="前のシートへ"
          >
            <ChevronLeft className="w-3.5 h-3.5" />
          </button>
          <button
            type="button"
            onClick={scrollRight}
            className="p-1 hover:text-zinc-300 hover:bg-zinc-800 rounded transition"
            title="次のシートへ"
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
          Sheet Tabs
        </span>
      </div>
    </div>
  );
};
