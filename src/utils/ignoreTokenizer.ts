/**
 * Copyright (c) 2026 soakaye
 *
 * @fileoverview Ignore pattern tokenizer and string manipulation utilities (src/utils/ignoreTokenizer.ts)
 *
 * ## Description
 * Provides functions to tokenize comma-delimited ignore pattern strings with support for double-quoted segments,
 * format and join pattern arrays into clean comma-separated strings, and sanitize individual patterns.
 * Complies with Constitution Principle I (English documentation) and Principle II (Centralized constants).
 */

import { IGNORE_PATTERNS_CONSTANTS } from "../constants";

/**
 * ## Description
 * Splits a comma-separated ignore pattern string into discrete pattern tokens,
 * respecting double-quoted substrings that may contain commas or whitespace.
 * Trims surrounding whitespace and quotes from each pattern, dropping empty tokens.
 *
 * ## Arguments
 * @param input - The raw ignore pattern string entered by the user
 *
 * ## Returns
 * @returns Array of unquoted, trimmed ignore pattern strings
 *
 * ## Errors / Exceptions
 * Does not throw. Malformed quotes or empty inputs return safe fallback empty arrays.
 */
export function splitIgnorePatterns(input: string): string[] {
  const trimmed = input.trim();
  if (!trimmed) return [];

  const results: string[] = [];
  let current = "";
  let inQuotes = false;

  for (let i = 0; i < trimmed.length; i++) {
    const char = trimmed[i];

    // Constant reference: IGNORE_PATTERNS_CONSTANTS.QUOTE
    if (char === IGNORE_PATTERNS_CONSTANTS.QUOTE) {
      if (inQuotes) {
        // Escaped quote: ""
        if (i + 1 < trimmed.length && trimmed[i + 1] === IGNORE_PATTERNS_CONSTANTS.QUOTE) {
          current += IGNORE_PATTERNS_CONSTANTS.QUOTE;
          i++;
        } else {
          inQuotes = false;
        }
      } else {
        inQuotes = true;
      }
    } else if (char === "\\" && inQuotes && i + 1 < trimmed.length && trimmed[i + 1] === IGNORE_PATTERNS_CONSTANTS.QUOTE) {
      // Escaped quote: \"
      // Constant reference: IGNORE_PATTERNS_CONSTANTS.QUOTE
      current += IGNORE_PATTERNS_CONSTANTS.QUOTE;
      i++;
    } else if (char === IGNORE_PATTERNS_CONSTANTS.DELIMITER && !inQuotes) {
      // Constant reference: IGNORE_PATTERNS_CONSTANTS.DELIMITER
      const seg = current.trim();
      if (seg) {
        results.push(seg);
      }
      current = "";
    } else {
      current += char;
    }
  }

  const trailing = current.trim();
  if (trailing) {
    results.push(trailing);
  }

  return results;
}

/**
 * ## Description
 * Formats an individual pattern segment, wrapping in double quotes if it contains
 * the delimiter character or leading/trailing whitespace.
 *
 * ## Arguments
 * @param pattern - An unquoted pattern string
 *
 * ## Returns
 * @returns Quoted pattern string if delimiter is present, otherwise trimmed pattern
 *
 * ## Errors / Exceptions
 * Does not throw.
 */
export function formatIgnorePattern(pattern: string): string {
  const trimmed = pattern.trim();
  if (!trimmed) return "";

  // Constant reference: IGNORE_PATTERNS_CONSTANTS.DELIMITER
  if (trimmed.includes(IGNORE_PATTERNS_CONSTANTS.DELIMITER)) {
    // Constant reference: IGNORE_PATTERNS_CONSTANTS.QUOTE
    const escaped = trimmed.split(IGNORE_PATTERNS_CONSTANTS.QUOTE).join('""');
    return `${IGNORE_PATTERNS_CONSTANTS.QUOTE}${escaped}${IGNORE_PATTERNS_CONSTANTS.QUOTE}`;
  }

  return trimmed;
}

/**
 * ## Description
 * Joins an array of ignore pattern segments into a single comma-separated string,
 * quoting segments containing delimiters as necessary.
 *
 * ## Arguments
 * @param patterns - Array of pattern strings
 *
 * ## Returns
 * @returns Comma-separated formatted ignore pattern string
 *
 * ## Errors / Exceptions
 * Does not throw.
 */
export function joinIgnorePatterns(patterns: string[]): string {
  return patterns
    .map(formatIgnorePattern)
    .filter((p) => p.length > 0)
    // Constant reference: IGNORE_PATTERNS_CONSTANTS.DELIMITER_SPACE
    .join(IGNORE_PATTERNS_CONSTANTS.DELIMITER_SPACE);
}
