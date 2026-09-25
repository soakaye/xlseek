import React from "react";
import { Zap, Minus, Square, X } from "lucide-react";
import { getCurrentWindow } from "@tauri-apps/api/window";

interface WindowFrameProps {
  children: React.ReactNode;
}

export const WindowFrame: React.FC<WindowFrameProps> = ({ children }) => {
  // ウィンドウ最小化
  const handleMinimize = async () => {
    try {
      const appWindow = getCurrentWindow();
      await appWindow.minimize();
    } catch (err) {
      console.error("Failed to minimize window:", err);
    }
  };

  // ウィンドウ最大化 / 元に戻す
  const handleToggleMaximize = async () => {
    try {
      const appWindow = getCurrentWindow();
      await appWindow.toggleMaximize();
    } catch (err) {
      console.error("Failed to toggle maximize window:", err);
    }
  };

  // ウィンドウ閉じる
  const handleClose = async () => {
    try {
      const appWindow = getCurrentWindow();
      await appWindow.close();
    } catch (err) {
      console.error("Failed to close window:", err);
    }
  };

  return (
    <div className="bg-[#121214] text-zinc-100 min-h-screen flex flex-col font-sans select-none overflow-hidden h-screen border border-zinc-800/80">
      {/* ウィンドウタイトルバー */}
      <div 
        data-tauri-drag-region
        onDoubleClick={handleToggleMaximize}
        className="h-9 bg-[#1e1e20] flex items-center justify-between px-3 border-b border-zinc-800 text-xs text-zinc-400 select-none flex-shrink-0"
      >
        <div className="flex items-center gap-2 pointer-events-none">
          {/* Excel風アイコン */}
          <div className="w-5 h-5 bg-excel rounded flex items-center justify-center text-white font-bold text-xs shadow-sm">
            X
          </div>
          <span className="font-semibold text-zinc-200">Excel Grep</span>
          <span className="text-[11px] text-zinc-500">v0.1.0 (Rust & Tauri)</span>
        </div>

        <div className="flex items-center gap-2">
          <span className="text-[11px] bg-zinc-800/80 px-2 py-0.5 rounded text-zinc-400 border border-zinc-700/60 flex items-center gap-1">
            <Zap className="w-3 h-3 text-emerald-400" /> Calamine Engine
          </span>
          {/* ウィンドウコントロールボタン */}
          <div className="flex items-center ml-1">
            <button 
              onClick={(e) => {
                e.stopPropagation();
                handleMinimize();
              }}
              className="w-8 h-7 flex items-center justify-center hover:bg-zinc-700/60 rounded text-zinc-400 hover:text-zinc-200 transition"
              title="最小化"
            >
              <Minus className="w-3.5 h-3.5" />
            </button>
            <button 
              onClick={(e) => {
                e.stopPropagation();
                handleToggleMaximize();
              }}
              className="w-8 h-7 flex items-center justify-center hover:bg-zinc-700/60 rounded text-zinc-400 hover:text-zinc-200 transition"
              title="最大化"
            >
              <Square className="w-3 h-3" />
            </button>
            <button 
              onClick={(e) => {
                e.stopPropagation();
                handleClose();
              }}
              className="w-8 h-7 flex items-center justify-center hover:bg-red-600/80 rounded text-zinc-400 hover:text-white transition"
              title="閉じる"
            >
              <X className="w-3.5 h-3.5" />
            </button>
          </div>
        </div>
      </div>

      {/* メインコンテンツ */}
      <div className="flex-1 flex flex-col overflow-hidden">
        {children}
      </div>
    </div>
  );
};
