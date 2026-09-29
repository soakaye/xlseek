/**
 * @fileoverview Status bar UI component (src/components/common/StatusBar.tsx)
 *
 * ## Description
 * Displays scan progress state, scanned file count, match count, and elapsed time at the bottom of the screen.
 * Renders progress bar and triggers CSV/Excel export buttons for search results.
 * Complies with Constitution Principle I (English documentation), Principle II (constant reference), and Principle III (comprehensive documentation).
 */

import React from "react";
import { FileText, FileSpreadsheet, Info, Settings } from "lucide-react";
import { save } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import { ScanProgress, SearchMatch, ExportRequest } from "../../types/search";
import { COMMANDS } from "../../constants";
import { DisplayLanguage } from "../../locale-core";
import { useTranslation } from "../../i18n";

interface StatusBarProps {
  progress: ScanProgress | null;
  items: SearchMatch[];
  onShowToast: (key: import("../../i18n").TranslationKey, values?: import("../../i18n").TranslationValues) => void;
  onOpenAbout: () => void;
  onOpenSettings: () => void;
  language: DisplayLanguage;
}

/**
 * ## Description
 * Component displaying the bottom status bar of the application.
 *
 * ## Arguments
 * @param props - Progress details, result match list, toast callback, About modal opener, settings opener
 *
 * ## Returns
 * @returns Rendered status bar element
 *
 * ## Errors / Exceptions
 * Export failures trigger toast notifications; no uncaught exceptions are thrown.
 */
export const StatusBar: React.FC<StatusBarProps> = ({
  progress,
  items,
  onShowToast,
  onOpenAbout,
  onOpenSettings,
  language,
}) => {
  const t = useTranslation();
  const isScanning = progress?.state === "Scanning";
  const isDetermined = !!(progress && progress.total_files > 0);

  const percent =
    isDetermined && progress
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
      // Translation reference: t("ui.EXPORT_NO_RESULTS_MSG")
      onShowToast("ui.EXPORT_NO_RESULTS_MSG");
      return;
    }

    try {
      // Translation reference: t("ui.EXPORT_DEFAULT_FILENAME_PREFIX")
      const defaultFilename = `${t("ui.EXPORT_DEFAULT_FILENAME_PREFIX")}${new Date()
        .toISOString()
        .slice(0, 10)}.${format}`;

      const selectedPath = await save({
        filters: [
          {
            // Translation reference: t("ui.EXPORT_CSV_FILTER_NAME") / t("ui.EXPORT_XLSX_FILTER_NAME")
            name:
              format === "csv"
                ? t("ui.EXPORT_CSV_FILTER_NAME")
                : t("ui.EXPORT_XLSX_FILTER_NAME"),
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
        language,
      };

      // Constant reference: COMMANDS.EXPORT_RESULTS
      await invoke(COMMANDS.EXPORT_RESULTS, { request: req });
      // Translation reference: formats path interpolation
      onShowToast(format === "csv" ? "ui.EXPORT_CSV_SAVED" : "ui.EXPORT_XLSX_SAVED", { path: selectedPath });
    } catch {
      console.error("[StatusBar] Export failed");
      // Translation reference: t("ui.EXPORT_FAILED")
      onShowToast("ui.EXPORT_FAILED");
    }
  };

  const statusText = (() => {
    if (!progress) {
      // Translation reference: t("ui.STATUS_IDLE")
      return t("ui.STATUS_IDLE");
    }
    const elapsedSec = (progress.elapsed_ms / 1000).toFixed(2);
    const fileUnit = t("ui.UNIT_FILES");
    const matchUnit = t("ui.UNIT_MATCHES");
    const elapsed = `${elapsedSec}${t("ui.UNIT_SECONDS_SUFFIX")}`;
    switch (progress.state) {
      case "Scanning":
        if (!isDetermined) {
          if (progress.phase === "preparing") return t("ui.STATUS_SCAN_PREPARING");
          if (progress.scanned_files === 0) {
            // Translation reference: t("ui.FOLDER_SCANNING_PREFIX") / FOLDER_SEARCHING_DEFAULT
            return `${t("ui.FOLDER_SCANNING_PREFIX")}${progress.current_file || t("ui.FOLDER_SEARCHING_DEFAULT")}`;
          }
          // Translation reference: t("ui.STATUS_DISCOVERING_DETAIL")
          return `${t("ui.STATUS_DISCOVERING_DETAIL")}: ${progress.scanned_files} ${fileUnit} (${progress.matches_found} ${matchUnit}, ${elapsed}) - ${progress.current_file}`;
        }
        // Translation reference: t("ui.STATUS_SCANNING_DETAIL")
        return `${t("ui.STATUS_SCANNING_DETAIL")}: ${progress.scanned_files}/${progress.total_files} ${fileUnit} (${progress.matches_found} ${matchUnit}, ${elapsed}) - ${progress.current_file}`;
      case "Completed":
        // Translation reference: t("ui.STATUS_COMPLETED_PREFIX")
        return `${t("ui.STATUS_COMPLETED_PREFIX")}${progress.matches_found} ${matchUnit} (${progress.scanned_files} ${fileUnit}, ${elapsed})`;
      case "Cancelled":
        return `${t("ui.STATUS_CANCELLED_DETAIL")}: ${progress.matches_found} ${matchUnit} (${progress.scanned_files} ${fileUnit})`;
      case "Error":
        return t("ui.STATUS_ERROR_DETAIL");
    }
  })();

  return (
    <footer className="h-10 bg-[#18181b] border-t border-zinc-800 px-4 flex items-center justify-between text-xs text-zinc-400 flex-shrink-0 select-none">
      {/* Search metrics & progress */}
      <div className="flex items-center gap-3 min-w-0 flex-1">
        {/* Status indicator light */}
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

        {/* Progress bar (fixed width w-44 placed before message to avoid layout shifts) */}
        <div className="flex items-center gap-2 flex-shrink-0 w-44">
          <div className="w-24 bg-zinc-800 h-1.5 rounded-full overflow-hidden border border-zinc-700/50 flex-shrink-0">
            <div
              className={`h-full rounded-full transition-all duration-300 ${
                isScanning && !isDetermined
                  ? "bg-amber-500 animate-pulse w-1/4"
                  : "bg-excel"
              }`}
              style={{
                width:
                  isScanning && !isDetermined
                    ? undefined
                    : `${percent}%`,
              }}
            />
          </div>
          <span className="text-[11px] text-zinc-400 font-mono min-w-0 flex-1 truncate">
            {isScanning && !isDetermined
              ? progress.scanned_files === 0
                ? t("ui.SEARCHING_DIR")
                : progress.scanned_files
              : progress
              ? `${percent}% (${progress.scanned_files}/${progress.total_files})`
              : t("ui.STATUS_WAITING")}
          </span>
        </div>

        {/* Vertical divider */}
        <div className="h-3.5 w-px bg-zinc-800 flex-shrink-0" />

        {/* Status text (variable length, truncated) */}
        <span
          className="text-zinc-200 font-medium truncate flex-1"
          title={statusText}
        >
          {statusText}
        </span>
      </div>

      {/* Right controls: Export and dialog triggers */}
      <div className="flex items-center gap-2 flex-shrink-0">
        <button type="button" onClick={onOpenSettings} title={t("ui.SETTINGS")} aria-label={t("ui.SETTINGS")} className="p-1 hover:bg-zinc-700 text-zinc-400 hover:text-zinc-200 rounded border border-zinc-700/60 transition flex items-center justify-center cursor-pointer">
          <Settings className="w-3.5 h-3.5" />
        </button>
        <button
          type="button"
          onClick={() => handleExport("csv")}
          disabled={items.length === 0}
          className="px-2.5 py-1 bg-zinc-800 hover:bg-zinc-700 disabled:opacity-40 disabled:cursor-not-allowed text-zinc-200 hover:text-white rounded border border-zinc-700 flex items-center gap-1.5 transition"
        >
          <FileText className="w-3.5 h-3.5 text-zinc-400" />
          <span>{t("ui.EXPORT_CSV_BUTTON")}</span>
        </button>
        <button
          type="button"
          onClick={() => handleExport("xlsx")}
          disabled={items.length === 0}
          className="px-2.5 py-1 bg-zinc-800 hover:bg-zinc-700 disabled:opacity-40 disabled:cursor-not-allowed text-emerald-400 hover:text-emerald-300 rounded border border-zinc-700 flex items-center gap-1.5 transition"
        >
          <FileSpreadsheet className="w-3.5 h-3.5 text-emerald-500" />
          <span>{t("ui.EXPORT_XLSX_BUTTON")}</span>
        </button>

        {/* Vertical divider */}
        <div className="h-3.5 w-px bg-zinc-800 flex-shrink-0" />

        {/* About dialog trigger button */}
        <button
          type="button"
          onClick={onOpenAbout}
          /* Translation reference: t("about.BUTTON_ABOUT_TOOLTIP") */
          title={t("about.BUTTON_ABOUT_TOOLTIP")}
          className="p-1 hover:bg-zinc-700 text-zinc-400 hover:text-zinc-200 rounded border border-zinc-700/60 transition flex items-center justify-center cursor-pointer"
        >
          <Info className="w-3.5 h-3.5" />
        </button>
      </div>
    </footer>
  );
};
