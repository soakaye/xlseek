import { describe, expect, it } from "vitest";
import { resolveTranslation } from "../src/i18n";

const englishCatalog = {
  "common.translationUnavailable": "Some text could not be translated.",
  "ui.greeting": "Hello, {name}.",
};

describe("plugin translation fallback", () => {
  it("uses an English catalog value when the plugin returns the key", () => {
    expect(resolveTranslation("ja", "ui.greeting", "ui.greeting", englishCatalog, { name: "Ada" }))
      .toBe("Hello, Ada.");
  });

  it("uses the required English fallback when both locale catalogs miss a key", () => {
    expect(resolveTranslation("en", "ui.missing", "ui.missing", englishCatalog)).toBe(englishCatalog["common.translationUnavailable"]);
  });

  it("keeps toast interpolation values while rendering them in the selected locale", () => {
    const values = { path: "/tmp/結果.csv" };
    const catalogs = {
      ...englishCatalog,
      "ui.EXPORT_CSV_SAVED": "CSV file saved: {path}",
    };
    expect(resolveTranslation("en", "ui.EXPORT_CSV_SAVED", "ui.EXPORT_CSV_SAVED", catalogs, values))
      .toBe("CSV file saved: /tmp/結果.csv");
    expect(resolveTranslation("ja", "ui.EXPORT_CSV_SAVED", "CSVファイルを保存しました: {path}", catalogs, values, "ja"))
      .toBe("CSVファイルを保存しました: /tmp/結果.csv");
  });
});
