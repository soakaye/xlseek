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
    onSetMaxEntries: () => undefined,
    defaultOptions: DEFAULT_SEARCH_OPTIONS as DefaultSearchOptions,
    onSaveDefaultOptions: vi.fn(),
  };

  it("renders default search options with initial state", () => {
    render(
      <LocaleProvider value="en">
        <SettingsDialog {...baseProps} />
      </LocaleProvider>
    );

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
    const onSaveDefaultOptions = vi.fn();
    render(
      <LocaleProvider value="en">
        <SettingsDialog {...baseProps} onSaveDefaultOptions={onSaveDefaultOptions} />
      </LocaleProvider>
    );
    fireEvent.click(screen.getByRole("radio", { name: "Burst" }));
    fireEvent.click(screen.getByRole("radio", { name: "Custom" }));
    const count = screen.getByRole("spinbutton", { name: "Burst parallel count" });
    fireEvent.change(count, { target: { value: "6" } });
    fireEvent.click(screen.getByRole("button", { name: "Save Options" }));
    expect(onSaveDefaultOptions).toHaveBeenCalledWith({
      ...DEFAULT_SEARCH_OPTIONS,
      directory_mode: "burst",
      burst_workers: 6,
    });
  });

  it("blocks saving custom counts outside the supported range", () => {
    const onSaveDefaultOptions = vi.fn();
    render(
      <LocaleProvider value="en">
        <SettingsDialog {...baseProps} onSaveDefaultOptions={onSaveDefaultOptions} />
      </LocaleProvider>
    );
    fireEvent.click(screen.getByRole("radio", { name: "Burst" }));
    fireEvent.click(screen.getByRole("radio", { name: "Custom" }));
    const count = screen.getByRole("spinbutton", { name: "Burst parallel count" });
    for (const value of ["1", "33", "2.5"]) {
      fireEvent.change(count, { target: { value } });
      expect((screen.getByRole("button", { name: "Save Options" }) as HTMLButtonElement).disabled).toBe(true);
      expect(onSaveDefaultOptions).not.toHaveBeenCalled();
    }
  });

  it("modifies toggles, resets to defaults, and saves", () => {
    const onSaveDefaultOptions = vi.fn();
    render(
      <LocaleProvider value="en">
        <SettingsDialog {...baseProps} onSaveDefaultOptions={onSaveDefaultOptions} />
      </LocaleProvider>
    );

    // Toggle formula OFF
    const formulaCheckbox = screen.getByRole("checkbox", { name: "Formula" });
    fireEvent.click(formulaCheckbox);
    expect((formulaCheckbox as HTMLInputElement).checked).toBe(false);

    // Click Reset
    const resetBtn = screen.getByRole("button", { name: "Reset to Defaults" });
    fireEvent.click(resetBtn);
    expect((formulaCheckbox as HTMLInputElement).checked).toBe(true);

    // Click Save Options
    const saveBtn = screen.getByRole("button", { name: "Save Options" });
    fireEvent.click(saveBtn);
    expect(onSaveDefaultOptions).toHaveBeenCalledWith(DEFAULT_SEARCH_OPTIONS);
  });

  it("disables save button when no extensions are selected", () => {
    render(
      <LocaleProvider value="en">
        <SettingsDialog {...baseProps} />
      </LocaleProvider>
    );

    // Deselect all extensions
    [".xlsx", ".xlsm", ".xlsb", ".xls"].forEach((ext) => {
      const btn = screen.getByRole("button", { name: ext });
      fireEvent.click(btn);
    });

    const saveBtn = screen.getByRole("button", { name: "Save Options" }) as HTMLButtonElement;
    expect(saveBtn.disabled).toBe(true);
    expect(screen.getByText("Select at least one target extension")).toBeDefined();
  });
});
