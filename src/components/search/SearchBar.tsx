/**
 * Copyright (c) 2026 soakaye
 *
 * @fileoverview Search parameters input and execution bar component (src/components/search/SearchBar.tsx)
 *
 * ## Description
 * Provides input fields for search keyword, target folder, and search options (match case, regex,
 * formula, comments, hidden sheets). Triggers search execution and cancellation. Supports folder
 * drag-and-drop path resolution and search history suggestions.
 * Complies with Constitution Principle I (English documentation), Principle II (constant reference), and Principle III (comprehensive documentation).
 */

import React, { KeyboardEvent, useState, useRef, useEffect } from "react";
import { Search, X, Folder, FolderOpen, SlidersHorizontal, Square, ArrowDownToLine, History } from "lucide-react";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { SearchQuery } from "../../types/search";
import { SEARCH_LABELS } from "../../constants";
import { COMMANDS, FILE_EXTENSIONS, KEYBOARD_KEYS, PATH_COMPLETION_CONSTANTS } from "../../constants";
import { useTranslation } from "../../i18n";
import { appendSearchPath, getActivePathSegment } from "../../utils/pathTokenizer";

interface SearchBarProps {
  query: SearchQuery;
  onChangeQuery: (newQuery: Partial<SearchQuery>) => void;
  onSearch: () => void;
  onCancel: () => void;
  isScanning: boolean;
  history: { keywords: string[]; directories: string[] };
  onSelectHistory: (field: "keyword" | "directory", value: string) => void;
}

/**
 * ## Description
 * Search bar UI component.
 *
 * ## Arguments
 * @param props - SearchBarProps
 *
 * ## Returns
 * @returns Rendered search bar element
 *
 * ## Errors / Exceptions
 * IPC errors during drag-and-drop or folder selection dialog are caught and logged to console to prevent crashes.
 */
export const SearchBar: React.FC<SearchBarProps> = ({
  query,
  onChangeQuery,
  onSearch,
  onCancel,
  isScanning,
  history,
  onSelectHistory,
}) => {
  const t = useTranslation();
  const [isDragOver, setIsDragOver] = useState(false);
  const keywordInputRef = useRef<HTMLDivElement>(null);
  const folderInputRef = useRef<HTMLDivElement>(null);
  const folderInputElementRef = useRef<HTMLInputElement>(null);
  const [openList, setOpenList] = useState<"keyword" | "directory" | "directory-history" | null>(null);
  const [directorySuggestions, setDirectorySuggestions] = useState<string[]>([]);
  const [activeOption, setActiveOption] = useState<number | null>(null);
  const [completionDismissed, setCompletionDismissed] = useState(false);
  const [directoryFocused, setDirectoryFocused] = useState(false);
  const completionRequestRef = useRef(0);
  const activeSegmentRangeRef = useRef<{ start: number; end: number }>({ start: 0, end: 0 });

  /**
   * Fetches path completion suggestions for directory input.
   */
  useEffect(() => {
    const request = ++completionRequestRef.current;
    if (!directoryFocused || completionDismissed || !query.target_dir) return;

    const cursorPos = folderInputElementRef.current?.selectionStart ?? query.target_dir.length;
    const { segment, startIndex, endIndex } = getActivePathSegment(query.target_dir, cursorPos);
    activeSegmentRangeRef.current = { start: startIndex, end: endIndex };

    if (!segment.trim()) {
      setDirectorySuggestions([]);
      setOpenList(null);
      return;
    }

    const timer = window.setTimeout(async () => {
      try {
        // Constant reference: COMMANDS.COMPLETE_DIRECTORY_PATH
        const values = await invoke<string[]>(COMMANDS.COMPLETE_DIRECTORY_PATH, { pathInput: segment });
        if (request === completionRequestRef.current) {
          setDirectorySuggestions(values);
          if (values.length) setOpenList("directory");
        }
      } catch {
        if (request === completionRequestRef.current) setDirectorySuggestions([]);
      }
    }, PATH_COMPLETION_CONSTANTS.DEBOUNCE_MS);
    return () => window.clearTimeout(timer);
  }, [query.target_dir, completionDismissed, directoryFocused]);

  /** Closes active history or suggestion dropdown list. */
  const closeList = () => { setOpenList(null); setActiveOption(null); };

  /** Handles blur on search or directory input field to close dropdown. */
  const handleFieldBlur = (event: React.FocusEvent<HTMLDivElement>, field: "keyword" | "directory") => {
    if (event.relatedTarget instanceof Node && event.currentTarget.contains(event.relatedTarget)) return;
    completionRequestRef.current++;
    if (field === "directory") setDirectoryFocused(false);
    closeList();
  };

  /** Closes dropdown on pointerdown outside field region. */
  useEffect(() => {
    if (!openList && !directoryFocused) return;
    const handlePointerDown = (event: PointerEvent) => {
      const region = openList === "keyword" ? keywordInputRef.current : folderInputRef.current;
      if (region?.contains(event.target as Node)) return;
      completionRequestRef.current++;
      setDirectoryFocused(false);
      setOpenList(null);
      setActiveOption(null);
    };
    document.addEventListener("pointerdown", handlePointerDown);
    return () => document.removeEventListener("pointerdown", handlePointerDown);
  }, [openList, directoryFocused]);

  /** Selects an item from the open history or suggestions list. */
  const selectOption = (index: number) => {
    const values = openList === "keyword" ? history.keywords : openList === "directory-history" ? history.directories : openList === "directory" ? directorySuggestions : [];
    const value = values[index];
    if (!value || !openList) return;
    completionRequestRef.current += 1;

    if (openList === "directory") {
      // Replace only active segment under cursor
      const current = query.target_dir;
      const { start, end } = activeSegmentRangeRef.current;
      const needsQuotes = value.includes(",") || value.includes(" ");
      const formatted = needsQuotes ? `"${value.replace(/"/g, '""')}"` : value;
      const nextTargetDir = current.slice(0, start) + formatted + current.slice(end);
      onChangeQuery({ target_dir: nextTargetDir });
      setCompletionDismissed(true);
    } else {
      onSelectHistory(openList === "keyword" ? "keyword" : "directory", value);
      if (openList !== "keyword") setCompletionDismissed(true);
    }
    closeList();
  };

  /** Handles keyboard navigation from history button into dropdown list. */
  // Constant reference: KEYBOARD_KEYS.TAB / KEYBOARD_KEYS.ARROW_DOWN
  const handleHistoryButtonKeyDown = (event: KeyboardEvent<HTMLButtonElement>, field: "keyword" | "directory") => {
    const isOpen = field === "keyword" ? openList === "keyword" : openList === "directory-history";
    if (!isOpen || (event.key !== KEYBOARD_KEYS.TAB && event.key !== KEYBOARD_KEYS.ARROW_DOWN)) return;
    const values = field === "keyword" ? history.keywords : history.directories;
    if (!values.length) return;
    const region = field === "keyword" ? keywordInputRef.current : folderInputRef.current;
    const firstOption = region?.querySelector<HTMLButtonElement>(`[role="option"]`);
    if (!firstOption) return;
    event.preventDefault();
    firstOption.focus();
  };

  /** Handles key navigation (arrows, Enter, Esc) within dropdown list items. */
  const handleHistoryOptionKeyDown = (event: KeyboardEvent<HTMLButtonElement>, index: number, field: "keyword" | "directory") => {
    const isOpen = field === "keyword" ? openList === "keyword" : openList === "directory-history";
    if (!isOpen) return;
    // Constant reference: KEYBOARD_KEYS.IME_COMPOSITION / KEYBOARD_KEYS.IME_COMPOSITION_KEY_CODE
    if (event.nativeEvent.isComposing || event.key === KEYBOARD_KEYS.IME_COMPOSITION || event.keyCode === KEYBOARD_KEYS.IME_COMPOSITION_KEY_CODE) {
      if (event.key === KEYBOARD_KEYS.ENTER) event.preventDefault();
      return;
    }
    const values = field === "keyword" ? history.keywords : history.directories;
    if (index < 0 || index >= values.length) return;
    const region = field === "keyword" ? keywordInputRef.current : folderInputRef.current;
    const options = region?.querySelectorAll<HTMLButtonElement>(`[role="option"]`);
    if (!options?.length) return;

    let nextIndex: number | null = null;
    if (event.key === KEYBOARD_KEYS.ARROW_DOWN) {
      nextIndex = Math.min(values.length - PATH_COMPLETION_CONSTANTS.KEYBOARD_STEP, index + PATH_COMPLETION_CONSTANTS.KEYBOARD_STEP);
    } else if (event.key === KEYBOARD_KEYS.ARROW_UP) {
      nextIndex = Math.max(0, index - PATH_COMPLETION_CONSTANTS.KEYBOARD_STEP);
    } else if (event.key === KEYBOARD_KEYS.TAB && !event.shiftKey) {
      nextIndex = (index + PATH_COMPLETION_CONSTANTS.KEYBOARD_STEP) % values.length;
    } else if (event.key === KEYBOARD_KEYS.ENTER) {
      event.preventDefault();
      selectOption(index);
      return;
    } else if (event.key === KEYBOARD_KEYS.ESCAPE) {
      event.preventDefault();
      closeList();
      region?.querySelector<HTMLInputElement>(`input`)?.focus();
      return;
    } else {
      return;
    }

    event.preventDefault();
    setActiveOption(nextIndex);
    options[nextIndex]?.focus();
  };

  /** Handles list keyboard navigation in input fields. */
  const handleListKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
    if (!openList) return false;
    const values = openList === "keyword" ? history.keywords : openList === "directory-history" ? history.directories : directorySuggestions;
    // Constant reference: KEYBOARD_KEYS.ESCAPE / ARROW_DOWN / ARROW_UP / ENTER
    if (e.key === KEYBOARD_KEYS.ESCAPE) { e.preventDefault(); closeList(); return true; }
    if (e.key === KEYBOARD_KEYS.ARROW_DOWN || e.key === KEYBOARD_KEYS.ARROW_UP) {
      e.preventDefault();
      if (!values.length) return true;
      if (e.key === KEYBOARD_KEYS.ARROW_DOWN && activeOption === null && (openList === "keyword" || openList === "directory-history")) {
        const region = openList === "keyword" ? keywordInputRef.current : folderInputRef.current;
        region?.querySelector<HTMLButtonElement>(`[role="option"]`)?.focus();
        return true;
      }
      const delta = e.key === KEYBOARD_KEYS.ARROW_DOWN ? PATH_COMPLETION_CONSTANTS.KEYBOARD_STEP : -PATH_COMPLETION_CONSTANTS.KEYBOARD_STEP;
      const current = activeOption === null ? (delta > 0 ? -1 : values.length) : activeOption;
      setActiveOption(Math.max(0, Math.min(values.length - 1, current + delta)));
      return true;
    }
    if (e.key === KEYBOARD_KEYS.ENTER && activeOption !== null) { e.preventDefault(); selectOption(activeOption); return true; }
    return false;
  };

  /** Toggles history list visibility. */
  const toggleHistory = (field: "keyword" | "directory") => {
    completionRequestRef.current += 1;
    if (field === "directory") setCompletionDismissed(true);
    setOpenList(field === "directory" ? (openList === "directory-history" ? null : "directory-history") : (openList === field ? null : field));
    setActiveOption(null);
  };

  // Tauri native drag-and-drop listener
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
                  // Constant reference: COMMANDS.RESOLVE_DROPPED_PATH
                  let updated = query.target_dir;
                  for (const dropped of payload.paths) {
                    const resolvedPath = await invoke<string>(COMMANDS.RESOLVE_DROPPED_PATH, {
                      path: dropped,
                    });
                    updated = appendSearchPath(updated, resolvedPath);
                  }
                  onChangeQuery({ target_dir: updated });
                } catch {
                  console.error("[SearchBar] Failed to resolve dropped folder path");
                }
              }
            }
            setIsDragOver(false);
          } else if (payload.type === "leave") {
            setIsDragOver(false);
          }
        });
      } catch {
        console.warn("[SearchBar] Failed to initialize native drag and drop listener");
      }
    };

    setupDragDrop();

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, [onChangeQuery, query.target_dir]);

  // Standard HTML5 drag-and-drop fallback
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
      try {
        let updated = query.target_dir;
        for (let i = 0; i < e.dataTransfer.files.length; i++) {
          const file = e.dataTransfer.files[i];
          const filePath = (file as File & { path?: string }).path;
          if (filePath) {
            // Constant reference: COMMANDS.RESOLVE_DROPPED_PATH
            const resolvedPath = await invoke<string>(COMMANDS.RESOLVE_DROPPED_PATH, {
              path: filePath,
            });
            updated = appendSearchPath(updated, resolvedPath);
          }
        }
        onChangeQuery({ target_dir: updated });
      } catch {
        console.error("[SearchBar] Failed to resolve dropped folder path");
      }
    }
  };

  const isComposingRef = useRef(false);
  const compositionEndTimeRef = useRef(0);

  const handleCompositionStart = () => {
    isComposingRef.current = true;
  };

  const handleCompositionEnd = () => {
    isComposingRef.current = false;
    compositionEndTimeRef.current = Date.now();
  };

  const handleKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
    // Avoid triggering search on Enter key when IME composition finishes
    if (
      e.nativeEvent.isComposing ||
      isComposingRef.current ||
      e.key === "Process" ||
      e.keyCode === 229
    ) {
      return;
    }
    if (Date.now() - compositionEndTimeRef.current < 50) {
      return;
    }
    if (handleListKeyDown(e)) return;
    if (e.key === "Enter" && !isScanning) {
      onSearch();
    }
  };

  const handleFolderKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
    if (e.nativeEvent.isComposing || e.key === "Process" || e.keyCode === 229) {
      return;
    }
    if (handleListKeyDown(e)) return;
    if (e.key === "Enter") {
      e.preventDefault();
    }
  };

  /** Closes suggestions and opens the system folder selection dialog. */
  const handleBrowseFolder = async () => {
    completionRequestRef.current++;
    setCompletionDismissed(true);
    setDirectoryFocused(false);
    closeList();
    try {
      // Constant reference: t("ui.SELECT_FOLDER_DIALOG_TITLE")
      const selected = await open({
        directory: true,
        multiple: false,
        title: t("ui.SELECT_FOLDER_DIALOG_TITLE"),
      });
      if (selected && typeof selected === "string") {
        onChangeQuery({ target_dir: appendSearchPath(query.target_dir, selected) });
      }
    } catch {
      console.error("[SearchBar] Failed to select folder");
    }
  };

  const currentExtensions = query.extensions ?? [...FILE_EXTENSIONS.DEFAULT_LIST];

  const handleToggleExtension = (ext: string) => {
    const isSelected = currentExtensions.includes(ext);
    if (isSelected) {
      // Keep at least one extension selected
      if (currentExtensions.length <= 1) {
        return;
      }
      onChangeQuery({
        extensions: currentExtensions.filter((e) => e !== ext),
      });
    } else {
      onChangeQuery({
        extensions: [...currentExtensions, ext],
      });
    }
  };

  return (
    <header className="bg-[#18181b] border-b border-zinc-800 p-4 shadow-md flex-shrink-0">
      <div className="max-w-[1920px] mx-auto space-y-3">
        {/* Row 1: Keyword, Folder, and Search/Cancel Button */}
        <div className="grid grid-cols-1 lg:grid-cols-12 gap-3 items-center">
          {/* Keyword input */}
          <div ref={keywordInputRef} onBlur={(event) => handleFieldBlur(event, "keyword")} className="lg:col-span-6 relative">
            <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none text-zinc-400">
              <Search className="w-4 h-4" />
            </div>
            <input
              type="text"
              value={query.keyword}
              onChange={(e) => onChangeQuery({ keyword: e.target.value })}
              onKeyDown={handleKeyDown}
              onCompositionStart={handleCompositionStart}
              onCompositionEnd={handleCompositionEnd}
              /* Constant reference: t("ui.KEYWORD_INPUT_PLACEHOLDER") */
              placeholder={t("ui.KEYWORD_INPUT_PLACEHOLDER")}
              className="w-full pl-9 pr-24 py-2 bg-[#202024] border border-zinc-700 rounded-lg text-sm text-zinc-100 placeholder-zinc-500 focus:outline-none focus:border-excel-light focus:ring-1 focus:ring-excel-light transition"
            />
            <div className="absolute inset-y-0 right-1 flex items-center gap-1 pr-1.5">
              {query.keyword && (
                <button
                  type="button"
                  onClick={() => onChangeQuery({ keyword: "" })}
                  /* Constant reference: t("ui.CLEAR_TOOLTIP") */
                  title={t("ui.CLEAR_TOOLTIP")}
                  className="p-1 hover:bg-zinc-700 text-zinc-400 hover:text-zinc-200 rounded"
                >
                  <X className="w-3.5 h-3.5" />
                </button>
              )}
              <button type="button" onClick={(event) => { event.currentTarget.focus(); toggleHistory("keyword"); }} onKeyDown={(event) => handleHistoryButtonKeyDown(event, "keyword")} title={t("ui.KEYWORD_HISTORY_TOOLTIP")} aria-label={t("ui.KEYWORD_HISTORY_TOOLTIP")} className="p-1 hover:bg-zinc-700 text-zinc-400 hover:text-zinc-200 rounded">
                <History className="w-3.5 h-3.5" />
              </button>
              <span className="text-[10px] text-zinc-400 font-mono bg-zinc-800 px-1.5 py-0.5 rounded border border-zinc-700">
                Enter
              </span>
            </div>
            {openList === "keyword" && (
              <div role="listbox" className="absolute z-20 top-full mt-1 w-full rounded-lg border border-zinc-700 bg-zinc-900 p-1 shadow-xl">
                {history.keywords.length ? history.keywords.map((value, index) => (
                  <button key={value} type="button" role="option" aria-selected={activeOption === index} onFocus={() => setActiveOption(index)} onKeyDown={(event) => handleHistoryOptionKeyDown(event, index, "keyword")} onMouseDown={(e) => e.preventDefault()} onClick={() => selectOption(index)} className="block w-full rounded px-2 py-1.5 text-left text-sm text-zinc-200 hover:bg-zinc-800 focus-visible:outline focus-visible:outline-2 focus-visible:outline-excel-light">{value}</button>
                )) : <div className="px-2 py-1.5 text-sm text-zinc-500">{t("ui.HISTORY_EMPTY")}</div>}
              </div>
            )}
          </div>

          {/* Folder path input & drag-and-drop zone */}
          <div
            ref={folderInputRef}
            onFocus={() => setDirectoryFocused(true)}
            onBlur={(event) => handleFieldBlur(event, "directory")}
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
              ref={folderInputElementRef}
              type="text"
              value={query.target_dir}
              onChange={(e) => { setCompletionDismissed(false); setOpenList(null); onChangeQuery({ target_dir: e.target.value }); }}
              onKeyDown={handleFolderKeyDown}
              /* Constant reference: t("ui.FOLDER_DROP_PLACEHOLDER") / FOLDER_INPUT_PLACEHOLDER */
              placeholder={isDragOver ? t("ui.FOLDER_DROP_PLACEHOLDER") : t("ui.FOLDER_INPUT_PLACEHOLDER")}
              className={`w-full pl-9 pr-20 py-2 border rounded-lg text-sm placeholder-zinc-500 focus:outline-none font-mono text-xs transition ${
                isDragOver
                  ? "bg-emerald-950/30 border-emerald-500 text-emerald-200 border-dashed"
                  : "bg-[#202024] border-zinc-700 text-zinc-300 focus:border-excel-light"
              }`}
            />
            {isDragOver && (
              <div className="absolute inset-0 flex items-center justify-center bg-emerald-900/80 border-2 border-dashed border-emerald-400 rounded-lg pointer-events-none text-xs font-medium text-emerald-100 gap-2 z-10 backdrop-blur-[1px]">
                <ArrowDownToLine className="w-4 h-4 text-emerald-300 animate-bounce" />
                {/* Constant reference: t("ui.FOLDER_DROP_PROMPT") */}
                <span>{t("ui.FOLDER_DROP_PROMPT")}</span>
              </div>
            )}
            <button
              type="button"
              onClick={handleBrowseFolder}
              /* Constant reference: t("ui.SELECT_FOLDER_TOOLTIP") */
              title={t("ui.SELECT_FOLDER_TOOLTIP")}
              className="absolute inset-y-1 right-1 px-2.5 flex items-center justify-center bg-zinc-700 hover:bg-zinc-600 rounded text-zinc-200 transition z-0"
            >
              <FolderOpen className="w-4 h-4" />
            </button>
            <button type="button" onClick={(event) => { event.currentTarget.focus(); toggleHistory("directory"); }} onKeyDown={(event) => handleHistoryButtonKeyDown(event, "directory")} title={t("ui.DIRECTORY_HISTORY_TOOLTIP")} aria-label={t("ui.DIRECTORY_HISTORY_TOOLTIP")} className="absolute inset-y-1 right-10 px-2 flex items-center justify-center bg-zinc-700 hover:bg-zinc-600 rounded text-zinc-200 transition z-0">
              <History className="w-4 h-4" />
            </button>
            {(openList === "directory" || openList === "directory-history") && (
              <div role="listbox" aria-label={t("ui.DIRECTORY_SUGGESTIONS")} className="absolute z-20 top-full mt-1 w-full rounded-lg border border-zinc-700 bg-zinc-900 p-1 shadow-xl">
                {(openList === "directory-history" ? history.directories : directorySuggestions).length ? (openList === "directory-history" ? history.directories : directorySuggestions).map((value, index) => (
                  <button key={value} type="button" role="option" aria-selected={activeOption === index} onFocus={openList === "directory-history" ? () => setActiveOption(index) : undefined} onKeyDown={openList === "directory-history" ? (event) => handleHistoryOptionKeyDown(event, index, "directory") : undefined} onMouseDown={(e) => e.preventDefault()} onClick={() => selectOption(index)} className="block w-full truncate rounded px-2 py-1.5 text-left text-xs font-mono text-zinc-200 hover:bg-zinc-800 focus-visible:outline focus-visible:outline-2 focus-visible:outline-excel-light">{value}</button>
                )) : <div className="px-2 py-1.5 text-sm text-zinc-500">{t("ui.HISTORY_EMPTY")}</div>}
              </div>
            )}
          </div>

          {/* Search Start / Cancel Button */}
          <div className="lg:col-span-2">
            {isScanning ? (
              <button
                type="button"
                onClick={onCancel}
                className="w-full py-2 px-4 bg-red-600 hover:bg-red-500 active:scale-[0.99] text-white font-semibold text-sm rounded-lg flex items-center justify-center gap-2 shadow-lg shadow-red-950/40 transition"
              >
                <Square className="w-4 h-4 fill-white" />
                {/* Constant reference: t("ui.BUTTON_CANCEL") */}
                <span>{t("ui.BUTTON_CANCEL")}</span>
              </button>
            ) : (
              <button
                type="button"
                onClick={onSearch}
                className="w-full py-2 px-4 bg-excel hover:bg-excel-hover active:scale-[0.99] text-white font-semibold text-sm rounded-lg flex items-center justify-center gap-2 shadow-lg shadow-emerald-950/40 transition"
              >
                <Search className="w-4 h-4" />
                {/* Constant reference: t("ui.BUTTON_SEARCH") */}
                <span>{t("ui.BUTTON_SEARCH")}</span>
              </button>
            )}
          </div>
        </div>

        {/* Row 2: Search Options (Toggle Pills) */}
        <div className="flex flex-wrap items-center justify-between gap-2 pt-1 text-xs select-none">
          <div className="flex flex-wrap items-center gap-2">
            <span className="text-zinc-400 mr-1 font-medium flex items-center gap-1">
              {/* Constant reference: t("ui.OPTIONS_LABEL") */}
              <SlidersHorizontal className="w-3.5 h-3.5" /> {t("ui.OPTIONS_LABEL")}
            </span>

            {/* Match case */}
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
              {/* Constant reference: t("ui.OPTION_MATCH_CASE") */}
              <span>{t("ui.OPTION_MATCH_CASE")}</span>
            </label>

            {/* Regex */}
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
              {/* Constant reference: t("ui.OPTION_USE_REGEX") */}
              <span>{t("ui.OPTION_USE_REGEX")}</span>
            </label>

            {/* Formula */}
            <label
              className={`cursor-pointer flex items-center gap-1.5 px-2.5 py-1 rounded-full border transition ${
                query.include_formula ?? true
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
              {/* Constant reference: t("ui.OPTION_INCLUDE_FORMULA") */}
              <span>{t("ui.OPTION_INCLUDE_FORMULA")}</span>
            </label>

            {/* Shape text */}
            <label
              className={`cursor-pointer flex items-center gap-1.5 px-2.5 py-1 rounded-full border transition ${
                query.include_shape ?? true
                  ? "bg-emerald-950/60 text-emerald-300 border-emerald-600/80 font-medium"
                  : "bg-zinc-800 hover:bg-zinc-700/80 text-zinc-300 border-zinc-700"
              }`}
            >
              <input
                type="checkbox"
                checked={query.include_shape ?? true}
                onChange={(event) => onChangeQuery({ include_shape: event.target.checked })}
                className="accent-excel rounded cursor-pointer"
              />
              {/* Constant reference: SEARCH_LABELS.INCLUDE_SHAPE */}
              <span>{t(SEARCH_LABELS.INCLUDE_SHAPE)}</span>
            </label>

            {/* Comment */}
            <label
              className={`cursor-pointer flex items-center gap-1.5 px-2.5 py-1 rounded-full border transition ${
                query.include_comment ?? true
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
              {/* Constant reference: t("ui.OPTION_INCLUDE_COMMENT") */}
              <span>{t("ui.OPTION_INCLUDE_COMMENT")}</span>
            </label>

            {/* Hidden sheet */}
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
              {/* Constant reference: t("ui.OPTION_INCLUDE_HIDDEN") */}
              <span>{t("ui.OPTION_INCLUDE_HIDDEN")}</span>
            </label>
          </div>

          {/* Target file extension toggle buttons */}
          <div className="flex items-center gap-2 text-zinc-400 text-xs">
            {/* Constant reference: t("ui.LABEL_TARGET_EXTENSIONS") */}
            <span>{t("ui.LABEL_TARGET_EXTENSIONS")}</span>
            <div className="flex gap-1">
              {FILE_EXTENSIONS.DEFAULT_LIST.map((ext) => {
                const isSelected = currentExtensions.includes(ext);
                return (
                  <button
                    key={ext}
                    type="button"
                    onClick={() => handleToggleExtension(ext)}
                    title={`${ext}${
                      isSelected
                        ? t("ui.EXTENSION_TOGGLE_EXCLUDE_SUFFIX")
                        : t("ui.EXTENSION_TOGGLE_INCLUDE_SUFFIX")
                    }`}
                    className={`px-2 py-0.5 rounded text-[11px] font-mono border transition-all cursor-pointer ${
                      isSelected
                        ? "bg-emerald-950/60 text-emerald-300 border-emerald-600/80 font-medium"
                        : "bg-zinc-900/80 hover:bg-zinc-800 text-zinc-500 border-zinc-800 hover:text-zinc-400"
                    }`}
                  >
                    {ext}
                  </button>
                );
              })}
            </div>
          </div>
        </div>
      </div>
    </header>
  );
};
