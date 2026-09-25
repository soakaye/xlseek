import { useState, useEffect, useCallback, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import {
  SearchQuery,
  SearchMatch,
  ScanProgress,
  CellPreviewData,
} from "../types/search";

export function useSearch() {
  const [query, setQuery] = useState<SearchQuery>({
    keyword: "",
    target_dir: "",
    match_case: false,
    use_regex: false,
    include_formula: true,
    include_comment: true,
    include_hidden: false,
    extensions: [".xlsx", ".xlsm", ".xlsb", ".xls"],
  });

  const [results, setResults] = useState<SearchMatch[]>([]);
  const [progress, setProgress] = useState<ScanProgress | null>(null);
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

  // 結果リスト蓄積用のバッファ（大量のマッチ受信時の再レンダリング頻度を抑える）
  const resultsBufferRef = useRef<SearchMatch[]>([]);
  const flushTimerRef = useRef<number | null>(null);

  // Tauri イベントリスナーの登録
  useEffect(() => {
    let unlistenMatch: UnlistenFn | undefined;
    let unlistenProg: UnlistenFn | undefined;

    const setupListeners = async () => {
      unlistenMatch = await listen<SearchMatch>("search-match", (event) => {
        resultsBufferRef.current.push(event.payload);

        if (!flushTimerRef.current) {
          flushTimerRef.current = window.setTimeout(() => {
            const buffered = resultsBufferRef.current;
            resultsBufferRef.current = [];
            setResults((prev) => [...prev, ...buffered]);
            flushTimerRef.current = null;
          }, 60);
        }
      });

      unlistenProg = await listen<ScanProgress>("scan-progress", (event) => {
        setProgress(event.payload);
        // スキャン完了または中断時にバッファを強制フラッシュ
        if (event.payload.state !== "Scanning") {
          if (flushTimerRef.current) {
            clearTimeout(flushTimerRef.current);
            flushTimerRef.current = null;
          }
          if (resultsBufferRef.current.length > 0) {
            const buffered = resultsBufferRef.current;
            resultsBufferRef.current = [];
            setResults((prev) => [...prev, ...buffered]);
          }
        }
      });
    };

    setupListeners();

    return () => {
      if (unlistenMatch) unlistenMatch();
      if (unlistenProg) unlistenProg();
      if (flushTimerRef.current) clearTimeout(flushTimerRef.current);
    };
  }, []);

  // プレビューの読み込み
  const loadPreview = useCallback(
    async (match: SearchMatch, sheetNameOverride?: string) => {
      setLoadingPreview(true);
      const targetSheet = sheetNameOverride || match.sheet_name;
      setActiveSheet(targetSheet);

      try {
        const data = await invoke<CellPreviewData>("get_cell_preview", {
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
      } catch (err) {
        console.error("プレビュー取得失敗:", err);
        setPreviewData(null);
      } finally {
        setLoadingPreview(false);
      }
    },
    []
  );

  // アイテム選択時
  const handleSelectItem = useCallback(
    (item: SearchMatch) => {
      setSelectedMatch(item);
      loadPreview(item);
    },
    [loadPreview]
  );

  // シート切り替え時
  const handleSelectSheet = useCallback(
    (sheetName: string) => {
      if (!selectedMatch) return;
      loadPreview(selectedMatch, sheetName);
    },
    [selectedMatch, loadPreview]
  );

  // グリッド内のセルクリック時
  const handleSelectCell = useCallback(
    (address: string, value: string, formula?: string | null) => {
      setSelectedCell({ address, value });
      setFormulaOrValue(formula || value);
    },
    []
  );

  // 検索開始
  const startSearch = useCallback(async () => {
    if (!query.keyword.trim()) return;
    if (!query.target_dir.trim()) return;

    setResults([]);
    resultsBufferRef.current = [];
    setSelectedMatch(null);
    setPreviewData(null);
    setSelectedCell(null);
    setFormulaOrValue("");

    setProgress({
      state: "Scanning",
      scanned_files: 0,
      total_files: 0,
      matches_found: 0,
      current_file: "開始準備中...",
      elapsed_ms: 0,
    });

    try {
      await invoke("start_search", { query });
    } catch (err) {
      console.error("検索開始エラー:", err);
      setProgress({
        state: "Error",
        scanned_files: 0,
        total_files: 0,
        matches_found: 0,
        current_file: `エラー: ${err}`,
        elapsed_ms: 0,
      });
    }
  }, [query]);

  // 検索中断
  const cancelSearch = useCallback(async () => {
    try {
      await invoke("cancel_search");
    } catch (err) {
      console.error("検索中断エラー:", err);
    }
  }, []);

  const updateQuery = useCallback((newQuery: Partial<SearchQuery>) => {
    setQuery((prev) => ({ ...prev, ...newQuery }));
  }, []);

  return {
    query,
    updateQuery,
    results,
    progress,
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
