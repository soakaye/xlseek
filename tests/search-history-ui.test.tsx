/**
 * Description: Unit and integration tests for search-history-ui.test.tsx.
 * Arguments & Returns: Vitest runs test suites; no public arguments or return values.
 * Errors: Test assertions fail if behavior deviates from requirements.
 */
import { act, cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { open } from "@tauri-apps/plugin-dialog";
import { KEYBOARD_KEYS, SEARCH_HISTORY_CONSTANTS } from "../src/constants";
import { SearchBar } from "../src/components/search/SearchBar";

const mocks = vi.hoisted(() => ({ invoke: vi.fn() }));
vi.mock("@tauri-apps/api/core", () => ({ invoke: mocks.invoke }));
vi.mock("@tauri-apps/plugin-dialog", () => ({ open: vi.fn() }));
vi.mock("@tauri-apps/api/webview", () => ({
  getCurrentWebview: () => ({ onDragDropEvent: async () => () => undefined }),
}));
vi.mock("../src/i18n", () => ({
  useTranslation: () => (key: string) => key,
}));

const query = {
  keyword: "invoice",
  target_dir: "/reports",
  match_case: false,
  use_regex: false,
  include_formula: true,
  include_comment: true,
  include_hidden: false,
  extensions: [".xlsx"],
};

describe("SearchBar history and path completion", () => {
  beforeEach(() => mocks.invoke.mockReset().mockResolvedValue([]));
  afterEach(() => { cleanup(); vi.restoreAllMocks(); });

  it("selects a keyword history item without starting a search", () => {
    const onSearch = vi.fn();
    const onSelectHistory = vi.fn();
    render(<SearchBar
      query={query}
      onChangeQuery={() => undefined}
      onSearch={onSearch}
      onCancel={() => undefined}
      isScanning={false}
      history={{ keywords: ["invoice", "customer"], directories: ["/reports"] }}
      onSelectHistory={onSelectHistory}
    />);

    fireEvent.click(screen.getByRole("button", { name: "ui.KEYWORD_HISTORY_TOOLTIP" }));
    fireEvent.click(screen.getByRole("option", { name: "customer" }));

    expect(onSelectHistory).toHaveBeenCalledWith("keyword", "customer");
    expect(onSearch).not.toHaveBeenCalled();
  });

  it("supports arrow and Enter selection without IME confirmation triggering search", () => {
    const onSearch = vi.fn();
    const onSelectHistory = vi.fn();
    render(<SearchBar query={query} onChangeQuery={() => undefined} onSearch={onSearch} onCancel={() => undefined} isScanning={false} history={{ keywords: ["invoice", "customer"], directories: [] }} onSelectHistory={onSelectHistory} />);
    fireEvent.click(screen.getByRole("button", { name: "ui.KEYWORD_HISTORY_TOOLTIP" }));
    fireEvent.keyDown(screen.getAllByRole("textbox")[0], { key: "ArrowDown" });
    fireEvent.keyDown(screen.getAllByRole("textbox")[0], { key: "Enter" });
    expect(onSelectHistory).toHaveBeenCalledWith("keyword", "invoice");
    fireEvent.keyDown(screen.getAllByRole("textbox")[0], { key: "Enter", isComposing: true });
    expect(onSearch).not.toHaveBeenCalled();
  });

  it("moves from both history buttons through options and selects without searching", () => {
    const onSearch = vi.fn();
    const onSelectHistory = vi.fn();
    render(<SearchBar query={query} onChangeQuery={() => undefined} onSearch={onSearch} onCancel={() => undefined} isScanning={false} history={{ keywords: ["invoice", "customer"], directories: ["/reports", "/archive"] }} onSelectHistory={onSelectHistory} />);

    const keywordButton = screen.getByRole("button", { name: "ui.KEYWORD_HISTORY_TOOLTIP" });
    fireEvent.click(keywordButton);
    fireEvent.keyDown(keywordButton, { key: KEYBOARD_KEYS.TAB });
    const keywordOptions = screen.getAllByRole("option");
    expect(document.activeElement).toBe(keywordOptions[0]);
    expect(keywordOptions[0].getAttribute("aria-selected")).toBe("true");
    expect(keywordOptions[0].className).toContain("focus-visible:outline");
    fireEvent.keyDown(keywordOptions[0], { key: KEYBOARD_KEYS.ARROW_DOWN });
    expect(document.activeElement).toBe(keywordOptions[1]);
    expect(keywordOptions[0].getAttribute("aria-selected")).toBe("false");
    expect(keywordOptions[1].getAttribute("aria-selected")).toBe("true");
    fireEvent.keyDown(keywordOptions[0], { key: KEYBOARD_KEYS.ARROW_UP });
    expect(document.activeElement).toBe(keywordOptions[0]);
    fireEvent.keyDown(keywordOptions[0], { key: KEYBOARD_KEYS.ARROW_UP });
    expect(document.activeElement).toBe(keywordOptions[0]);
    fireEvent.keyDown(keywordOptions[1], { key: KEYBOARD_KEYS.TAB });
    expect(document.activeElement).toBe(keywordOptions[0]);
    fireEvent.keyDown(keywordOptions[0], { key: KEYBOARD_KEYS.ENTER });
    expect(onSelectHistory).toHaveBeenLastCalledWith("keyword", "invoice");

    const directoryButton = screen.getByRole("button", { name: "ui.DIRECTORY_HISTORY_TOOLTIP" });
    fireEvent.click(directoryButton);
    fireEvent.keyDown(directoryButton, { key: KEYBOARD_KEYS.ARROW_DOWN });
    const directoryOptions = screen.getAllByRole("option");
    expect(document.activeElement).toBe(directoryOptions[0]);
    fireEvent.keyDown(directoryOptions[0], { key: KEYBOARD_KEYS.ARROW_DOWN });
    fireEvent.keyDown(directoryOptions[1], { key: KEYBOARD_KEYS.ENTER });
    expect(onSelectHistory).toHaveBeenLastCalledWith("directory", "/archive");
    expect(onSearch).not.toHaveBeenCalled();
  });

  it("focuses the history button on click so ArrowDown reliably reaches the first option", () => {
    render(<SearchBar query={query} onChangeQuery={() => undefined} onSearch={() => undefined} onCancel={() => undefined} isScanning={false} history={{ keywords: ["invoice"], directories: ["/saved"] }} onSelectHistory={() => undefined} />);
    const cases = [
      { button: "ui.KEYWORD_HISTORY_TOOLTIP", input: 0, value: "invoice" },
      { button: "ui.DIRECTORY_HISTORY_TOOLTIP", input: 1, value: "/saved" },
    ];
    for (const item of cases) {
      const input = screen.getAllByRole("textbox")[item.input];
      input.focus();
      const button = screen.getByRole("button", { name: item.button });
      fireEvent.click(button);
      expect(document.activeElement).toBe(button);
      fireEvent.keyDown(button, { key: KEYBOARD_KEYS.ARROW_DOWN });
      expect(document.activeElement).toBe(screen.getByRole("option", { name: item.value }));
    }
  });

  it("reaches and selects every item in a 20-entry keyboard history", () => {
    const onSelectHistory = vi.fn();
    const values = Array.from({ length: SEARCH_HISTORY_CONSTANTS.DEFAULT_MAX_ENTRIES }, (_, index) => `Entry ${index + SEARCH_HISTORY_CONSTANTS.STEP}`);
    render(<SearchBar query={query} onChangeQuery={() => undefined} onSearch={() => undefined} onCancel={() => undefined} isScanning={false} history={{ keywords: values, directories: [] }} onSelectHistory={onSelectHistory} />);

    const button = screen.getByRole("button", { name: "ui.KEYWORD_HISTORY_TOOLTIP" });
    fireEvent.click(button);
    fireEvent.keyDown(button, { key: KEYBOARD_KEYS.TAB });
    for (const value of values) {
      const option = screen.getByRole("option", { name: value });
      expect(document.activeElement).toBe(option);
      if (value !== values[values.length - SEARCH_HISTORY_CONSTANTS.STEP]) {
        fireEvent.keyDown(option, { key: KEYBOARD_KEYS.TAB });
      }
    }
    fireEvent.keyDown(screen.getByRole("option", { name: values[values.length - SEARCH_HISTORY_CONSTANTS.STEP] }), { key: KEYBOARD_KEYS.ENTER });

    expect(onSelectHistory).toHaveBeenCalledWith("keyword", values[values.length - SEARCH_HISTORY_CONSTANTS.STEP]);
  });

  it("does not trap Tab for an empty history or a single history item", () => {
    const emptyProps = { onChangeQuery: () => undefined, onSearch: () => undefined, onCancel: () => undefined, isScanning: false, history: { keywords: [], directories: [] }, onSelectHistory: () => undefined };
    const { rerender } = render(<SearchBar query={query} {...emptyProps} />);
    const emptyButton = screen.getByRole("button", { name: "ui.KEYWORD_HISTORY_TOOLTIP" });
    fireEvent.click(emptyButton);
    const emptyTab = new KeyboardEvent("keydown", { key: KEYBOARD_KEYS.TAB, bubbles: true, cancelable: true });
    fireEvent(emptyButton, emptyTab);
    expect(emptyTab.defaultPrevented).toBe(false);

    fireEvent.click(emptyButton);
    rerender(<SearchBar query={query} {...emptyProps} history={{ keywords: ["invoice"], directories: [] }} />);
    const button = screen.getByRole("button", { name: "ui.KEYWORD_HISTORY_TOOLTIP" });
    fireEvent.click(button);
    fireEvent.keyDown(button, { key: KEYBOARD_KEYS.TAB });
    const option = screen.getByRole("option", { name: "invoice" });
    fireEvent.keyDown(option, { key: KEYBOARD_KEYS.TAB });
    expect(document.activeElement).toBe(option);
  });

  it("closes either history with Escape and returns focus to its input", () => {
    render(<SearchBar query={query} onChangeQuery={() => undefined} onSearch={() => undefined} onCancel={() => undefined} isScanning={false} history={{ keywords: ["invoice"], directories: ["/saved"] }} onSelectHistory={() => undefined} />);
    const cases = [
      { button: "ui.KEYWORD_HISTORY_TOOLTIP", value: "invoice", input: 0 },
      { button: "ui.DIRECTORY_HISTORY_TOOLTIP", value: "/saved", input: 1 },
    ];
    for (const item of cases) {
      fireEvent.click(screen.getByRole("button", { name: item.button }));
      const option = screen.getByRole("option", { name: item.value });
      option.focus();
      fireEvent.keyDown(option, { key: KEYBOARD_KEYS.ESCAPE });
      expect(screen.queryByRole("option", { name: item.value })).toBeNull();
      expect(document.activeElement).toBe(screen.getAllByRole("textbox")[item.input]);
    }
  });

  it("ignores IME confirmation keys on history options", () => {
    const onSelectHistory = vi.fn();
    render(<SearchBar query={query} onChangeQuery={() => undefined} onSearch={() => undefined} onCancel={() => undefined} isScanning={false} history={{ keywords: ["invoice"], directories: [] }} onSelectHistory={onSelectHistory} />);
    fireEvent.click(screen.getByRole("button", { name: "ui.KEYWORD_HISTORY_TOOLTIP" }));
    const option = screen.getByRole("option", { name: "invoice" });
    fireEvent.keyDown(option, { key: KEYBOARD_KEYS.ENTER, isComposing: true });
    expect(onSelectHistory).not.toHaveBeenCalled();
  });

  it("uses directory suggestions and lets the history button replace them", async () => {
    mocks.invoke.mockResolvedValue(["/reports/Archive"]);
    const props = {
      onChangeQuery: () => undefined,
      onSearch: () => undefined,
      onCancel: () => undefined,
      isScanning: false,
      history: { keywords: [], directories: ["/saved"] },
      onSelectHistory: () => undefined,
    };
    render(<SearchBar
      query={query}
      {...props}
    />);

    act(() => { screen.getAllByRole("textbox")[1].focus(); });

    await screen.findByRole("option", { name: "/reports/Archive" });
    fireEvent.click(screen.getByRole("button", { name: "ui.DIRECTORY_HISTORY_TOOLTIP" }));

    expect(screen.queryByRole("option", { name: "/reports/Archive" })).toBeNull();
    expect(screen.getByRole("option", { name: "/saved" })).toBeTruthy();
  });

  it("sends the entered directory to the completion command", async () => {
    const onChangeQuery = vi.fn();
    const props = {
      onChangeQuery,
      onSearch: () => undefined,
      onCancel: () => undefined,
      isScanning: false,
      history: { keywords: [], directories: [] },
      onSelectHistory: () => undefined,
    };
    const { rerender } = render(<SearchBar
      query={query}
      {...props}
    />);
    const folder = screen.getAllByRole("textbox")[1];
    act(() => { folder.focus(); });
    fireEvent.change(folder, { target: { value: "/reports/Arc" } });
    rerender(<SearchBar {...props} query={{ ...query, target_dir: "/reports/Arc" }} />);

    await waitFor(() => expect(mocks.invoke).toHaveBeenCalledWith(
      expect.any(String),
      { pathInput: "/reports/Arc" },
    ));
  });

  it("ignores stale directory completion responses", async () => {
    let resolveOld!: (values: string[]) => void;
    let resolveNew!: (values: string[]) => void;
    mocks.invoke
      .mockImplementationOnce(() => new Promise<string[]>((resolve) => { resolveOld = resolve; }))
      .mockImplementationOnce(() => new Promise<string[]>((resolve) => { resolveNew = resolve; }));
    const props = { onChangeQuery: () => undefined, onSearch: () => undefined, onCancel: () => undefined, isScanning: false, history: { keywords: [], directories: [] }, onSelectHistory: () => undefined };
    const { rerender } = render(<SearchBar query={query} {...props} />);
    act(() => { screen.getAllByRole("textbox")[1].focus(); });
    await waitFor(() => expect(mocks.invoke).toHaveBeenCalledTimes(1));
    rerender(<SearchBar query={{ ...query, target_dir: "/new" }} {...props} />);
    await waitFor(() => expect(mocks.invoke).toHaveBeenCalledTimes(2));
    resolveOld(["/reports/old"]);
    resolveNew(["/new/folder"]);
    await screen.findByRole("option", { name: "/new/folder" });
    expect(screen.queryByRole("option", { name: "/reports/old" })).toBeNull();
  });

  it("closes keyword and directory histories when focus leaves their controls", () => {
    render(<SearchBar query={query} onChangeQuery={() => undefined} onSearch={() => undefined} onCancel={() => undefined} isScanning={false} history={{ keywords: ["customer"], directories: ["/saved"] }} onSelectHistory={() => undefined} />);
    const keywordHistory = screen.getByRole("button", { name: "ui.KEYWORD_HISTORY_TOOLTIP" });
    const directoryHistory = screen.getByRole("button", { name: "ui.DIRECTORY_HISTORY_TOOLTIP" });
    act(() => { keywordHistory.focus(); });
    expect(document.activeElement).toBe(keywordHistory);
    fireEvent.click(keywordHistory);
    expect(document.activeElement).toBe(keywordHistory);
    expect(screen.getByRole("option", { name: "customer" })).toBeTruthy();
    act(() => { directoryHistory.focus(); });
    expect(document.activeElement).toBe(directoryHistory);
    expect(screen.queryByRole("option", { name: "customer" })).toBeNull();
    fireEvent.click(directoryHistory);
    expect(screen.getByRole("option", { name: "/saved" })).toBeTruthy();
    act(() => { screen.getByRole("button", { name: "ui.BUTTON_SEARCH" }).focus(); });
    expect(screen.queryByRole("option", { name: "/saved" })).toBeNull();
  });

  it("keeps history available while focus moves to an option", () => {
    const onSelectHistory = vi.fn();
    render(<SearchBar query={query} onChangeQuery={() => undefined} onSearch={() => undefined} onCancel={() => undefined} isScanning={false} history={{ keywords: ["customer"], directories: [] }} onSelectHistory={onSelectHistory} />);
    const button = screen.getByRole("button", { name: "ui.KEYWORD_HISTORY_TOOLTIP" });
    act(() => { button.focus(); });
    fireEvent.click(button);
    const option = screen.getByRole("option", { name: "customer" });
    act(() => { option.focus(); });
    expect(screen.getByRole("option", { name: "customer" })).toBeTruthy();
    fireEvent.click(option);
    expect(onSelectHistory).toHaveBeenCalledWith("keyword", "customer");
  });

  it("closes history when clicking a nonfocusable area outside", () => {
    render(<SearchBar query={query} onChangeQuery={() => undefined} onSearch={() => undefined} onCancel={() => undefined} isScanning={false} history={{ keywords: ["customer"], directories: [] }} onSelectHistory={() => undefined} />);
    fireEvent.click(screen.getByRole("button", { name: "ui.KEYWORD_HISTORY_TOOLTIP" }));
    expect(screen.getByRole("option", { name: "customer" })).toBeTruthy();
    fireEvent.pointerDown(document.body);
    expect(screen.queryByRole("option", { name: "customer" })).toBeNull();
  });

  it("closes a visible path suggestion when focus leaves", async () => {
    mocks.invoke.mockResolvedValue(["/reports/Archive"]);
    render(<SearchBar query={query} onChangeQuery={() => undefined} onSearch={() => undefined} onCancel={() => undefined} isScanning={false} history={{ keywords: [], directories: [] }} onSelectHistory={() => undefined} />);
    act(() => { screen.getAllByRole("textbox")[1].focus(); });
    await screen.findByRole("option", { name: "/reports/Archive" });
    act(() => { screen.getByRole("button", { name: "ui.BUTTON_SEARCH" }).focus(); });
    expect(screen.queryByRole("option", { name: "/reports/Archive" })).toBeNull();
  });

  it("closes directory history when opening the folder dialog", () => {
    vi.mocked(open).mockImplementation(() => new Promise(() => undefined));
    render(<SearchBar query={query} onChangeQuery={() => undefined} onSearch={() => undefined} onCancel={() => undefined} isScanning={false} history={{ keywords: [], directories: ["/saved"] }} onSelectHistory={() => undefined} />);
    fireEvent.click(screen.getByRole("button", { name: "ui.DIRECTORY_HISTORY_TOOLTIP" }));
    expect(screen.getByRole("option", { name: "/saved" })).toBeTruthy();
    fireEvent.click(screen.getByRole("button", { name: "ui.SELECT_FOLDER_TOOLTIP" }));
    expect(screen.queryByRole("option", { name: "/saved" })).toBeNull();
  });

  it("ignores a path suggestion returned after focus leaves", async () => {
    let resolveCompletion!: (values: string[]) => void;
    mocks.invoke.mockImplementationOnce(() => new Promise<string[]>((resolve) => { resolveCompletion = resolve; }));
    render(<SearchBar query={query} onChangeQuery={() => undefined} onSearch={() => undefined} onCancel={() => undefined} isScanning={false} history={{ keywords: [], directories: [] }} onSelectHistory={() => undefined} />);
    act(() => { screen.getAllByRole("textbox")[1].focus(); });
    await waitFor(() => expect(mocks.invoke).toHaveBeenCalledTimes(1));
    act(() => { screen.getByRole("button", { name: "ui.BUTTON_SEARCH" }).focus(); });
    await act(async () => { resolveCompletion(["/reports/Archive"]); });
    expect(screen.queryByRole("option", { name: "/reports/Archive" })).toBeNull();
  });
});
