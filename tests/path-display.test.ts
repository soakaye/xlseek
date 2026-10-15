/**
 * Copyright (c) 2026 soakaye
 *
 * @fileoverview Unit tests for pathDisplay normalization utility (tests/path-display.test.ts)
 *
 * ## Description
 * Validates the behavior of `normalizeDisplayPath` in stripping Windows verbatim extended-length
 * path prefixes (`\\?\` and `//?/`) for local paths and converting verbatim UNC prefixes to standard format.
 * Complies with Constitution Principle I (English documentation) and Principle V (robust error handling and testing).
 */

import { describe, expect, it } from "vitest";
import { normalizeDisplayPath } from "../src/utils/pathDisplay";

describe("normalizeDisplayPath", () => {
  describe("Windows verbatim local drive paths", () => {
    it("strips backslash verbatim prefix \\\\?\\ from Windows local drive path", () => {
      const input = "\\\\?\\C:\\Users\\soakaye\\Documents\\Reports\\Q3_Summary.xlsx";
      const expected = "C:\\Users\\soakaye\\Documents\\Reports\\Q3_Summary.xlsx";
      expect(normalizeDisplayPath(input)).toBe(expected);
    });

    it("strips backslash verbatim prefix \\\\?\\ with lowercase drive letter", () => {
      const input = "\\\\?\\d:\\data\\financials.xlsm";
      const expected = "d:\\data\\financials.xlsm";
      expect(normalizeDisplayPath(input)).toBe(expected);
    });

    it("strips forward-slash verbatim prefix //?/ from Windows local drive path", () => {
      const input = "//?/C:/Users/soakaye/Documents/Reports/Q3_Summary.xlsx";
      const expected = "C:/Users/soakaye/Documents/Reports/Q3_Summary.xlsx";
      expect(normalizeDisplayPath(input)).toBe(expected);
    });

    it("strips forward-slash verbatim prefix //?/ with mixed backslashes", () => {
      const input = "//?/C:\\Users\\soakaye\\Documents\\Reports\\Q3_Summary.xlsx";
      const expected = "C:\\Users\\soakaye\\Documents\\Reports\\Q3_Summary.xlsx";
      expect(normalizeDisplayPath(input)).toBe(expected);
    });
  });

  describe("Windows verbatim UNC network paths", () => {
    it("converts backslash verbatim UNC prefix \\\\?\\UNC\\ to standard \\\\", () => {
      const input = "\\\\?\\UNC\\fileserver\\shared\\Accounting\\Budget.xlsx";
      const expected = "\\\\fileserver\\shared\\Accounting\\Budget.xlsx";
      expect(normalizeDisplayPath(input)).toBe(expected);
    });

    it("converts forward-slash verbatim UNC prefix //?/UNC/ to standard \\\\", () => {
      const input = "//?/UNC/fileserver/shared/Accounting/Budget.xlsx";
      const expected = "\\\\fileserver/shared/Accounting/Budget.xlsx";
      expect(normalizeDisplayPath(input)).toBe(expected);
    });

    it("converts lowercase verbatim unc prefix \\\\?\\unc\\ to standard \\\\", () => {
      const input = "\\\\?\\unc\\fileserver\\shared\\Accounting\\Budget.xlsx";
      const expected = "\\\\fileserver\\shared\\Accounting\\Budget.xlsx";
      expect(normalizeDisplayPath(input)).toBe(expected);
    });
  });

  describe("Standard and non-verbatim paths", () => {
    it("leaves standard Windows drive path unchanged", () => {
      const input = "C:\\Program Files\\Microsoft Office\\root\\Templates.xlsx";
      expect(normalizeDisplayPath(input)).toBe(input);
    });

    it("leaves standard UNC network path unchanged", () => {
      const input = "\\\\server\\share\\data\\workbook.xlsb";
      expect(normalizeDisplayPath(input)).toBe(input);
    });

    it("leaves POSIX absolute path unchanged", () => {
      const input = "/Users/soakaye/Documents/Reports/summary.xlsx";
      expect(normalizeDisplayPath(input)).toBe(input);
    });

    it("leaves relative path unchanged", () => {
      const input = "nested/folder/report.xlsx";
      expect(normalizeDisplayPath(input)).toBe(input);
    });

    it("handles empty or whitespace strings safely", () => {
      expect(normalizeDisplayPath("")).toBe("");
      expect(normalizeDisplayPath("   ")).toBe("   ");
    });
  });
});
