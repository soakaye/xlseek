/**
 * Copyright (c) 2026 soakaye
 *
 * @fileoverview Unit tests for filePatternHistory utility (src/utils/__tests__/filePatternHistory.test.ts)
 */

import { describe, expect, it, beforeEach, vi } from "vitest";
import {
  loadFilePatternHistory,
  saveFilePattern,
  clearFilePatternHistory,
} from "../filePatternHistory";
import { FILE_PATTERNS_CONSTANTS } from "../../constants";

describe("filePatternHistory", () => {
  beforeEach(() => {
    window.localStorage.clear();
    vi.restoreAllMocks();
  });

  it("returns empty array when storage is empty", () => {
    expect(loadFilePatternHistory()).toEqual([]);
  });

  it("saves a valid pattern to history", () => {
    const res = saveFilePattern("*budget*, *report*");
    expect(res).toEqual(["*budget*, *report*"]);
    expect(loadFilePatternHistory()).toEqual(["*budget*, *report*"]);
  });

  it("ignores empty or whitespace-only patterns", () => {
    expect(saveFilePattern("")).toEqual([]);
    expect(saveFilePattern("   ")).toEqual([]);
    expect(loadFilePatternHistory()).toEqual([]);
  });

  it("promotes duplicates to the top (MRU order)", () => {
    saveFilePattern("*budget*");
    saveFilePattern("quarterly_*");
    saveFilePattern("report.xlsx");
    expect(loadFilePatternHistory()).toEqual(["report.xlsx", "quarterly_*", "*budget*"]);

    // Re-save "*budget*"
    saveFilePattern("*budget*");
    expect(loadFilePatternHistory()).toEqual(["*budget*", "report.xlsx", "quarterly_*"]);
  });

  it("trims surrounding whitespace before saving", () => {
    saveFilePattern("   *budget*, *.xlsm   ");
    expect(loadFilePatternHistory()).toEqual(["*budget*, *.xlsm"]);
  });

  it("enforces maximum 20 entries", () => {
    for (let i = 1; i <= 25; i++) {
      saveFilePattern(`pattern_${i}`);
    }
    const history = loadFilePatternHistory();
    expect(history.length).toBe(FILE_PATTERNS_CONSTANTS.MAX_HISTORY_ENTRIES);
    expect(history[0]).toBe("pattern_25");
    expect(history[history.length - 1]).toBe("pattern_6");
  });

  it("clears file pattern history", () => {
    saveFilePattern("*budget*");
    expect(loadFilePatternHistory().length).toBe(1);
    clearFilePatternHistory();
    expect(loadFilePatternHistory()).toEqual([]);
  });

  it("handles corrupted storage data safely", () => {
    window.localStorage.setItem(FILE_PATTERNS_CONSTANTS.STORAGE_KEY_HISTORY, "invalid json{{{");
    expect(loadFilePatternHistory()).toEqual([]);
  });
});
