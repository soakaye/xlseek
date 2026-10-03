/**
 * Copyright (c) 2026 soakaye
 *
 * Description: Verifies application locale selection, persistence fallback, and preservation of search state.
 * Arguments & Returns: Vitest renders the app with mocked locale, storage, and backend events; tests return no values.
 * Errors: Assertions fail if locale switching loses current search content or exposes untranslated keys.
 */
import { act, cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { parse } from "yaml";
import enSource from "../src-tauri/locales/en.yml?raw";
import jaSource from "../src-tauri/locales/ja.yml?raw";
import App from "../src/App";
import { EVENT_NAMES } from "../src/constants";

const mocks = vi.hoisted(() => ({
  locale: vi.fn<() => Promise<string | null>>(),
  invoke: vi.fn(async () => undefined),
  listen: vi.fn(async () => () => undefined),
  loadTranslations: vi.fn<() => Promise<void>>(),
  pluginSetLocale: vi.fn<(language: string) => Promise<void>>(),
  pluginTranslate: vi.fn<(key: string) => string>(),
  listeners: {} as Record<string, (event: { payload: unknown }) => void>,
  pluginLocale: "en",
  catalogs: {} as Record<string, Record<string, string>>,
}));
let savedPreference: string | null = null;
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

describe("App localization", () => {
  beforeEach(() => {
    mocks.listeners = {};
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
    savedPreference = null;
    Object.defineProperty(window, "localStorage", {
      configurable: true,
      value: { getItem: () => savedPreference, setItem: (_key: string, value: string) => { savedPreference = value; } } as Storage,
    });
  });
  afterEach(cleanup);

  it("does not render localized content until plugin catalogs are initialized", async () => {
    let finishLoading: (() => void) | undefined;
    mocks.loadTranslations.mockImplementation(() => new Promise((resolve) => { finishLoading = resolve; }));
    render(<App />);

    expect(screen.queryByPlaceholderText("Enter text or a regular expression... (Enter to search)")).toBeNull();
    expect(screen.queryByPlaceholderText("検索するテキストまたは正規表現を入力... (Enterで検索)")).toBeNull();

    await waitFor(() => expect(mocks.loadTranslations).toHaveBeenCalledOnce());
    finishLoading?.();
    await screen.findByPlaceholderText("Enter text or a regular expression... (Enter to search)");
    expect(mocks.loadTranslations).toHaveBeenCalledOnce();
    expect(mocks.pluginSetLocale).toHaveBeenCalledWith("en");
  });

  it.each(["en-US", "ja-JP"])("continues with bundled English strings after plugin failure under %s", async (systemLocale) => {
    mocks.locale.mockResolvedValue(systemLocale);
    mocks.loadTranslations.mockRejectedValue(new Error("plugin unavailable"));
    const diagnostic = vi.spyOn(console, "error").mockImplementation(() => undefined);

    render(<App />);

    expect(await screen.findByPlaceholderText("Enter text or a regular expression... (Enter to search)")).toBeTruthy();
    expect(diagnostic).toHaveBeenCalledWith(expect.stringContaining("Failed to load translation catalogs"));
    expect(screen.queryByText("ui.KEYWORD_INPUT_PLACEHOLDER")).toBeNull();
    diagnostic.mockRestore();
  });

  it("starts from system language, switches immediately, preserves the query, and updates the menu", async () => {
    render(<App />);
    const query = await screen.findByPlaceholderText("Enter text or a regular expression... (Enter to search)");
    fireEvent.click(screen.getByRole("button", { name: "SEARCH" }));
    expect(screen.getByText("Enter a search keyword")).toBeTruthy();
    fireEvent.change(query, { target: { value: "customer" } });
    fireEvent.click(screen.getByRole("button", { name: "Settings" }));
    fireEvent.click(screen.getByRole("radio", { name: "日本語" }));

    await waitFor(() => expect(screen.getByPlaceholderText("検索するテキストまたは正規表現を入力... (Enterで検索)")).toBeTruthy());
    expect(screen.getByText("検索キーワードを入力してください")).toBeTruthy();
    expect((screen.getByPlaceholderText("検索するテキストまたは正規表現を入力... (Enterで検索)") as HTMLInputElement).value).toBe("customer");
    expect(mocks.invoke).toHaveBeenCalledWith("set_menu_locale", { language: "ja" });
  });

  it("uses the system language when the saved preference is default", async () => {
    mocks.locale.mockResolvedValue("ja-JP");
    render(<App />);
    await screen.findByPlaceholderText("検索するテキストまたは正規表現を入力... (Enterで検索)");
    expect(savedPreference).toBeNull();
  });

  it("keeps the selected language for the current session if saving fails", async () => {
    Object.defineProperty(window, "localStorage", {
      configurable: true,
      value: { getItem: () => null, setItem: () => { throw new Error("storage unavailable"); } } as Storage,
    });
    render(<App />);
    await screen.findByPlaceholderText("Enter text or a regular expression... (Enter to search)");
    fireEvent.click(screen.getByRole("button", { name: "Settings" }));
    fireEvent.click(screen.getByRole("radio", { name: "日本語" }));

    expect(await screen.findByPlaceholderText("検索するテキストまたは正規表現を入力... (Enterで検索)")).toBeTruthy();
    expect(screen.getByText("設定を保存できませんでした。次回起動時は保存済み設定を使用します。")).toBeTruthy();
  });

  it("preserves displayed and selected results when changing language", async () => {
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
    const resultRow = file.closest(".flex.items-center") as HTMLElement;
    fireEvent.click(resultRow);
    expect(screen.getAllByText("report.xlsx")[0].closest(".flex.items-center")?.className).toContain("bg-emerald-950/70");
    fireEvent.click(screen.getByRole("button", { name: "Settings" }));
    fireEvent.click(screen.getByRole("radio", { name: "日本語" }));

    await screen.findByPlaceholderText("検索するテキストまたは正規表現を入力... (Enterで検索)");
    expect(screen.getAllByText("report.xlsx")).toHaveLength(2);
    expect(screen.getAllByText("shape result").length).toBeGreaterThan(0);
    expect(screen.getAllByText("report.xlsx")[0].closest(".flex.items-center")?.className).toContain("bg-emerald-950/70");
  });

  it.each(["ja", "en"] as const)("loads the saved %s preference before showing the screen", async (language) => {
    savedPreference = language;
    mocks.locale.mockResolvedValue("fr-FR");
    render(<App />);
    const placeholder = language === "ja"
      ? "検索するテキストまたは正規表現を入力... (Enterで検索)"
      : "Enter text or a regular expression... (Enter to search)";
    await screen.findByPlaceholderText(placeholder);
    fireEvent.click(screen.getByRole("button", { name: language === "ja" ? "設定" : "Settings" }));
    expect((screen.getByRole("radio", { name: language === "ja" ? "日本語" : "English" }) as HTMLInputElement).checked).toBe(true);
  });
});
