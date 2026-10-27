/**
 * Copyright (c) 2026 soakaye
 *
 * @fileoverview File search pattern persistent history store (src/utils/filePatternHistory.ts)
 *
 * ## Description
 * Manages loading, saving, deduplicating, and trimming file search pattern history strings
 * in localStorage. Enforces Most-Recently-Used (MRU) ordering, empty-string filtering,
 * and maximum capacity (20 entries).
 * Complies with Constitution Principle I (English documentation), Principle II (Centralized constants),
 * and Principle III (Comprehensive header comments).
 */

import { FILE_PATTERNS_CONSTANTS } from "../constants";

/**
 * ## Description
 * Loads the saved list of file search pattern history entries from local storage.
 * Safely parses the stored JSON array, prunes empty or whitespace-only entries,
 * and limits the result to the configured maximum entries.
 *
 * ## Arguments
 * None
 *
 * ## Returns
 * @returns Array of saved file search pattern strings (MRU ordered)
 *
 * ## Errors / Exceptions
 * Catches JSON parse or local storage access errors without throwing, returning an empty array.
 */
export function loadFilePatternHistory(): string[] {
  try {
    // Constant reference: FILE_PATTERNS_CONSTANTS.STORAGE_KEY_HISTORY
    const raw = window.localStorage.getItem(FILE_PATTERNS_CONSTANTS.STORAGE_KEY_HISTORY);
    if (!raw || !raw.trim().startsWith("[")) return [];

    const parsed: unknown = JSON.parse(raw);
    if (!Array.isArray(parsed)) return [];

    const validEntries = parsed
      .filter((entry): entry is string => typeof entry === "string" && entry.trim().length > 0)
      .map((entry) => entry.trim());

    // Constant reference: FILE_PATTERNS_CONSTANTS.MAX_HISTORY_ENTRIES
    return [...new Set(validEntries)].slice(0, FILE_PATTERNS_CONSTANTS.MAX_HISTORY_ENTRIES);
  } catch (error) {
    console.error("[filePatternHistory] Failed to load file pattern history from localStorage", error);
    return [];
  }
}

/**
 * ## Description
 * Records a newly executed file search pattern string into persistent local storage history.
 * Trims the input, skips if empty/whitespace-only, dedupes against existing history,
 * prepends to the front (MRU ordering), limits to max entries, and saves to localStorage.
 *
 * ## Arguments
 * @param pattern - Raw pattern string to save into history
 *
 * ## Returns
 * @returns The updated array of saved history strings
 *
 * ## Errors / Exceptions
 * Catches storage quota or access errors without throwing, returning the in-memory array.
 */
export function saveFilePattern(pattern: string): string[] {
  const trimmed = pattern.trim();
  if (!trimmed) {
    return loadFilePatternHistory();
  }

  const existing = loadFilePatternHistory();
  // Filter out any existing case-sensitive duplicate so it is promoted to index 0
  const filtered = existing.filter((item) => item !== trimmed);
  // Constant reference: FILE_PATTERNS_CONSTANTS.MAX_HISTORY_ENTRIES
  const updated = [trimmed, ...filtered].slice(0, FILE_PATTERNS_CONSTANTS.MAX_HISTORY_ENTRIES);

  try {
    // Constant reference: FILE_PATTERNS_CONSTANTS.STORAGE_KEY_HISTORY
    window.localStorage.setItem(
      FILE_PATTERNS_CONSTANTS.STORAGE_KEY_HISTORY,
      JSON.stringify(updated)
    );
  } catch (error) {
    console.error("[filePatternHistory] Failed to persist file pattern history to localStorage", error);
  }

  return updated;
}

/**
 * ## Description
 * Clears all saved file search pattern history entries from local storage.
 *
 * ## Arguments
 * None
 *
 * ## Returns
 * None
 *
 * ## Errors / Exceptions
 * Catches storage access errors without throwing.
 */
export function clearFilePatternHistory(): void {
  try {
    // Constant reference: FILE_PATTERNS_CONSTANTS.STORAGE_KEY_HISTORY
    window.localStorage.removeItem(FILE_PATTERNS_CONSTANTS.STORAGE_KEY_HISTORY);
  } catch (error) {
    console.error("[filePatternHistory] Failed to clear file pattern history from localStorage", error);
  }
}
