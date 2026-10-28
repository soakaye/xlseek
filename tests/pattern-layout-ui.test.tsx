/**
 * Copyright (c) 2026 soakaye
 *
 * Description: Unit and integration tests for horizontal side-by-side pattern inputs layout (SearchBar and mockup).
 * Arguments & Returns: Vitest test suite; no public arguments or return values.
 * Errors: Test assertions fail if component layout, responsiveness, or clear/history interactions deviate from specifications.
 */

import { act, cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import * as fs from "node:fs";
import * as path from "node:path";
import { FILE_PATTERNS_CONSTANTS, IGNORE_PATTERNS_CONSTANTS } from "../src/constants";
import { SearchBar } from "../src/components/search/SearchBar";

const mocks = vi.hoisted(() => ({ invoke: vi.fn() }));
vi.mock("@tauri-apps/api/core", () => ({ invoke: mocks.invoke }));
vi.mock("@tauri-apps/plugin-dialog", () => ({ open: vi.fn() }));
vi.mock("@tauri-apps/api/webview", () => ({
  getCurrentWebview: () => ({ onDragDropEvent: async () => () => undefined }),
}));
vi.mock("../src/i18n", () => ({
  useTranslation: () => (key: string) => key,
}));

const baseQuery = {
  keyword: "sample",
  target_dir: "/data",
  paths: ["/data"],
  match_case: false,
  use_regex: false,
  include_formula: true,
  include_comment: true,
  include_hidden: false,
  extensions: [".xlsx"],
  file_patterns: "*budget*",
  ignore_patterns: "~$*",
  content_types: {
    values: true,
    formulas: false,
    comments: false,
    shapes: false,
    hidden_sheets: false,
  },
};

describe("Horizontal Side-by-Side Pattern Inputs Layout (SearchBar)", () => {
  beforeEach(() => {
    mocks.invoke.mockReset().mockResolvedValue([]);
    localStorage.clear();
    localStorage.setItem(
      FILE_PATTERNS_CONSTANTS.STORAGE_KEY_HISTORY,
      JSON.stringify(["*budget*", "quarterly_*"])
    );
    localStorage.setItem(
      IGNORE_PATTERNS_CONSTANTS.STORAGE_KEY_HISTORY,
      JSON.stringify(["~$*", "*.tmp"])
    );
  });

  afterEach(() => {
    cleanup();
    vi.restoreAllMocks();
  });

  it("renders both pattern inputs inside a shared responsive 2-column grid container (US1)", () => {
    render(
      <SearchBar
        query={baseQuery}
        onChangeQuery={() => undefined}
        onSearch={() => undefined}
        onCancel={() => undefined}
        isScanning={false}
        history={{ keywords: [], directories: [] }}
        onSelectHistory={() => undefined}
      />
    );

    const fileInput = document.getElementById(FILE_PATTERNS_CONSTANTS.INPUT_ID);
    const ignoreInput = document.getElementById(IGNORE_PATTERNS_CONSTANTS.INPUT_ID);

    expect(fileInput).not.toBeNull();
    expect(ignoreInput).not.toBeNull();

    // Find the common ancestor row
    // fileInput -> div.relative.flex-1 -> div.flex -> div.relative -> row container
    const fileContainer = fileInput?.closest(".relative");
    const ignoreContainer = ignoreInput?.closest(".relative");
    expect(fileContainer).not.toBeNull();
    expect(ignoreContainer).not.toBeNull();

    // The shared row container enclosing both pattern columns
    const sharedRow = fileInput?.closest(".grid-cols-1");
    expect(sharedRow).not.toBeNull();
    expect(sharedRow?.className).toContain("md:grid-cols-2");
    expect(sharedRow?.contains(ignoreInput!)).toBe(true);

    // Verify ordering: File Search Patterns appears before Ignore Patterns
    const inputs = sharedRow?.querySelectorAll("input");
    expect(inputs?.[0]?.id).toBe(FILE_PATTERNS_CONSTANTS.INPUT_ID);
    expect(inputs?.[1]?.id).toBe(IGNORE_PATTERNS_CONSTANTS.INPUT_ID);
  });

  it("allows clearing each pattern input independently without affecting the other (US2)", () => {
    const onChangeQuery = vi.fn();
    render(
      <SearchBar
        query={baseQuery}
        onChangeQuery={onChangeQuery}
        onSearch={() => undefined}
        onCancel={() => undefined}
        isScanning={false}
        history={{ keywords: [], directories: [] }}
        onSelectHistory={() => undefined}
      />
    );

    const fileInput = document.getElementById(FILE_PATTERNS_CONSTANTS.INPUT_ID);
    const ignoreInput = document.getElementById(IGNORE_PATTERNS_CONSTANTS.INPUT_ID);

    const fileWrapper = fileInput?.parentElement;
    const ignoreWrapper = ignoreInput?.parentElement;

    const fileClearBtn = fileWrapper?.querySelector('button[title="ui.CLEAR_TOOLTIP"]');
    const ignoreClearBtn = ignoreWrapper?.querySelector('button[title="ui.CLEAR_TOOLTIP"]');

    expect(fileClearBtn).not.toBeNull();
    expect(ignoreClearBtn).not.toBeNull();

    // Clear file patterns
    fireEvent.click(fileClearBtn!);
    expect(onChangeQuery).toHaveBeenCalledWith({ file_patterns: "" });

    onChangeQuery.mockClear();

    // Clear ignore patterns
    fireEvent.click(ignoreClearBtn!);
    expect(onChangeQuery).toHaveBeenCalledWith({ ignore_patterns: "" });
  });

  it("manages history dropdown visibility and mutual exclusivity cleanly (US2)", () => {
    render(
      <SearchBar
        query={baseQuery}
        onChangeQuery={() => undefined}
        onSearch={() => undefined}
        onCancel={() => undefined}
        isScanning={false}
        history={{ keywords: [], directories: [] }}
        onSelectHistory={() => undefined}
      />
    );

    const fileHistoryBtn = screen.getByTitle("ui.FILE_PATTERN_HISTORY_BUTTON");
    const ignoreHistoryBtn = screen.getByTitle("ui.IGNORE_HISTORY_BUTTON");

    // Initially neither dropdown is visible
    expect(screen.queryByText("quarterly_*")).toBeNull();
    expect(screen.queryByText("*.tmp")).toBeNull();

    // Open file pattern history
    act(() => {
      fireEvent.click(fileHistoryBtn);
    });
    expect(screen.getByText("quarterly_*")).toBeDefined();
    expect(screen.queryByText("*.tmp")).toBeNull();

    // Open ignore pattern history - should close file pattern history
    act(() => {
      fireEvent.click(ignoreHistoryBtn);
    });
    expect(screen.queryByText("quarterly_*")).toBeNull();
    expect(screen.getByText("*.tmp")).toBeDefined();
  });

  it("synchronizes standalone HTML prototype in design/mainui/index.html with side-by-side grid (US3)", () => {
    const mockupPath = path.resolve(process.cwd(), "design/mainui/index.html");
    expect(fs.existsSync(mockupPath)).toBe(true);

    const html = fs.readFileSync(mockupPath, "utf-8");

    // Both controls should exist in the mockup
    expect(html).toContain('id="filePatternControl"');
    expect(html).toContain('id="ignorePatternControl"');

    // Both controls should be inside a single grid with grid-cols-1 md:grid-cols-2
    const sideBySidePattern = /class="[^"]*grid-cols-1\s+md:grid-cols-2[^"]*"[^>]*>[\s\S]*?id="filePatternControl"[\s\S]*?id="ignorePatternControl"/;
    expect(sideBySidePattern.test(html)).toBe(true);
  });
});
