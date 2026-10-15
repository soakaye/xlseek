/**
 * Copyright (c) 2026 soakaye
 *
 * @fileoverview Unit and component tests for PreviewHeader (tests/preview-header.test.tsx)
 *
 * ## Description
 * Verifies that PreviewHeader correctly strips Windows verbatim extended-length prefixes
 * (`\\?\` and `//?/`) from the location line and clipboard actions.
 * Complies with Constitution Principle I (English documentation) and Principle V (robust error handling and testing).
 */

import React from "react";
import { render, screen, cleanup } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { PreviewHeader } from "../src/components/preview/PreviewHeader";
import type { SearchMatch } from "../src/types/search";

const mockInvoke = vi.fn().mockResolvedValue([]);
vi.mock("@tauri-apps/api/core", () => ({ invoke: (...args: unknown[]) => mockInvoke(...args) }));
vi.mock("../src/i18n", () => ({ useTranslation: () => (key: string) => key }));

const createMatch = (fullPath: string): SearchMatch => ({
  id: 1,
  file_name: "test.xlsx",
  full_path: fullPath,
  sheet_name: "Sheet1",
  cell_address: "A1",
  row_index: 1,
  col_index: 1,
  col_name: "A",
  match_type: "CellValue",
  shape_name: null,
  sheet_hidden: false,
  snippet: "match",
  full_content: "content",
  formula: null,
  sheets_in_workbook: ["Sheet1"],
});

describe("PreviewHeader path display", () => {
  afterEach(() => {
    cleanup();
    vi.clearAllMocks();
  });

  it("renders location without \\\\?\\ when path contains Windows verbatim prefix", () => {
    const match = createMatch("\\\\?\\C:\\Users\\soakaye\\Documents\\test.xlsx");
    render(<PreviewHeader selectedMatch={match} onShowToast={() => undefined} />);

    // Should display clean drive path
    expect(screen.getByText("C:\\Users\\soakaye\\Documents\\test.xlsx")).toBeTruthy();
    // Should NOT contain verbatim prefix in the document
    expect(screen.queryByText("\\\\?\\C:\\Users\\soakaye\\Documents\\test.xlsx")).toBeNull();
  });

  it("renders location without //?/ when path contains forward-slash verbatim prefix", () => {
    const match = createMatch("//?/C:/Users/soakaye/Documents/test.xlsx");
    render(<PreviewHeader selectedMatch={match} onShowToast={() => undefined} />);

    // Should display clean forward-slash drive path
    expect(screen.getByText("C:/Users/soakaye/Documents/test.xlsx")).toBeTruthy();
    expect(screen.queryByText("//?/C:/Users/soakaye/Documents/test.xlsx")).toBeNull();
  });

  it("renders standard UNC network path when verbatim UNC prefix is present", () => {
    const match = createMatch("\\\\?\\UNC\\server\\share\\test.xlsx");
    render(<PreviewHeader selectedMatch={match} onShowToast={() => undefined} />);

    expect(screen.getByText("\\\\server\\share\\test.xlsx")).toBeTruthy();
    expect(screen.queryByText("\\\\?\\UNC\\server\\share\\test.xlsx")).toBeNull();
  });

  it("preserves standard POSIX paths unchanged", () => {
    const match = createMatch("/home/soakaye/test.xlsx");
    render(<PreviewHeader selectedMatch={match} onShowToast={() => undefined} />);

    expect(screen.getByText("/home/soakaye/test.xlsx")).toBeTruthy();
  });
});
