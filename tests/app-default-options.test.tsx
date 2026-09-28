import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { parse } from "yaml";
import enSource from "../src-tauri/locales/en.yml?raw";
import jaSource from "../src-tauri/locales/ja.yml?raw";
import App from "../src/App";
import { DEFAULT_OPTIONS_STORAGE_KEY } from "../src/constants";

const mocks = vi.hoisted(() => ({
  locale: vi.fn<() => Promise<string | null>>(),
  invoke: vi.fn(async () => undefined),
  listen: vi.fn(async () => () => undefined),
  loadTranslations: vi.fn<() => Promise<void>>(),
  pluginSetLocale: vi.fn<(language: string) => Promise<void>>(),
  pluginTranslate: vi.fn<(key: string) => string>(),
  pluginLocale: "en",
  catalogs: {} as Record<string, Record<string, string>>,
}));

const storageMap = new Map<string, string>();
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
vi.mock("@tauri-apps/api/webview", () => ({
  getCurrentWebview: () => ({ onDragDropEvent: async () => () => undefined }),
}));

describe("App - Default Search Options Integration", () => {
  beforeEach(() => {
    storageMap.clear();
    mocks.locale.mockReset().mockResolvedValue("en-US");
    mocks.invoke.mockReset().mockResolvedValue(undefined);
    mocks.listen.mockReset().mockResolvedValue(() => undefined);
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
        setItem: (key: string, value: string) => { storageMap.set(key, value); },
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
    render(<App />);
    await waitFor(() => expect(screen.getByPlaceholderText("Enter text or a regular expression... (Enter to search)")).toBeDefined());

    // Open settings dialog
    const settingsBtn = screen.getByTitle("Settings");
    fireEvent.click(settingsBtn);

    expect(screen.getByText("Default Search Options")).toBeDefined();

    // Toggle formula OFF in SettingsDialog
    const settingsFormula = screen.getAllByRole("checkbox", { name: "Formula" })[1]; // Dialog is second
    fireEvent.click(settingsFormula);

    // Save in dialog
    const saveBtn = screen.getByRole("button", { name: "Save Options" });
    fireEvent.click(saveBtn);

    // Main search bar formula checkbox should now be unchecked
    const mainFormula = screen.getAllByRole("checkbox", { name: "Formula" })[0];
    expect((mainFormula as HTMLInputElement).checked).toBe(false);
  });
});
