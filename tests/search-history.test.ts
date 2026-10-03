/**
 * Copyright (c) 2026 soakaye
 *
 * Description: Unit and integration tests for search-history.test.ts.
 * Arguments & Returns: Vitest runs test suites; no public arguments or return values.
 * Errors: Test assertions fail if behavior deviates from requirements.
 */
import { act, renderHook } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { SEARCH_HISTORY_CONSTANTS } from "../src/constants";
import { DEFAULT_SEARCH_OPTIONS } from "../src/constants";
import { useSearchHistory } from "../src/hooks/useSearchHistory";

describe("useSearchHistory", () => {
  beforeEach(() => {
    const saved = new Map<string, string>();
    Object.defineProperty(window, "localStorage", {
      configurable: true,
      value: {
        get length() { return saved.size; },
        clear: () => saved.clear(),
        getItem: (key: string) => saved.get(key) ?? null,
        key: (index: number) => [...saved.keys()][index] ?? null,
        removeItem: (key: string) => saved.delete(key),
        setItem: (key: string, value: string) => saved.set(key, value),
      } as Storage,
    });
  });
  afterEach(() => vi.restoreAllMocks());

  it("starts empty with the default limit and restores the saved state", () => {
    const { result, unmount } = renderHook(() => useSearchHistory());
    expect(result.current.maxEntries).toBe(SEARCH_HISTORY_CONSTANTS.DEFAULT_MAX_ENTRIES);
    expect(result.current.keywords).toEqual([]);
    expect(result.current.directories).toEqual([]);

    act(() => result.current.addSearch("invoice", "/reports"));
    unmount();

    const restored = renderHook(() => useSearchHistory());
    expect(restored.result.current.keywords).toEqual(["invoice"]);
    expect(restored.result.current.directories).toEqual(["/reports"]);
  });

  it("moves exact repeated values to the front and excludes whitespace-only values", () => {
    const { result } = renderHook(() => useSearchHistory());

    act(() => {
      result.current.savePreferences(2, DEFAULT_SEARCH_OPTIONS, null);
      result.current.addSearch("first", "/one");
      result.current.addSearch("second", "/two");
      result.current.addSearch("first", "/one");
      result.current.addSearch("   ", "  ");
    });

    expect(result.current.keywords).toEqual(["first", "second"]);
    expect(result.current.directories).toEqual(["/one", "/two"]);
  });

  it("clears and stops storing both histories when the limit becomes zero", () => {
    const { result } = renderHook(() => useSearchHistory());
    act(() => result.current.addSearch("invoice", "/reports"));

    act(() => {
      result.current.savePreferences(0, DEFAULT_SEARCH_OPTIONS, null);
      result.current.addSearch("later", "/later");
    });

    expect(result.current.maxEntries).toBe(0);
    expect(result.current.keywords).toEqual([]);
    expect(result.current.directories).toEqual([]);
  });

  it("keeps the new in-memory history and reports storage write failures", () => {
    const onStorageError = vi.fn();
    const { result } = renderHook(() => useSearchHistory(onStorageError));
    vi.spyOn(window.localStorage, "setItem").mockImplementation(() => {
      throw new Error("storage unavailable");
    });

    act(() => result.current.addSearch("invoice", "/reports"));

    expect(result.current.keywords).toEqual(["invoice"]);
    expect(result.current.directories).toEqual(["/reports"]);
    expect(onStorageError).toHaveBeenCalledOnce();
  });

  it("saves preferences with the latest history and trims only after persistence succeeds", () => {
    const { result } = renderHook(() => useSearchHistory());
    act(() => {
      result.current.addSearch("first", "/one");
      result.current.addSearch("second", "/two");
    });
    let saved = false;
    act(() => { saved = result.current.savePreferences(1, { ...DEFAULT_SEARCH_OPTIONS, match_case: true }, null); });
    expect(saved).toBe(true);
    expect(result.current.keywords).toEqual(["second"]);
    expect(result.current.directories).toEqual(["/two"]);
    expect(result.current.defaultOptions.match_case).toBe(true);
    expect(JSON.parse(window.localStorage.getItem(SEARCH_HISTORY_CONSTANTS.STORAGE_KEY) ?? "{}").keywords).toEqual(["second"]);
  });

  it("does not publish preference or history changes when a combined write fails", () => {
    const onError = vi.fn();
    const { result } = renderHook(() => useSearchHistory(onError));
    act(() => result.current.addSearch("first", "/one"));
    vi.spyOn(window.localStorage, "setItem").mockImplementation(() => { throw new Error("storage unavailable"); });
    let saved = true;
    act(() => { saved = result.current.savePreferences(0, { ...DEFAULT_SEARCH_OPTIONS, match_case: true }, null); });
    expect(saved).toBe(false);
    expect(result.current.maxEntries).toBe(SEARCH_HISTORY_CONSTANTS.DEFAULT_MAX_ENTRIES);
    expect(result.current.keywords).toEqual(["first"]);
    expect(result.current.defaultOptions.match_case).toBe(false);
    expect(onError).not.toHaveBeenCalled();
  });

  it("preserves committed preferences on ordinary history writes", () => {
    const { result } = renderHook(() => useSearchHistory());
    act(() => { result.current.savePreferences(4, { ...DEFAULT_SEARCH_OPTIONS, match_case: true }, null); });
    act(() => result.current.addSearch("after save", "/folder"));
    const saved = JSON.parse(window.localStorage.getItem(SEARCH_HISTORY_CONSTANTS.STORAGE_KEY) ?? "{}");
    expect(saved.defaultOptions.match_case).toBe(true);
    expect(saved.maxEntries).toBe(4);
  });

  it("replaces malformed saved values with an empty default history", () => {
    window.localStorage.setItem(SEARCH_HISTORY_CONSTANTS.STORAGE_KEY, JSON.stringify({
      maxEntries: 51,
      keywords: ["old"],
      directories: [],
    }));

    const { result } = renderHook(() => useSearchHistory());

    expect(result.current.maxEntries).toBe(SEARCH_HISTORY_CONSTANTS.DEFAULT_MAX_ENTRIES);
    expect(result.current.keywords).toEqual([]);
    expect(result.current.directories).toEqual([]);
  });
});
