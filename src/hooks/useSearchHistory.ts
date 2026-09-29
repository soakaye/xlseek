/**
 * Description: Validates and persists search query and directory path history to local storage.
 * Arguments & Returns: Accepts an optional storage failure callback; returns history state and update operations.
 * Errors: Storage read/write errors trigger the callback while keeping in-memory history active.
 */
import { useEffect, useRef, useState } from "react";
import { SEARCH_HISTORY_CONSTANTS } from "../constants";

/**
 * Description: Represents the structure of persisted search history.
 * Arguments & Returns: Holds maxEntries, keywords, and directories.
 * Errors: None. Validation is performed by isSearchHistory.
 */
export interface SearchHistory {
  maxEntries: number;
  keywords: string[];
  directories: string[];
}

/**
 * Description: Verifies whether an unknown value conforms to the SearchHistory structure.
 * Arguments & Returns: Accepts unknown value; returns boolean type guard.
 * Errors: Returns false on invalid input without throwing exceptions.
 */
function isSearchHistory(value: unknown): value is SearchHistory {
  if (!value || typeof value !== "object") return false;
  const candidate = value as Partial<SearchHistory>;
  // Constant reference: Validate entry limit against SEARCH_HISTORY_CONSTANTS range.
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
 * Description: Loads history from localStorage, falling back to initial defaults on missing or corrupted data.
 * Arguments & Returns: Accepts an optional storage error callback; returns validated SearchHistory.
 * Errors: Storage read or JSON parsing failures trigger the callback and return default initial state.
 */
function loadHistory(onStorageError?: () => void): SearchHistory {
  const initial: SearchHistory = {
    maxEntries: SEARCH_HISTORY_CONSTANTS.DEFAULT_MAX_ENTRIES,
    keywords: [],
    directories: [],
  };
  try {
    // Constant reference: SEARCH_HISTORY_CONSTANTS.STORAGE_KEY
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
 * Description: Provides history loading, addition, and limit reconfiguration, persisted to local storage.
 * Arguments & Returns: Accepts storage error callback; returns history state and mutation operations.
 * Errors: Storage read/write errors trigger the callback; in-memory state remains operational.
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

  /** Description: Commits history state to memory and localStorage. Arguments & Returns: Accepts SearchHistory, returns void. Errors: Calls onStorageError on write error. */
  const commit = (next: SearchHistory) => {
    historyRef.current = next;
    setHistory(next);
    try {
      // Constant reference: SEARCH_HISTORY_CONSTANTS.STORAGE_KEY
      window.localStorage.setItem(SEARCH_HISTORY_CONSTANTS.STORAGE_KEY, JSON.stringify(next));
    } catch {
      onStorageError?.();
    }
  };

  /** Description: Records a successful search to history. Arguments & Returns: Accepts keyword and directory, returns void. Errors: Storage failure notified via commit. */
  const addSearch = (keyword: string, directory: string) => {
    const current = historyRef.current;
    if (current.maxEntries === SEARCH_HISTORY_CONSTANTS.MIN_ENTRIES) return;
    /** Description: Removes whitespace-only values, deduplicates, and places latest at front. Arguments & Returns: Array and value, returns trimmed array. Errors: None. */
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

  /** Description: Validates and applies new max entry limit, trimming entries immediately. Arguments & Returns: Limit number, returns boolean success. Errors: Returns false on invalid input. */
  const setMaxEntries = (maxEntries: number): boolean => {
    // Constant reference: Validates range against SEARCH_HISTORY_CONSTANTS
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
