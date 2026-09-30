/**
 * Description: Verifies combined settings storage compatibility and failure behavior.
 * Arguments & Returns: Vitest executes storage boundary cases; tests take no arguments or return values.
 * Errors: Assertions fail when loading, normalization, or storage replacement violates the settings contract.
 */
import { beforeEach, describe, expect, it, vi } from "vitest";
import { DEFAULT_OPTIONS_STORAGE_KEY, DEFAULT_SEARCH_OPTIONS, SEARCH_HISTORY_CONSTANTS } from "../src/constants";
import { loadSavedSettings, saveSavedSettings } from "../src/settings-core";

describe("settings-core", () => {
  const values = new Map<string, string>();

  beforeEach(() => {
    values.clear();
    Object.defineProperty(window, "localStorage", {
      configurable: true,
      value: {
        getItem: (key: string) => values.get(key) ?? null,
        setItem: (key: string, value: string) => { values.set(key, value); },
      } as Storage,
    });
  });

  it("loads empty defaults without writing during startup", () => {
    const setItem = vi.spyOn(window.localStorage, "setItem");
    expect(loadSavedSettings()).toEqual({
      maxEntries: SEARCH_HISTORY_CONSTANTS.DEFAULT_MAX_ENTRIES,
      keywords: [],
      directories: [],
      defaultOptions: DEFAULT_SEARCH_OPTIONS,
      rememberedBurstWorkers: null,
    });
    expect(setItem).not.toHaveBeenCalled();
  });

  it("loads legacy history and options, then prefers embedded extended options", () => {
    values.set(SEARCH_HISTORY_CONSTANTS.STORAGE_KEY, JSON.stringify({ maxEntries: 3, keywords: ["a"], directories: ["/a"] }));
    values.set(DEFAULT_OPTIONS_STORAGE_KEY, JSON.stringify({ ...DEFAULT_SEARCH_OPTIONS, match_case: true }));
    expect(loadSavedSettings().defaultOptions.match_case).toBe(true);
    values.set(SEARCH_HISTORY_CONSTANTS.STORAGE_KEY, JSON.stringify({
      maxEntries: 3, keywords: ["a"], directories: ["/a"], defaultOptions: DEFAULT_SEARCH_OPTIONS,
      rememberedBurstWorkers: null,
    }));
    expect(loadSavedSettings().defaultOptions.match_case).toBe(false);
  });

  it("restores a remembered manual worker count from an extended record", () => {
    values.set(SEARCH_HISTORY_CONSTANTS.STORAGE_KEY, JSON.stringify({
      maxEntries: 20, keywords: [], directories: [], defaultOptions: DEFAULT_SEARCH_OPTIONS,
      rememberedBurstWorkers: 12,
    }));
    expect(loadSavedSettings().rememberedBurstWorkers).toBe(12);
  });

  it("ignores malformed legacy options when an extended record already has preferences", () => {
    values.set(SEARCH_HISTORY_CONSTANTS.STORAGE_KEY, JSON.stringify({
      maxEntries: 2, keywords: ["saved"], directories: [], defaultOptions: DEFAULT_SEARCH_OPTIONS,
      rememberedBurstWorkers: null,
    }));
    values.set(DEFAULT_OPTIONS_STORAGE_KEY, "malformed stale legacy value");
    const loaded = loadSavedSettings();
    expect(loaded.keywords).toEqual(["saved"]);
    expect(loaded.defaultOptions).toEqual(DEFAULT_SEARCH_OPTIONS);
  });

  it("normalizes malformed records and reports storage read failures without throwing", () => {
    values.set(SEARCH_HISTORY_CONSTANTS.STORAGE_KEY, "{");
    const error = vi.fn();
    expect(loadSavedSettings(error).keywords).toEqual([]);
    expect(error).toHaveBeenCalledOnce();
    error.mockClear();
    Object.defineProperty(window, "localStorage", {
      configurable: true,
      get: () => { throw new Error("storage unavailable"); },
    });
    expect(loadSavedSettings(error).maxEntries).toBe(SEARCH_HISTORY_CONSTANTS.DEFAULT_MAX_ENTRIES);
    expect(error).toHaveBeenCalledOnce();
  });

  it("replaces exactly one key and leaves the legacy options key untouched", () => {
    values.set(DEFAULT_OPTIONS_STORAGE_KEY, JSON.stringify(DEFAULT_SEARCH_OPTIONS));
    const setItem = vi.spyOn(window.localStorage, "setItem");
    const saved = loadSavedSettings();
    expect(saveSavedSettings({ ...saved, defaultOptions: { ...saved.defaultOptions, match_case: true } })).toBe(true);
    expect(setItem).toHaveBeenCalledOnce();
    expect(setItem).toHaveBeenCalledWith(SEARCH_HISTORY_CONSTANTS.STORAGE_KEY, expect.any(String));
    expect(values.has(DEFAULT_OPTIONS_STORAGE_KEY)).toBe(true);
  });

  it("returns false and preserves the saved record when storage rejects replacement", () => {
    const original = JSON.stringify({ maxEntries: 20, keywords: [], directories: [], defaultOptions: DEFAULT_SEARCH_OPTIONS, rememberedBurstWorkers: null });
    values.set(SEARCH_HISTORY_CONSTANTS.STORAGE_KEY, original);
    vi.spyOn(window.localStorage, "setItem").mockImplementation(() => { throw new Error("quota"); });
    expect(saveSavedSettings(loadSavedSettings())).toBe(false);
    expect(values.get(SEARCH_HISTORY_CONSTANTS.STORAGE_KEY)).toBe(original);
  });

  it("returns false when candidate serialization fails", () => {
    const stringify = vi.spyOn(JSON, "stringify").mockImplementation(() => { throw new Error("serialization failed"); });
    expect(saveSavedSettings(loadSavedSettings())).toBe(false);
    stringify.mockRestore();
  });
});
