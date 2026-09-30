/**
 * Description: Verifies combined settings persistence and its effect on future searches in the App.
 * Arguments & Returns: Vitest renders the app with controlled browser storage and backend events; tests return no values.
 * Errors: Assertions fail if saving changes the active search or fails to commit preferences atomically.
 */
import { act, cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { parse } from "yaml";
import enSource from "../src-tauri/locales/en.yml?raw";
import jaSource from "../src-tauri/locales/ja.yml?raw";
import App from "../src/App";
import { DEFAULT_OPTIONS_STORAGE_KEY, EVENT_NAMES, SEARCH_HISTORY_CONSTANTS } from "../src/constants";

const mocks = vi.hoisted(() => ({
  locale: vi.fn<() => Promise<string | null>>(),
  invoke: vi.fn(async () => undefined),
  listen: vi.fn(async () => () => undefined),
  loadTranslations: vi.fn<() => Promise<void>>(),
  pluginSetLocale: vi.fn<(language: string) => Promise<void>>(),
  pluginTranslate: vi.fn<(key: string) => string>(),
  pluginLocale: "en",
  catalogs: {} as Record<string, Record<string, string>>,
  listeners: {} as Record<string, (event: { payload: unknown }) => void>,
}));

const storageMap = new Map<string, string>();
let rejectWrites = false;
vi.mock("@tauri-apps/plugin-os", () => ({ locale: mocks.locale }));
vi.mock("@tauri-apps/api/core", () => ({ invoke: mocks.invoke }));
vi.mock("@tauri-apps/api/event", () => ({ listen: mocks.listen }));
vi.mock("@razein97/tauri-plugin-i18n", () => ({
  default: {
    getInstance: () => ({ load: mocks.loadTranslations, translate: mocks.pluginTranslate }),
    setLocale: mocks.pluginSetLocale,
  },
}));
vi.mock("@tauri-apps/plugin-dialog", () => ({ open: vi.fn(), save: vi.fn() }));
vi.mock("@tanstack/react-virtual", () => ({
  useVirtualizer: ({ count }: { count: number }) => ({
    getTotalSize: () => count * 36,
    getVirtualItems: () => Array.from({ length: count }, (_, index) => ({ key: index, index, size: 36, start: index * 36 })),
    scrollToIndex: () => undefined,
  }),
}));
vi.mock("@tauri-apps/api/webview", () => ({
  getCurrentWebview: () => ({ onDragDropEvent: async () => () => undefined }),
}));

describe("App - Default Search Options Integration", () => {
  beforeEach(() => {
    mocks.listeners = {};
    storageMap.clear();
    rejectWrites = false;
    mocks.locale.mockReset().mockResolvedValue("en-US");
    mocks.invoke.mockReset().mockResolvedValue(undefined);
    mocks.listen.mockReset().mockImplementation(async (event, callback) => {
      mocks.listeners[event] = callback as (event: { payload: unknown }) => void;
      return () => undefined;
    });
    mocks.loadTranslations.mockReset().mockResolvedValue(undefined);
    mocks.pluginSetLocale.mockReset().mockResolvedValue(undefined);
    mocks.pluginLocale = "en";
    mocks.catalogs = { en: parse(enSource) as Record<string, string>, ja: parse(jaSource) as Record<string, string> };
    mocks.pluginSetLocale.mockImplementation(async (language) => { mocks.pluginLocale = language; });
    mocks.pluginTranslate.mockReset().mockImplementation((key) => mocks.catalogs[mocks.pluginLocale]?.[key] ?? key);

    Object.defineProperty(window, "localStorage", {
      configurable: true,
      value: {
        getItem: (key: string) => storageMap.get(key) ?? null,
        setItem: (key: string, value: string) => { if (rejectWrites) throw new Error("storage unavailable"); storageMap.set(key, value); },
        removeItem: (key: string) => { storageMap.delete(key); },
        clear: () => storageMap.clear(),
      } as Storage,
    });
  });
  afterEach(cleanup);

  it("initializes search options from custom saved defaults in localStorage", async () => {
    const customDefaults = {
      match_case: true,
      use_regex: false,
      include_formula: false,
      include_shape: true,
      include_comment: true,
      include_hidden: true,
      extensions: [".xlsx", ".xlsm"],
    };
    storageMap.set(DEFAULT_OPTIONS_STORAGE_KEY, JSON.stringify(customDefaults));

    render(<App />);
    await waitFor(() => expect(screen.getByPlaceholderText("Enter text or a regular expression... (Enter to search)")).toBeDefined());

    // Check that custom defaults are reflected on SearchBar
    const caseCheckbox = screen.getByRole("checkbox", { name: "Match case" }) as HTMLInputElement;
    expect(caseCheckbox.checked).toBe(true);

    const formulaCheckbox = screen.getByRole("checkbox", { name: "Formula" }) as HTMLInputElement;
    expect(formulaCheckbox.checked).toBe(false);

    const shapeCheckbox = screen.getByRole("checkbox", { name: "Shape text" }) as HTMLInputElement;
    expect(shapeCheckbox.checked).toBe(true);

    const hiddenCheckbox = screen.getByRole("checkbox", { name: "Hidden sheets" }) as HTMLInputElement;
    expect(hiddenCheckbox.checked).toBe(true);
  });

  it("updates SearchBar immediately when default options are saved in SettingsDialog", async () => {
    const app = render(<App />);
    const query = await screen.findByPlaceholderText("Enter text or a regular expression... (Enter to search)");
    fireEvent.change(query, { target: { value: "preserve this query" } });
    fireEvent.change(screen.getByPlaceholderText("Select or drop a folder..."), { target: { value: "/reports" } });
    fireEvent.click(screen.getByRole("button", { name: "SEARCH" }));
    expect(screen.getByText("Preparing scan...")).toBeTruthy();

    // Open settings dialog
    const settingsBtn = screen.getByTitle("Settings");
    fireEvent.click(settingsBtn);
    fireEvent.click(screen.getByRole("tab", { name: "Search settings · Save to apply" }));

    expect(screen.getByText("Default Search Options")).toBeDefined();

    // Toggle formula OFF in SettingsDialog
    const settingsFormula = screen.getAllByRole("checkbox", { name: "Formula" })[1]; // Dialog is second
    fireEvent.click(settingsFormula);

    // Save in dialog
    const saveBtn = screen.getByRole("button", { name: "Save Settings" });
    fireEvent.click(saveBtn);

    // Main search bar formula checkbox should now be unchecked
    const mainFormula = screen.getAllByRole("checkbox", { name: "Formula" })[0];
    expect((mainFormula as HTMLInputElement).checked).toBe(false);
    expect((screen.getByPlaceholderText("Enter text or a regular expression... (Enter to search)") as HTMLInputElement).value).toBe("preserve this query");
    expect(screen.getByText("Preparing scan...")).toBeTruthy();
    expect(storageMap.has(DEFAULT_OPTIONS_STORAGE_KEY)).toBe(false);
    expect(storageMap.has(SEARCH_HISTORY_CONSTANTS.STORAGE_KEY)).toBe(true);
    app.unmount();
    render(<App />);
    await waitFor(() => expect((screen.getByRole("checkbox", { name: "Formula" }) as HTMLInputElement).checked).toBe(false));
  });

  it("retains a failed draft and allows a successful retry", async () => {
    render(<App />);
    await screen.findByPlaceholderText("Enter text or a regular expression... (Enter to search)");
    fireEvent.click(screen.getByTitle("Settings"));
    fireEvent.click(screen.getByRole("tab", { name: "Search settings · Save to apply" }));
    fireEvent.change(screen.getByLabelText("Search history limit"), { target: { value: "0" } });
    fireEvent.click(screen.getAllByRole("checkbox", { name: "Formula" })[1]);
    rejectWrites = true;
    fireEvent.click(screen.getByRole("button", { name: "Save Settings" }));
    expect(screen.getByRole("dialog")).toBeTruthy();
    expect((screen.getByLabelText("Search history limit") as HTMLInputElement).value).toBe("0");
    expect(screen.getByText("Could not save settings. The saved preference will be used next time.")).toBeTruthy();
    rejectWrites = false;
    fireEvent.click(screen.getByRole("button", { name: "Save Settings" }));
    await waitFor(() => expect(screen.queryByRole("dialog")).toBeNull());
    const saved = JSON.parse(storageMap.get(SEARCH_HISTORY_CONSTANTS.STORAGE_KEY) ?? "{}");
    expect(saved.maxEntries).toBe(0);
    expect(saved.defaultOptions.include_formula).toBe(false);
  });

  it("guards a reentrant save event while replacing the settings record", async () => {
    render(<App />);
    await screen.findByPlaceholderText("Enter text or a regular expression... (Enter to search)");
    fireEvent.click(screen.getByTitle("Settings"));
    fireEvent.click(screen.getByRole("tab", { name: "Search settings · Save to apply" }));
    const save = screen.getByRole("button", { name: "Save Settings" });
    const originalSetItem = window.localStorage.setItem.bind(window.localStorage);
    const setItem = vi.spyOn(window.localStorage, "setItem").mockImplementation((key, value) => {
      originalSetItem(key, value);
      if (key === SEARCH_HISTORY_CONSTANTS.STORAGE_KEY) fireEvent.click(save);
    });
    fireEvent.click(save);
    expect(setItem).toHaveBeenCalledOnce();
    await waitFor(() => expect(screen.queryByRole("dialog")).toBeNull());
  });

  it("applies saved defaults to the next search without clearing a live scan or selected result", async () => {
    render(<App />);
    const query = await screen.findByPlaceholderText("Enter text or a regular expression... (Enter to search)");
    fireEvent.change(query, { target: { value: "shape" } });
    fireEvent.change(screen.getByPlaceholderText("Select or drop a folder..."), { target: { value: "/reports" } });
    await waitFor(() => expect(mocks.listeners[EVENT_NAMES.SEARCH_MATCH]).toBeDefined());
    fireEvent.click(screen.getByRole("button", { name: "SEARCH" }));
    const match = {
      id: 1, file_name: "report.xlsx", full_path: "/reports/report.xlsx", sheet_name: "Summary",
      cell_address: "A1", row_index: 0, col_index: 0, col_name: "A", match_type: "Shape",
      shape_name: "Report title", sheet_hidden: false, snippet: "shape", full_content: "shape result",
      formula: null, sheets_in_workbook: ["Summary"],
    };
    act(() => {
      mocks.listeners[EVENT_NAMES.SEARCH_MATCH]?.({ payload: match });
      mocks.listeners[EVENT_NAMES.SCAN_PROGRESS]?.({ payload: {
        state: "Completed", phase: "finished", error_code: null, scanned_files: 1, total_files: 1,
        matches_found: 1, current_file: "", elapsed_ms: 1,
      } });
    });
    const file = await screen.findByText("report.xlsx");
    fireEvent.click(file.closest(".flex.items-center") as HTMLElement);
    act(() => mocks.listeners[EVENT_NAMES.SCAN_PROGRESS]?.({ payload: {
      state: "Scanning", phase: "scanning", error_code: null, scanned_files: 1, total_files: 4,
      matches_found: 1, current_file: "next.xlsx", elapsed_ms: 2,
    } }));
    expect(screen.getByText(/Scanning:/)).toBeTruthy();

    fireEvent.click(screen.getByRole("button", { name: "Settings" }));
    fireEvent.click(screen.getByRole("tab", { name: "Search settings · Save to apply" }));
    fireEvent.click(screen.getAllByRole("checkbox", { name: "Formula" })[1]);
    fireEvent.click(screen.getByRole("button", { name: "Save Settings" }));

    expect(screen.getByText(/Scanning:/)).toBeTruthy();
    expect((screen.getByPlaceholderText("Enter text or a regular expression... (Enter to search)") as HTMLInputElement).value).toBe("shape");
    expect(screen.getAllByText("report.xlsx")[0].closest(".flex.items-center")?.className).toContain("bg-emerald-950/70");
    expect(JSON.parse(storageMap.get(SEARCH_HISTORY_CONSTANTS.STORAGE_KEY) ?? "{}").defaultOptions.include_formula).toBe(false);
  });
});
