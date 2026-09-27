/**
 * @fileoverview 検索条件入力・実行バーコンポーネント (src/components/search/SearchBar.tsx)
 *
 * ## 処理内容
 * 検索キーワード、対象フォルダ、各種検索オプション（大文字小文字区別、正規表現、数式、コメント、非表示シート）の
 * 入力フォームを提供し、検索の開始・中断アクションをトリガーする。フォルダのドラッグ＆ドロップによる
 * パス自動設定機能もサポートする。
 * 憲章原則I（自然かつ正確な日本語）、原則II（定数の一元化）、原則III（網羅的なヘッダコメント）に準拠。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 初版策定。
 * - v1.1.0 (2026-09-26, AI Agent): 憲章準拠改修。IPCコマンド名およびUI文言を外部定数化、4要素ヘッダコメントを付与。
 * - v1.2.0 (2026-09-26, AI Agent): 検索対象拡張子のトグル選択ボタンを実装。
 * - v1.3.0 (2026-09-26, AI Agent): 検索キーワード入力欄で日本語入力(IME)確定時のEnter誤爆を防止。フォルダ入力欄でのEnter検索は行わず入力中のEnterを無視するよう制御。
 * - v1.4.0 (2026-09-27, Codex): 検索履歴・ディレクトリ補完 UI を追加。
 * - v1.5.0 (2026-09-27, Codex): フォーカス離脱時に履歴・候補を閉じる。
 * - v1.5.1 (2026-09-27, Codex): フォルダ選択ダイアログを開く際にも履歴・候補を閉じる。
 * - v1.6.0 (2026-09-27, Codex): 履歴ボタンと項目のキー移動・循環・取消しを追加。
 * - v1.7.0 (2026-09-27, Codex): 入力欄から下矢印で履歴先頭へ移動できるよう修正。
 * - v1.8.0 (2026-09-27, Codex): クリック後に履歴ボタンへフォーカスを明示する。
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

/**
 * 検索バーコンポーネントのプロパティ定義
 *
 * ## プロパティ一覧
 * - `query`: SearchQuery - 現在の検索条件
 * - `onChangeQuery`: (newQuery: Partial<SearchQuery>) => void - 検索条件変更コールバック
 * - `onSearch`: () => void - 検索実行コールバック
 * - `onCancel`: () => void - 検索中断コールバック
 * - `isScanning`: boolean - 現在スキャン実行中かどうかのフラグ
 */
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
 * 検索バーコンポーネント
 *
 * ## 処理詳細
 * ユーザーからのキーワード入力、フォルダ選択（ダイアログまたはドラッグ＆ドロップ）、検索オプションの変更を受け付け、
 * 検索開始または中断のハンドラを呼び出す。
 *
 * ## 引数
 * - `props`: SearchBarProps - コンポーネントプロパティ
 *
 * ## 戻り値
 * - `React.ReactElement`: 検索バーUI要素
 *
 * ## エラー・例外条件
 * - ドラッグ＆ドロップやフォルダ選択ダイアログのIPC通信失敗時は、コンソールにエラーを出力しUIクラッシュを防止する。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 初版作成。
 * - v1.1.0 (2026-09-26, AI Agent): 憲章原則に準拠し、定数参照と4要素コメントを追加。
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
  const [openList, setOpenList] = useState<"keyword" | "directory" | "directory-history" | null>(null);
  const [directorySuggestions, setDirectorySuggestions] = useState<string[]>([]);
  const [activeOption, setActiveOption] = useState<number | null>(null);
  const [completionDismissed, setCompletionDismissed] = useState(false);
  const [directoryFocused, setDirectoryFocused] = useState(false);
  const completionRequestRef = useRef(0);

  /**
   * ディレクトリ入力の補完候補を取得する。
   * 引数はなく、戻り値は void。IPC 失敗時は候補を空にし、古い要求結果は破棄する。
   * 変更履歴: v1.0.0 (2026-09-27, Codex): 入力補完を追加。
  */
  useEffect(() => {
    const request = ++completionRequestRef.current;
    if (!directoryFocused || completionDismissed || !query.target_dir) return;
    const timer = window.setTimeout(async () => {
      try {
        // 定数参照: COMMANDS.COMPLETE_DIRECTORY_PATH
        const values = await invoke<string[]>(COMMANDS.COMPLETE_DIRECTORY_PATH, { pathInput: query.target_dir });
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

  /**
   * 履歴または補完リストを閉じる。
   * 引数はなく、戻り値は void。例外は発生しない。
   * 変更履歴: v1.0.0 (2026-09-27, Codex): リスト操作を追加。
   */
  const closeList = () => { setOpenList(null); setActiveOption(null); };

  /**
   * 処理内容: 入力欄と対応する履歴・候補の操作領域からフォーカスが外れた時に一覧を閉じる。
   * 引数・戻り値: フォーカスイベントと対象欄を受け、void を返す。
   * エラー: 領域内の項目への移動は閉じず、補完の未完了応答は無効化する。
   * 変更履歴: v1.0.0 (2026-09-27, Codex): フォーカス離脱の処理を追加。
   */
  const handleFieldBlur = (event: React.FocusEvent<HTMLDivElement>, field: "keyword" | "directory") => {
    if (event.relatedTarget instanceof Node && event.currentTarget.contains(event.relatedTarget)) return;
    completionRequestRef.current++;
    if (field === "directory") setDirectoryFocused(false);
    closeList();
  };

  /**
   * 処理内容: フォーカスが移らない領域へのクリックでも一覧と待機中の補完を閉じる。
   * 引数・戻り値: なし。イベント購読の解除関数を返す。
   * エラー: 操作領域内のクリックは無視し、フォーカス状態を維持する。
   * 変更履歴: v1.0.0 (2026-09-27, Codex): 領域外クリックの処理を追加。
   */
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

  /**
   * 表示中リストの項目を選択する。
   * 引数は項目インデックス、戻り値は void。範囲外は何もせず、履歴選択は検索を開始しない。
   * 変更履歴: v1.0.0 (2026-09-27, Codex): リスト選択を追加。
   */
  const selectOption = (index: number) => {
    const values = openList === "keyword" ? history.keywords : openList === "directory-history" ? history.directories : openList === "directory" ? directorySuggestions : [];
    const value = values[index];
    if (!value || !openList) return;
    completionRequestRef.current += 1;
    onSelectHistory(openList === "keyword" ? "keyword" : "directory", value);
    if (openList !== "keyword") setCompletionDismissed(true);
    closeList();
  };

  /**
   * 履歴ボタンからキーボードで最初の項目へ移動する。
   * 引数はキーイベントと対象欄、戻り値は void。履歴が空または未表示なら既定の Tab 動作を保つ。
   * エラー: 対象項目が見つからない場合はフォーカスを変更しない。
   * 変更履歴: v1.0.0 (2026-09-27, Codex): 履歴一覧へのキー移動を追加。
   */
  // 定数参照: KEYBOARD_KEYS.TAB / KEYBOARD_KEYS.ARROW_DOWN
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

  /**
   * 履歴項目の矢印・Tab・Enter・Escape 操作を処理する。
   * 引数はキーイベント、項目位置、対象欄。戻り値は void。選択は既存の selectOption を使う。
   * エラー: IME 変換中、一覧外、範囲外の項目では操作しない。
   * 変更履歴: v1.0.0 (2026-09-27, Codex): 履歴項目のキー操作を追加。
   */
  const handleHistoryOptionKeyDown = (event: KeyboardEvent<HTMLButtonElement>, index: number, field: "keyword" | "directory") => {
    const isOpen = field === "keyword" ? openList === "keyword" : openList === "directory-history";
    if (!isOpen) return;
    // 定数参照: KEYBOARD_KEYS.IME_COMPOSITION / KEYBOARD_KEYS.IME_COMPOSITION_KEY_CODE
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

  /**
   * 開いている候補リストのキーボード操作を処理する。
   * 引数はキーボードイベント、戻り値は処理済みかを示す boolean。リスト外では false。
   * エラー: 空の一覧では既定の移動や確定を行わず false または true を返して安全に終了する。
   * 変更履歴: v1.0.0 (2026-09-27, Codex): アクセシブルなキー操作を追加。
   */
  const handleListKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
    if (!openList) return false;
    const values = openList === "keyword" ? history.keywords : openList === "directory-history" ? history.directories : directorySuggestions;
    // 定数参照: KEYBOARD_KEYS.ESCAPE / ARROW_DOWN / ARROW_UP / ENTER
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

  /** 履歴表示を切り替える。引数は対象欄、戻り値は void。例外は発生しない。変更履歴: v1.0.0 (2026-09-27, Codex)。 */
  const toggleHistory = (field: "keyword" | "directory") => {
    completionRequestRef.current += 1;
    if (field === "directory") setCompletionDismissed(true);
    setOpenList(field === "directory" ? (openList === "directory-history" ? null : "directory-history") : (openList === field ? null : field));
    setActiveOption(null);
  };

  // Tauri ネイティブのドラッグ＆ドロップイベントの購読
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
                  // 定数参照: COMMANDS.RESOLVE_DROPPED_PATH (ドロップパス解決コマンド)
                  const resolvedPath = await invoke<string>(COMMANDS.RESOLVE_DROPPED_PATH, {
                    path: payload.paths[0],
                  });
                  onChangeQuery({ target_dir: resolvedPath });
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
  }, [onChangeQuery]);

  // HTML5 標準のドラッグ＆ドロップ（ブラウザ環境向けフォールバック）
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
      const file = e.dataTransfer.files[0];
      const filePath = (file as File & { path?: string }).path;
      if (filePath) {
        try {
          // 定数参照: COMMANDS.RESOLVE_DROPPED_PATH (ドロップパス解決コマンド)
          const resolvedPath = await invoke<string>(COMMANDS.RESOLVE_DROPPED_PATH, {
            path: filePath,
          });
          onChangeQuery({ target_dir: resolvedPath });
        } catch {
          console.error("[SearchBar] Failed to resolve dropped folder path");
        }
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
    // 日本語入力(IME)確定時のEnterによる誤爆発火を防止 (WebKit/SafariおよびChromium両対応)
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
    // IME変換確定時はIME側の処理に任せ、入力中のEnterキー押下は無視（検索実行は行わない）
    if (e.nativeEvent.isComposing || e.key === "Process" || e.keyCode === 229) {
      return;
    }
    if (handleListKeyDown(e)) return;
    if (e.key === "Enter") {
      e.preventDefault();
    }
  };

  /**
   * 処理内容: 履歴・補完を閉じてフォルダ選択ダイアログを開き、選択されたパスを反映する。
   * 引数・戻り値: 引数なし。Promise<void> を返す。
   * エラー: ダイアログの失敗はログへ記録し、キャンセル時は検索条件を変更しない。
   * 変更履歴: v1.1.0 (2026-09-27, Codex): ダイアログ表示前に一覧と補完要求を閉じる。
   */
  const handleBrowseFolder = async () => {
    completionRequestRef.current++;
    setCompletionDismissed(true);
    setDirectoryFocused(false);
    closeList();
    try {
      // 定数参照: t("ui.SELECT_FOLDER_DIALOG_TITLE") (ダイアログタイトル)
      const selected = await open({
        directory: true,
        multiple: false,
        title: t("ui.SELECT_FOLDER_DIALOG_TITLE"),
      });
      if (selected && typeof selected === "string") {
        onChangeQuery({ target_dir: selected });
      }
    } catch {
      console.error("[SearchBar] Failed to select folder");
    }
  };

  const currentExtensions = query.extensions ?? [...FILE_EXTENSIONS.DEFAULT_LIST];

  const handleToggleExtension = (ext: string) => {
    const isSelected = currentExtensions.includes(ext);
    if (isSelected) {
      // 最低1つの拡張子は選択維持
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
        {/* 上段: 検索キーワード入力 & フォルダ選択 & 検索実行/中断ボタン */}
        <div className="grid grid-cols-1 lg:grid-cols-12 gap-3 items-center">
          {/* 検索キーワード入力 */}
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
              /* 定数参照: t("ui.KEYWORD_INPUT_PLACEHOLDER") */
              placeholder={t("ui.KEYWORD_INPUT_PLACEHOLDER")}
              className="w-full pl-9 pr-24 py-2 bg-[#202024] border border-zinc-700 rounded-lg text-sm text-zinc-100 placeholder-zinc-500 focus:outline-none focus:border-excel-light focus:ring-1 focus:ring-excel-light transition"
            />
            <div className="absolute inset-y-0 right-1 flex items-center gap-1 pr-1.5">
              {query.keyword && (
                <button
                  type="button"
                  onClick={() => onChangeQuery({ keyword: "" })}
                  /* 定数参照: t("ui.CLEAR_TOOLTIP") */
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

          {/* フォルダパス選択 & DnD ドロップゾーン */}
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
              type="text"
              value={query.target_dir}
              onChange={(e) => { setCompletionDismissed(false); setOpenList(null); onChangeQuery({ target_dir: e.target.value }); }}
              onKeyDown={handleFolderKeyDown}
              /* 定数参照: t("ui.FOLDER_DROP_PLACEHOLDER") / FOLDER_INPUT_PLACEHOLDER */
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
                {/* 定数参照: t("ui.FOLDER_DROP_PROMPT") */}
                <span>{t("ui.FOLDER_DROP_PROMPT")}</span>
              </div>
            )}
            <button
              type="button"
              onClick={handleBrowseFolder}
              /* 定数参照: t("ui.SELECT_FOLDER_TOOLTIP") */
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

          {/* 検索開始 / 中断ボタン */}
          <div className="lg:col-span-2">
            {isScanning ? (
              <button
                type="button"
                onClick={onCancel}
                className="w-full py-2 px-4 bg-red-600 hover:bg-red-500 active:scale-[0.99] text-white font-semibold text-sm rounded-lg flex items-center justify-center gap-2 shadow-lg shadow-red-950/40 transition"
              >
                <Square className="w-4 h-4 fill-white" />
                {/* 定数参照: t("ui.BUTTON_CANCEL") */}
                <span>{t("ui.BUTTON_CANCEL")}</span>
              </button>
            ) : (
              <button
                type="button"
                onClick={onSearch}
                className="w-full py-2 px-4 bg-excel hover:bg-excel-hover active:scale-[0.99] text-white font-semibold text-sm rounded-lg flex items-center justify-center gap-2 shadow-lg shadow-emerald-950/40 transition"
              >
                <Search className="w-4 h-4" />
                {/* 定数参照: t("ui.BUTTON_SEARCH") */}
                <span>{t("ui.BUTTON_SEARCH")}</span>
              </button>
            )}
          </div>
        </div>

        {/* 下段: 検索オプション (トグルピル) */}
        <div className="flex flex-wrap items-center justify-between gap-2 pt-1 text-xs select-none">
          <div className="flex flex-wrap items-center gap-2">
            <span className="text-zinc-400 mr-1 font-medium flex items-center gap-1">
              {/* 定数参照: t("ui.OPTIONS_LABEL") */}
              <SlidersHorizontal className="w-3.5 h-3.5" /> {t("ui.OPTIONS_LABEL")}
            </span>

            {/* 大文字/小文字 */}
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
              {/* 定数参照: t("ui.OPTION_MATCH_CASE") */}
              <span>{t("ui.OPTION_MATCH_CASE")}</span>
            </label>

            {/* 正規表現 */}
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
              {/* 定数参照: t("ui.OPTION_USE_REGEX") */}
              <span>{t("ui.OPTION_USE_REGEX")}</span>
            </label>

            {/* 数式 */}
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
              {/* 定数参照: t("ui.OPTION_INCLUDE_FORMULA") */}
              <span>{t("ui.OPTION_INCLUDE_FORMULA")}</span>
            </label>

            {/* Shape 内テキスト */}
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
              {/* 定数参照: SEARCH_LABELS.INCLUDE_SHAPE */}
              <span>{t(SEARCH_LABELS.INCLUDE_SHAPE)}</span>
            </label>

            {/* コメント */}
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
              {/* 定数参照: t("ui.OPTION_INCLUDE_COMMENT") */}
              <span>{t("ui.OPTION_INCLUDE_COMMENT")}</span>
            </label>

            {/* 非表示シート */}
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
              {/* 定数参照: t("ui.OPTION_INCLUDE_HIDDEN") */}
              <span>{t("ui.OPTION_INCLUDE_HIDDEN")}</span>
            </label>
          </div>

          {/* 対象拡張子トグルボタン群 */}
          <div className="flex items-center gap-2 text-zinc-400 text-xs">
            {/* 定数参照: t("ui.LABEL_TARGET_EXTENSIONS") */}
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
