/**
 * Copyright (c) 2026 soakaye
 *
 * @fileoverview Unit tests for useLocale hook (src/hooks/__tests__/useLocale.test.tsx)
 *
 * ## Description
 * Verifies that `useLocale` resolves the initial display language and transitions `ready`
 * to `true` even when Tauri IPC calls (`locale()` or `initializeI18n()`) stall or reject
 * after background resume.
 *
 * ## Arguments & Returns
 * None. Executed by Vitest test runner.
 *
 * ## Errors / Exceptions
 * Fails assertions if initialization stays blocked or falls back to an unexpected locale.
 */

import { describe, expect, it, beforeEach, afterEach, vi } from "vitest";
import { renderHook, act } from "@testing-library/react";
import { DISPLAY_LANGUAGES, LANGUAGE_PREFERENCES, TIMING_CONSTANTS } from "../../constants";

const mockLocale = vi.fn<() => Promise<string | null>>();
const mockInitializeI18n = vi.fn<(lang: string) => Promise<boolean>>();
const mockSetI18nLocale = vi.fn<(lang: string) => Promise<boolean>>();

vi.mock("@tauri-apps/plugin-os", () => ({
  locale: () => mockLocale(),
}));

vi.mock("../../i18n", () => ({
  initializeI18n: (lang: string) => mockInitializeI18n(lang),
  setI18nLocale: (lang: string) => mockSetI18nLocaleWrapper(lang),
}));

/**
 * ## Description
 * Forwards locale change calls in test mocks to `mockSetI18nLocale`.
 *
 * ## Arguments & Returns
 * @param lang - Target language code
 * @returns Promise resolving to boolean
 *
 * ## Errors / Exceptions
 * Propagates mocked rejections if configured by a test case.
 */
const mockSetI18nLocaleWrapper = (lang: string): Promise<boolean> => mockSetI18nLocale(lang);

import { useLocale } from "../useLocale";

describe("useLocale", () => {
  beforeEach(() => {
    const values = new Map<string, string>();
    Object.defineProperty(window, "localStorage", {
      configurable: true,
      value: {
        get length() {
          return values.size;
        },
        key: (index: number) => Array.from(values.keys())[index] ?? null,
        getItem: (key: string) => values.get(key) ?? null,
        setItem: (key: string, value: string) => {
          values.set(key, value);
        },
        removeItem: (key: string) => {
          values.delete(key);
        },
        clear: () => {
          values.clear();
        },
      } satisfies Storage,
    });
    vi.useFakeTimers();
    mockLocale.mockReset();
    mockInitializeI18n.mockReset();
    mockSetI18nLocale.mockReset();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("marks ready=true immediately when locale and i18n resolve normally", async () => {
    mockLocale.mockResolvedValue(DISPLAY_LANGUAGES.JA);
    mockInitializeI18n.mockResolvedValue(true);

    const { result } = renderHook(() => useLocale());

    await act(async () => {
      await Promise.resolve();
    });

    expect(result.current.ready).toBe(true);
    expect(result.current.language).toBe(DISPLAY_LANGUAGES.JA);
    expect(result.current.preference).toBe(LANGUAGE_PREFERENCES.DEFAULT);
  });

  it("falls back and marks ready=true after timeout when Tauri IPC hangs", async () => {
    // Simulate a stalled IPC call that never settles
    mockLocale.mockReturnValue(new Promise<string | null>(() => {}));
    mockInitializeI18n.mockReturnValue(new Promise<boolean>(() => {}));

    const { result } = renderHook(() => useLocale());
    expect(result.current.ready).toBe(false);

    // Advance past first timeout (locale()) and second timeout (initializeI18n())
    // Constant reference: TIMING_CONSTANTS.I18N_INIT_TIMEOUT_MS
    await act(async () => {
      await vi.advanceTimersByTimeAsync(TIMING_CONSTANTS.I18N_INIT_TIMEOUT_MS);
    });
    await act(async () => {
      await vi.advanceTimersByTimeAsync(TIMING_CONSTANTS.I18N_INIT_TIMEOUT_MS);
    });

    expect(result.current.ready).toBe(true);
    expect([DISPLAY_LANGUAGES.EN, DISPLAY_LANGUAGES.JA]).toContain(result.current.language);
  });
});
