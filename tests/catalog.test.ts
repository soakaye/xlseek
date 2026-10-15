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

  it("uses matching interpolation placeholders in each locale", () => {
    const placeholders = (message: string) => [...message.matchAll(/\{([A-Za-z0-9_]+)\}/g)].map((match) => match[1]).sort();
    const mismatched = Object.keys(japanese)
      .filter((key) => key !== "_version" && placeholders(japanese[key]).join("\0") !== placeholders(english[key]).join("\0"));
    expect(mismatched).toEqual([]);
  });
});
