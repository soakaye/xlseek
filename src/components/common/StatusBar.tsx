/**
 * Copyright (c) 2026 soakaye
 *
 * @fileoverview Status bar UI component (src/components/common/StatusBar.tsx)
 *
 * ## Description
 * Displays scan progress state, scanned file count, match count, and elapsed time at the bottom of the screen.
 * Renders progress bar, consolidated export split button with format selection menu, and dialog triggers.
 * Complies with Constitution Principle I (English documentation), Principle II (constant reference), and Principle III (comprehensive documentation).
 */

import React, { useState, useEffect, useRef } from "react";
import { FileText, FileSpreadsheet, FileCode, Braces, ChevronDown, Settings, Info } from "lucide-react";
import { save } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import { ScanProgress, SearchMatch, ExportRequest, ExportFormat } from "../../types/search";
import { LlmCopyOptions } from "../../types/llm";
import { COMMANDS, EXPORT_CONSTANTS } from "../../constants";
import { DisplayLanguage } from "../../locale-core";
import { useTranslation, TranslationKey, TranslationValues } from "../../i18n";

interface ExportFormatOption {
  format: ExportFormat;
  labelKey: TranslationKey;
  filterKey: TranslationKey;
  savedKey: TranslationKey;
  icon: React.ComponentType<{ className?: string }>;
  iconColor: string;
}

const EXPORT_FORMAT_OPTIONS: ExportFormatOption[] = [
  {
    format: "xlsx",
    labelKey: EXPORT_CONSTANTS.LABEL_XLSX,
    filterKey: EXPORT_CONSTANTS.FILTER_XLSX,
    savedKey: EXPORT_CONSTANTS.SAVED_XLSX,
    icon: FileSpreadsheet,
    iconColor: "text-emerald-500",
  },
  {
    format: "csv",
    labelKey: EXPORT_CONSTANTS.LABEL_CSV,
    filterKey: EXPORT_CONSTANTS.FILTER_CSV,
    savedKey: EXPORT_CONSTANTS.SAVED_CSV,
    icon: FileText,
    iconColor: "text-zinc-400",
  },
  {
    format: "md",
    labelKey: EXPORT_CONSTANTS.LABEL_MD,
    filterKey: EXPORT_CONSTANTS.FILTER_MD,
    savedKey: EXPORT_CONSTANTS.SAVED_MD,
    icon: FileCode,
    iconColor: "text-sky-400",
  },
  {
    format: "json",
    labelKey: EXPORT_CONSTANTS.LABEL_JSON,
    filterKey: EXPORT_CONSTANTS.FILTER_JSON,
    savedKey: EXPORT_CONSTANTS.SAVED_JSON,
    icon: Braces,
    iconColor: "text-amber-400",
  },
];

interface StatusBarProps {
  progress: ScanProgress | null;
  items: SearchMatch[];
  onShowToast: (key: TranslationKey, values?: TranslationValues) => void;
  onOpenAbout: () => void;
  onOpenSettings: () => void;
  language: DisplayLanguage;
  llmOptions?: LlmCopyOptions;
}

/**
 * ## Description
 * Component displaying the bottom status bar of the application with consolidated export controls.
 *
 * ## Arguments
 * @param props - Progress details, result match list, toast callback, About modal opener, settings opener, and optional LLM context options
 *
 * ## Returns
 * @returns Rendered status bar element with export split button and format menu
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
  llmOptions,
}) => {
  const t = useTranslation();
  const isScanning = progress?.state === "Scanning";
  const isDetermined = !!(progress && progress.total_files > 0);

  const [activeFormat, setActiveFormat] = useState<ExportFormat>(() => {
    try {
      // Constant reference: EXPORT_CONSTANTS.STORAGE_KEY_FORMAT
      const stored = window.localStorage.getItem(EXPORT_CONSTANTS.STORAGE_KEY_FORMAT);
      if (stored === "xlsx" || stored === "csv" || stored === "md" || stored === "json") {
        return stored;
      }
    } catch {
      // Storage access error fallback
    }
    // Constant reference: EXPORT_CONSTANTS.DEFAULT_FORMAT
    return EXPORT_CONSTANTS.DEFAULT_FORMAT as ExportFormat;
  });

  const [isMenuOpen, setIsMenuOpen] = useState(false);
  const menuContainerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!isMenuOpen) return;

    const handleClickOutside = (event: MouseEvent) => {
      if (menuContainerRef.current && !menuContainerRef.current.contains(event.target as Node)) {
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

  const handleExport = async (format: ExportFormat) => {
    if (items.length === 0) {
      // Translation reference: t(EXPORT_CONSTANTS.NO_RESULTS_MSG)
      onShowToast(EXPORT_CONSTANTS.NO_RESULTS_MSG);
      return;
    }

    const option = EXPORT_FORMAT_OPTIONS.find((opt) => opt.format === format) ?? EXPORT_FORMAT_OPTIONS[0];

    try {
      // Translation reference: t(EXPORT_CONSTANTS.DEFAULT_FILENAME_PREFIX)
      const defaultFilename = `${t(EXPORT_CONSTANTS.DEFAULT_FILENAME_PREFIX)}${new Date()
        .toISOString()
        .slice(0, 10)}.${format}`;

      const selectedPath = await save({
        filters: [
          {
            name: t(option.filterKey),
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
        llm_options: llmOptions,
      };

      // Constant reference: COMMANDS.EXPORT_RESULTS
      await invoke(COMMANDS.EXPORT_RESULTS, { request: req });
      onShowToast(option.savedKey, { path: selectedPath });
    } catch {
      console.error("[StatusBar] Export failed");
      // Translation reference: t(EXPORT_CONSTANTS.EXPORT_FAILED)
      onShowToast(EXPORT_CONSTANTS.EXPORT_FAILED);
    }
  };

  const handleSelectFormat = (format: ExportFormat) => {
    setIsMenuOpen(false);
    setActiveFormat(format);
    try {
      // Constant reference: EXPORT_CONSTANTS.STORAGE_KEY_FORMAT
      window.localStorage.setItem(EXPORT_CONSTANTS.STORAGE_KEY_FORMAT, format);
    } catch {
      // Ignore localStorage write error
    }
    void handleExport(format);
  };

  const activeOption =
    EXPORT_FORMAT_OPTIONS.find((opt) => opt.format === activeFormat) ?? EXPORT_FORMAT_OPTIONS[0];
  const ActiveIcon = activeOption.icon;

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

      {/* Right controls: Export Split Button and dialog triggers */}
      <div className="flex items-center gap-2 flex-shrink-0">
        <button
          type="button"
          onClick={onOpenSettings}
          title={t("ui.SETTINGS")}
          aria-label={t("ui.SETTINGS")}
          className="p-1 hover:bg-zinc-700 text-zinc-400 hover:text-zinc-200 rounded border border-zinc-700/60 transition flex items-center justify-center cursor-pointer"
        >
          <Settings className="w-3.5 h-3.5" />
        </button>

        {/* Consolidated Export Split Button */}
        <div ref={menuContainerRef} className="relative flex items-center">
          <button
            type="button"
            onClick={() => handleExport(activeFormat)}
            disabled={items.length === 0}
            title={t(EXPORT_CONSTANTS.BUTTON_TOOLTIP)}
            aria-label={t(EXPORT_CONSTANTS.BUTTON_TOOLTIP)}
            className="px-2.5 py-1 bg-zinc-800 hover:bg-zinc-700 disabled:opacity-40 disabled:cursor-not-allowed text-zinc-200 hover:text-white rounded-l border border-zinc-700 flex items-center gap-1.5 transition text-xs font-medium cursor-pointer"
          >
            <ActiveIcon className={`w-3.5 h-3.5 ${activeOption.iconColor}`} />
            <span>{t(activeOption.labelKey)}</span>
          </button>
          <button
            type="button"
            onClick={() => setIsMenuOpen((prev) => !prev)}
            disabled={items.length === 0}
            title={t(EXPORT_CONSTANTS.MENU_TOOLTIP)}
            aria-label={t(EXPORT_CONSTANTS.MENU_TOOLTIP)}
            aria-haspopup="menu"
            aria-expanded={isMenuOpen}
            className="px-1.5 py-1 bg-zinc-800 hover:bg-zinc-700 disabled:opacity-40 disabled:cursor-not-allowed text-zinc-400 hover:text-zinc-200 rounded-r border-y border-r border-zinc-700 flex items-center justify-center transition cursor-pointer border-l-0"
          >
            <ChevronDown className={`w-3 h-3 transition-transform ${isMenuOpen ? "rotate-180" : ""}`} />
          </button>

          {/* Upward format selection menu */}
          {isMenuOpen && (
            <div
              role="menu"
              className="absolute bottom-full right-0 mb-1.5 w-44 bg-[#1f1f23] border border-zinc-700/80 rounded-md shadow-xl py-1 z-50 text-xs overflow-hidden"
            >
              {EXPORT_FORMAT_OPTIONS.map((opt) => {
                const Icon = opt.icon;
                const isSelected = opt.format === activeFormat;
                return (
                  <button
                    key={opt.format}
                    type="button"
                    role="menuitem"
                    onClick={() => handleSelectFormat(opt.format)}
                    className={`w-full px-3 py-1.5 text-left flex items-center gap-2 hover:bg-zinc-700/70 transition cursor-pointer ${
                      isSelected ? "bg-zinc-800 text-white font-medium" : "text-zinc-300"
                    }`}
                  >
                    <Icon className={`w-3.5 h-3.5 ${opt.iconColor}`} />
                    <span className="flex-1">{t(opt.labelKey)}</span>
                    {isSelected && <span className="w-1.5 h-1.5 rounded-full bg-emerald-400" />}
                  </button>
                );
              })}
            </div>
          )}
        </div>

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
