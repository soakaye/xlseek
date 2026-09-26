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

  it("uses matching interpolation placeholders in each locale", () => {
    const placeholders = (message: string) => [...message.matchAll(/\{([A-Za-z0-9_]+)\}/g)].map((match) => match[1]).sort();
    const mismatched = Object.keys(japanese)
      .filter((key) => key !== "_version" && placeholders(japanese[key]).join("\0") !== placeholders(english[key]).join("\0"));
    expect(mismatched).toEqual([]);
  });
});
