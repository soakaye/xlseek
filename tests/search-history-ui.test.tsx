/**
 * 処理内容: 検索欄の履歴・補完リスト操作を検証する。
 * 引数・戻り値: Vitest が SearchBar の操作テストを実行する。
 * エラー: 選択時に検索が始まる、またはリスト操作に失敗するとテストを失敗させる。
 * 変更履歴: v1.0.0 (2026-09-27, Codex): 履歴・補完 UI テストを追加。
 * 変更履歴: v1.1.0 (2026-09-27, Codex): フォーカス離脱と遅延補完の回帰テストを追加。
 * 変更履歴: v1.2.0 (2026-09-27, Codex): フォルダ選択時の履歴非表示を検証する。
 */
import { act, cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { open } from "@tauri-apps/plugin-dialog";
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
