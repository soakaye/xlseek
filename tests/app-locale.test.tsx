import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import App from "../src/App";

const mocks = vi.hoisted(() => ({
  locale: vi.fn<() => Promise<string | null>>(),
  invoke: vi.fn(async () => undefined),
  listen: vi.fn(async () => () => undefined),
}));
vi.mock("@tauri-apps/plugin-os", () => ({ locale: mocks.locale }));
vi.mock("@tauri-apps/api/core", () => ({ invoke: mocks.invoke }));
vi.mock("@tauri-apps/api/event", () => ({ listen: mocks.listen }));
vi.mock("@tauri-apps/plugin-dialog", () => ({ open: vi.fn(), save: vi.fn() }));
vi.mock("@tauri-apps/api/webview", () => ({
  getCurrentWebview: () => ({ onDragDropEvent: async () => () => undefined }),
}));

describe("App localization", () => {
  beforeEach(() => {
    mocks.locale.mockReset().mockResolvedValue("en-US");
    mocks.invoke.mockReset().mockResolvedValue(undefined);
    mocks.listen.mockReset().mockResolvedValue(() => undefined);
    Object.defineProperty(window, "localStorage", {
      configurable: true,
      value: { getItem: () => null, setItem: () => undefined } as Storage,
    });
  });
  afterEach(cleanup);

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
});
