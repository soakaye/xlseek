import React, { useState } from "react";
import { FileSpreadsheet, Layers, Folder, Copy, Check } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import { SearchMatch } from "../../types/search";

interface PreviewHeaderProps {
  selectedMatch: SearchMatch | null;
  onShowToast: (msg: string) => void;
}

export const PreviewHeader: React.FC<PreviewHeaderProps> = ({
  selectedMatch,
  onShowToast,
}) => {
  const [copied, setCopied] = useState(false);

  if (!selectedMatch) {
    return (
      <div className="p-3 bg-[#1c1c1f] border-b border-zinc-800 text-xs text-zinc-500">
        項目を選択するとプレビューが表示されます
      </div>
    );
  }

  const handleOpenInExcel = async () => {
    try {
      await invoke("open_in_excel", { filePath: selectedMatch.full_path });
    } catch (err) {
      onShowToast(`Excel起動エラー: ${err}`);
    }
  };

  const handleOpenFolder = async () => {
    try {
      await invoke("open_in_folder", { filePath: selectedMatch.full_path });
    } catch (err) {
      onShowToast(`フォルダを開けませんでした: ${err}`);
    }
  };

  const handleCopyPath = async () => {
    try {
      await navigator.clipboard.writeText(selectedMatch.full_path);
      setCopied(true);
      onShowToast("ファイルパスをコピーしました");
      setTimeout(() => setCopied(false), 2000);
    } catch (err) {
      onShowToast("クリップボードへのコピーに失敗しました");
    }
  };

  return (
    <div className="p-3 pr-4 bg-[#1c1c1f] border-b border-zinc-800 flex flex-col gap-2 flex-shrink-0">
      {/* 1行目: ファイル名 & シート名バッジ と アクションボタン群 */}
      <div className="flex items-center justify-between gap-3">
        <div className="flex items-center gap-2 min-w-0 flex-1">
          <FileSpreadsheet className="w-4 h-4 text-emerald-400 flex-shrink-0" />
          <span
            className="text-xs font-semibold text-zinc-100 truncate"
            title={selectedMatch.file_name}
          >
            {selectedMatch.file_name}
          </span>

          {/* シート名バッジ */}
          <div
            className="flex items-center gap-1.5 bg-emerald-950/80 text-emerald-300 border border-emerald-700/70 px-2 py-0.5 rounded text-[11px] font-medium flex-shrink-0 shadow-sm"
            title="検索一致が発生したワークシート"
          >
            <Layers className="w-3 h-3 text-emerald-400" />
            <span className="text-zinc-400 font-normal">シート:</span>
            <span className="font-bold text-emerald-200">
              {selectedMatch.sheet_name}
            </span>
          </div>
        </div>

        {/* アクションボタン群 */}
        <div className="flex items-center gap-2 flex-shrink-0 mr-1">
          <button
            onClick={handleOpenInExcel}
            className="px-2.5 py-1.5 bg-excel hover:bg-excel-hover active:scale-[0.98] text-white text-xs font-medium rounded flex items-center gap-1.5 shadow transition flex-shrink-0"
          >
            <FileSpreadsheet className="w-3.5 h-3.5" />
            <span>Excel で開く</span>
          </button>
          <button
            onClick={handleOpenFolder}
            title="ファイルの保存場所を開く"
            className="p-1.5 bg-zinc-800 hover:bg-zinc-700 active:scale-[0.98] text-zinc-300 hover:text-white rounded border border-zinc-700 transition flex-shrink-0"
          >
            <Folder className="w-3.5 h-3.5" />
          </button>
        </div>
      </div>

      {/* 2行目: ファイルパス専用行 (独立行 + 右端マージン) */}
      <div className="flex items-center gap-1.5 text-[10px] text-zinc-400 font-mono bg-zinc-900/80 px-2.5 py-1 rounded border border-zinc-800/80 min-w-0 mr-1">
        <Folder className="w-3 h-3 text-zinc-500 flex-shrink-0" />
        <span className="text-zinc-500 select-none flex-shrink-0">場所:</span>
        <span
          className="text-zinc-400 truncate select-text flex-1"
          title={selectedMatch.full_path}
        >
          {selectedMatch.full_path}
        </span>
        <button
          onClick={handleCopyPath}
          title="パスをコピー"
          className="p-0.5 hover:text-zinc-200 text-zinc-500 rounded flex-shrink-0 transition"
        >
          {copied ? (
            <Check className="w-3 h-3 text-emerald-400" />
          ) : (
            <Copy className="w-3 h-3" />
          )}
        </button>
      </div>
    </div>
  );
};
