/**
 * Copyright (c) 2026 soakaye
 *
 * @fileoverview Application root component (src/App.tsx)
 *
 * ## Description
 * Top-level component for Excel Grep. Manages search state, results, preview data,
 * and toast notifications via `useSearch`, integrating the title bar, search bar,
 * results table, preview pane, and status bar.
 * Complies with Constitution Principle I (English documentation), Principle III (comprehensive documentation), and Principle IV (modular design).
 */

import React, { Suspense, lazy, useState, useEffect } from "react";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { WindowFrame } from "./components/layout/WindowFrame";
import { SearchBar } from "./components/search/SearchBar";
import { ResultTable } from "./components/results/ResultTable";
import { PreviewHeader } from "./components/preview/PreviewHeader";
import { FormulaBar } from "./components/preview/FormulaBar";
import { SpreadsheetGrid } from "./components/preview/SpreadsheetGrid";
import { SheetTabs } from "./components/preview/SheetTabs";
import { MetaInfoCard } from "./components/preview/MetaInfoCard";
import { StatusBar } from "./components/common/StatusBar";
import { SearchWarnings } from "./components/common/SearchWarnings";
import { Toast } from "./components/common/Toast";
import { SettingsDialog } from "./components/settings/SettingsDialog";
import { useSearch } from "./hooks/useSearch";
import { useSearchHistory } from "./hooks/useSearchHistory";
import { APP_LOGS, COMMANDS, EVENT_NAMES } from "./constants";
import { useLocale } from "./hooks/useLocale";
import { LocaleProvider, t, TranslationKey, TranslationValues } from "./i18n";

const AboutDialog = lazy(() => import("./components/about/AboutDialog").then((module) => ({ default: module.AboutDialog })));

/**
 * ## Description
 * Excel Grep application main screen root component.
 *
 * ## Arguments
 * None (root component).
 *
 * ## Returns
 * @returns Application full UI tree
 *
 * ## Errors / Exceptions
 * Async operation errors are surfaced via toast notices without crashing the UI.
 */
export const App: React.FC = () => {
  const [toastNotice, setToastNotice] = useState<{ key: TranslationKey; values?: TranslationValues } | null>(null);
  const [isAboutOpen, setIsAboutOpen] = useState(false);
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);
  const { language, preference, selectLanguage, ready } = useLocale();
  const toastMessage = toastNotice
    ? t(language, toastNotice.key, toastNotice.values)
    : null;

  useEffect(() => {
    if (!ready) return;
    void invoke(COMMANDS.SET_MENU_LOCALE, { language }).catch(() => {
      console.error(APP_LOGS.MENU_UPDATE_FAILED);
      setToastNotice({ key: "ui.MENU_UPDATE_FAILED" });
    });
  }, [language, ready]);

  // Subscribe to About dialog open event from macOS system menu
  useEffect(() => {
    let unlisten: UnlistenFn | undefined;
    let isCancelled = false;

    const setupListener = async () => {
      try {
        // Constant reference: EVENT_NAMES.OPEN_ABOUT_DIALOG
        const u = await listen(EVENT_NAMES.OPEN_ABOUT_DIALOG, () => {
          if (isCancelled) return;
          setIsAboutOpen(true);
        });

        if (isCancelled) {
          u();
        } else {
          unlisten = u;
        }
      } catch {
        console.error(APP_LOGS.ABOUT_LISTENER_FAILED);
      }
    };

    setupListener();

    return () => {
      isCancelled = true;
      if (unlisten) {
        unlisten();
      }
    };
  }, []);

  const showToast = (key: TranslationKey, values?: TranslationValues) => {
    setToastNotice({ key, values });
  };

  const { maxEntries, keywords, directories, defaultOptions, rememberedBurstWorkers, addSearch, savePreferences } = useSearchHistory(() => showToast("ui.SAVE_FAILED"));

  const {
    query,
    updateQuery,
    applyDefaultOptions,
    results,
    progress,
    searchIssues,
    isScanning,
    selectedMatch,
    previewData,
    loadingPreview,
    activeSheet,
    selectedCell,
    formulaOrValue,
    handleSelectItem,
    handleSelectSheet,
    handleSelectCell,
    startSearch,
    cancelSearch,
  } = useSearch({ onShowToast: showToast, onSearchAccepted: (acceptedQuery) => addSearch(acceptedQuery.keyword, acceptedQuery.target_dir) });

  const handleSavePreferences = (settings: { maxEntries: number; defaultOptions: typeof defaultOptions; rememberedBurstWorkers: number | null }): boolean => {
    const saved = savePreferences(settings.maxEntries, settings.defaultOptions, settings.rememberedBurstWorkers);
    if (!saved) {
      showToast("ui.SAVE_FAILED");
      return false;
    }
    applyDefaultOptions(settings.defaultOptions);
    showToast("ui.DEFAULT_OPTIONS_SAVED");
    return true;
  };

  if (!ready) return null;

  return (
    <LocaleProvider value={language}>
    <WindowFrame>
      {/* Search inputs & options control panel */}
      <SearchBar
        query={query}
        onChangeQuery={updateQuery}
        onSearch={startSearch}
        onCancel={cancelSearch}
        isScanning={isScanning}
        history={{ keywords, directories }}
        onSelectHistory={(field, value) => updateQuery(field === "keyword" ? { keyword: value } : { target_dir: value })}
      />

      {/* Constant reference: EVENT_NAMES.SEARCH_ISSUE is subscribed by useSearch. */}
      <SearchWarnings issues={searchIssues} language={language} />

      {/* Main two-pane workspace: left 58%, right 42% */}
      <div className="flex-1 flex overflow-hidden">
        {/* Left pane: Search results table */}
        <ResultTable
          items={results}
          selectedId={selectedMatch?.id ?? null}
          onSelectItem={handleSelectItem}
        />

        {/* Right pane: Cell and spreadsheet preview */}
        <div className="flex-1 flex flex-col bg-[#161618] min-w-0 min-h-0">
          {/* Preview top header & action bar */}
          <PreviewHeader
            selectedMatch={selectedMatch}
            onShowToast={showToast}
          />

          {selectedMatch?.match_type === "Shape" && (!selectedMatch.row_index || !selectedMatch.col_index) ? (
            <div className="flex-1 min-h-0 min-w-0 p-6 flex flex-col items-center justify-center text-zinc-400 bg-[#141416]">
              <div className="max-w-md w-full bg-[#1a1a1d] border border-zinc-800 rounded-lg p-5 text-center shadow-sm">
                <span className="inline-block px-2.5 py-1 rounded text-xs font-medium bg-amber-950/60 text-amber-300 border border-amber-800/80 mb-3">
                  {selectedMatch.shape_name || "Shape"}
                </span>
                <p className="text-xs text-zinc-300 mb-3 whitespace-pre-wrap text-left bg-zinc-900/80 p-3 rounded border border-zinc-800/80 font-mono max-h-48 overflow-y-auto">
                  {selectedMatch.full_content}
                </p>
                <p className="text-[11px] text-zinc-500">
                  {/* Constant reference: t(language, "ui.PREVIEW_SHAPE_NO_ANCHOR") */}
                  {t(language, "ui.PREVIEW_SHAPE_NO_ANCHOR")}
                </p>
              </div>
            </div>
          ) : (
            <>
              {/* Formula bar */}
              <FormulaBar
                cellAddress={selectedCell?.address || selectedMatch?.cell_address || ""}
                formulaOrValue={formulaOrValue}
              />

              {/* Spreadsheet grid */}
              <SpreadsheetGrid
                previewData={previewData}
                isLoading={loadingPreview}
                selectedCell={selectedCell}
                onSelectCell={handleSelectCell}
              />
            </>
          )}

          {/* Worksheet tabs bar */}
          {previewData && (
            <div className="px-3.5 pb-1 bg-[#141416] flex-shrink-0">
              <SheetTabs
                sheets={previewData.sheets_in_workbook}
                activeSheet={activeSheet}
                onSelectSheet={handleSelectSheet}
              />
            </div>
          )}

          {/* Preview bottom: Cell metadata card */}
          {selectedMatch && (
            <div className="px-3.5 pb-3.5 bg-[#141416] flex-shrink-0">
              <MetaInfoCard match={selectedMatch} />
            </div>
          )}
        </div>
      </div>

      {/* Footer & status bar */}
      <StatusBar
        progress={progress}
        items={results}
        onShowToast={showToast}
        onOpenAbout={() => setIsAboutOpen(true)}
        onOpenSettings={() => setIsSettingsOpen(true)}
        language={language}
      />

      <SettingsDialog
        isOpen={isSettingsOpen}
        preference={preference}
        language={language}
        maxEntries={maxEntries}
        defaultOptions={defaultOptions}
        rememberedBurstWorkers={rememberedBurstWorkers}
        onSaveSettings={handleSavePreferences}
        onSelect={async (value) => {
          const saved = await selectLanguage(value);
          if (!saved) setToastNotice({ key: "ui.SAVE_FAILED" });
        }}
        onClose={() => setIsSettingsOpen(false)}
      />

      {/* About dialog modal */}
      <Suspense fallback={null}>
        {isAboutOpen && <AboutDialog
          isOpen={isAboutOpen}
          onClose={() => setIsAboutOpen(false)}
          onShowToast={showToast}
        />}
      </Suspense>

      {/* Toast notification */}
      <Toast
        message={toastMessage}
        onClose={() => setToastNotice(null)}
      />
    </WindowFrame>
    </LocaleProvider>
  );
};

export default App;
