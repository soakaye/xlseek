import React from "react";

interface FormulaBarProps {
  cellAddress: string;
  formulaOrValue: string;
}

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
