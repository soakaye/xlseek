/**
 * @fileoverview Preview pane header and action bar component (src/components/preview/PreviewHeader.tsx)
 *
 * ## Description
 * Displays selected filename, worksheet badge, and file path. Provides external application launch options
 * (Excel, Numbers, Calc, etc.), Open With dialog trigger, folder explorer navigation, and clipboard copy.
 * Complies with Constitution Principle I (English documentation), Principle II (constant reference), and Principle III (comprehensive documentation).
 */

import React, { useState, useEffect, useRef } from "react";
import { FileSpreadsheet, Layers, Folder, Copy, Check, ChevronDown, ExternalLink, BarChart2, Table } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import { SearchMatch, SupportedApp } from "../../types/search";
import { COMMANDS, LAYOUT_CONSTANTS } from "../../constants";
import { useTranslation } from "../../i18n";

interface PreviewHeaderProps {
  selectedMatch: SearchMatch | null;
  onShowToast: (key: import("../../i18n").TranslationKey) => void;
}

/**
 * ## Description
 * Preview header action bar component.
 *
 * ## Arguments
 * @param props - PreviewHeaderProps
 *
 * ## Returns
 * @returns Rendered element
 *
 * ## Errors / Exceptions
 * IPC command exceptions are caught and surfaced via toast notifications without crashing the UI.
 */
export const PreviewHeader: React.FC<PreviewHeaderProps> = ({
  selectedMatch,
  onShowToast,
}) => {
  const t = useTranslation();
  const [copied, setCopied] = useState(false);
  const [isMenuOpen, setIsMenuOpen] = useState(false);
  const [supportedApps, setSupportedApps] = useState<SupportedApp[]>([]);
  const menuRef = useRef<HTMLDivElement>(null);

  // Retrieve supported applications on mount
  useEffect(() => {
    let isMounted = true;
    const fetchApps = async () => {
      try {
        // Constant reference: COMMANDS.GET_SUPPORTED_APPS
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

  // Dismiss popup menu on outside click or Escape key
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
        {/* Constant reference: t("ui.PREVIEW_SELECT_ITEM_PROMPT") */}
        {t("ui.PREVIEW_SELECT_ITEM_PROMPT")}
      </div>
    );
  }

  // OS default application name for tooltip
  const defaultApp = supportedApps.find((app) => app.is_default);
  const defaultAppTooltip = defaultApp
    ? `${defaultApp.name}${t("ui.OPEN_WITH_SPECIFIC_APP_PREFIX")}`
    : t("ui.OPEN_IN_APP_DEFAULT");

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

  // Launch with OS default application (appPath: null)
  const handleLaunchDefaultApp = async () => {
    try {
      // Constant reference: COMMANDS.LAUNCH_ASSOCIATED_APP
      await invoke(COMMANDS.LAUNCH_ASSOCIATED_APP, {
        filePath: selectedMatch.full_path,
        appPath: null,
      });
    } catch {
      // Constant reference: t("ui.LAUNCH_APP_FAILED")
      onShowToast("ui.LAUNCH_APP_FAILED");
    }
  };

  // Launch with specific selected application
  const handleLaunchSpecificApp = async (app: SupportedApp) => {
    setIsMenuOpen(false);
    try {
      // Constant reference: COMMANDS.LAUNCH_ASSOCIATED_APP
      await invoke(COMMANDS.LAUNCH_ASSOCIATED_APP, {
        filePath: selectedMatch.full_path,
        appPath: app.executable_path,
      });
    } catch {
      // Constant reference: t("ui.LAUNCH_APP_FAILED")
      onShowToast("ui.LAUNCH_APP_FAILED");
    }
  };

  // Open OS "Open With" dialog
  const handleShowOpenWithDialog = async () => {
    setIsMenuOpen(false);
    try {
      // Constant reference: COMMANDS.SHOW_OPEN_WITH_DIALOG
      await invoke(COMMANDS.SHOW_OPEN_WITH_DIALOG, {
        filePath: selectedMatch.full_path,
      });
    } catch {
      // Constant reference: t("ui.SHOW_OPEN_WITH_FAILED")
      onShowToast("ui.SHOW_OPEN_WITH_FAILED");
    }
  };

  const handleOpenFolder = async () => {
    try {
      // Constant reference: COMMANDS.OPEN_IN_FOLDER
      await invoke(COMMANDS.OPEN_IN_FOLDER, { filePath: selectedMatch.full_path });
    } catch {
      // Constant reference: t("ui.OPEN_FOLDER_FAILED")
      onShowToast("ui.OPEN_FOLDER_FAILED");
    }
  };

  const handleCopyPath = async () => {
    try {
      await navigator.clipboard.writeText(selectedMatch.full_path);
      setCopied(true);
      // Constant reference: t("ui.COPIED_FILE_PATH")
      onShowToast("ui.COPIED_FILE_PATH");
      // Constant reference: LAYOUT_CONSTANTS.COPY_FEEDBACK_DURATION_MS
      setTimeout(() => setCopied(false), LAYOUT_CONSTANTS.COPY_FEEDBACK_DURATION_MS);
    } catch {
      // Constant reference: t("ui.COPY_TO_CLIPBOARD_FAILED")
      onShowToast("ui.COPY_TO_CLIPBOARD_FAILED");
    }
  };

  return (
    <div className="p-3 pr-4 bg-[#1c1c1f] border-b border-zinc-800 flex flex-col gap-2 flex-shrink-0">
      {/* Row 1: File name & sheet badge with action button group */}
      <div className="flex items-center justify-between gap-3">
        <div className="flex items-center gap-2 min-w-0 flex-1">
          <FileSpreadsheet className="w-4 h-4 text-emerald-400 flex-shrink-0" />
          <span
            className="text-xs font-semibold text-zinc-100 truncate"
            title={selectedMatch.file_name}
          >
            {selectedMatch.file_name}
          </span>

          {/* Worksheet badge */}
          <div
            className="flex items-center gap-1.5 bg-emerald-950/80 text-emerald-300 border border-emerald-700/70 px-2 py-0.5 rounded text-[11px] font-medium flex-shrink-0 shadow-sm"
            title={t("ui.WORKSHEET_TITLE")}
          >
            <Layers className="w-3 h-3 text-emerald-400" />
            <span className="text-zinc-400 font-normal">{t("ui.LABEL_SHEET")}</span>
            <span className="font-bold text-emerald-200">
              {selectedMatch.sheet_name}
            </span>
          </div>
        </div>

        {/* Action button controls */}
        <div className="flex items-center gap-2 flex-shrink-0 mr-1">
          {/* Split button (left: launch default app, right: arrow menu) */}
          <div className="relative inline-flex items-stretch rounded shadow" ref={menuRef}>
            <button
              onClick={handleLaunchDefaultApp}
              title={defaultAppTooltip}
              className="px-2.5 py-1.5 bg-emerald-600 hover:bg-emerald-500 active:scale-[0.98] text-white text-xs font-medium rounded-l flex items-center gap-1.5 transition flex-shrink-0"
            >
              {renderAppIcon(defaultApp?.icon_hint, "w-3.5 h-3.5 text-white")}
              {/* Constant reference: t("ui.OPEN_IN_APP_DEFAULT") */}
              <span>{t("ui.OPEN_IN_APP_DEFAULT")}</span>
            </button>
            <button
              onClick={() => setIsMenuOpen((prev) => !prev)}
              /* Constant reference: t("ui.OPEN_WITH_APP_TOOLTIP") */
              title={t("ui.OPEN_WITH_APP_TOOLTIP")}
              aria-expanded={isMenuOpen}
              aria-haspopup="true"
              className="px-1.5 py-1.5 bg-emerald-700 hover:bg-emerald-600 active:scale-[0.98] text-white rounded-r border-l border-emerald-800 transition flex items-center justify-center flex-shrink-0"
            >
              <ChevronDown className="w-3.5 h-3.5" />
            </button>

            {/* Dropdown menu */}
            {isMenuOpen && (
              <div className="absolute right-0 top-full mt-1 w-64 bg-zinc-900 border border-zinc-700 rounded-md shadow-2xl py-1 z-50 text-xs animate-in fade-in zoom-in-95 duration-100">
                {/* Supported applications list */}
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
                            {/* Constant reference: t("ui.DEFAULT_APP_LABEL") */}
                            {t("ui.DEFAULT_APP_LABEL")}
                          </span>
                        )}
                      </button>
                    ))}
                  </div>
                ) : (
                  <div className="px-3 py-2 text-[11px] text-zinc-500 select-none">
                    {/* Constant reference: t("ui.NO_SUPPORTED_APPS") */}
                    {t("ui.NO_SUPPORTED_APPS")}
                  </div>
                )}

                {/* Divider */}
                <div className="border-t border-zinc-800 my-1" />

                {/* Open With Other Program... */}
                <button
                  onClick={handleShowOpenWithDialog}
                  className="w-full text-left px-3 py-2 hover:bg-zinc-800 text-zinc-300 hover:text-white flex items-center gap-2 transition"
                >
                  <ExternalLink className="w-3.5 h-3.5 text-zinc-400 flex-shrink-0" />
                  {/* Constant reference: t("ui.OPEN_WITH_OTHER_APP") */}
                  <span>{t("ui.OPEN_WITH_OTHER_APP")}</span>
                </button>
              </div>
            )}
          </div>

          <button
            onClick={handleOpenFolder}
            /* Constant reference: t("ui.OPEN_LOCATION_TOOLTIP") */
            title={t("ui.OPEN_LOCATION_TOOLTIP")}
            className="p-1.5 bg-zinc-800 hover:bg-zinc-700 active:scale-[0.98] text-zinc-300 hover:text-white rounded border border-zinc-700 transition flex-shrink-0"
          >
            <Folder className="w-3.5 h-3.5" />
          </button>
        </div>
      </div>

      {/* Row 2: File path dedicated line */}
      <div className="flex items-center gap-1.5 text-[10px] text-zinc-400 font-mono bg-zinc-900/80 px-2.5 py-1 rounded border border-zinc-800/80 min-w-0 mr-1">
        <Folder className="w-3 h-3 text-zinc-500 flex-shrink-0" />
        {/* Constant reference: t("ui.LOCATION_LABEL") */}
        <span className="text-zinc-500 select-none flex-shrink-0">{t("ui.LOCATION_LABEL")}</span>
        <span
          className="text-zinc-400 truncate select-text flex-1"
          title={selectedMatch.full_path}
        >
          {selectedMatch.full_path}
        </span>
        <button
          onClick={handleCopyPath}
          /* Constant reference: t("ui.COPY_PATH_TOOLTIP") */
          title={t("ui.COPY_PATH_TOOLTIP")}
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
