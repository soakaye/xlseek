/**
 * @fileoverview プレビューペインヘッダー＆アクションバーコンポーネント (src/components/preview/PreviewHeader.tsx)
 *
 * ## 処理内容
 * 選択されたファイル名、シート名バッジ、ファイルパスを表示し、外部アプリケーションでの起動（Excel、Numbers、Calc等）、
 * 関連付けダイアログの表示、保存フォルダを開く操作、およびファイルパスのクリップボードコピーを提供する。
 * 憲章原則I（自然かつ正確な日本語）、原則II（定数の外部抽出とハードコード禁止）、原則III（網羅的なヘッダコメント）に準拠。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 初版策定。
 * - v1.1.0 (2026-09-26, AI Agent): 憲章原則に準拠し、IPCコマンド名・UI文言の定数参照化および4要素ヘッダコメントを追加。
 */

import React, { useState, useEffect, useRef } from "react";
import { FileSpreadsheet, Layers, Folder, Copy, Check, ChevronDown, ExternalLink, BarChart2, Table } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import { SearchMatch, SupportedApp } from "../../types/search";
import { COMMANDS, LAYOUT_CONSTANTS, UI_MESSAGES } from "../../constants";

/**
 * プレビューヘッダーコンポーネントのプロパティ定義
 *
 * ## プロパティ一覧
 * - `selectedMatch`: SearchMatch | null - 現在選択されている検索一致データオブジェクト
 * - `onShowToast`: (msg: string) => void - トースト通知表示コールバック
 */
interface PreviewHeaderProps {
  selectedMatch: SearchMatch | null;
  onShowToast: (msg: string) => void;
}

/**
 * プレビューヘッダーコンポーネント
 *
 * ## 処理詳細
 * OSの既定アプリおよびサポートアプリ一覧の取得、アプリ起動・フォルダ表示のIPC呼び出し、
 * パスのクリップボードコピーとフィードバック制御を行う。
 *
 * ## 引数
 * - `props`: PreviewHeaderProps - コンポーネントプロパティ
 *
 * ## 戻り値
 * - `React.ReactElement`: プレビューヘッダーUI要素
 *
 * ## エラー・例外条件
 * - 各種IPCコマンド実行時の例外はキャッチし、トースト通知としてユーザーに自然な日本語で表示する。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 初版作成。
 * - v1.1.0 (2026-09-26, AI Agent): 憲章原則に準拠し、定数参照と4要素コメントを追加。
 */
export const PreviewHeader: React.FC<PreviewHeaderProps> = ({
  selectedMatch,
  onShowToast,
}) => {
  const [copied, setCopied] = useState(false);
  const [isMenuOpen, setIsMenuOpen] = useState(false);
  const [supportedApps, setSupportedApps] = useState<SupportedApp[]>([]);
  const menuRef = useRef<HTMLDivElement>(null);

  // マウント時にサポートアプリ一覧を取得
  useEffect(() => {
    let isMounted = true;
    const fetchApps = async () => {
      try {
        // 定数参照: COMMANDS.GET_SUPPORTED_APPS
        const apps = await invoke<SupportedApp[]>(COMMANDS.GET_SUPPORTED_APPS);
        if (isMounted) {
          setSupportedApps(apps || []);
        }
      } catch {
        console.error("[PreviewHeader] Failed to load supported applications");
      }
    };
    fetchApps();
    return () => {
      isMounted = false;
    };
  }, []);

  // 外側クリックおよび Esc キーでポップアップメニューを閉じる
  useEffect(() => {
    if (!isMenuOpen) return;

    const handleClickOutside = (event: MouseEvent) => {
      if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
        setIsMenuOpen(false);
      }
    };

    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        setIsMenuOpen(false);
      }
    };

    document.addEventListener("mousedown", handleClickOutside);
    document.addEventListener("keydown", handleKeyDown);
    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
      document.removeEventListener("keydown", handleKeyDown);
    };
  }, [isMenuOpen]);

  if (!selectedMatch) {
    return (
      <div className="p-3 bg-[#1c1c1f] border-b border-zinc-800 text-xs text-zinc-500">
        {/* 定数参照: UI_MESSAGES.PREVIEW_SELECT_ITEM_PROMPT */}
        {UI_MESSAGES.PREVIEW_SELECT_ITEM_PROMPT}
      </div>
    );
  }

  // OS 既定アプリ名を取得（ツールチップ表示用）
  const defaultApp = supportedApps.find((app) => app.is_default);
  const defaultAppTooltip = defaultApp
    ? `${defaultApp.name}${UI_MESSAGES.OPEN_WITH_SPECIFIC_APP_PREFIX}`
    : UI_MESSAGES.OPEN_IN_APP_DEFAULT;

  const renderAppIcon = (iconHint?: string | null, className?: string) => {
    switch (iconHint) {
      case "excel":
        return <FileSpreadsheet className={className || "w-3.5 h-3.5 text-emerald-400 flex-shrink-0"} />;
      case "numbers":
        return <BarChart2 className={className || "w-3.5 h-3.5 text-amber-400 flex-shrink-0"} />;
      case "calc":
        return <Table className={className || "w-3.5 h-3.5 text-blue-400 flex-shrink-0"} />;
      default:
        return <FileSpreadsheet className={className || "w-3.5 h-3.5 text-zinc-400 flex-shrink-0"} />;
    }
  };

  // メインボタン押下: OS 既定アプリで直接起動 (appPath: null)
  const handleLaunchDefaultApp = async () => {
    try {
      // 定数参照: COMMANDS.LAUNCH_ASSOCIATED_APP
      await invoke(COMMANDS.LAUNCH_ASSOCIATED_APP, {
        filePath: selectedMatch.full_path,
        appPath: null,
      });
    } catch {
      // 定数参照: UI_MESSAGES.LAUNCH_APP_FAILED
      onShowToast(UI_MESSAGES.LAUNCH_APP_FAILED);
    }
  };

  // メニュー内の個別アプリ押下: 指定アプリで起動
  const handleLaunchSpecificApp = async (app: SupportedApp) => {
    setIsMenuOpen(false);
    try {
      // 定数参照: COMMANDS.LAUNCH_ASSOCIATED_APP
      await invoke(COMMANDS.LAUNCH_ASSOCIATED_APP, {
        filePath: selectedMatch.full_path,
        appPath: app.executable_path,
      });
    } catch {
      // 定数参照: UI_MESSAGES.LAUNCH_APP_FAILED
      onShowToast(UI_MESSAGES.LAUNCH_APP_FAILED);
    }
  };

  // メニュー末尾の「別のプログラムを選択...」押下
  const handleShowOpenWithDialog = async () => {
    setIsMenuOpen(false);
    try {
      // 定数参照: COMMANDS.SHOW_OPEN_WITH_DIALOG
      await invoke(COMMANDS.SHOW_OPEN_WITH_DIALOG, {
        filePath: selectedMatch.full_path,
      });
    } catch {
      // 定数参照: UI_MESSAGES.SHOW_OPEN_WITH_FAILED
      onShowToast(UI_MESSAGES.SHOW_OPEN_WITH_FAILED);
    }
  };

  const handleOpenFolder = async () => {
    try {
      // 定数参照: COMMANDS.OPEN_IN_FOLDER
      await invoke(COMMANDS.OPEN_IN_FOLDER, { filePath: selectedMatch.full_path });
    } catch {
      // 定数参照: UI_MESSAGES.OPEN_FOLDER_FAILED
      onShowToast(UI_MESSAGES.OPEN_FOLDER_FAILED);
    }
  };

  const handleCopyPath = async () => {
    try {
      await navigator.clipboard.writeText(selectedMatch.full_path);
      setCopied(true);
      // 定数参照: UI_MESSAGES.COPIED_FILE_PATH
      onShowToast(UI_MESSAGES.COPIED_FILE_PATH);
      // 定数参照: LAYOUT_CONSTANTS.COPY_FEEDBACK_DURATION_MS
      setTimeout(() => setCopied(false), LAYOUT_CONSTANTS.COPY_FEEDBACK_DURATION_MS);
    } catch {
      // 定数参照: UI_MESSAGES.COPY_TO_CLIPBOARD_FAILED
      onShowToast(UI_MESSAGES.COPY_TO_CLIPBOARD_FAILED);
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
            title={UI_MESSAGES.WORKSHEET_TITLE}
          >
            <Layers className="w-3 h-3 text-emerald-400" />
            <span className="text-zinc-400 font-normal">{UI_MESSAGES.LABEL_SHEET}</span>
            <span className="font-bold text-emerald-200">
              {selectedMatch.sheet_name}
            </span>
          </div>
        </div>

        {/* アクションボタン群 */}
        <div className="flex items-center gap-2 flex-shrink-0 mr-1">
          {/* スプリットボタン（左: アプリで開く, 右: ▼） */}
          <div className="relative inline-flex items-stretch rounded shadow" ref={menuRef}>
            <button
              onClick={handleLaunchDefaultApp}
              title={defaultAppTooltip}
              className="px-2.5 py-1.5 bg-emerald-600 hover:bg-emerald-500 active:scale-[0.98] text-white text-xs font-medium rounded-l flex items-center gap-1.5 transition flex-shrink-0"
            >
              {renderAppIcon(defaultApp?.icon_hint, "w-3.5 h-3.5 text-white")}
              {/* 定数参照: UI_MESSAGES.OPEN_IN_APP_DEFAULT */}
              <span>{UI_MESSAGES.OPEN_IN_APP_DEFAULT}</span>
            </button>
            <button
              onClick={() => setIsMenuOpen((prev) => !prev)}
              /* 定数参照: UI_MESSAGES.OPEN_WITH_APP_TOOLTIP */
              title={UI_MESSAGES.OPEN_WITH_APP_TOOLTIP}
              aria-expanded={isMenuOpen}
              aria-haspopup="true"
              className="px-1.5 py-1.5 bg-emerald-700 hover:bg-emerald-600 active:scale-[0.98] text-white rounded-r border-l border-emerald-800 transition flex items-center justify-center flex-shrink-0"
            >
              <ChevronDown className="w-3.5 h-3.5" />
            </button>

            {/* ドロップダウンメニュー */}
            {isMenuOpen && (
              <div className="absolute right-0 top-full mt-1 w-64 bg-zinc-900 border border-zinc-700 rounded-md shadow-2xl py-1 z-50 text-xs animate-in fade-in zoom-in-95 duration-100">
                {/* サポートアプリ一覧 */}
                {supportedApps.length > 0 ? (
                  <div className="max-h-60 overflow-y-auto">
                    {supportedApps.map((app) => (
                      <button
                        key={app.id}
                        onClick={() => handleLaunchSpecificApp(app)}
                        className="w-full text-left px-3 py-2 hover:bg-zinc-800 text-zinc-200 flex items-center justify-between gap-2 transition"
                      >
                        <div className="flex items-center gap-2 min-w-0 flex-1">
                          {renderAppIcon(app.icon_hint)}
                          <span className="truncate" title={app.name}>
                            {app.name}
                          </span>
                        </div>
                        {app.is_default && (
                          <span className="text-[10px] bg-emerald-950 text-emerald-400 border border-emerald-700/60 px-1.5 py-0.5 rounded flex-shrink-0 font-medium">
                            {/* 定数参照: UI_MESSAGES.DEFAULT_APP_LABEL */}
                            {UI_MESSAGES.DEFAULT_APP_LABEL}
                          </span>
                        )}
                      </button>
                    ))}
                  </div>
                ) : (
                  <div className="px-3 py-2 text-[11px] text-zinc-500 select-none">
                    {/* 定数参照: UI_MESSAGES.NO_SUPPORTED_APPS */}
                    {UI_MESSAGES.NO_SUPPORTED_APPS}
                  </div>
                )}

                {/* 区切り線 */}
                <div className="border-t border-zinc-800 my-1" />

                {/* 別のプログラムを選択... */}
                <button
                  onClick={handleShowOpenWithDialog}
                  className="w-full text-left px-3 py-2 hover:bg-zinc-800 text-zinc-300 hover:text-white flex items-center gap-2 transition"
                >
                  <ExternalLink className="w-3.5 h-3.5 text-zinc-400 flex-shrink-0" />
                  {/* 定数参照: UI_MESSAGES.OPEN_WITH_OTHER_APP */}
                  <span>{UI_MESSAGES.OPEN_WITH_OTHER_APP}</span>
                </button>
              </div>
            )}
          </div>

          <button
            onClick={handleOpenFolder}
            /* 定数参照: UI_MESSAGES.OPEN_LOCATION_TOOLTIP */
            title={UI_MESSAGES.OPEN_LOCATION_TOOLTIP}
            className="p-1.5 bg-zinc-800 hover:bg-zinc-700 active:scale-[0.98] text-zinc-300 hover:text-white rounded border border-zinc-700 transition flex-shrink-0"
          >
            <Folder className="w-3.5 h-3.5" />
          </button>
        </div>
      </div>

      {/* 2行目: ファイルパス専用行 (独立行 + 右端マージン) */}
      <div className="flex items-center gap-1.5 text-[10px] text-zinc-400 font-mono bg-zinc-900/80 px-2.5 py-1 rounded border border-zinc-800/80 min-w-0 mr-1">
        <Folder className="w-3 h-3 text-zinc-500 flex-shrink-0" />
        {/* 定数参照: UI_MESSAGES.LOCATION_LABEL */}
        <span className="text-zinc-500 select-none flex-shrink-0">{UI_MESSAGES.LOCATION_LABEL}</span>
        <span
          className="text-zinc-400 truncate select-text flex-1"
          title={selectedMatch.full_path}
        >
          {selectedMatch.full_path}
        </span>
        <button
          onClick={handleCopyPath}
          /* 定数参照: UI_MESSAGES.COPY_PATH_TOOLTIP */
          title={UI_MESSAGES.COPY_PATH_TOOLTIP}
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
