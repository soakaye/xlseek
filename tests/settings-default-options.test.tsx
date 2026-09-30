/**
 * Description: Verifies saved search-option drafts, validation, reset, and worker-count memory.
 * Arguments & Returns: Vitest renders SettingsDialog with controlled preferences; tests return no values.
 * Errors: Assertions fail if invalid or discarded settings are saved or valid values are lost.
 */
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { parse } from "yaml";
import enSource from "../src-tauri/locales/en.yml?raw";
import jaSource from "../src-tauri/locales/ja.yml?raw";
import { initializeI18n, LocaleProvider } from "../src/i18n";
import { SettingsDialog } from "../src/components/settings/SettingsDialog";
import { DEFAULT_SEARCH_OPTIONS } from "../src/constants";
import { DefaultSearchOptions } from "../src/types/defaultOptions";

const plugin = vi.hoisted(() => ({ language: "en", catalogs: {} as Record<string, Record<string, string>> }));
vi.mock("@razein97/tauri-plugin-i18n", () => ({
  default: {
    getInstance: () => ({
      load: async () => undefined,
      translate: (key: string) => plugin.catalogs[plugin.language]?.[key] ?? key,
    }),
    setLocale: async (language: string) => { plugin.language = language; },
  },
}));

describe("SettingsDialog - Default Search Options", () => {
  afterEach(cleanup);
  beforeEach(async () => {
    plugin.language = "en";
    plugin.catalogs = {
      en: parse(enSource) as Record<string, string>,
      ja: parse(jaSource) as Record<string, string>,
    };
    await initializeI18n("en");
  });

  const baseProps = {
    isOpen: true,
    preference: "default" as const,
    language: "en" as const,
    onSelect: () => undefined,
    onClose: () => undefined,
    maxEntries: 20,
    defaultOptions: DEFAULT_SEARCH_OPTIONS as DefaultSearchOptions,
    rememberedBurstWorkers: null,
    onSaveSettings: vi.fn(() => true),
  };

  /** Description: Selects the saved preferences page for settings assertions. Arguments & Returns: Takes no arguments; returns void. Errors: Testing Library throws if the page tab is missing. */
  const openSavedPage = () => fireEvent.click(screen.getByRole("tab", { name: "Search settings · Save to apply" }));

  it("renders default search options with initial state", () => {
    render(
      <LocaleProvider value="en">
        <SettingsDialog {...baseProps} />
      </LocaleProvider>
    );
    openSavedPage();

    expect(screen.getByText("Default Search Options")).toBeDefined();
    const formulaCheckbox = screen.getByRole("checkbox", { name: "Formula" }) as HTMLInputElement;
    expect(formulaCheckbox.checked).toBe(true);

    const shapeCheckbox = screen.getByRole("checkbox", { name: "Shape text" }) as HTMLInputElement;
    expect(shapeCheckbox.checked).toBe(false);

    const xlsxBtn = screen.getByRole("button", { name: ".xlsx" });
    expect(xlsxBtn).toBeDefined();
    expect((screen.getByRole("radio", { name: "Sequential" }) as HTMLInputElement).checked).toBe(true);
    expect(screen.getByRole("radio", { name: "Burst" })).toBeDefined();
    expect(screen.getByRole("radio", { name: "Automatic" })).toBeDefined();
  });

  it("saves a custom Burst worker count", () => {
    const onSaveSettings = vi.fn(() => true);
    render(
      <LocaleProvider value="en">
        <SettingsDialog {...baseProps} onSaveSettings={onSaveSettings} />
      </LocaleProvider>
    );
    openSavedPage();
    fireEvent.click(screen.getByRole("radio", { name: "Burst" }));
    fireEvent.click(screen.getByRole("radio", { name: "Custom" }));
    const count = screen.getByRole("spinbutton", { name: "Burst parallel count" });
    fireEvent.change(count, { target: { value: "6" } });
    fireEvent.click(screen.getByRole("button", { name: "Save Settings" }));
    expect(onSaveSettings).toHaveBeenCalledWith(expect.objectContaining({
      defaultOptions: { ...DEFAULT_SEARCH_OPTIONS, directory_mode: "burst", burst_workers: 6 },
    }));
  });

  it("blocks saving custom counts outside the supported range", () => {
    const onSaveSettings = vi.fn(() => true);
    render(
      <LocaleProvider value="en">
        <SettingsDialog {...baseProps} onSaveSettings={onSaveSettings} />
      </LocaleProvider>
    );
    openSavedPage();
    fireEvent.click(screen.getByRole("radio", { name: "Burst" }));
    fireEvent.click(screen.getByRole("radio", { name: "Custom" }));
    const count = screen.getByRole("spinbutton", { name: "Burst parallel count" });
    for (const value of ["", "1", "33", "2.5"]) {
      fireEvent.change(count, { target: { value } });
      expect((screen.getByRole("button", { name: "Save Settings" }) as HTMLButtonElement).disabled).toBe(true);
      expect(onSaveSettings).not.toHaveBeenCalled();
    }
  });

  it.each(["0", "50"]) ("accepts history limit boundary %s", (value) => {
    const onSaveSettings = vi.fn(() => true);
    render(<LocaleProvider value="en"><SettingsDialog {...baseProps} onSaveSettings={onSaveSettings} /></LocaleProvider>);
    openSavedPage();
    fireEvent.change(screen.getByLabelText("Search history limit"), { target: { value } });
    fireEvent.click(screen.getByRole("button", { name: "Save Settings" }));
    expect(onSaveSettings).toHaveBeenCalledWith(expect.objectContaining({ maxEntries: Number(value) }));
  });

  it.each(["2", "32"]) ("accepts custom worker boundary %s", (value) => {
    const onSaveSettings = vi.fn(() => true);
    render(<LocaleProvider value="en"><SettingsDialog {...baseProps} onSaveSettings={onSaveSettings} /></LocaleProvider>);
    openSavedPage();
    fireEvent.click(screen.getByRole("radio", { name: "Burst" }));
    fireEvent.click(screen.getByRole("radio", { name: "Custom" }));
    fireEvent.change(screen.getByRole("spinbutton", { name: "Burst parallel count" }), { target: { value } });
    fireEvent.click(screen.getByRole("button", { name: "Save Settings" }));
    expect(onSaveSettings).toHaveBeenCalledWith(expect.objectContaining({
      defaultOptions: expect.objectContaining({ burst_workers: Number(value) }),
    }));
  });

  it("modifies toggles, resets to defaults, and saves", () => {
    const onSaveSettings = vi.fn(() => true);
    render(
      <LocaleProvider value="en">
        <SettingsDialog {...baseProps} onSaveSettings={onSaveSettings} />
      </LocaleProvider>
    );
    openSavedPage();

    // Toggle formula OFF
    const formulaCheckbox = screen.getByRole("checkbox", { name: "Formula" });
    fireEvent.click(formulaCheckbox);
    expect((formulaCheckbox as HTMLInputElement).checked).toBe(false);

    // Click Reset
    const resetBtn = screen.getByRole("button", { name: "Reset to Defaults" });
    fireEvent.click(resetBtn);
    expect((formulaCheckbox as HTMLInputElement).checked).toBe(true);

    // Click Save Options
    const saveBtn = screen.getByRole("button", { name: "Save Settings" });
    fireEvent.click(saveBtn);
    expect(onSaveSettings).toHaveBeenCalledWith(expect.objectContaining({ defaultOptions: DEFAULT_SEARCH_OPTIONS }));
  });

  it("disables save button when no extensions are selected", () => {
    render(
      <LocaleProvider value="en">
        <SettingsDialog {...baseProps} />
      </LocaleProvider>
    );
    openSavedPage();

    // Deselect all extensions
    [".xlsx", ".xlsm", ".xlsb", ".xls"].forEach((ext) => {
      const btn = screen.getByRole("button", { name: ext });
      fireEvent.click(btn);
    });

    const saveBtn = screen.getByRole("button", { name: "Save Settings" }) as HTMLButtonElement;
    expect(saveBtn.disabled).toBe(true);
    expect(screen.getByText("Select at least one target extension")).toBeDefined();
  });

  it("retains the last valid manual count through sequential and automatic modes", () => {
    const onSaveSettings = vi.fn(() => true);
    render(<LocaleProvider value="en"><SettingsDialog {...baseProps} onSaveSettings={onSaveSettings} /></LocaleProvider>);
    openSavedPage();
    fireEvent.click(screen.getByRole("radio", { name: "Burst" }));
    fireEvent.click(screen.getByRole("radio", { name: "Custom" }));
    const count = screen.getByRole("spinbutton", { name: "Burst parallel count" });
    fireEvent.change(count, { target: { value: "8" } });
    fireEvent.click(screen.getByRole("radio", { name: "Sequential" }));
    fireEvent.click(screen.getByRole("button", { name: "Save Settings" }));
    expect(onSaveSettings.mock.lastCall?.[0].defaultOptions.burst_workers).toBe(8);
    expect(onSaveSettings.mock.lastCall?.[0].rememberedBurstWorkers).toBe(8);
  });

  it("resets only the saved-page draft while retaining history and immediate language", () => {
    const onSaveSettings = vi.fn(() => true);
    const onSelect = vi.fn();
    render(<LocaleProvider value="en"><SettingsDialog {...baseProps} onSelect={onSelect} onSaveSettings={onSaveSettings} /></LocaleProvider>);
    fireEvent.click(screen.getByRole("radio", { name: "日本語" }));
    fireEvent.click(screen.getByRole("tab", { name: "Search settings · Save to apply" }));
    fireEvent.change(screen.getByLabelText("Search history limit"), { target: { value: "6" } });
    fireEvent.click(screen.getByRole("radio", { name: "Burst" }));
    fireEvent.click(screen.getByRole("radio", { name: "Custom" }));
    fireEvent.change(screen.getByRole("spinbutton", { name: "Burst parallel count" }), { target: { value: "8" } });
    fireEvent.click(screen.getByRole("button", { name: "Reset to Defaults" }));

    expect((screen.getByLabelText("Search history limit") as HTMLInputElement).value).toBe("6");
    expect(onSelect).toHaveBeenCalledWith("ja");
    expect(onSaveSettings).not.toHaveBeenCalled();
    fireEvent.click(screen.getByRole("button", { name: "Save Settings" }));
    expect(onSaveSettings).toHaveBeenCalledWith(expect.objectContaining({ maxEntries: 6, rememberedBurstWorkers: null }));
  });
});
