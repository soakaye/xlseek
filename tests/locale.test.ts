import { beforeEach, describe, expect, it } from "vitest";
import {
  getLanguagePreference,
  isLanguagePreference,
  readLanguagePreference,
  resolveLanguage,
  saveLanguagePreference,
} from "../src/locale-core";
import { retranslateMessage, resolveTranslation, translate, UI_MESSAGES } from "../src/constants";

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

  it("falls back per message to English", () => {
    expect(translate("en", "APP_TITLE")).toBe("Excel Grep");
    expect(resolveTranslation("ja", "missing", {}, { missing: "English fallback" })).toBe("English fallback");
  });

  it("defines English text for every UI message", async () => {
    const missing = Object.keys(UI_MESSAGES).filter((key) => translate("en", key) === key);
    expect(missing).toEqual([]);
  });

  it("retranslates an open toast and preserves interpolated user data", () => {
    const original = `${translate("ja", "EXPORT_SAVED_PREFIX")}/tmp/結果.xlsx`;
    expect(retranslateMessage(original, "ja", "en")).toBe("Saved file: /tmp/結果.xlsx");
  });
});
