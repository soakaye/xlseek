/**
 * Copyright (c) 2026 soakaye
 *
 * @fileoverview Unit tests for StatusBar component (src/components/common/__tests__/StatusBar.test.tsx)
 *
 * ## Description
 * Validates StatusBar component rendering, progress calculation, Export Split Button
 * disabled state, format menu dropdown toggling, outside-click / Escape key dismissal,
 * format selection persistence, and IPC export dispatching.
 * Complies with Constitution Principle I, II, and III.
 *
 * ## Arguments & Returns
 * None. Vitest test runner executes exported suites.
 *
 * ## Errors / Exceptions
 * Test assertions fail if UI components or event handling deviate from expectations.
 */

import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";
import { render, screen, fireEvent, waitFor, cleanup } from "@testing-library/react";
import { parse } from "yaml";
import enSource from "../../../../src-tauri/locales/en.yml?raw";
import jaSource from "../../../../src-tauri/locales/ja.yml?raw";
import { initializeI18n, LocaleProvider } from "../../../i18n";
import { StatusBar } from "../StatusBar";
import { SearchMatch } from "../../../types/search";
import { LlmCopyOptions } from "../../../types/llm";
import { EXPORT_CONSTANTS } from "../../../constants";

const mocks = vi.hoisted(() => ({
  save: vi.fn(),
  invoke: vi.fn(async () => undefined),
  pluginLocale: "en",
  catalogs: {} as Record<string, Record<string, string>>,
}));

vi.mock("@tauri-apps/plugin-dialog", () => ({ save: mocks.save }));
vi.mock("@tauri-apps/api/core", () => ({ invoke: mocks.invoke }));
vi.mock("@razein97/tauri-plugin-i18n", () => ({
  default: {
    getInstance: () => ({
      load: async () => undefined,
      translate: (key: string) => mocks.catalogs[mocks.pluginLocale]?.[key] ?? key,
    }),
    setLocale: async (language: string) => {
      mocks.pluginLocale = language;
    },
  },
}));

describe("StatusBar Component - Export Split Button & Format Menu (US1)", () => {
  beforeEach(async () => {
    window.localStorage.clear();
    mocks.save.mockReset();
    mocks.invoke.mockReset().mockResolvedValue(undefined);
    mocks.pluginLocale = "en";
    mocks.catalogs = {
      en: parse(enSource) as Record<string, string>,
      ja: parse(jaSource) as Record<string, string>,
    };
    await initializeI18n("en");
  });

  afterEach(cleanup);

  const mockItem: SearchMatch = {
    id: 1,
    file_name: "sales.xlsx",
    full_path: "C:\\data\\sales.xlsx",
    sheet_name: "Q1",
    cell_address: "B4",
    row_index: 3,
    col_index: 1,
    col_name: "B",
    match_type: "CellValue",
    shape_name: null,
    sheet_hidden: false,
    snippet: "15000",
    full_content: "15000",
    formula: null,
    sheets_in_workbook: ["Q1"],
  };

  const defaultLlmOptions: LlmCopyOptions = {
    column_window_size: 5,
    copy_type: "table",
    word_mode: "headings_and_paragraph",
  };

  it("renders disabled export split button when results are empty", () => {
    render(
      <LocaleProvider value="en">
        <StatusBar
          progress={null}
          items={[]}
          onShowToast={() => undefined}
          onOpenAbout={() => undefined}
          onOpenSettings={() => undefined}
          language="en"
        />
      </LocaleProvider>
    );

    const primaryBtn = screen.getByRole("button", { name: "Export search results" }) as HTMLButtonElement;
    const menuBtn = screen.getByRole("button", { name: "Select export format" }) as HTMLButtonElement;

    expect(primaryBtn.disabled).toBe(true);
    expect(menuBtn.disabled).toBe(true);
    expect(screen.queryByRole("menu")).toBeNull();
  });

  it("toggles format menu open and closed when clicking menu chevron button", () => {
    render(
      <LocaleProvider value="en">
        <StatusBar
          progress={null}
          items={[mockItem]}
          onShowToast={() => undefined}
          onOpenAbout={() => undefined}
          onOpenSettings={() => undefined}
          language="en"
        />
      </LocaleProvider>
    );

    const menuBtn = screen.getByRole("button", { name: "Select export format" });
    expect(menuBtn.getAttribute("aria-expanded")).toBe("false");
    expect(screen.queryByRole("menu")).toBeNull();

    // Click to open
    fireEvent.click(menuBtn);
    expect(menuBtn.getAttribute("aria-expanded")).toBe("true");
    const menu = screen.getByRole("menu");
    expect(menu).toBeTruthy();

    // Check that 4 options are rendered
    const menuItems = screen.getAllByRole("menuitem");
    expect(menuItems.length).toBe(4);

    // Click to close
    fireEvent.click(menuBtn);
    expect(menuBtn.getAttribute("aria-expanded")).toBe("false");
    expect(screen.queryByRole("menu")).toBeNull();
  });

  it("closes format menu when pressing Escape key", () => {
    render(
      <LocaleProvider value="en">
        <StatusBar
          progress={null}
          items={[mockItem]}
          onShowToast={() => undefined}
          onOpenAbout={() => undefined}
          onOpenSettings={() => undefined}
          language="en"
        />
      </LocaleProvider>
    );

    const menuBtn = screen.getByRole("button", { name: "Select export format" });
    fireEvent.click(menuBtn);
    expect(screen.getByRole("menu")).toBeTruthy();

    // Fire Escape key
    fireEvent.keyDown(document, { key: "Escape" });
    expect(screen.queryByRole("menu")).toBeNull();
  });

  it("closes format menu when clicking outside the container", () => {
    render(
      <LocaleProvider value="en">
        <div>
          <span data-testid="outside-target">Outside</span>
          <StatusBar
            progress={null}
            items={[mockItem]}
            onShowToast={() => undefined}
            onOpenAbout={() => undefined}
            onOpenSettings={() => undefined}
            language="en"
          />
        </div>
      </LocaleProvider>
    );

    const menuBtn = screen.getByRole("button", { name: "Select export format" });
    fireEvent.click(menuBtn);
    expect(screen.getByRole("menu")).toBeTruthy();

    // Click outside
    const outside = screen.getByTestId("outside-target");
    fireEvent.mouseDown(outside);
    expect(screen.queryByRole("menu")).toBeNull();
  });

  it("persists selected format to localStorage and triggers export with chosen format", async () => {
    mocks.save.mockResolvedValue("C:\\dest\\export.md");
    const onShowToast = vi.fn();

    render(
      <LocaleProvider value="en">
        <StatusBar
          progress={null}
          items={[mockItem]}
          onShowToast={onShowToast}
          onOpenAbout={() => undefined}
          onOpenSettings={() => undefined}
          language="en"
          llmOptions={defaultLlmOptions}
        />
      </LocaleProvider>
    );

    // Open menu
    const menuBtn = screen.getByRole("button", { name: "Select export format" });
    fireEvent.click(menuBtn);

    // Click Markdown option
    const mdOption = screen.getByRole("menuitem", { name: /markdown/i });
    fireEvent.click(mdOption);

    // Verify localStorage persistence
    // Constant reference: EXPORT_CONSTANTS.STORAGE_KEY_FORMAT
    expect(window.localStorage.getItem(EXPORT_CONSTANTS.STORAGE_KEY_FORMAT)).toBe("md");

    // Verify save dialog was called with Markdown filters
    await waitFor(() => expect(mocks.save).toHaveBeenCalled());
    expect(mocks.save).toHaveBeenCalledWith({
      filters: [
        {
          name: "Markdown document",
          extensions: ["md"],
        },
      ],
      defaultPath: expect.stringMatching(/ExcelGrep_Results_.*\.md$/),
    });

    // Verify invoke was called with llm_options
    expect(mocks.invoke).toHaveBeenCalledWith("export_results", {
      request: {
        format: "md",
        output_path: "C:\\dest\\export.md",
        items: [mockItem],
        language: "en",
        llm_options: defaultLlmOptions,
      },
    });
    expect(onShowToast).toHaveBeenCalledWith("ui.EXPORT_MD_SAVED", { path: "C:\\dest\\export.md" });
  });

  it("restores previously stored format preference on mount", async () => {
    // Constant reference: EXPORT_CONSTANTS.STORAGE_KEY_FORMAT
    window.localStorage.setItem(EXPORT_CONSTANTS.STORAGE_KEY_FORMAT, "json");
    mocks.save.mockResolvedValue("C:\\dest\\export.json");
    const onShowToast = vi.fn();

    render(
      <LocaleProvider value="en">
        <StatusBar
          progress={null}
          items={[mockItem]}
          onShowToast={onShowToast}
          onOpenAbout={() => undefined}
          onOpenSettings={() => undefined}
          language="en"
        />
      </LocaleProvider>
    );

    // Main button label should reflect JSON format
    const primaryBtn = screen.getByRole("button", { name: "Export search results" });
    expect(primaryBtn.textContent).toContain("JSON (.json)");

    // Clicking primary button directly exports JSON
    fireEvent.click(primaryBtn);

    await waitFor(() => expect(mocks.save).toHaveBeenCalled());
    expect(mocks.save).toHaveBeenCalledWith({
      filters: [
        {
          name: "JSON file",
          extensions: ["json"],
        },
      ],
      defaultPath: expect.stringMatching(/ExcelGrep_Results_.*\.json$/),
    });
    expect(mocks.invoke).toHaveBeenCalledWith("export_results", {
      request: {
        format: "json",
        output_path: "C:\\dest\\export.json",
        items: [mockItem],
        language: "en",
        llm_options: undefined,
      },
    });
  });
});
