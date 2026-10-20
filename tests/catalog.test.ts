/**
 * Copyright (c) 2026 soakaye
 *
 * Description: Tests translation catalog consistency and completeness between English and Japanese.
 * Arguments & Returns: Vitest runs test suites; no public arguments or return values.
 * Errors: Test assertions fail if translation keys or structures mismatch.
 */

import { describe, expect, it } from "vitest";
import { parse } from "yaml";
import enSource from "../src-tauri/locales/en.yml?raw";
import jaSource from "../src-tauri/locales/ja.yml?raw";

const english = parse(enSource) as Record<string, string>;
const japanese = parse(jaSource) as Record<string, string>;

describe("shared localization catalogs", () => {
  it("provides an English fallback for each Japanese message", () => {
    const missing = Object.keys(japanese).filter((key) => key !== "_version" && typeof english[key] !== "string");
    expect(missing).toEqual([]);
    expect(english["common.translationUnavailable"]).toBeTruthy();
  });

  it("defines all Word document search localization keys in English and Japanese", () => {
    const wordKeys = [
      "ui.OPTION_ENABLE_WORD_SEARCH",
      "ui.OPTION_ENABLE_WORD_SEARCH_DESC",
      "ui.MATCH_TYPE_WORD_TEXT",
      "ui.RESULT_MATCH_TYPE_WORD_TEXT",
      "export.match.wordText",
    ];

    for (const key of wordKeys) {
      expect(english[key], `Missing English key: ${key}`).toBeTruthy();
      expect(japanese[key], `Missing Japanese key: ${key}`).toBeTruthy();
    }
  });

  it("defines all ignore pattern localization keys in English and Japanese", () => {
    const ignoreKeys = [
      "ui.IGNORE_PATTERNS_LABEL",
      "ui.IGNORE_PATTERNS_PLACEHOLDER",
      "ui.IGNORE_PATTERNS_TOOLTIP",
      "ui.IGNORE_PATTERNS_CLEAR",
      "ui.IGNORE_HISTORY_BUTTON",
      "ui.SETTINGS_DEFAULT_IGNORE_PATTERNS_LABEL",
      "ui.SETTINGS_DEFAULT_IGNORE_PATTERNS_HELP",
    ];

    for (const key of ignoreKeys) {
      expect(english[key], `Missing English key: ${key}`).toBeTruthy();
      expect(japanese[key], `Missing Japanese key: ${key}`).toBeTruthy();
    }
  });

  it("defines all LLM context copy localization keys in English and Japanese", () => {
    const llmKeys = [
      "ui.COPY_FOR_LLM",
      "ui.COPY_FOR_LLM_BTN",
      "ui.COPY_FOR_LLM_TABLE",
      "ui.COPY_FOR_LLM_HEADER_ONLY",
      "ui.COPY_FOR_LLM_WORD_HEADING",
      "ui.COPY_FOR_LLM_WORD_PARAGRAPH",
      "ui.COPIED_FOR_LLM_SUCCESS",
      "ui.SETTINGS_LLM_SECTION",
      "ui.SETTINGS_LLM_WINDOW_SIZE",
      "ui.SETTINGS_LLM_WINDOW_SIZE_DESC",
      "ui.SETTINGS_LLM_DEFAULT_COPY_TYPE",
      "ui.SETTINGS_LLM_WORD_EXTRACTION_MODE",
    ];

    for (const key of llmKeys) {
      expect(english[key], `Missing English key: ${key}`).toBeTruthy();
      expect(japanese[key], `Missing Japanese key: ${key}`).toBeTruthy();
    }
  });

  it("uses matching interpolation placeholders in each locale", () => {
    const placeholders = (message: string) => [...message.matchAll(/\{([A-Za-z0-9_]+)\}/g)].map((match) => match[1]).sort();
    const mismatched = Object.keys(japanese)
      .filter((key) => key !== "_version" && placeholders(japanese[key]).join("\0") !== placeholders(english[key]).join("\0"));
    expect(mismatched).toEqual([]);
  });
});
