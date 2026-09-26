import { fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { setActiveLanguage } from "../src/constants";
import { SettingsDialog } from "../src/components/settings/SettingsDialog";

describe("SettingsDialog", () => {
  afterEach(() => setActiveLanguage("ja"));

  it("shows the localized default option and selects a language", () => {
    setActiveLanguage("en");
    const onSelect = vi.fn();
    render(<SettingsDialog isOpen preference="default" language="en" onSelect={onSelect} onClose={() => undefined} />);
    expect((screen.getByRole("radio", { name: "Default" }) as HTMLInputElement).checked).toBe(true);
    fireEvent.click(screen.getByRole("radio", { name: "日本語" }));
    expect(onSelect).toHaveBeenCalledWith("ja");
  });

  it("closes from the keyboard with Escape", () => {
    const onClose = vi.fn();
    render(<SettingsDialog isOpen preference="default" language="ja" onSelect={() => undefined} onClose={onClose} />);
    fireEvent.keyDown(screen.getByRole("radio", { name: "デフォルト" }), { key: "Escape" });
    expect(onClose).toHaveBeenCalledOnce();
  });
});
