/**
 * @fileoverview Path tokenizer and multi-path helpers (src/utils/pathTokenizer.ts)
 *
 * ## Description
 * Provides functions to tokenize comma-delimited path strings with support for double-quoted segments,
 * append newly selected/dropped paths with automated quote escaping, and extract the active path segment under cursor.
 * Complies with Constitution Principle I (English documentation) and Principle II (Centralized constants).
 */

import { MULTI_PATH_CONSTANTS } from "../constants";

/**
 * ## Description
 * Splits a comma-separated path string into discrete path segments,
 * respecting double-quoted substrings that contain spaces or commas.
 *
 * ## Arguments
 * @param input - The raw search path input string
 *
 * ## Returns
 * @returns Array of unquoted, trimmed path segment strings
 *
 * ## Errors / Exceptions
 * Does not throw. Malformed quotes or empty inputs return safe fallback arrays.
 */
export function splitSearchPaths(input: string): string[] {
  const trimmed = input.trim();
  if (!trimmed) return [];

  const results: string[] = [];
  let current = "";
  let inQuotes = false;

  for (let i = 0; i < trimmed.length; i++) {
    const char = trimmed[i];

    if (char === MULTI_PATH_CONSTANTS.QUOTE) {
      if (inQuotes) {
        // Escaped quote: ""
        if (i + 1 < trimmed.length && trimmed[i + 1] === MULTI_PATH_CONSTANTS.QUOTE) {
          current += MULTI_PATH_CONSTANTS.QUOTE;
          i++; // skip second quote
        } else {
          inQuotes = false;
        }
      } else {
        inQuotes = true;
      }
    } else if (char === "\\" && inQuotes && i + 1 < trimmed.length && trimmed[i + 1] === MULTI_PATH_CONSTANTS.QUOTE) {
      current += MULTI_PATH_CONSTANTS.QUOTE;
      i++; // skip escaped quote
    } else if (char === MULTI_PATH_CONSTANTS.DELIMITER && !inQuotes) {
      const seg = current.trim();
      if (seg) results.push(seg);
      current = "";
    } else {
      current += char;
    }
  }

  const lastSeg = current.trim();
  if (lastSeg) results.push(lastSeg);

  return results;
}

/**
 * ## Description
 * Appends a new folder path to an existing search path input,
 * wrapping the new path in double quotes if it contains commas or whitespace.
 *
 * ## Arguments
 * @param currentInput - Current value of the search path input field
 * @param newPath - New folder or file path to append
 *
 * ## Returns
 * @returns Combined search path string
 *
 * ## Errors / Exceptions
 * Does not throw. Safe string concatenation.
 */
export function appendSearchPath(currentInput: string, newPath: string): string {
  const trimmedPath = newPath.trim();
  if (!trimmedPath) return currentInput;

  const needsQuotes =
    trimmedPath.includes(MULTI_PATH_CONSTANTS.DELIMITER) ||
    trimmedPath.includes(" ") ||
    trimmedPath.includes("\t");

  const formattedPath = needsQuotes
    ? `${MULTI_PATH_CONSTANTS.QUOTE}${trimmedPath.replace(/"/g, '""')}${MULTI_PATH_CONSTANTS.QUOTE}`
    : trimmedPath;

  const trimmedInput = currentInput.trim();
  if (!trimmedInput) {
    return formattedPath;
  }

  // Remove trailing comma if already present
  const base = trimmedInput.endsWith(MULTI_PATH_CONSTANTS.DELIMITER)
    ? trimmedInput.slice(0, -1).trim()
    : trimmedInput;

  return `${base}${MULTI_PATH_CONSTANTS.DELIMITER_SPACE}${formattedPath}`;
}

/**
 * ## Description
 * Identifies the path segment and character range currently active at the cursor position.
 *
 * ## Arguments
 * @param input - The raw search path input string
 * @param cursorPosition - The numeric cursor index within the input string
 *
 * ## Returns
 * @returns Object with the segment string and its start/end offsets in the original input
 *
 * ## Errors / Exceptions
 * Does not throw. Boundary checks clamp cursor within input length bounds.
 */
export function getActivePathSegment(
  input: string,
  cursorPosition: number
): { segment: string; startIndex: number; endIndex: number } {
  if (!input) {
    return { segment: "", startIndex: 0, endIndex: 0 };
  }

  const safeCursor = Math.max(0, Math.min(cursorPosition, input.length));

  // Find boundaries respecting quotes
  let inQuotes = false;
  let currentStart = 0;
  const segments: { start: number; end: number }[] = [];

  for (let i = 0; i < input.length; i++) {
    const char = input[i];
    if (char === MULTI_PATH_CONSTANTS.QUOTE) {
      if (inQuotes && i + 1 < input.length && input[i + 1] === MULTI_PATH_CONSTANTS.QUOTE) {
        i++;
      } else {
        inQuotes = !inQuotes;
      }
    } else if (char === MULTI_PATH_CONSTANTS.DELIMITER && !inQuotes) {
      segments.push({ start: currentStart, end: i });
      currentStart = i + 1;
    }
  }
  segments.push({ start: currentStart, end: input.length });

  // Locate the segment enclosing or nearest to safeCursor
  for (const seg of segments) {
    if (safeCursor >= seg.start && safeCursor <= seg.end) {
      const raw = input.slice(seg.start, seg.end);
      const leadingSpaces = raw.length - raw.trimStart().length;
      const trailingSpaces = raw.length - raw.trimEnd().length;
      return {
        segment: raw.trim(),
        startIndex: seg.start + leadingSpaces,
        endIndex: seg.end - trailingSpaces,
      };
    }
  }

  return { segment: input.trim(), startIndex: 0, endIndex: input.length };
}
