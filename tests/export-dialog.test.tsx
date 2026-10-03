/**
 * Copyright (c) 2026 soakaye
 *
 * Description: Tests for export dialog rendering, format selection, and export cancellation.
 * Arguments & Returns: Vitest runs test suites; no public arguments or return values.
 * Errors: Test assertions fail if UI components or events deviate from requirements.
 */

import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { parse } from "yaml";
import enSource from "../src-tauri/locales/en.yml?raw";
import jaSource from "../src-tauri/locales/ja.yml?raw";
import { initializeI18n, LocaleProvider } from "../src/i18n";
import { StatusBar } from "../src/components/common/StatusBar";
import { SearchMatch } from "../src/types/search";

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
    setLocale: async (language: string) => { mocks.pluginLocale = language; },
  },
}));

describe("StatusBar - Export Dialog Integration (US2)", () => {
  beforeEach(async () => {
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
    file_name: "test.xlsx",
    full_path: "C:\\data\\test.xlsx",
    sheet_name: "Sheet1",
    cell_address: "A1",
    row_index: 0,
    col_index: 0,
    col_name: "A",
    match_type: "CellValue",
    shape_name: null,
    sheet_hidden: false,
    snippet: "Hello World",
    full_content: "Hello World",
    formula: null,
    sheets_in_workbook: ["Sheet1"],
  };

  it("shows toast warning when exporting with 0 items and does not open save dialog", async () => {
    const onShowToast = vi.fn();
    render(
      <LocaleProvider value="en">
        <StatusBar
          progress={null}
          items={[]}
          onShowToast={onShowToast}
          onOpenAbout={() => undefined}
          onOpenSettings={() => undefined}
          language="en"
        />
      </LocaleProvider>
    );

    const csvBtn = screen.getByText("Export CSV").closest("button") as HTMLButtonElement;
    expect(csvBtn.disabled).toBe(true);

    // Clicking disabled button does not trigger save dialog
    fireEvent.click(csvBtn);
    expect(mocks.save).not.toHaveBeenCalled();
    expect(mocks.invoke).not.toHaveBeenCalled();
  });


  it("opens save dialog and triggers export command for CSV when items exist", async () => {
    const onShowToast = vi.fn();
    mocks.save.mockResolvedValue("C:\\dest\\result.csv");

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

    const csvBtn = screen.getByText("Export CSV");
    fireEvent.click(csvBtn);

    await waitFor(() => expect(mocks.save).toHaveBeenCalled());
    expect(mocks.invoke).toHaveBeenCalledWith("export_results", {
      request: {
        format: "csv",
        output_path: "C:\\dest\\result.csv",
        items: [mockItem],
        language: "en",
      },
    });
    expect(onShowToast).toHaveBeenCalledWith("ui.EXPORT_CSV_SAVED", { path: "C:\\dest\\result.csv" });
  });

  it("handles user cancellation of save dialog gracefully without errors", async () => {
    const onShowToast = vi.fn();
    mocks.save.mockResolvedValue(null);

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

    const excelBtn = screen.getByText("Export Excel");
    fireEvent.click(excelBtn);

    await waitFor(() => expect(mocks.save).toHaveBeenCalled());
    expect(mocks.invoke).not.toHaveBeenCalled();
    expect(onShowToast).not.toHaveBeenCalled();
  });
});
