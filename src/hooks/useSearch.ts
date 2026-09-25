/**
 * @fileoverview 検索・プレビュー管理カスタムフック (src/hooks/useSearch.ts)
 *
 * ## 処理内容
 * 検索クエリの管理、Tauriバックエンドへの検索開始・中断コマンド発行、
 * バックグラウンド走査進捗およびヒット結果イベントの購読とバッファリング処理、
 * 選択行に応じたセル周辺プレビューデータの取得・シート切り替えを提供する。
 * 憲章原則I（日本語通知）、原則II（定数参照）、原則III（ヘッダコメント）に準拠。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化および4要素JSDocコメントの付与。
 */

import { useState, useEffect, useCallback, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import {
  SearchQuery,
  SearchMatch,
  ScanProgress,
  CellPreviewData,
} from "../types/search";
import {
  COMMANDS,
  EVENT_NAMES,
  FILE_EXTENSIONS,
  TIMING_CONSTANTS,
  UI_MESSAGES,
} from "../constants";

interface UseSearchOptions {
  onShowToast?: (message: string) => void;
}

/**
 * ## 処理内容
 * 検索機能、進捗同期、およびセルプレビュー管理を行うメインカスタムフック。
 *
 * ## 引数
 * @param options - トースト表示用コールバックを含むオプション設定
 *
 * ## 戻り値
 * @returns 検索状態、クエリ、結果リスト、プレビューデータ、および各種ハンドラ関数
 *
 * ## エラー / 例外発生条件
 * Tauri IPC呼び出し失敗時はエラーをコンソールログおよびトースト通知で処理し、例外は外部へスローしない。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 初版策定 / 憲章準拠。
 */
export function useSearch(options?: UseSearchOptions) {
  // 定数参照: FILE_EXTENSIONS.DEFAULT_LIST を使用
  const [query, setQuery] = useState<SearchQuery>({
    keyword: "",
    target_dir: "",
    match_case: false,
    use_regex: false,
    include_formula: true,
    include_comment: true,
    include_hidden: false,
    extensions: [...FILE_EXTENSIONS.DEFAULT_LIST],
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
        // 定数参照: EVENT_NAMES.SEARCH_MATCH ("search-match") を使用
        const uMatch = await listen<SearchMatch>(EVENT_NAMES.SEARCH_MATCH, (event) => {
          if (isCancelled || isCancellingRef.current) return;
          resultsBufferRef.current.push(event.payload);

          if (!flushTimerRef.current) {
            // 定数参照: TIMING_CONSTANTS.PROGRESS_THROTTLE_MS を使用
            flushTimerRef.current = window.setTimeout(() => {
              if (isCancelled || isCancellingRef.current) return;
              const buffered = resultsBufferRef.current;
              resultsBufferRef.current = [];
              appendResultsSafely(buffered);
              flushTimerRef.current = null;
            }, TIMING_CONSTANTS.PROGRESS_THROTTLE_MS);
          }
        });

        // 定数参照: EVENT_NAMES.SCAN_PROGRESS ("scan-progress") を使用
        const uProg = await listen<ScanProgress>(EVENT_NAMES.SCAN_PROGRESS, (event) => {
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
        // 定数参照: COMMANDS.GET_CELL_PREVIEW を使用
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
      // 定数参照: UI_MESSAGES.KEYWORD_PLACEHOLDER を使用
      options?.onShowToast?.("検索キーワードを入力してください");
      return;
    }
    if (!query.target_dir.trim()) {
      // 定数参照: UI_MESSAGES.SELECT_FOLDER_PROMPT を使用
      options?.onShowToast?.(UI_MESSAGES.SELECT_FOLDER_PROMPT);
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
      // 定数参照: COMMANDS.START_SEARCH を使用
      await invoke(COMMANDS.START_SEARCH, { query });
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

      // 定数参照: COMMANDS.CANCEL_SEARCH を使用
      await invoke(COMMANDS.CANCEL_SEARCH);
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
