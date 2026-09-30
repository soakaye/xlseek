/**
 * Description: Validates and persists search query and directory path history to local storage.
 * Arguments & Returns: Accepts an optional storage failure callback; returns history state and update operations.
 * Errors: Storage read/write errors trigger the callback while keeping in-memory history active.
 */
import { useEffect, useRef, useState } from "react";
import { SEARCH_HISTORY_CONSTANTS } from "../constants";
import { DefaultSearchOptions } from "../types/defaultOptions";
import { loadSavedSettings, saveSavedSettings, SavedSettings } from "../settings-core";

/**
 * Description: Provides history loading, addition, and limit reconfiguration, persisted to local storage.
 * Arguments & Returns: Accepts storage error callback; returns history state and mutation operations.
 * Errors: Storage read/write errors trigger the callback; in-memory state remains operational.
 */
export function useSearchHistory(onStorageError?: () => void) {
  const readFailed = useRef(false);
  const [settings, setSettings] = useState(() => loadSavedSettings(() => { readFailed.current = true; }));
  const settingsRef = useRef(settings);

  useEffect(() => {
    if (readFailed.current) {
      readFailed.current = false;
      onStorageError?.();
    }
  }, [onStorageError]);

  /**
   * Description: Publishes accepted-search history immediately and persists it with the committed preferences.
   * Arguments & Returns: Accepts a complete SavedSettings record; returns void.
   * Errors: Storage failures notify the caller while retaining the in-memory record.
   */
  const commitHistory = (next: SavedSettings): void => {
    settingsRef.current = next;
    setSettings(next);
    if (!saveSavedSettings(next)) onStorageError?.();
  };

  /**
   * Description: Records a successful search while retaining every committed preference.
   * Arguments & Returns: Accepts keyword and directory strings; returns void.
   * Errors: Storage failures notify the caller and preserve the in-memory history update.
   */
  const addSearch = (keyword: string, directory: string) => {
    const current = settingsRef.current;
    if (current.maxEntries === SEARCH_HISTORY_CONSTANTS.MIN_ENTRIES) return;
    /** Description: Removes whitespace-only values, deduplicates, and places latest at front. Arguments & Returns: Array and value, returns trimmed array. Errors: None. */
    const addRecent = (entries: string[], value: string) =>
      value.trim()
        ? [value, ...entries.filter((entry) => entry !== value)].slice(0, current.maxEntries)
        : entries;
    commitHistory({
      ...current,
      keywords: addRecent(current.keywords, keyword),
      directories: addRecent(current.directories, directory),
    });
  };

  /**
   * Description: Atomically saves history limit and search defaults using the latest committed histories.
   * Arguments & Returns: Accepts limit, default options, and remembered worker count; returns save success.
   * Errors: Invalid limits or storage failures return false and leave committed memory unchanged.
   */
  const savePreferences = (
    maxEntries: number,
    defaultOptions: DefaultSearchOptions,
    rememberedBurstWorkers: number | null,
  ): boolean => {
    if (!Number.isInteger(maxEntries) || maxEntries < SEARCH_HISTORY_CONSTANTS.MIN_ENTRIES || maxEntries > SEARCH_HISTORY_CONSTANTS.MAX_ENTRIES) return false;
    const current = settingsRef.current;
    const next: SavedSettings = {
      ...current,
      maxEntries,
      keywords: maxEntries ? current.keywords.slice(0, maxEntries) : [],
      directories: maxEntries ? current.directories.slice(0, maxEntries) : [],
      defaultOptions: { ...defaultOptions, extensions: [...defaultOptions.extensions] },
      rememberedBurstWorkers,
    };
    if (!saveSavedSettings(next)) {
      return false;
    }
    settingsRef.current = next;
    setSettings(next);
    return true;
  };

  return { ...settings, addSearch, savePreferences };
}
