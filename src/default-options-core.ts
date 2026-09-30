/**
 * @fileoverview Default search options core management module (src/default-options-core.ts)
 *
 * ## Description
 * Provides validation, local storage persistence, loading,
 * and resetting to default values for customizable search options.
 * Complies with Constitution Principle II (external constants), Principle III (comprehensive documentation), and Principle V (robust error handling).
 */

import { DEFAULT_SEARCH_OPTIONS, DEFAULT_OPTIONS_STORAGE_KEY, FILE_EXTENSIONS } from "./constants";
import { DefaultSearchOptions } from "./types/defaultOptions";

/**
 * ## Description
 * Type guard that verifies whether an unknown value conforms to `DefaultSearchOptions`.
 *
 * ## Arguments
 * @param value - Unknown candidate value to validate
 *
 * ## Returns
 * @returns `true` if valid, `false` otherwise
 *
 * ## Errors / Exceptions
 * Does not throw exceptions; returns `false` on invalid input.
 */
export function isDefaultSearchOptions(value: unknown): value is DefaultSearchOptions {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    return false;
  }
  const candidate = value as Record<string, unknown>;

  if (
    typeof candidate.match_case !== "boolean" ||
    typeof candidate.use_regex !== "boolean" ||
    typeof candidate.include_formula !== "boolean" ||
    typeof candidate.include_shape !== "boolean" ||
    typeof candidate.include_comment !== "boolean" ||
    typeof candidate.include_hidden !== "boolean"
  ) {
    return false;
  }

  if (!Array.isArray(candidate.extensions) || candidate.extensions.length === 0) {
    return false;
  }

  // Constant reference: FILE_EXTENSIONS.DEFAULT_LIST defines the permitted extensions
  const allowed = new Set<string>(FILE_EXTENSIONS.DEFAULT_LIST);
  const allValid = candidate.extensions.every(
    (ext) => typeof ext === "string" && allowed.has(ext)
  );

  return allValid;
}

/**
 * ## Description
 * Returns a new deep copy of the standard default search options.
 *
 * ## Arguments
 * None
 *
 * ## Returns
 * @returns `DefaultSearchOptions`: Fresh copy of standard default options
 *
 * ## Errors / Exceptions
 * None.
 */
export function resetDefaultSearchOptions(): DefaultSearchOptions {
  // Constant reference: DEFAULT_SEARCH_OPTIONS
  return {
    match_case: DEFAULT_SEARCH_OPTIONS.match_case,
    use_regex: DEFAULT_SEARCH_OPTIONS.use_regex,
    include_formula: DEFAULT_SEARCH_OPTIONS.include_formula,
    include_shape: DEFAULT_SEARCH_OPTIONS.include_shape,
    include_comment: DEFAULT_SEARCH_OPTIONS.include_comment,
    include_hidden: DEFAULT_SEARCH_OPTIONS.include_hidden,
    extensions: [...DEFAULT_SEARCH_OPTIONS.extensions],
  };
}

/**
 * ## Description
 * Loads default search options from local storage.
 * If not found, corrupted, or schema invalid, falls back to standard default options.
 *
 * ## Arguments
 * None
 *
 * ## Returns
 * @returns `DefaultSearchOptions`: Validated default search options
 *
 * ## Errors / Exceptions
 * Catches storage errors without throwing and safely falls back to standard defaults.
 */
export function loadDefaultSearchOptions(): DefaultSearchOptions {
  try {
    // Constant reference: DEFAULT_OPTIONS_STORAGE_KEY
    const raw = typeof window !== "undefined" && window.localStorage
      ? window.localStorage.getItem(DEFAULT_OPTIONS_STORAGE_KEY)
      : null;

    if (!raw) {
      return resetDefaultSearchOptions();
    }

    const parsed: unknown = JSON.parse(raw);
    if (isDefaultSearchOptions(parsed)) {
      return parsed;
    }

    return resetDefaultSearchOptions();
  } catch {
    return resetDefaultSearchOptions();
  }
}

/**
 * ## Description
 * Persists default search options into local storage.
 *
 * ## Arguments
 * @param options - `DefaultSearchOptions` to persist
 *
 * ## Returns
 * @returns `true` if saved successfully, `false` otherwise
 *
 * ## Errors / Exceptions
 * Catches storage exceptions (such as QuotaExceededError) and returns `false` to avoid application crashes.
 */
export function saveDefaultSearchOptions(options: DefaultSearchOptions): boolean {
  if (!isDefaultSearchOptions(options)) {
    return false;
  }
  try {
    if (typeof window !== "undefined" && window.localStorage) {
      // Constant reference: DEFAULT_OPTIONS_STORAGE_KEY
      window.localStorage.setItem(DEFAULT_OPTIONS_STORAGE_KEY, JSON.stringify(options));
      return true;
    }
    return false;
  } catch {
    return false;
  }
}
