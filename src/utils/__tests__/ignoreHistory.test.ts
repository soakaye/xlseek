/**
 * Copyright (c) 2026 soakaye
 *
 * @fileoverview Unit tests for ignoreHistory utility (src/utils/__tests__/ignoreHistory.test.ts)
 */

import { describe, expect, it, beforeEach, vi } from "vitest";
import {
  loadIgnoreHistory,
  saveIgnorePattern,
  clearIgnoreHistory,
} from "../ignoreHistory";
import { IGNORE_PATTERNS_CONSTANTS } from "../../constants";

describe("ignoreHistory", () => {
  beforeEach(() => {
    window.localStorage.clear();
    vi.restoreAllMocks();
  });

  it("returns empty array when storage is empty", () => {
    expect(loadIgnoreHistory()).toEqual([]);
  });

  it("saves a valid pattern to history", () => {
    const res = saveIgnorePattern("~$*, *.tmp");
    expect(res).toEqual(["~$*, *.tmp"]);
    expect(loadIgnoreHistory()).toEqual(["~$*, *.tmp"]);
  });

  it("ignores empty or whitespace-only patterns", () => {
    expect(saveIgnorePattern("")).toEqual([]);
    expect(saveIgnorePattern("   ")).toEqual([]);
    expect(loadIgnoreHistory()).toEqual([]);
  });

  it("promotes duplicates to the top (MRU order)", () => {
    saveIgnorePattern("~$*");
    saveIgnorePattern("*.tmp");
    saveIgnorePattern("backup/");
    expect(loadIgnoreHistory()).toEqual(["backup/", "*.tmp", "~$*"]);

    // Re-save "~$*"
    saveIgnorePattern("~$*");
    expect(loadIgnoreHistory()).toEqual(["~$*", "backup/", "*.tmp"]);
  });

  it("trims surrounding whitespace before saving", () => {
    saveIgnorePattern("   ~$*, *.bak   ");
    expect(loadIgnoreHistory()).toEqual(["~$*, *.bak"]);
  });

  it("enforces maximum 20 entries", () => {
    for (let i = 1; i <= 25; i++) {
      saveIgnorePattern(`pattern_${i}`);
    }
    const history = loadIgnoreHistory();
    expect(history.length).toBe(IGNORE_PATTERNS_CONSTANTS.MAX_HISTORY_ENTRIES);
    expect(history[0]).toBe("pattern_25");
    expect(history[history.length - 1]).toBe("pattern_6");
  });

  it("clears ignore history", () => {
    saveIgnorePattern("~$*");
    expect(loadIgnoreHistory().length).toBe(1);
    clearIgnoreHistory();
    expect(loadIgnoreHistory()).toEqual([]);
  });

  it("handles corrupted storage data safely", () => {
    window.localStorage.setItem(IGNORE_PATTERNS_CONSTANTS.STORAGE_KEY_HISTORY, "invalid json{{{");
    expect(loadIgnoreHistory()).toEqual([]);
  });
});
