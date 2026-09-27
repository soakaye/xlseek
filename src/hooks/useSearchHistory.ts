/**
 * 処理内容: 検索テキストとディレクトリの履歴を検証し、端末内へ保存する。
 * 引数・戻り値: 保存失敗時の通知コールバックを受け、履歴と更新操作を返す。
 * エラー: localStorage の読書き失敗は通知し、メモリ上の検索状態を維持する。
 * 変更履歴: v1.0.0 (2026-09-27, Codex): 検索履歴フックを追加。
 */
import { useEffect, useRef, useState } from "react";
import { SEARCH_HISTORY_CONSTANTS } from "../constants";

/**
 * 処理内容: 保存された検索履歴の形を表す。
 * 引数・戻り値: maxEntries、keywords、directories を保持する。
 * エラー: なし。実データの検証は loadHistory が行う。
 * 変更履歴: v1.0.0 (2026-09-27, Codex): 保存履歴型を追加。
 */
export interface SearchHistory {
  maxEntries: number;
  keywords: string[];
  directories: string[];
}

/**
 * 処理内容: 保存値が履歴契約を満たすか確認する。
 * 引数・戻り値: unknown を受け、妥当なら型ガードとして true を返す。
 * エラー: 不正値では false を返し、例外は送出しない。
 * 変更履歴: v1.0.0 (2026-09-27, Codex): 保存値検証を追加。
 */
function isSearchHistory(value: unknown): value is SearchHistory {
  if (!value || typeof value !== "object") return false;
  const candidate = value as Partial<SearchHistory>;
  // 定数参照: SEARCH_HISTORY_CONSTANTS の件数範囲を検証する。
  if (
    !Number.isInteger(candidate.maxEntries) ||
    (candidate.maxEntries as number) < SEARCH_HISTORY_CONSTANTS.MIN_ENTRIES ||
    (candidate.maxEntries as number) > SEARCH_HISTORY_CONSTANTS.MAX_ENTRIES ||
    !Array.isArray(candidate.keywords) ||
    !Array.isArray(candidate.directories)
  ) return false;

  return [candidate.keywords, candidate.directories].every((entries) => {
    if (!entries || entries.some((entry) => typeof entry !== "string" || !entry.trim())) return false;
    if (new Set(entries).size !== entries.length) return false;
    return entries.length <= (candidate.maxEntries as number);
  });
}

/**
 * 処理内容: localStorage から履歴を読み、存在しない値や不正値は初期状態にする。
 * 引数・戻り値: 保存失敗時の通知関数を受け、検証済み履歴を返す。
 * エラー: 読込・JSON 解析に失敗した場合は通知して初期状態を返す。
 * 変更履歴: v1.0.0 (2026-09-27, Codex): 永続化読込を追加。
 */
function loadHistory(onStorageError?: () => void): SearchHistory {
  const initial: SearchHistory = {
    maxEntries: SEARCH_HISTORY_CONSTANTS.DEFAULT_MAX_ENTRIES,
    keywords: [],
    directories: [],
  };
  try {
    // 定数参照: SEARCH_HISTORY_CONSTANTS.STORAGE_KEY を使用。
    const stored = window.localStorage.getItem(SEARCH_HISTORY_CONSTANTS.STORAGE_KEY);
    if (!stored) return initial;
    const parsed: unknown = JSON.parse(stored);
    return isSearchHistory(parsed) ? parsed : initial;
  } catch {
    onStorageError?.();
    return initial;
  }
}

/**
 * 処理内容: 履歴の読み込み・追加・件数変更を提供し、単一の保存値へ永続化する。
 * 引数・戻り値: 保存失敗通知を受け、履歴状態と検索追加・件数変更操作を返す。
 * エラー: localStorage の読書き失敗時は通知し、状態はメモリ上で利用可能なままにする。
 * 変更履歴: v1.0.0 (2026-09-27, Codex): 検索履歴管理フックを追加。
 */
export function useSearchHistory(onStorageError?: () => void) {
  const readFailed = useRef(false);
  const [history, setHistory] = useState(() => loadHistory(() => { readFailed.current = true; }));
  const historyRef = useRef(history);

  useEffect(() => {
    if (readFailed.current) {
      readFailed.current = false;
      onStorageError?.();
    }
  }, [onStorageError]);

  /** 処理内容: 履歴をメモリと端末保存領域へ反映する。引数・戻り値: SearchHistory を受け void を返す。エラー: 保存失敗時は通知し状態を維持。変更履歴: v1.0.0 (2026-09-27, Codex)。 */
  const commit = (next: SearchHistory) => {
    historyRef.current = next;
    setHistory(next);
    try {
      // 定数参照: SEARCH_HISTORY_CONSTANTS.STORAGE_KEY を使用。
      window.localStorage.setItem(SEARCH_HISTORY_CONSTANTS.STORAGE_KEY, JSON.stringify(next));
    } catch {
      onStorageError?.();
    }
  };

  /** 処理内容: 成功した検索を履歴へ追加する。引数・戻り値: 検索語とディレクトリを受け void を返す。エラー: 永続化失敗は commit が通知。変更履歴: v1.0.0 (2026-09-27, Codex)。 */
  const addSearch = (keyword: string, directory: string) => {
    const current = historyRef.current;
    if (current.maxEntries === SEARCH_HISTORY_CONSTANTS.MIN_ENTRIES) return;
    /** 処理内容: 空白のみを除き、最新値を重複排除して先頭へ置く。引数・戻り値: 文字列配列と値を受け上限付き配列を返す。エラー: なし。変更履歴: v1.0.0 (2026-09-27, Codex)。 */
    const addRecent = (entries: string[], value: string) =>
      value.trim()
        ? [value, ...entries.filter((entry) => entry !== value)].slice(0, current.maxEntries)
        : entries;
    commit({
      ...current,
      keywords: addRecent(current.keywords, keyword),
      directories: addRecent(current.directories, directory),
    });
  };

  /** 処理内容: 両履歴の保存上限を検証し即時切り詰める。引数・戻り値: 件数を受け成功状態を boolean で返す。エラー: 範囲外・非整数は false。変更履歴: v1.0.0 (2026-09-27, Codex)。 */
  const setMaxEntries = (maxEntries: number): boolean => {
    // 定数参照: SEARCH_HISTORY_CONSTANTS の有効範囲を検証する。
    if (
      !Number.isInteger(maxEntries) ||
      maxEntries < SEARCH_HISTORY_CONSTANTS.MIN_ENTRIES ||
      maxEntries > SEARCH_HISTORY_CONSTANTS.MAX_ENTRIES
    ) return false;
    const current = historyRef.current;
    commit({
      maxEntries,
      keywords: maxEntries ? current.keywords.slice(0, maxEntries) : [],
      directories: maxEntries ? current.directories.slice(0, maxEntries) : [],
    });
    return true;
  };

  return { ...history, addSearch, setMaxEntries };
}
