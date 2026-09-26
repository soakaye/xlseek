import { beforeEach, describe, expect, it } from "vitest";
import {
  getLanguagePreference,
  isLanguagePreference,
  readLanguagePreference,
  resolveLanguage,
  saveLanguagePreference,
} from "../src/locale-core";
import { resolveTranslation } from "../src/i18n";

describe("locale selection", () => {
  beforeEach(() => {
    const values = new Map<string, string>();
    Object.defineProperty(window, "localStorage", {
      configurable: true,
      value: {
        getItem: (key: string) => values.get(key) ?? null,
        setItem: (key: string, value: string) => values.set(key, value),
        clear: () => values.clear(),
      } as Storage,
    });
  });
  it("uses Japanese for ja tags and English otherwise", () => {
    expect(resolveLanguage("ja")).toBe("ja");
    expect(resolveLanguage("ja-JP")).toBe("ja");
    expect(resolveLanguage("fr-FR")).toBe("en");
    expect(resolveLanguage(null)).toBe("en");
  });

  it("accepts only supported stored preferences", () => {
    expect(isLanguagePreference("default")).toBe(true);
    expect(isLanguagePreference("ja")).toBe(true);
    expect(isLanguagePreference("en")).toBe(true);
    expect(getLanguagePreference("fr")).toBe("default");
  });

  it("persists a preference and falls back to default for invalid data", () => {
    window.localStorage.clear();
    expect(readLanguagePreference()).toBe("default");
    expect(saveLanguagePreference("ja")).toBe(true);
    expect(readLanguagePreference()).toBe("ja");
    window.localStorage.setItem("exlgrep.language", "fr");
    expect(readLanguagePreference()).toBe("default");
  });

  it("reports storage write failures without throwing", () => {
    Object.defineProperty(window, "localStorage", {
      configurable: true,
      value: { setItem: () => { throw new Error("storage unavailable"); } },
    });
    expect(saveLanguagePreference("en")).toBe(false);
  });

  it("falls back per missing message to English and interpolates values", () => {
    const english = {
      "common.translationUnavailable": "Some text could not be translated.",
      "ui.greeting": "Hello, {name}.",
    };
    expect(resolveTranslation("ja", "ui.greeting", "ui.greeting", english, { name: "Ada" }, "ja"))
      .toBe("Hello, Ada.");
    expect(resolveTranslation("ja", "ui.unknown", "ui.unknown", english, {}, "ja"))
      .toBe("Some text could not be translated.");
  });

  it("does not reuse a stale plugin locale after a switch failure", () => {
    const english = { "ui.title": "Search", "common.translationUnavailable": "Some text could not be translated." };
    expect(resolveTranslation("en", "ui.title", "検索", english, {}, "ja")).toBe("Search");
  });
});
