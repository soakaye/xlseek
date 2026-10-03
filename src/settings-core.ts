/**
 * Copyright (c) 2026 soakaye
 *
 * Description: Loads, validates, and atomically replaces the persisted search history and saved preferences.
 * Arguments & Returns: Accepts candidate settings and optional error callback; returns normalized settings or save success.
 * Errors: Storage, serialization, and malformed JSON failures are caught and reported through the callback.
 */
import { DEFAULT_OPTIONS_STORAGE_KEY, LEGACY_DEFAULT_OPTIONS_STORAGE_KEY, SEARCH_HISTORY_CONSTANTS, DIRECTORY_SEARCH_CONSTANTS } from "./constants";
import { isDefaultSearchOptions, normalizeDefaultSearchOptions, resetDefaultSearchOptions } from "./default-options-core";
import { DefaultSearchOptions } from "./types/defaultOptions";

/**
 * Description: Represents the one persisted record for history and search preferences.
 * Arguments & Returns: Contains history fields, default options, and the remembered manual worker count.
 * Errors: Values from storage remain untrusted and must pass normalization.
 */
export interface SavedSettings {
  maxEntries: number;
  keywords: string[];
  directories: string[];
  defaultOptions: DefaultSearchOptions;
  rememberedBurstWorkers: number | null;
}

/**
 * Description: Creates safe initial settings with empty history and standard preferences.
 * Arguments & Returns: Takes no arguments; returns a fresh SavedSettings object.
 * Errors: None.
 */
function initialSettings(): SavedSettings {
  return {
    maxEntries: SEARCH_HISTORY_CONSTANTS.DEFAULT_MAX_ENTRIES,
    keywords: [],
    directories: [],
    defaultOptions: resetDefaultSearchOptions(),
    rememberedBurstWorkers: null,
  };
}

/**
 * Description: Validates an integer history limit against the configured range.
 * Arguments & Returns: Accepts unknown limit; returns whether it is an allowed integer.
 * Errors: Returns false for invalid values without throwing.
 */
function validLimit(value: unknown): value is number {
  return Number.isInteger(value) && (value as number) >= SEARCH_HISTORY_CONSTANTS.MIN_ENTRIES &&
    (value as number) <= SEARCH_HISTORY_CONSTANTS.MAX_ENTRIES;
}

/**
 * Description: Filters each history list to unique, nonblank strings and enforces its current limit.
 * Arguments & Returns: Accepts unknown entries and a validated limit; returns normalized string array.
 * Errors: Invalid list values become an empty list.
 */
function normalizeEntries(value: unknown, limit: number): string[] {
  if (!Array.isArray(value)) return [];
  const entries = value.filter((entry): entry is string => typeof entry === "string" && Boolean(entry.trim()));
  return [...new Set(entries)].slice(0, limit);
}

/**
 * Description: Loads the combined settings record with backward-compatible fallback to legacy options.
 * Arguments & Returns: Accepts optional storage failure callback; returns normalized SavedSettings.
 * Errors: Read and JSON errors are caught, reported, and replaced with initial settings without writing storage.
 */
export function loadSavedSettings(onStorageError?: () => void): SavedSettings {
  try {
    // Constant reference: SEARCH_HISTORY_CONSTANTS.STORAGE_KEY is the single settings record.
    let raw = window.localStorage.getItem(SEARCH_HISTORY_CONSTANTS.STORAGE_KEY);
    if (!raw) {
      for (const legacyKey of SEARCH_HISTORY_CONSTANTS.LEGACY_STORAGE_KEYS) {
        const legacyVal = window.localStorage.getItem(legacyKey);
        if (legacyVal) {
          raw = legacyVal;
          window.localStorage.setItem(SEARCH_HISTORY_CONSTANTS.STORAGE_KEY, legacyVal);
          break;
        }
      }
    }
    if (!raw) {
      const legacyRaw = window.localStorage.getItem(DEFAULT_OPTIONS_STORAGE_KEY) ??
        window.localStorage.getItem(LEGACY_DEFAULT_OPTIONS_STORAGE_KEY);
      const legacyOptions = legacyRaw ? normalizeDefaultSearchOptions(JSON.parse(legacyRaw)) : resetDefaultSearchOptions();
      return { ...initialSettings(), defaultOptions: legacyOptions, rememberedBurstWorkers: legacyOptions.burst_workers };
    }

    const parsed: unknown = JSON.parse(raw);
    if (!parsed || typeof parsed !== "object" || Array.isArray(parsed)) return initialSettings();
    const record = parsed as Record<string, unknown>;
    if (!validLimit(record.maxEntries)) return initialSettings();
    const maxEntries = record.maxEntries;
    const hasEmbeddedOptions = Object.prototype.hasOwnProperty.call(record, "defaultOptions");
    const legacyRaw = hasEmbeddedOptions ? null : window.localStorage.getItem(DEFAULT_OPTIONS_STORAGE_KEY);
    const legacyOptions = legacyRaw ? normalizeDefaultSearchOptions(JSON.parse(legacyRaw)) : resetDefaultSearchOptions();
    const defaultOptions = hasEmbeddedOptions
      ? normalizeDefaultSearchOptions(record.defaultOptions)
      : legacyOptions;
    const remembered = record.rememberedBurstWorkers;
    const validRemembered = typeof remembered === "number" && Number.isInteger(remembered) &&
      remembered >= DIRECTORY_SEARCH_CONSTANTS.CUSTOM_WORKERS_MIN && remembered <= DIRECTORY_SEARCH_CONSTANTS.CUSTOM_WORKERS_MAX;
    return {
      maxEntries,
      keywords: normalizeEntries(record.keywords, maxEntries),
      directories: normalizeEntries(record.directories, maxEntries),
      defaultOptions,
      rememberedBurstWorkers: validRemembered ? remembered : defaultOptions.burst_workers,
    };
  } catch {
    onStorageError?.();
    return initialSettings();
  }
}

/**
 * Description: Checks that a history collection contains unique nonblank text entries.
 * Arguments & Returns: Accepts unknown data; returns a string-array type guard.
 * Errors: Invalid arrays and entries return false without throwing.
 */
function isValidEntries(entries: unknown): entries is string[] {
  return Array.isArray(entries) && entries.every((entry) => typeof entry === "string" && Boolean(entry.trim())) &&
    new Set(entries).size === entries.length;
}

/**
 * Description: Replaces the single persisted settings record with a fully formed candidate.
 * Arguments & Returns: Accepts SavedSettings candidate; returns true only after localStorage accepts the write.
 * Errors: Invalid shape, serialization, and storage errors return false without publishing state.
 */
export function saveSavedSettings(settings: SavedSettings): boolean {
  const validRemembered = settings.rememberedBurstWorkers === null || (
    typeof settings.rememberedBurstWorkers === "number" && Number.isInteger(settings.rememberedBurstWorkers) &&
    settings.rememberedBurstWorkers >= DIRECTORY_SEARCH_CONSTANTS.CUSTOM_WORKERS_MIN &&
    settings.rememberedBurstWorkers <= DIRECTORY_SEARCH_CONSTANTS.CUSTOM_WORKERS_MAX
  );
  if (!validLimit(settings.maxEntries) || !isValidEntries(settings.keywords) || !isValidEntries(settings.directories) ||
    settings.keywords.length > settings.maxEntries || settings.directories.length > settings.maxEntries ||
    !isDefaultSearchOptions(settings.defaultOptions) || !validRemembered) return false;
  try {
    // Constant reference: SEARCH_HISTORY_CONSTANTS.STORAGE_KEY stores settings and history together.
    window.localStorage.setItem(SEARCH_HISTORY_CONSTANTS.STORAGE_KEY, JSON.stringify(settings));
    return true;
  } catch {
    return false;
  }
}
