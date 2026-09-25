import React from "react";
import { Move } from "lucide-react";
import { CellPreviewData } from "../../types/search";

interface SpreadsheetGridProps {
  previewData: CellPreviewData | null;
  isLoading: boolean;
  selectedCell: {
    address: string;
    value: string;
  } | null;
  onSelectCell: (address: string, value: string, formula?: string | null) => void;
}

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
          <span>プレビュー読み込み中...</span>
        </div>
      </div>
    );
  }

  if (!previewData || previewData.rows.length === 0) {
    return (
      <div className="flex-1 p-6 flex items-center justify-center text-xs text-zinc-500">
        プレビューデータがありません
      </div>
    );
  }

  return (
    <div className="flex-1 p-3.5 pr-5 pb-3 overflow-auto bg-[#141416] flex flex-col">
      {/* ガイダンスミニバー */}
      <div className="flex items-center justify-between text-[11px] text-zinc-400 mb-2 px-1 select-none flex-shrink-0">
        <div className="flex items-center gap-1.5">
          <span className="w-1.5 h-1.5 rounded-full bg-emerald-500"></span>
          <span className="font-medium text-zinc-300">周辺セルプレビュー (前後3行・前後2列)</span>
        </div>
        <div className="flex items-center gap-1 text-zinc-500 bg-zinc-900/90 px-2 py-0.5 rounded border border-zinc-800 text-[10px]">
          <Move className="w-2.5 h-2.5 text-zinc-400" />
          <span>縦横スクロール可能 (固定見出し)</span>
        </div>
      </div>

      {/* テーブルラッパー (右辺境界線が絶対に欠けないように inline-block + pr-2) */}
      <div className="min-w-full inline-block pb-2 pr-2">
        <div className="rounded border border-zinc-700/80 bg-[#1a1a1d] shadow-sm overflow-hidden">
          <table className="excel-grid w-full text-xs border-collapse font-sans">
            <thead>
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
                            ? `数式: ${cellInfo.formula}`
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
    </div>
  );
};
