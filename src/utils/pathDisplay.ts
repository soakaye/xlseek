/**
 * Copyright (c) 2026 soakaye
 *
 * @fileoverview Path normalization utility for user-facing UI display (src/utils/pathDisplay.ts)
 *
 * ## Description
 * Provides functions to format file paths for presentation in UI components (such as preview headers and clipboard copies).
 * Automatically removes Windows verbatim extended-length prefixes (`\\?\` and `//?/`) for local drive paths,
 * and converts verbatim UNC paths (`\\?\UNC\` and `//?/UNC/`) to standard UNC representations (`\\`).
 * Complies with Constitution Principle I (English documentation), Principle II (centralized constants),
 * and Principle III (comprehensive documentation).
 */

import { PATH_DISPLAY_CONSTANTS } from "../constants";

/**
 * ## Description
 * Normalizes a file path string for user-facing presentation.
 * If the path contains a Windows extended-length (verbatim) prefix:
 * - Local drive paths such as `\\?\C:\path` or `//?/C:/path` have the `\\?\` or `//?/` prefix stripped to yield `C:\path` or `C:/path`.
 * - Verbatim UNC paths such as `\\?\UNC\server\share` or `//?/UNC/server/share` have the prefix converted to standard UNC format (`\\server\share`).
 * Standard local paths, POSIX paths, relative paths, and empty strings are returned unchanged.
 *
 * ## Arguments
 * @param path - The raw file path string to format
 *
 * ## Returns
 * @returns Clean, user-friendly display path string
 *
 * ## Errors / Exceptions
 * Does not throw. Empty, undefined, or non-string inputs safely return empty string or original value.
 */
export function normalizeDisplayPath(path: string): string {
  if (!path) {
    return "";
  }

  // Check for verbatim UNC path prefix (e.g. "\\?\UNC\" or "//?/UNC/")
  // Constant reference: PATH_DISPLAY_CONSTANTS.VERBATIM_UNC_PREFIX_BACKSLASH
  if (path.startsWith(PATH_DISPLAY_CONSTANTS.VERBATIM_UNC_PREFIX_BACKSLASH)) {
    // Constant reference: PATH_DISPLAY_CONSTANTS.STANDARD_UNC_PREFIX
    return `${PATH_DISPLAY_CONSTANTS.STANDARD_UNC_PREFIX}${path.slice(PATH_DISPLAY_CONSTANTS.VERBATIM_UNC_PREFIX_BACKSLASH.length)}`;
  }

  // Constant reference: PATH_DISPLAY_CONSTANTS.VERBATIM_UNC_PREFIX_SLASH
  if (path.startsWith(PATH_DISPLAY_CONSTANTS.VERBATIM_UNC_PREFIX_SLASH)) {
    // Constant reference: PATH_DISPLAY_CONSTANTS.STANDARD_UNC_PREFIX
    return `${PATH_DISPLAY_CONSTANTS.STANDARD_UNC_PREFIX}${path.slice(PATH_DISPLAY_CONSTANTS.VERBATIM_UNC_PREFIX_SLASH.length)}`;
  }

  // Also check case-insensitive UNC prefix (e.g. "\\?\unc\" or "//?/unc/")
  const upper = path.toUpperCase();
  if (upper.startsWith(PATH_DISPLAY_CONSTANTS.VERBATIM_UNC_PREFIX_BACKSLASH)) {
    // Constant reference: PATH_DISPLAY_CONSTANTS.STANDARD_UNC_PREFIX
    return `${PATH_DISPLAY_CONSTANTS.STANDARD_UNC_PREFIX}${path.slice(PATH_DISPLAY_CONSTANTS.VERBATIM_UNC_PREFIX_BACKSLASH.length)}`;
  }
  if (upper.startsWith(PATH_DISPLAY_CONSTANTS.VERBATIM_UNC_PREFIX_SLASH)) {
    // Constant reference: PATH_DISPLAY_CONSTANTS.STANDARD_UNC_PREFIX
    return `${PATH_DISPLAY_CONSTANTS.STANDARD_UNC_PREFIX}${path.slice(PATH_DISPLAY_CONSTANTS.VERBATIM_UNC_PREFIX_SLASH.length)}`;
  }

  // Check for verbatim local drive path prefix (e.g. "\\?\" or "//?/")
  // Constant reference: PATH_DISPLAY_CONSTANTS.VERBATIM_PREFIX_BACKSLASH
  if (path.startsWith(PATH_DISPLAY_CONSTANTS.VERBATIM_PREFIX_BACKSLASH)) {
    return path.slice(PATH_DISPLAY_CONSTANTS.VERBATIM_PREFIX_BACKSLASH.length);
  }

  // Constant reference: PATH_DISPLAY_CONSTANTS.VERBATIM_PREFIX_SLASH
  if (path.startsWith(PATH_DISPLAY_CONSTANTS.VERBATIM_PREFIX_SLASH)) {
    return path.slice(PATH_DISPLAY_CONSTANTS.VERBATIM_PREFIX_SLASH.length);
  }

  return path;
}
