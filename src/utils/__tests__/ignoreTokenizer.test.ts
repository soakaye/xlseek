/**
 * Copyright (c) 2026 soakaye
 *
 * @fileoverview Unit tests for ignoreTokenizer utility (src/utils/__tests__/ignoreTokenizer.test.ts)
 */

import { describe, expect, it } from "vitest";
import {
  splitIgnorePatterns,
  formatIgnorePattern,
  joinIgnorePatterns,
} from "../ignoreTokenizer";

describe("splitIgnorePatterns", () => {
  it("splits unquoted comma-separated patterns and trims whitespace", () => {
    const input = "  ~$* , *.tmp  ,  node_modules/ ";
    expect(splitIgnorePatterns(input)).toEqual(["~$*", "*.tmp", "node_modules/"]);
  });

  it("preserves commas and whitespace within double quotes", () => {
    const input = `"~$*, temp", "Folder with space/", normal*.xlsx`;
    expect(splitIgnorePatterns(input)).toEqual([
      "~$*, temp",
      "Folder with space/",
      "normal*.xlsx",
    ]);
  });

  it("handles escaped quotes properly", () => {
    const input = `"pattern""with""quote", "escaped\\"quote"`;
    expect(splitIgnorePatterns(input)).toEqual([
      'pattern"with"quote',
      'escaped"quote',
    ]);
  });

  it("returns empty array for empty or whitespace-only input", () => {
    expect(splitIgnorePatterns("   ")).toEqual([]);
    expect(splitIgnorePatterns(",,,")).toEqual([]);
    expect(splitIgnorePatterns("")).toEqual([]);
  });

  it("handles trailing and leading delimiters gracefully", () => {
    expect(splitIgnorePatterns(",~$*, *.bak,")).toEqual(["~$*", "*.bak"]);
  });
});

describe("formatIgnorePattern", () => {
  it("returns trimmed pattern if no delimiter is present", () => {
    expect(formatIgnorePattern("  ~$*  ")).toBe("~$*");
    expect(formatIgnorePattern("*.xlsx")).toBe("*.xlsx");
  });

  it("wraps in quotes and escapes internal quotes if delimiter is present", () => {
    expect(formatIgnorePattern("pattern, with comma")).toBe('"pattern, with comma"');
    expect(formatIgnorePattern('pat, "quote"')).toBe('"pat, ""quote"""');
  });

  it("returns empty string for empty input", () => {
    expect(formatIgnorePattern("   ")).toBe("");
  });
});

describe("joinIgnorePatterns", () => {
  it("joins patterns separated by comma and space", () => {
    expect(joinIgnorePatterns(["~$*", "*.tmp", "backup/"])).toBe("~$*, *.tmp, backup/");
  });

  it("quotes segments containing commas", () => {
    expect(joinIgnorePatterns(["~$*", "folder, with comma/", "*.bak"])).toBe(
      '~$*, "folder, with comma/", *.bak'
    );
  });

  it("omits empty pattern entries", () => {
    expect(joinIgnorePatterns(["~$*", "", "   ", "*.tmp"])).toBe("~$*, *.tmp");
  });
});
