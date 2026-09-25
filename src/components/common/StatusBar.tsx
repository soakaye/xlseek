import React from "react";
import { FileText, FileSpreadsheet } from "lucide-react";
import { save } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import { ScanProgress, SearchMatch, ExportRequest } from "../../types/search";

interface StatusBarProps {
  progress: ScanProgress | null;
  items: SearchMatch[];
  onShowToast: (msg: string) => void;
}

export const StatusBar: React.FC<StatusBarProps> = ({
  progress,
  items,
  onShowToast,
}) => {
  const percent =
    progress && progress.total_files > 0
      ? progress.state === "Completed"
        ? 100
        : Math.min(99, Math.max(1, Math.round((progress.scanned_files / progress.total_files) * 100)))
      : progress?.state === "Completed"
      ? 100
      : 0;

  const handleExport = async (format: "csv" | "xlsx") => {
    if (items.length === 0) {
      onShowToast("エクスポート対象の結果がありません");
      return;
    }

    try {
      const defaultFilename = `ExcelGrep_Results_${new Date()
        .toISOString()
        .slice(0, 10)}.${format}`;

      const selectedPath = await save({
        filters: [
          {
            name: format === "csv" ? "CSVファイル" : "Excelブック",
            extensions: [format],
          },
        ],
        defaultPath: defaultFilename,
      });

      if (!selectedPath) return;

      const req: ExportRequest = {
        format,
        output_path: selectedPath,
        items,
      };

      await invoke("export_results", { request: req });
      onShowToast(
        `${format.toUpperCase()}ファイルを保存しました: ${selectedPath}`
      );
    } catch (err) {
      console.error("エクスポートエラー:", err);
      onShowToast(`エクスポート失敗: ${err}`);
    }
  };

  const statusText = (() => {
    if (!progress) {
      return "検索待機中";
    }
    const elapsedSec = (progress.elapsed_ms / 1000).toFixed(2);
    switch (progress.state) {
      case "Scanning":
        return `スキャン中: ${progress.scanned_files}/${progress.total_files} ファイル (${progress.matches_found} 件一致, ${elapsedSec}s) - ${progress.current_file}`;
      case "Completed":
        return `完了: ${progress.matches_found} 件一致 (${progress.scanned_files} ファイル, ${elapsedSec}s)`;
      case "Cancelled":
        return `検索中断: ${progress.matches_found} 件一致 (${progress.scanned_files} ファイル走査済)`;
      case "Error":
        return "エラーが発生しました";
    }
  })();

  const isScanning = progress?.state === "Scanning";

  return (
    <footer className="h-10 bg-[#18181b] border-t border-zinc-800 px-4 flex items-center justify-between text-xs text-zinc-400 flex-shrink-0 select-none">
      {/* 検索メトリクス & 進捗 */}
      <div className="flex items-center gap-4 min-w-0 flex-1">
        <div className="flex items-center gap-2 truncate">
          <span
            className={`w-2 h-2 rounded-full flex-shrink-0 ${
              isScanning
                ? "bg-amber-400 animate-pulse"
                : progress?.state === "Completed"
                ? "bg-emerald-500"
                : progress?.state === "Cancelled"
                ? "bg-red-400"
                : "bg-zinc-600"
            }`}
          />
          <span
            className="text-zinc-200 font-medium truncate"
            title={statusText}
          >
            {statusText}
          </span>
        </div>

        {/* プログレスバー */}
        {(isScanning || progress?.state === "Completed") && (
          <div className="hidden md:flex items-center gap-2 flex-shrink-0">
            <div className="w-28 bg-zinc-800 h-1.5 rounded-full overflow-hidden border border-zinc-700/50">
              <div
                className="bg-excel h-full rounded-full transition-all duration-300"
                style={{ width: `${percent}%` }}
              />
            </div>
            <span className="text-[11px] text-zinc-400 font-mono">
              {percent}% ({progress?.scanned_files}/{progress?.total_files})
            </span>
          </div>
        )}
      </div>

      {/* エクスポートボタン群 */}
      <div className="flex items-center gap-2 flex-shrink-0">
        <button
          type="button"
          onClick={() => handleExport("csv")}
          disabled={items.length === 0}
          className="px-2.5 py-1 bg-zinc-800 hover:bg-zinc-700 disabled:opacity-40 disabled:cursor-not-allowed text-zinc-200 hover:text-white rounded border border-zinc-700 flex items-center gap-1.5 transition"
        >
          <FileText className="w-3.5 h-3.5 text-zinc-400" />
          <span>CSV 出力</span>
        </button>
        <button
          type="button"
          onClick={() => handleExport("xlsx")}
          disabled={items.length === 0}
          className="px-2.5 py-1 bg-zinc-800 hover:bg-zinc-700 disabled:opacity-40 disabled:cursor-not-allowed text-emerald-400 hover:text-emerald-300 rounded border border-zinc-700 flex items-center gap-1.5 transition"
        >
          <FileSpreadsheet className="w-3.5 h-3.5 text-emerald-500" />
          <span>Excel 出力</span>
        </button>
      </div>
    </footer>
  );
};
