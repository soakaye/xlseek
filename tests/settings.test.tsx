/**
 * Description: Unit and integration tests for settings.test.tsx.
 * Arguments & Returns: Vitest runs test suites; no public arguments or return values.
 * Errors: Test assertions fail if behavior deviates from requirements.
 */
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { parse } from "yaml";
import enSource from "../src-tauri/locales/en.yml?raw";
import jaSource from "../src-tauri/locales/ja.yml?raw";
import { initializeI18n, LocaleProvider, setI18nLocale } from "../src/i18n";
import { SettingsDialog } from "../src/components/settings/SettingsDialog";

const plugin = vi.hoisted(() => ({ language: "en", catalogs: {} as Record<string, Record<string, string>> }));
vi.mock("@razein97/tauri-plugin-i18n", () => ({
  default: {
    getInstance: () => ({ load: async () => undefined, translate: (key: string) => plugin.catalogs[plugin.language]?.[key] ?? key }),
    setLocale: async (language: string) => { plugin.language = language; },
  },
}));

describe("SettingsDialog", () => {
  afterEach(cleanup);
  beforeEach(async () => {
    plugin.language = "en";
    plugin.catalogs = { en: parse(enSource) as Record<string, string>, ja: parse(jaSource) as Record<string, string> };
    await initializeI18n("en");
  });

  it("shows the localized default option and selects a language", () => {
    const onSelect = vi.fn();
    render(<LocaleProvider value="en"><SettingsDialog isOpen preference="default" language="en" onSelect={onSelect} onClose={() => undefined} maxEntries={20} onSetMaxEntries={() => undefined} /></LocaleProvider>);
    expect((screen.getByRole("radio", { name: "Default" }) as HTMLInputElement).checked).toBe(true);
    fireEvent.click(screen.getByRole("radio", { name: "日本語" }));
    expect(onSelect).toHaveBeenCalledWith("ja");
  });

  it("closes from the keyboard with Escape", async () => {
    const onClose = vi.fn();
    plugin.language = "ja";
    await setI18nLocale("ja");
    render(<LocaleProvider value="ja"><SettingsDialog isOpen preference="default" language="ja" onSelect={() => undefined} onClose={onClose} maxEntries={20} onSetMaxEntries={() => undefined} /></LocaleProvider>);
    fireEvent.keyDown(screen.getByRole("radio", { name: "デフォルト" }), { key: "Escape" });
    expect(onClose).toHaveBeenCalledOnce();
  });

  it("commits only integer limits from zero through fifty", () => {
    const onSetMaxEntries = vi.fn();
    render(<LocaleProvider value="en"><SettingsDialog isOpen preference="default" language="en" onSelect={() => undefined} onClose={() => undefined} maxEntries={20} onSetMaxEntries={onSetMaxEntries} /></LocaleProvider>);
    const input = screen.getByLabelText("Search history limit");
    const save = screen.getByRole("button", { name: "Save limit" });
    expect((input as HTMLInputElement).value).toBe("20");
    fireEvent.change(input, { target: { value: "51" } });
    expect((save as HTMLButtonElement).disabled).toBe(true);
    fireEvent.change(input, { target: { value: "2.5" } });
    expect((save as HTMLButtonElement).disabled).toBe(true);
    fireEvent.change(input, { target: { value: "0" } });
    fireEvent.click(save);
    expect(onSetMaxEntries).toHaveBeenCalledWith(0);
  });
});
