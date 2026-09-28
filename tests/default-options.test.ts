import { beforeEach, describe, expect, it } from "vitest";
import {
  isDefaultSearchOptions,
  loadDefaultSearchOptions,
  saveDefaultSearchOptions,
  resetDefaultSearchOptions,
} from "../src/default-options-core";
import { DEFAULT_SEARCH_OPTIONS, DEFAULT_OPTIONS_STORAGE_KEY } from "../src/constants";

describe("default-options-core", () => {
  beforeEach(() => {
    const values = new Map<string, string>();
    Object.defineProperty(window, "localStorage", {
      configurable: true,
      value: {
        getItem: (key: string) => values.get(key) ?? null,
        setItem: (key: string, value: string) => values.set(key, value),
        removeItem: (key: string) => values.delete(key),
        clear: () => values.clear(),
      } as Storage,
    });
  });

  it("returns default values when storage is empty", () => {
    window.localStorage.clear();
    const options = loadDefaultSearchOptions();
    expect(options).toEqual(DEFAULT_SEARCH_OPTIONS);
  });

  it("saves and loads valid custom options", () => {
    const custom = {
      match_case: true,
      use_regex: true,
      include_formula: false,
      include_shape: true,
      include_comment: false,
      include_hidden: true,
      extensions: [".xlsx", ".xls"],
    };
    const saved = saveDefaultSearchOptions(custom);
    expect(saved).toBe(true);

    const loaded = loadDefaultSearchOptions();
    expect(loaded).toEqual(custom);
  });

  it("validates default options schema correctly", () => {
    expect(isDefaultSearchOptions(DEFAULT_SEARCH_OPTIONS)).toBe(true);
    expect(isDefaultSearchOptions(null)).toBe(false);
    expect(isDefaultSearchOptions({})).toBe(false);
    expect(
      isDefaultSearchOptions({
        ...DEFAULT_SEARCH_OPTIONS,
        match_case: "invalid",
      })
    ).toBe(false);
    // 空の拡張子配列は無効
    expect(
      isDefaultSearchOptions({
        ...DEFAULT_SEARCH_OPTIONS,
        extensions: [],
      })
    ).toBe(false);
    // 拡張子以外の不正文字列を含む場合は無効
    expect(
      isDefaultSearchOptions({
        ...DEFAULT_SEARCH_OPTIONS,
        extensions: [".txt"],
      })
    ).toBe(false);
  });

  it("falls back to default values when storage contains corrupted JSON or invalid data", () => {
    window.localStorage.setItem(DEFAULT_OPTIONS_STORAGE_KEY, "invalid-json{");
    expect(loadDefaultSearchOptions()).toEqual(DEFAULT_SEARCH_OPTIONS);

    window.localStorage.setItem(
      DEFAULT_OPTIONS_STORAGE_KEY,
      JSON.stringify({ match_case: "not-bool" })
    );
    expect(loadDefaultSearchOptions()).toEqual(DEFAULT_SEARCH_OPTIONS);
  });

  it("resets options to official defaults and returns the copy", () => {
    const reset = resetDefaultSearchOptions();
    expect(reset).toEqual(DEFAULT_SEARCH_OPTIONS);
  });

  it("gracefully handles localStorage write failures without throwing", () => {
    Object.defineProperty(window, "localStorage", {
      configurable: true,
      value: {
        setItem: () => {
          throw new Error("QuotaExceededError");
        },
        getItem: () => null,
      },
    });
    expect(saveDefaultSearchOptions(DEFAULT_SEARCH_OPTIONS)).toBe(false);
  });
});
