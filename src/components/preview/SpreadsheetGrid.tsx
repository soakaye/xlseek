/**
 * @fileoverview Cell preview spreadsheet grid component (src/components/preview/SpreadsheetGrid.tsx)
 *
 * ## Description
 * Renders surrounding cells (e.g. +/-3 rows, +/-2 columns) around the search match in a virtual spreadsheet grid table.
 * Provides sticky row/column headers, target match and active selection cell highlighting, and cell click event handling.
 * Supports smooth 2D scrolling and freeze panes.
 * Complies with Constitution Principle I (English documentation), Principle II (constant reference), and Principle III (comprehensive documentation).
 */

import React from "react";
import { Move } from "lucide-react";
import { CellPreviewData } from "../../types/search";
import { useTranslation } from "../../i18n";

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
 * ## Description
 * Surrounding cells preview grid component.
 *
 * ## Arguments
 * @param props - SpreadsheetGridProps
 *
 * ## Returns
 * @returns Rendered grid table element
 *
 * ## Errors / Exceptions
 * Displays placeholder guidance when previewData is null or empty.
 */
export const SpreadsheetGrid: React.FC<SpreadsheetGridProps> = ({
  previewData,
  isLoading,
  selectedCell,
  onSelectCell,
}) => {
  const t = useTranslation();
  if (isLoading) {
    return (
      <div className="flex-1 p-6 flex items-center justify-center text-xs text-zinc-500">
        <div className="flex items-center gap-2">
          <div className="w-4 h-4 border-2 border-emerald-500 border-t-transparent rounded-full animate-spin"></div>
          {/* Constant reference: t("ui.PREVIEW_LOADING") */}
          <span>{t("ui.PREVIEW_LOADING")}</span>
        </div>
      </div>
    );
  }

  if (!previewData || previewData.rows.length === 0) {
    return (
      <div className="flex-1 p-6 flex items-center justify-center text-xs text-zinc-500">
        {/* Constant reference: t("ui.PREVIEW_EMPTY") */}
        {t("ui.PREVIEW_EMPTY")}
      </div>
    );
  }

  return (
    <div className="flex-1 min-h-0 min-w-0 p-3.5 pr-4 pb-2 bg-[#141416] flex flex-col">
      {/* Guidance mini bar */}
      <div className="flex items-center justify-between text-[11px] text-zinc-400 mb-2 px-1 select-none flex-shrink-0">
        <div className="flex items-center gap-1.5">
          <span className="w-1.5 h-1.5 rounded-full bg-emerald-500"></span>
          {/* Constant reference: t("ui.PREVIEW_GUIDANCE") */}
          <span className="font-medium text-zinc-300">{t("ui.PREVIEW_GUIDANCE")}</span>
        </div>
        <div className="flex items-center gap-1 text-zinc-500 bg-zinc-900/90 px-2 py-0.5 rounded border border-zinc-800 text-[10px]">
          <Move className="w-2.5 h-2.5 text-zinc-400" />
          {/* Constant reference: t("ui.PREVIEW_SCROLL_GUIDANCE") */}
          <span>{t("ui.PREVIEW_SCROLL_GUIDANCE")}</span>
        </div>
      </div>

      {/* Spreadsheet grid scroll container */}
      <div className="flex-1 min-h-0 min-w-0 overflow-auto rounded border border-zinc-700/80 bg-[#1a1a1d] shadow-sm relative">
        <table className="excel-grid w-max min-w-full text-xs border-collapse font-sans">
          <thead className="sticky top-0 z-20">
            <tr className="bg-[#242428] text-zinc-400 text-center font-medium">
              {/* Row number header cell */}
              <th className="w-12 py-1.5 px-2 bg-[#202024] select-none text-zinc-500 font-mono text-[11px] sticky left-0 top-0 z-30">
                #
              </th>
              {/* Column header cells */}
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
                {/* Row number header (sticky left) */}
                <td className="w-12 py-1.5 px-2 bg-[#202024] select-none text-zinc-500 font-mono text-[11px] text-center sticky left-0 z-10 border border-zinc-800">
                  {row.row_number}
                </td>
                {/* Cells */}
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
                          ? `${t("ui.FORMULA_TOOLTIP_PREFIX")} ${cellInfo.formula}`
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
