import { useState, useEffect, useCallback, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import {
  SearchQuery,
  SearchMatch,
  ScanProgress,
  CellPreviewData,
} from "../types/search";

interface UseSearchOptions {
  onShowToast?: (message: string) => void;
}

export function useSearch(options?: UseSearchOptions) {
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
  // 中断要求中フラグ (中断要求後に届く遅延Scanningイベントを破棄するため)
  const isCancellingRef = useRef(false);

  // Tauri イベントリスナーの登録 (React 18 StrictMode での2重登録を防止)
  useEffect(() => {
    let unlistenMatch: UnlistenFn | undefined;
    let unlistenProg: UnlistenFn | undefined;
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
        const uMatch = await listen<SearchMatch>("search-match", (event) => {
          if (isCancelled || isCancellingRef.current) return;
          resultsBufferRef.current.push(event.payload);

          if (!flushTimerRef.current) {
            flushTimerRef.current = window.setTimeout(() => {
              if (isCancelled || isCancellingRef.current) return;
              const buffered = resultsBufferRef.current;
              resultsBufferRef.current = [];
              appendResultsSafely(buffered);
              flushTimerRef.current = null;
            }, 50);
          }
        });

        const uProg = await listen<ScanProgress>("scan-progress", (event) => {
          if (isCancelled) return;

          // 中断要求後の遅延 Scanning イベントは破棄して状態の巻き戻りを防止
          if (isCancellingRef.current && event.payload.state === "Scanning") {
            console.log("[useSearch] 中断要求後の遅延Scanningイベントをスキップ");
            return;
          }

          console.log("[useSearch] 進捗通知受信:", event.payload);
          setProgress(event.payload);

          if (event.payload.state === "Cancelled") {
            isCancellingRef.current = false;
          }

          // スキャン完了または中断時にバッファを強制フラッシュ
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

        if (isCancelled) {
          uMatch();
          uProg();
        } else {
          unlistenMatch = uMatch;
          unlistenProg = uProg;
          console.log("[useSearch] Tauri イベントリスナー登録完了");
        }
      } catch (err) {
        console.error("[useSearch] イベントリスナー登録エラー:", err);
      }
    };

    setupListeners();

    return () => {
      isCancelled = true;
      if (unlistenMatch) unlistenMatch();
      if (unlistenProg) unlistenProg();
      if (flushTimerRef.current) {
        clearTimeout(flushTimerRef.current);
        flushTimerRef.current = null;
      }
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
    if (!query.keyword.trim()) {
      options?.onShowToast?.("検索キーワードを入力してください");
      return;
    }
    if (!query.target_dir.trim()) {
      options?.onShowToast?.("検索対象フォルダを選択してください");
      return;
    }

    console.log("[useSearch] 検索リクエスト送信:", query);
    isCancellingRef.current = false;
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
      current_file: "スキャン開始準備中...",
      elapsed_ms: 0,
    });

    try {
      await invoke("start_search", { query });
    } catch (err) {
      console.error("[useSearch] 検索開始エラー:", err);
      const errMsg = String(err);
      options?.onShowToast?.(`検索開始エラー: ${errMsg}`);
      setProgress({
        state: "Error",
        scanned_files: 0,
        total_files: 0,
        matches_found: 0,
        current_file: `エラー: ${errMsg}`,
        elapsed_ms: 0,
      });
    }
  }, [query, options]);

  // 検索中断
  const cancelSearch = useCallback(async () => {
    try {
      console.log("[useSearch] 検索中断を要求");
      isCancellingRef.current = true;

      // ユーザーへの即時フィードバック: 中断状態へ切り替えてボタンを即座にSEARCHに戻す
      setProgress((prev) =>
        prev
          ? {
              ...prev,
              state: "Cancelled",
              current_file: "スキャンが中断されました",
            }
          : {
              state: "Cancelled",
              scanned_files: 0,
              total_files: 0,
              matches_found: 0,
              current_file: "スキャンが中断されました",
              elapsed_ms: 0,
            }
      );

      // バッファに残っている結果を即時フラッシュ
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

      await invoke("cancel_search");
    } catch (err) {
      console.error("[useSearch] 検索中断エラー:", err);
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
