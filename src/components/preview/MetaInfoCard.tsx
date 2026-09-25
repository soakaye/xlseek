import React from "react";
import { Info } from "lucide-react";
import { SearchMatch } from "../../types/search";

interface MetaInfoCardProps {
  match: SearchMatch | null;
}

export const MetaInfoCard: React.FC<MetaInfoCardProps> = ({ match }) => {
  if (!match) return null;

  const matchTypeLabel = (() => {
    switch (match.match_type) {
      case "CellValue":
        return "セル値 (Text / Number)";
      case "Formula":
        return "数式 (Formula)";
      case "Comment":
        return "コメント / メモ";
      case "HiddenSheet":
        return "非表示シート一致";
    }
  })();

  return (
    <div className="mt-3 p-3 bg-zinc-900/90 rounded-lg border border-zinc-800 text-xs space-y-2 mr-2 flex-shrink-0">
      <div className="flex items-center justify-between text-zinc-400">
        <span className="font-medium text-zinc-300 flex items-center gap-1.5">
          <Info className="w-3.5 h-3.5 text-emerald-400" /> 一致の詳細情報
        </span>
        <span className="text-[11px] text-zinc-500">
          タイプ: <strong className="text-zinc-300">{matchTypeLabel}</strong>
        </span>
      </div>

      <div className="text-zinc-300 bg-zinc-950/80 p-2 rounded border border-zinc-800/80 font-mono text-[11px] select-text break-words">
        {match.full_content || "(空)"}
      </div>

      <div className="flex items-center gap-4 text-[11px] text-zinc-400 pt-1">
        <span>
          行番号: <strong className="text-zinc-200">{match.row_index}</strong>
        </span>
        <span>
          列番号:{" "}
          <strong className="text-zinc-200">
            {match.col_index} ({match.col_name})
          </strong>
        </span>
        <span>
          非表示状態:{" "}
          <strong className="text-zinc-200">
            {match.match_type === "HiddenSheet" ? "非表示" : "表示"}
          </strong>
        </span>
      </div>
    </div>
  );
};
