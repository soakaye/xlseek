/**
 * 処理内容: 検索欄の履歴・補完リスト操作を検証する。
 * 引数・戻り値: Vitest が SearchBar の操作テストを実行する。
 * エラー: 選択時に検索が始まる、またはリスト操作に失敗するとテストを失敗させる。
 * 変更履歴: v1.0.0 (2026-09-27, Codex): 履歴・補完 UI テストを追加。
 */
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
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
    await waitFor(() => expect(mocks.invoke).toHaveBeenCalledTimes(1));
    rerender(<SearchBar query={{ ...query, target_dir: "/new" }} {...props} />);
    await waitFor(() => expect(mocks.invoke).toHaveBeenCalledTimes(2));
    resolveOld(["/reports/old"]);
    resolveNew(["/new/folder"]);
    await screen.findByRole("option", { name: "/new/folder" });
    expect(screen.queryByRole("option", { name: "/reports/old" })).toBeNull();
  });
});
