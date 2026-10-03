/**
 * Copyright (c) 2026 soakaye
 *
 * Description: Unit tests for default search options storage, normalization, and validation.
 * Arguments & Returns: Vitest runs test suites; no public arguments or return values.
 * Errors: Test assertions fail if option manipulation deviates from specification.
 */

import { beforeEach, describe, expect, it } from "vitest";
import {
  isDefaultSearchOptions,
  normalizeDefaultSearchOptions,
  resetDefaultSearchOptions,
} from "../src/default-options-core";
import { DEFAULT_SEARCH_OPTIONS } from "../src/constants";

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

  it("normalizes legacy preferences and invalid directory fields", () => {
    const legacy = {
      ...DEFAULT_SEARCH_OPTIONS,
      directory_mode: undefined,
      burst_workers: undefined,
      match_case: true,
    };
    expect(normalizeDefaultSearchOptions(legacy)).toEqual({
      ...DEFAULT_SEARCH_OPTIONS,
      match_case: true,
    });

    expect(normalizeDefaultSearchOptions({ ...legacy, directory_mode: "invalid", burst_workers: 33 })).toEqual({
      ...DEFAULT_SEARCH_OPTIONS,
      match_case: true,
    });
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

  it("normalizes malformed objects to the standard option defaults", () => {
    expect(normalizeDefaultSearchOptions({ match_case: "not-bool" })).toEqual(DEFAULT_SEARCH_OPTIONS);
  });

  it("resets options to official defaults and returns the copy", () => {
    const reset = resetDefaultSearchOptions();
    expect(reset).toEqual(DEFAULT_SEARCH_OPTIONS);
  });
});
