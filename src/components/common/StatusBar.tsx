/**
 * @fileoverview ステータスバーUIコンポーネント (src/components/common/StatusBar.tsx)
 *
 * ## 処理内容
 * 画面下部にスキャン進捗状態、走査ファイル数、ヒット件数、経過時間を表示する。
 * プログレスバー描画および検索結果のCSV/Excelエクスポート実行ボタンを提供する。
 * 憲章原則I（日本語表示）、原則II（定数参照）、原則III（ヘッダコメント）に準拠。
 *
 * ## 変更履歴
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化（COMMANDS.EXPORT_RESULTS）およびJSDoc付与。
 * - v1.1.0 (2026-09-26, AI Agent): デザイン改善フィードバック対応。プログレスバーをメッセージ前（固定幅 w-44）へ配置変更し、ファイル読み込み前のフォルダスキャン中表示（対象フォルダ名表示）を導入。
 * - v1.2.0 (2026-09-26, AI Agent): デザインフィードバック対応。フッター右側のCalamine Engineバッジ表示を削除。
 * - v1.3.0 (2026-09-26, AI Agent): 最右端にAboutダイアログ起動ボタン（Infoアイコン）を追加。
 */

import React from "react";
import { FileText, FileSpreadsheet, Info } from "lucide-react";
import { save } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import { ScanProgress, SearchMatch, ExportRequest } from "../../types/search";
import { COMMANDS, UI_MESSAGES, ABOUT_DIALOG_CONSTANTS } from "../../constants";

interface StatusBarProps {
  progress: ScanProgress | null;
  items: SearchMatch[];
  onShowToast: (msg: string) => void;
  onOpenAbout: () => void;
}

/**
 * ## 処理内容
 * アプリケーションの最下部ステータスバーを表示するコンポーネント。
 *
 * ## 引数
 * @param props - 進捗情報、検索結果一覧、トースト通知コールバック、Aboutダイアログ表示ハンドラ
 *
 * ## 戻り値
 * @returns レンダリング要素
 *
 * ## エラー / 例外発生条件
 * エクスポート失敗時はトースト通知で日本語エラーを表示する。例外は外部へスローしない。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化。
 * - v1.3.0 (2026-09-26, AI Agent): onOpenAbout プロパティの追加。
 */
export const StatusBar: React.FC<StatusBarProps> = ({
  progress,
  items,
  onShowToast,
  onOpenAbout,
}) => {
  const percent =
    progress && progress.total_files > 0
      ? progress.state === "Completed"
        ? 100
        : Math.min(
            99,
            Math.max(
              1,
              Math.round((progress.scanned_files / progress.total_files) * 100)
            )
          )
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

      // 定数参照: COMMANDS.EXPORT_RESULTS を使用
      await invoke(COMMANDS.EXPORT_RESULTS, { request: req });
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
      // 定数参照: UI_MESSAGES.STATUS_IDLE
      return UI_MESSAGES.STATUS_IDLE;
    }
    const elapsedSec = (progress.elapsed_ms / 1000).toFixed(2);
    switch (progress.state) {
      case "Scanning":
        if (progress.scanned_files === 0) {
          // 定数参照: UI_MESSAGES.FOLDER_SCANNING_PREFIX / FOLDER_SEARCHING_DEFAULT
          return `${UI_MESSAGES.FOLDER_SCANNING_PREFIX}${progress.current_file || UI_MESSAGES.FOLDER_SEARCHING_DEFAULT}`;
        }
        return `スキャン中: ${progress.scanned_files}/${progress.total_files} ファイル (${progress.matches_found} 件一致, ${elapsedSec}s) - ${progress.current_file}`;
      case "Completed":
        return `完了: ${progress.matches_found} 件一致 (${progress.scanned_files} ファイル, ${elapsedSec}s)`;
      case "Cancelled":
        return `${UI_MESSAGES.CANCELLED}: ${progress.matches_found} 件一致 (${progress.scanned_files} ファイル走査済)`;
      case "Error":
        return `${UI_MESSAGES.ERROR}が発生しました`;
    }
  })();

  const isScanning = progress?.state === "Scanning";

  return (
    <footer className="h-10 bg-[#18181b] border-t border-zinc-800 px-4 flex items-center justify-between text-xs text-zinc-400 flex-shrink-0 select-none">
      {/* 検索メトリクス & 進捗 */}
      <div className="flex items-center gap-3 min-w-0 flex-1">
        {/* 状態インジケーター */}
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

        {/* プログレスバー (メッセージの前に固定幅 w-44 で配置: 可変テキストによる位置ズレを防止) */}
        <div className="flex items-center gap-2 flex-shrink-0 w-44">
          <div className="w-24 bg-zinc-800 h-1.5 rounded-full overflow-hidden border border-zinc-700/50 flex-shrink-0">
            <div
              className={`h-full rounded-full transition-all duration-300 ${
                isScanning && progress?.scanned_files === 0
                  ? "bg-amber-500 animate-pulse w-1/4"
                  : "bg-excel"
              }`}
              style={{
                width:
                  isScanning && progress?.scanned_files === 0
                    ? undefined
                    : `${percent}%`,
              }}
            />
          </div>
          <span className="text-[11px] text-zinc-400 font-mono flex-shrink-0">
            {isScanning && progress?.scanned_files === 0
              ? UI_MESSAGES.SEARCHING_DIR
              : progress
              ? `${percent}% (${progress.scanned_files}/${progress.total_files})`
              : UI_MESSAGES.STATUS_WAITING}
          </span>
        </div>

        {/* 縦仕切り線 */}
        <div className="h-3.5 w-px bg-zinc-800 flex-shrink-0" />

        {/* ステータステキスト (可変長・右側へ伸長・truncate) */}
        <span
          className="text-zinc-200 font-medium truncate flex-1"
          title={statusText}
        >
          {statusText}
        </span>
      </div>

      {/* 右側: エクスポートボタン群 */}
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

        {/* 縦仕切り線 */}
        <div className="h-3.5 w-px bg-zinc-800 flex-shrink-0" />

        {/* Aboutダイアログ起動ボタン */}
        <button
          type="button"
          onClick={onOpenAbout}
          /* 定数参照: ABOUT_DIALOG_CONSTANTS.BUTTON_ABOUT_TOOLTIP */
          title={ABOUT_DIALOG_CONSTANTS.BUTTON_ABOUT_TOOLTIP}
          className="p-1 hover:bg-zinc-700 text-zinc-400 hover:text-zinc-200 rounded border border-zinc-700/60 transition flex items-center justify-center cursor-pointer"
        >
          <Info className="w-3.5 h-3.5" />
        </button>
      </div>
    </footer>
  );
};
