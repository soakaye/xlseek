/**
 * @fileoverview Search and preview management custom hook (src/hooks/useSearch.ts)
 *
 * ## Description
 * Manages search query state, issues Tauri backend start/cancel commands,
 * subscribes to background scanning progress and match hit events with buffering,
 * and fetches surrounding cell preview data and sheet switching.
 * Complies with Constitution Principle I (English documentation), Principle II (constant reference), and Principle III (comprehensive documentation).
 */

import { useState, useEffect, useCallback, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import {
  SearchQuery,
  SearchIssue,
  SearchMatch,
  ScanProgress,
  CellPreviewData,
} from "../types/search";
import { DefaultSearchOptions } from "../types/defaultOptions";
import { loadDefaultSearchOptions } from "../default-options-core";

import {
  COMMANDS,
  EVENT_NAMES,
  TIMING_CONSTANTS,
} from "../constants";
import { TranslationKey } from "../i18n";

interface UseSearchOptions {
  onShowToast?: (key: TranslationKey) => void;
  onSearchAccepted?: (query: SearchQuery) => void;
}

/**
 * ## Description
 * Main custom hook managing search execution, progress synchronization, and cell previews.
 *
 * ## Arguments
 * @param options - Configuration options including toast notification callbacks
 *
 * ## Returns
 * @returns Search state, query, result lists, preview data, and action handlers
 *
 * ## Errors / Exceptions
 * IPC invocation failures are handled gracefully via logs and toast notifications; never throws.
 */
export function useSearch(options?: UseSearchOptions) {
  const [query, setQuery] = useState<SearchQuery>(() => {
    const defaults = loadDefaultSearchOptions();
    return {
      keyword: "",
      target_dir: "",
      match_case: defaults.match_case,
      use_regex: defaults.use_regex,
      include_formula: defaults.include_formula,
      include_shape: defaults.include_shape,
      include_comment: defaults.include_comment,
      include_hidden: defaults.include_hidden,
      extensions: [...defaults.extensions],
      directory_mode: defaults.directory_mode,
      burst_workers: defaults.burst_workers,
    };
  });

  const [results, setResults] = useState<SearchMatch[]>([]);
  const [progress, setProgress] = useState<ScanProgress | null>(null);
  const [searchIssues, setSearchIssues] = useState<SearchIssue[]>([]);
  const [selectedMatch, setSelectedMatch] = useState<SearchMatch | null>(null);
  const [previewData, setPreviewData] = useState<CellPreviewData | null>(null);
  const [loadingPreview, setLoadingPreview] = useState(false);
  const [activeSheet, setActiveSheet] = useState<string>("");
  const [selectedCell, setSelectedCell] = useState<{
    address: string;
    value: string;
  } | null>(null);
  const [formulaOrValue, setFormulaOrValue] = useState<string>("");

  const isScanning = progress?.state === "Scanning";

  // Result batch buffer to minimize re-renders during high-volume match streams
  const resultsBufferRef = useRef<SearchMatch[]>([]);
  const flushTimerRef = useRef<number | null>(null);
  // Cancellation in-progress flag to drop delayed scanning events
  const isCancellingRef = useRef(false);

  // Tauri event listener registration (preventing duplicate bindings under React 18 StrictMode)
  useEffect(() => {
    let unlistenMatch: UnlistenFn | undefined;
    let unlistenProg: UnlistenFn | undefined;
    let unlistenIssue: UnlistenFn | undefined;
    let isCancelled = false;

    const appendResultsSafely = (newItems: SearchMatch[]) => {
      if (newItems.length === 0) return;
      setResults((prev) => {
        const existingIds = new Set(prev.map((it) => it.id));
        const uniqueItems = newItems.filter((it) => !existingIds.has(it.id));
        if (uniqueItems.length === 0) return prev;
        return [...prev, ...uniqueItems];
      });
    };

    const setupListeners = async () => {
      try {
        // Constant reference: EVENT_NAMES.SEARCH_MATCH ("search-match")
        unlistenMatch = await listen<SearchMatch | SearchMatch[]>(EVENT_NAMES.SEARCH_MATCH, (event) => {
          if (isCancelled || isCancellingRef.current) return;
          if (Array.isArray(event.payload)) {
            resultsBufferRef.current.push(...event.payload);
          } else {
            resultsBufferRef.current.push(event.payload);
          }

          if (!flushTimerRef.current) {
            // Constant reference: TIMING_CONSTANTS.PROGRESS_THROTTLE_MS
            flushTimerRef.current = window.setTimeout(() => {
              if (isCancelled || isCancellingRef.current) return;
              const buffered = resultsBufferRef.current;
              resultsBufferRef.current = [];
              appendResultsSafely(buffered);
              flushTimerRef.current = null;
            }, TIMING_CONSTANTS.PROGRESS_THROTTLE_MS);
          }
        });

        // Constant reference: EVENT_NAMES.SCAN_PROGRESS ("scan-progress")
        unlistenProg = await listen<ScanProgress>(EVENT_NAMES.SCAN_PROGRESS, (event) => {
          if (isCancelled) return;

          // Discard delayed scanning events after cancellation to prevent state regressions
          if (isCancellingRef.current && event.payload.state === "Scanning") {
            console.log("[useSearch] Ignored delayed scan event after cancellation");
            return;
          }

          console.log("[useSearch] Scan progress received");
          setProgress(event.payload);

          if (event.payload.state === "Cancelled") {
            isCancellingRef.current = false;
          }

          // Force flush buffer on scan finish or cancel
          if (event.payload.state !== "Scanning") {
            if (flushTimerRef.current) {
              clearTimeout(flushTimerRef.current);
              flushTimerRef.current = null;
            }
            if (resultsBufferRef.current.length > 0) {
              const buffered = resultsBufferRef.current;
              resultsBufferRef.current = [];
              appendResultsSafely(buffered);
            }
          }
        });

        // Constant reference: EVENT_NAMES.SEARCH_ISSUE.
        unlistenIssue = await listen<SearchIssue>(EVENT_NAMES.SEARCH_ISSUE, (event) => {
          if (!isCancelled) {
            setSearchIssues((previous) => previous.some((issue) => issue.path === event.payload.path && issue.code === event.payload.code)
              ? previous
              : [...previous, event.payload]);
          }
        });

        if (isCancelled) {
          unlistenMatch();
          unlistenProg();
          unlistenIssue();
        } else {
          console.log("[useSearch] Search event listeners registered");
        }
      } catch {
        unlistenMatch?.();
        unlistenProg?.();
        unlistenIssue?.();
        console.error("[useSearch] Failed to register search event listeners");
      }
    };

    setupListeners();

    return () => {
      isCancelled = true;
      if (unlistenMatch) unlistenMatch();
      if (unlistenProg) unlistenProg();
      if (unlistenIssue) unlistenIssue();
      if (flushTimerRef.current) {
        clearTimeout(flushTimerRef.current);
        flushTimerRef.current = null;
      }
    };
  }, []);

  // Cell preview loading
  const loadPreview = useCallback(
    async (match: SearchMatch, sheetNameOverride?: string) => {
      setLoadingPreview(true);
      const targetSheet = sheetNameOverride || match.sheet_name;
      setActiveSheet(targetSheet);

      try {
        // Constant reference: COMMANDS.GET_CELL_PREVIEW
        const data = await invoke<CellPreviewData>(COMMANDS.GET_CELL_PREVIEW, {
          filePath: match.full_path,
          sheetName: targetSheet,
          rowIndex: match.row_index,
          colIndex: match.col_index,
        });
        setPreviewData(data);
        setSelectedCell({
          address: match.cell_address,
          value: match.formula || match.full_content,
        });
        setFormulaOrValue(match.formula || match.full_content);
      } catch {
        console.error("[useSearch] Failed to load cell preview");
        options?.onShowToast?.("ui.PREVIEW_LOAD_ERROR");
        setPreviewData(null);
      } finally {
        setLoadingPreview(false);
      }
    },
    [options]
  );

  // Match item selection handler
  const handleSelectItem = useCallback(
    (item: SearchMatch) => {
      setSelectedMatch(item);
      if (item.match_type === "Shape" && (!item.row_index || !item.col_index)) {
        setPreviewData(null);
        setLoadingPreview(false);
        setActiveSheet(item.sheet_name);
        setSelectedCell(null);
        setFormulaOrValue(item.full_content);
        return;
      }
      loadPreview(item);
    },
    [loadPreview]
  );

  // Sheet tab selection handler
  const handleSelectSheet = useCallback(
    (sheetName: string) => {
      if (!selectedMatch) return;
      if (
        selectedMatch.match_type === "Shape" &&
        (!selectedMatch.row_index || !selectedMatch.col_index)
      ) {
        setActiveSheet(sheetName);
        return;
      }
      loadPreview(selectedMatch, sheetName);
    },
    [selectedMatch, loadPreview]
  );

  // Grid cell selection handler
  const handleSelectCell = useCallback(
    (address: string, value: string, formula?: string | null) => {
      setSelectedCell({ address, value });
      setFormulaOrValue(formula || value);
    },
    []
  );

  // Search execution
  const startSearch = useCallback(async () => {
    if (!query.keyword.trim()) {
      // Translation reference: ui.TOAST_SEARCH_KEYWORD
      options?.onShowToast?.("ui.TOAST_SEARCH_KEYWORD");
      return;
    }
    if (!query.target_dir.trim()) {
      // Translation reference: ui.SELECT_FOLDER_PROMPT
      options?.onShowToast?.("ui.SELECT_FOLDER_PROMPT");
      return;
    }
    if (!query.extensions || query.extensions.length === 0) {
      // Translation reference: ui.SELECT_EXTENSION_PROMPT
      options?.onShowToast?.("ui.SELECT_EXTENSION_PROMPT");
      return;
    }

    console.log("[useSearch] Search request submitted");
    isCancellingRef.current = false;
    setResults([]);
    setSearchIssues([]);
    resultsBufferRef.current = [];
    setSelectedMatch(null);
    setPreviewData(null);
    setSelectedCell(null);
    setFormulaOrValue("");

    // Progress initialization
    setProgress({
      state: "Scanning",
      phase: "preparing",
      error_code: null,
      scanned_files: 0,
      total_files: 0,
      matches_found: 0,
      current_file: "",
      elapsed_ms: 0,
    });

    try {
      // Constant reference: COMMANDS.START_SEARCH
      await invoke(COMMANDS.START_SEARCH, { query });
      options?.onSearchAccepted?.(query);
    } catch (error: unknown) {
      console.error("[useSearch] Failed to start search");
      const isInvalidRegex =
        typeof error === "object" &&
        error !== null &&
        "code" in error &&
        error.code === "invalid_regex";
      options?.onShowToast?.(isInvalidRegex ? "ui.INVALID_REGEX" : "ui.SEARCH_START_ERROR");
      setProgress({
        state: "Error",
        phase: "finished",
        error_code: "internal_error",
        scanned_files: 0,
        total_files: 0,
        matches_found: 0,
        current_file: "",
        elapsed_ms: 0,
      });
    }
  }, [query, options]);

  // Search cancellation
  const cancelSearch = useCallback(async () => {
    try {
      console.log("[useSearch] Search cancellation requested");
      isCancellingRef.current = true;

      // Immediately switch state to Cancelled for responsive UI
      setProgress((prev) =>
        prev
          ? {
              ...prev,
              state: "Cancelled",
              current_file: "",
            }
          : {
              state: "Cancelled",
              phase: "finished",
              error_code: null,
              scanned_files: 0,
              total_files: 0,
              matches_found: 0,
              current_file: "",
              elapsed_ms: 0,
            }
      );

      // Flush remaining buffered matches
      if (flushTimerRef.current) {
        clearTimeout(flushTimerRef.current);
        flushTimerRef.current = null;
      }
      if (resultsBufferRef.current.length > 0) {
        const buffered = resultsBufferRef.current;
        resultsBufferRef.current = [];
        setResults((prev) => {
          const existingIds = new Set(prev.map((it) => it.id));
          const uniqueItems = buffered.filter((it) => !existingIds.has(it.id));
          return uniqueItems.length > 0 ? [...prev, ...uniqueItems] : prev;
        });
      }

      // Constant reference: COMMANDS.CANCEL_SEARCH
      await invoke(COMMANDS.CANCEL_SEARCH);
    } catch {
      console.error("[useSearch] Failed to cancel search");
    }
  }, []);

  const updateQuery = useCallback((newQuery: Partial<SearchQuery>) => {
    setQuery((prev) => ({ ...prev, ...newQuery }));
  }, []);

  const applyDefaultOptions = useCallback((options: DefaultSearchOptions) => {
    setQuery((prev) => ({
      ...prev,
      match_case: options.match_case,
      use_regex: options.use_regex,
      include_formula: options.include_formula,
      include_shape: options.include_shape,
      include_comment: options.include_comment,
      include_hidden: options.include_hidden,
      extensions: [...options.extensions],
      directory_mode: options.directory_mode,
      burst_workers: options.burst_workers,
    }));
  }, []);

  return {
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
  };
}
