/**
 * @fileoverview Default search options core management module (src/default-options-core.ts)
 *
 * ## Description
 * Provides validation, local storage persistence, loading,
 * and resetting to default values for customizable search options.
 * Complies with Constitution Principle II (external constants), Principle III (comprehensive documentation), and Principle V (robust error handling).
 */

import { DEFAULT_SEARCH_OPTIONS, DIRECTORY_SEARCH_CONSTANTS, FILE_EXTENSIONS } from "./constants";
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

  if (!hasValidExistingOptions(candidate)) {
    return false;
  }

  const validMode = candidate.directory_mode === undefined ||
    candidate.directory_mode === DIRECTORY_SEARCH_CONSTANTS.SEQUENTIAL ||
    candidate.directory_mode === DIRECTORY_SEARCH_CONSTANTS.BURST;
  const validWorkers = candidate.burst_workers === undefined ||
    candidate.burst_workers === null ||
    (typeof candidate.burst_workers === "number" &&
      Number.isInteger(candidate.burst_workers) &&
      candidate.burst_workers >= DIRECTORY_SEARCH_CONSTANTS.CUSTOM_WORKERS_MIN &&
      candidate.burst_workers <= DIRECTORY_SEARCH_CONSTANTS.CUSTOM_WORKERS_MAX);

  return validMode && validWorkers;
}

/** Validates persisted search fields that predate the directory search mode. */
function hasValidExistingOptions(candidate: Record<string, unknown>): boolean {
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

  return allValid && new Set(candidate.extensions).size === candidate.extensions.length;
}

/**
 * Description: Normalizes a legacy or partially invalid persisted options object into safe current defaults.
 * Arguments & Returns: Accepts unknown stored data; returns validated DefaultSearchOptions with directory fields normalized.
 * Errors: Invalid core fields or extension lists return standard defaults; malformed objects do not throw.
 */
export function normalizeDefaultSearchOptions(value: unknown): DefaultSearchOptions {
  if (!value || typeof value !== "object" || Array.isArray(value)) return resetDefaultSearchOptions();
  const candidate = value as Record<string, unknown>;
  if (!hasValidExistingOptions(candidate)) return resetDefaultSearchOptions();
  const directoryMode = candidate.directory_mode === DIRECTORY_SEARCH_CONSTANTS.BURST
    ? DIRECTORY_SEARCH_CONSTANTS.BURST
    : DIRECTORY_SEARCH_CONSTANTS.SEQUENTIAL;
  const workers = candidate.burst_workers;
  const validWorkers = typeof workers === "number" && Number.isInteger(workers) &&
    workers >= DIRECTORY_SEARCH_CONSTANTS.CUSTOM_WORKERS_MIN && workers <= DIRECTORY_SEARCH_CONSTANTS.CUSTOM_WORKERS_MAX;
  return {
    ...resetDefaultSearchOptions(),
    ...candidate,
    directory_mode: directoryMode,
    burst_workers: validWorkers ? workers : null,
  } as DefaultSearchOptions;
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
    directory_mode: DEFAULT_SEARCH_OPTIONS.directory_mode,
    burst_workers: DEFAULT_SEARCH_OPTIONS.burst_workers,
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
