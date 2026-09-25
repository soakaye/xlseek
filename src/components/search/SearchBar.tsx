import React, { KeyboardEvent, useState, useRef, useEffect } from "react";
import { Search, X, Folder, FolderOpen, SlidersHorizontal, Square, ArrowDownToLine } from "lucide-react";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { SearchQuery } from "../../types/search";

interface SearchBarProps {
  query: SearchQuery;
  onChangeQuery: (newQuery: Partial<SearchQuery>) => void;
  onSearch: () => void;
  onCancel: () => void;
  isScanning: boolean;
}

export const SearchBar: React.FC<SearchBarProps> = ({
  query,
  onChangeQuery,
  onSearch,
  onCancel,
  isScanning,
}) => {
  const [isDragOver, setIsDragOver] = useState(false);
  const folderInputRef = useRef<HTMLDivElement>(null);

  // Tauri ネイティブのドラッグ＆ドロップイベントの購読
  useEffect(() => {
    let unlisten: (() => void) | undefined;

    const setupDragDrop = async () => {
      try {
        const webview = getCurrentWebview();
        unlisten = await webview.onDragDropEvent(async (event) => {
          const payload = event.payload;

          if (payload.type === "over" || payload.type === "enter") {
            if (folderInputRef.current) {
              const rect = folderInputRef.current.getBoundingClientRect();
              const scale = window.devicePixelRatio || 1;
              const logicalX = payload.position.x / scale;
              const logicalY = payload.position.y / scale;

              const isInside =
                logicalX >= rect.left &&
                logicalX <= rect.right &&
                logicalY >= rect.top &&
                logicalY <= rect.bottom;

              setIsDragOver(isInside);
            }
          } else if (payload.type === "drop") {
            if (folderInputRef.current) {
              const rect = folderInputRef.current.getBoundingClientRect();
              const scale = window.devicePixelRatio || 1;
              const logicalX = payload.position.x / scale;
              const logicalY = payload.position.y / scale;

              const isInside =
                logicalX >= rect.left &&
                logicalX <= rect.right &&
                logicalY >= rect.top &&
                logicalY <= rect.bottom;

              if (isInside && payload.paths && payload.paths.length > 0) {
                try {
                  const resolvedPath = await invoke<string>("resolve_dropped_path", {
                    path: payload.paths[0],
                  });
                  onChangeQuery({ target_dir: resolvedPath });
                } catch (err) {
                  console.error("ドロップパス解決エラー:", err);
                }
              }
            }
            setIsDragOver(false);
          } else if (payload.type === "leave") {
            setIsDragOver(false);
          }
        });
      } catch (err) {
        console.warn("Tauri drag-drop listener 初期化エラー:", err);
      }
    };

    setupDragDrop();

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, [onChangeQuery]);

  // HTML5 標準のドラッグ＆ドロップ（ブラウザ環境向けフォールバック）
  const handleHtmlDragOver = (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragOver(true);
  };

  const handleHtmlDragLeave = (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragOver(false);
  };

  const handleHtmlDrop = async (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragOver(false);

    if (e.dataTransfer.files && e.dataTransfer.files.length > 0) {
      const file = e.dataTransfer.files[0];
      const filePath = (file as any).path;
      if (filePath) {
        try {
          const resolvedPath = await invoke<string>("resolve_dropped_path", {
            path: filePath,
          });
          onChangeQuery({ target_dir: resolvedPath });
        } catch (err) {
          console.error("ドロップパス解決エラー:", err);
        }
      }
    }
  };

  const handleKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter" && !isScanning) {
      onSearch();
    }
  };

  const handleBrowseFolder = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: "検索対象フォルダを選択",
      });
      if (selected && typeof selected === "string") {
        onChangeQuery({ target_dir: selected });
      }
    } catch (err) {
      console.error("フォルダ選択エラー:", err);
    }
  };

  return (
    <header className="bg-[#18181b] border-b border-zinc-800 p-4 shadow-md flex-shrink-0">
      <div className="max-w-[1920px] mx-auto space-y-3">
        {/* 上段: 検索キーワード入力 & フォルダ選択 & 検索実行/中断ボタン */}
        <div className="grid grid-cols-1 lg:grid-cols-12 gap-3 items-center">
          {/* 検索キーワード入力 */}
          <div className="lg:col-span-6 relative">
            <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none text-zinc-400">
              <Search className="w-4 h-4" />
            </div>
            <input
              type="text"
              value={query.keyword}
              onChange={(e) => onChangeQuery({ keyword: e.target.value })}
              onKeyDown={handleKeyDown}
              placeholder="検索するテキストまたは正規表現を入力... (Enterで検索)"
              className="w-full pl-9 pr-24 py-2 bg-[#202024] border border-zinc-700 rounded-lg text-sm text-zinc-100 placeholder-zinc-500 focus:outline-none focus:border-excel-light focus:ring-1 focus:ring-excel-light transition"
            />
            <div className="absolute inset-y-0 right-1 flex items-center gap-1 pr-1.5">
              {query.keyword && (
                <button
                  type="button"
                  onClick={() => onChangeQuery({ keyword: "" })}
                  title="クリア"
                  className="p-1 hover:bg-zinc-700 text-zinc-400 hover:text-zinc-200 rounded"
                >
                  <X className="w-3.5 h-3.5" />
                </button>
              )}
              <span className="text-[10px] text-zinc-400 font-mono bg-zinc-800 px-1.5 py-0.5 rounded border border-zinc-700">
                Enter
              </span>
            </div>
          </div>

          {/* フォルダパス選択 & DnD ドロップゾーン */}
          <div
            ref={folderInputRef}
            onDragOver={handleHtmlDragOver}
            onDragEnter={handleHtmlDragOver}
            onDragLeave={handleHtmlDragLeave}
            onDrop={handleHtmlDrop}
            className={`lg:col-span-4 relative rounded-lg transition-all ${
              isDragOver
                ? "ring-2 ring-emerald-500 bg-emerald-950/40 shadow-lg shadow-emerald-950/50"
                : ""
            }`}
          >
            <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none text-zinc-400">
              {isDragOver ? (
                <ArrowDownToLine className="w-4 h-4 text-emerald-400 animate-bounce" />
              ) : (
                <Folder className="w-4 h-4" />
              )}
            </div>
            <input
              type="text"
              value={query.target_dir}
              onChange={(e) => onChangeQuery({ target_dir: e.target.value })}
              placeholder={isDragOver ? "ここにフォルダをドロップ..." : "フォルダを選択またはドラッグ＆ドロップ..."}
              className={`w-full pl-9 pr-10 py-2 border rounded-lg text-sm placeholder-zinc-500 focus:outline-none font-mono text-xs transition ${
                isDragOver
                  ? "bg-emerald-950/30 border-emerald-500 text-emerald-200 border-dashed"
                  : "bg-[#202024] border-zinc-700 text-zinc-300 focus:border-excel-light"
              }`}
            />
            {isDragOver && (
              <div className="absolute inset-0 flex items-center justify-center bg-emerald-900/80 border-2 border-dashed border-emerald-400 rounded-lg pointer-events-none text-xs font-medium text-emerald-100 gap-2 z-10 backdrop-blur-[1px]">
                <ArrowDownToLine className="w-4 h-4 text-emerald-300 animate-bounce" />
                <span>フォルダをここにドロップ</span>
              </div>
            )}
            <button
              type="button"
              onClick={handleBrowseFolder}
              title="フォルダを選択"
              className="absolute inset-y-1 right-1 px-2.5 flex items-center justify-center bg-zinc-700 hover:bg-zinc-600 rounded text-zinc-200 transition z-0"
            >
              <FolderOpen className="w-4 h-4" />
            </button>
          </div>

          {/* 検索開始 / 中断ボタン */}
          <div className="lg:col-span-2">
            {isScanning ? (
              <button
                type="button"
                onClick={onCancel}
                className="w-full py-2 px-4 bg-red-600 hover:bg-red-500 active:scale-[0.99] text-white font-semibold text-sm rounded-lg flex items-center justify-center gap-2 shadow-lg shadow-red-950/40 transition"
              >
                <Square className="w-4 h-4 fill-white" />
                <span>CANCEL</span>
              </button>
            ) : (
              <button
                type="button"
                onClick={onSearch}
                className="w-full py-2 px-4 bg-excel hover:bg-excel-hover active:scale-[0.99] text-white font-semibold text-sm rounded-lg flex items-center justify-center gap-2 shadow-lg shadow-emerald-950/40 transition"
              >
                <Search className="w-4 h-4" />
                <span>SEARCH</span>
              </button>
            )}
          </div>
        </div>

        {/* 下段: 検索オプション (トグルピル) */}
        <div className="flex flex-wrap items-center justify-between gap-2 pt-1 text-xs select-none">
          <div className="flex flex-wrap items-center gap-2">
            <span className="text-zinc-400 mr-1 font-medium flex items-center gap-1">
              <SlidersHorizontal className="w-3.5 h-3.5" /> オプション:
            </span>

            {/* 大文字/小文字 */}
            <label
              className={`cursor-pointer flex items-center gap-1.5 px-2.5 py-1 rounded-full border transition ${
                query.match_case
                  ? "bg-emerald-950/60 text-emerald-300 border-emerald-600/80 font-medium"
                  : "bg-zinc-800 hover:bg-zinc-700/80 text-zinc-300 border-zinc-700"
              }`}
            >
              <input
                type="checkbox"
                checked={query.match_case ?? false}
                onChange={(e) => onChangeQuery({ match_case: e.target.checked })}
                className="accent-excel rounded cursor-pointer"
              />
              <span>大文字/小文字を区別</span>
            </label>

            {/* 正規表現 */}
            <label
              className={`cursor-pointer flex items-center gap-1.5 px-2.5 py-1 rounded-full border transition ${
                query.use_regex
                  ? "bg-emerald-950/60 text-emerald-300 border-emerald-600/80 font-medium"
                  : "bg-zinc-800 hover:bg-zinc-700/80 text-zinc-300 border-zinc-700"
              }`}
            >
              <input
                type="checkbox"
                checked={query.use_regex ?? false}
                onChange={(e) => onChangeQuery({ use_regex: e.target.checked })}
                className="accent-emerald-500 rounded cursor-pointer"
              />
              <span>正規表現 (Regex)</span>
            </label>

            {/* 数式 */}
            <label
              className={`cursor-pointer flex items-center gap-1.5 px-2.5 py-1 rounded-full border transition ${
                query.include_formula
                  ? "bg-emerald-950/60 text-emerald-300 border-emerald-600/80 font-medium"
                  : "bg-zinc-800 hover:bg-zinc-700/80 text-zinc-300 border-zinc-700"
              }`}
            >
              <input
                type="checkbox"
                checked={query.include_formula ?? true}
                onChange={(e) => onChangeQuery({ include_formula: e.target.checked })}
                className="accent-excel rounded cursor-pointer"
              />
              <span>数式 (Formula)</span>
            </label>

            {/* コメント */}
            <label
              className={`cursor-pointer flex items-center gap-1.5 px-2.5 py-1 rounded-full border transition ${
                query.include_comment
                  ? "bg-emerald-950/60 text-emerald-300 border-emerald-600/80 font-medium"
                  : "bg-zinc-800 hover:bg-zinc-700/80 text-zinc-300 border-zinc-700"
              }`}
            >
              <input
                type="checkbox"
                checked={query.include_comment ?? true}
                onChange={(e) => onChangeQuery({ include_comment: e.target.checked })}
                className="accent-excel rounded cursor-pointer"
              />
              <span>コメント / メモ</span>
            </label>

            {/* 非表示シート */}
            <label
              className={`cursor-pointer flex items-center gap-1.5 px-2.5 py-1 rounded-full border transition ${
                query.include_hidden
                  ? "bg-emerald-950/60 text-emerald-300 border-emerald-600/80 font-medium"
                  : "bg-zinc-800 hover:bg-zinc-700/80 text-zinc-300 border-zinc-700"
              }`}
            >
              <input
                type="checkbox"
                checked={query.include_hidden ?? false}
                onChange={(e) => onChangeQuery({ include_hidden: e.target.checked })}
                className="accent-excel rounded cursor-pointer"
              />
              <span>非表示シート</span>
            </label>
          </div>

          {/* 対象拡張子表示 */}
          <div className="flex items-center gap-2 text-zinc-400 text-xs">
            <span>対象拡張子:</span>
            <div className="flex gap-1">
              {query.extensions?.map((ext) => (
                <span
                  key={ext}
                  className="bg-zinc-800 px-1.5 py-0.5 rounded text-[11px] font-mono border border-zinc-700 text-zinc-300"
                >
                  {ext}
                </span>
              ))}
            </div>
          </div>
        </div>
      </div>
    </header>
  );
};
