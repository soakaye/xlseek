/**
 * Description: Unit and integration tests for settings.test.tsx.
 * Arguments & Returns: Vitest runs test suites; no public arguments or return values.
 * Errors: Test assertions fail if behavior deviates from requirements.
 */
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { parse } from "yaml";
import enSource from "../src-tauri/locales/en.yml?raw";
import jaSource from "../src-tauri/locales/ja.yml?raw";
import { initializeI18n, LocaleProvider, setI18nLocale } from "../src/i18n";
import { SettingsDialog } from "../src/components/settings/SettingsDialog";
import { DEFAULT_SEARCH_OPTIONS } from "../src/constants";

const settingsProps = {
  isOpen: true, preference: "default" as const, language: "en" as const, onSelect: () => undefined,
  onClose: () => undefined, maxEntries: 20, defaultOptions: DEFAULT_SEARCH_OPTIONS,
  rememberedBurstWorkers: null, onSaveSettings: () => true,
};

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
    render(<LocaleProvider value="en"><SettingsDialog {...settingsProps} onSelect={onSelect} /></LocaleProvider>);
    expect(screen.getByRole("tab", { name: "Display language · Apply immediately" }).getAttribute("aria-selected")).toBe("true");
    expect(screen.queryByRole("button", { name: "Save Settings" })).toBeNull();
    expect((screen.getByRole("radio", { name: "Default" }) as HTMLInputElement).checked).toBe(true);
    fireEvent.click(screen.getByRole("radio", { name: "日本語" }));
    expect(onSelect).toHaveBeenCalledWith("ja");
  });

  it("closes from the keyboard with Escape", async () => {
    const onClose = vi.fn();
    plugin.language = "ja";
    await setI18nLocale("ja");
    render(<LocaleProvider value="ja"><SettingsDialog {...settingsProps} language="ja" onClose={onClose} />
    </LocaleProvider>);
    fireEvent.keyDown(screen.getByRole("radio", { name: "デフォルト" }), { key: "Escape" });
    expect(onClose).toHaveBeenCalledOnce();
  });

  it("closes from the header and immediate-page action without saving", () => {
    const onClose = vi.fn();
    const onSaveSettings = vi.fn(() => true);
    const { rerender } = render(<LocaleProvider value="en"><SettingsDialog {...settingsProps} onClose={onClose} onSaveSettings={onSaveSettings} /></LocaleProvider>);
    fireEvent.click(screen.getAllByRole("button", { name: "Close" })[0]);
    expect(onClose).toHaveBeenCalledOnce();
    expect(onSaveSettings).not.toHaveBeenCalled();
    rerender(<LocaleProvider value="en"><SettingsDialog {...settingsProps} isOpen={false} onClose={onClose} onSaveSettings={onSaveSettings} /></LocaleProvider>);
    rerender(<LocaleProvider value="en"><SettingsDialog {...settingsProps} onClose={onClose} onSaveSettings={onSaveSettings} /></LocaleProvider>);
    fireEvent.click(screen.getAllByRole("button", { name: "Close" }).at(-1)!);
    expect(onClose).toHaveBeenCalledTimes(2);
    expect(onSaveSettings).not.toHaveBeenCalled();
  });

  it("saves the history limit and defaults together once", () => {
    const onSaveSettings = vi.fn(() => true);
    render(<LocaleProvider value="en"><SettingsDialog {...settingsProps} onSaveSettings={onSaveSettings} /></LocaleProvider>);
    fireEvent.click(screen.getByRole("tab", { name: "Search settings · Save to apply" }));
    const input = screen.getByLabelText("Search history limit");
    fireEvent.change(input, { target: { value: "0" } });
    fireEvent.click(screen.getByRole("button", { name: "Save Settings" }));
    expect(onSaveSettings).toHaveBeenCalledOnce();
    expect(onSaveSettings).toHaveBeenCalledWith(expect.objectContaining({ maxEntries: 0 }));
  });

  it("discards changes on cancel and keeps drafts after a failed save", () => {
    const onClose = vi.fn();
    const onSaveSettings = vi.fn(() => false);
    const { rerender } = render(<LocaleProvider value="en"><SettingsDialog {...settingsProps} onClose={onClose} onSaveSettings={onSaveSettings} /></LocaleProvider>);
    fireEvent.click(screen.getByRole("tab", { name: "Search settings · Save to apply" }));
    fireEvent.change(screen.getByLabelText("Search history limit"), { target: { value: "0" } });
    fireEvent.click(screen.getByRole("button", { name: "Save Settings" }));
    expect(onClose).not.toHaveBeenCalled();
    expect((screen.getByLabelText("Search history limit") as HTMLInputElement).value).toBe("0");
    fireEvent.click(screen.getByRole("button", { name: "Cancel" }));
    expect(onClose).toHaveBeenCalledOnce();
    rerender(<LocaleProvider value="en"><SettingsDialog {...settingsProps} isOpen={false} onClose={onClose} onSaveSettings={onSaveSettings} /></LocaleProvider>);
    rerender(<LocaleProvider value="en"><SettingsDialog {...settingsProps} onClose={onClose} onSaveSettings={onSaveSettings} /></LocaleProvider>);
    fireEvent.click(screen.getByRole("tab", { name: "Search settings · Save to apply" }));
    expect((screen.getByLabelText("Search history limit") as HTMLInputElement).value).toBe("20");
  });

  it("keeps an invalid draft while switching pages and discards it on backdrop exit", () => {
    const onClose = vi.fn();
    const { rerender } = render(<LocaleProvider value="en"><SettingsDialog {...settingsProps} onClose={onClose} /></LocaleProvider>);
    const tabs = screen.getAllByRole("tab");
    fireEvent.keyDown(tabs[0], { key: "ArrowRight" });
    expect(tabs[1].getAttribute("aria-selected")).toBe("true");
    fireEvent.change(screen.getByLabelText("Search history limit"), { target: { value: "51" } });
    fireEvent.keyDown(tabs[1], { key: "ArrowLeft" });
    expect(screen.getByRole("tab", { name: "Display language · Apply immediately" }).getAttribute("aria-selected")).toBe("true");
    fireEvent.keyDown(screen.getByRole("radio", { name: "English" }), { key: "Escape" });
    expect(onClose).toHaveBeenCalledOnce();
    rerender(<LocaleProvider value="en"><SettingsDialog {...settingsProps} isOpen={false} onClose={onClose} /></LocaleProvider>);
    rerender(<LocaleProvider value="en"><SettingsDialog {...settingsProps} onClose={onClose} /></LocaleProvider>);
    fireEvent.click(screen.getByRole("tab", { name: "Search settings · Save to apply" }));
    expect((screen.getByLabelText("Search history limit") as HTMLInputElement).value).toBe("20");
  });

  it("focuses the dialog, traps Tab, and restores launcher focus on close", async () => {
    const launcher = document.createElement("button");
    document.body.append(launcher);
    launcher.focus();
    const { rerender } = render(<LocaleProvider value="en"><SettingsDialog {...settingsProps} /></LocaleProvider>);
    const closeButtons = screen.getAllByRole("button", { name: "Close" });
    await waitFor(() => expect(document.activeElement).toBe(closeButtons[0]));
    const dialog = screen.getByRole("dialog");
    const focusable = dialog.querySelectorAll<HTMLElement>("button:not(:disabled), input:not(:disabled), [tabindex='0']");
    const first = focusable[0];
    const last = focusable[focusable.length - 1];
    last.focus();
    fireEvent.keyDown(last, { key: "Tab" });
    expect(document.activeElement).toBe(first);
    first.focus();
    fireEvent.keyDown(first, { key: "Tab", shiftKey: true });
    expect(document.activeElement).toBe(last);
    rerender(<LocaleProvider value="en"><SettingsDialog {...settingsProps} isOpen={false} /></LocaleProvider>);
    expect(document.activeElement).toBe(launcher);
    launcher.remove();
  });

  it("associates invalid history input with its localized field error", () => {
    render(<LocaleProvider value="en"><SettingsDialog {...settingsProps} /></LocaleProvider>);
    fireEvent.click(screen.getByRole("tab", { name: "Search settings · Save to apply" }));
    const field = screen.getByLabelText("Search history limit");
    fireEvent.change(field, { target: { value: "-1" } });
    expect(field.getAttribute("aria-invalid")).toBe("true");
    expect(field.getAttribute("aria-describedby")).toContain("history-limit-error");
    expect(screen.getByRole("alert").textContent).toContain("0 through 50");
  });
});
