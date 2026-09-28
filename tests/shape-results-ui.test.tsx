/**
 * 処理内容: Shape 結果表示とアンカー有無に応じたセルプレビューの選択経路を検証する。
 * 引数・戻り値: Vitest が ResultTable、MetaInfoCard、useSearch の振る舞いを確認する。
 * エラー: Shape 名・全文が欠ける、またはアンカー判定時のプレビュー呼び出し制御が崩れた場合にテストが失敗する。
 * 変更履歴:
 *   - v1.0.0 (2026-09-28, Codex): Shape 結果 UI テストを追加。
 *   - v1.1.0 (2026-09-28, Antigravity): アンカー付き Shape でセルプレビューが読み込まれる動作への更新。
 */
import { act, cleanup, render, renderHook, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { SearchMatch } from "../src/types/search";
import { ResultTable } from "../src/components/results/ResultTable";
import { MetaInfoCard } from "../src/components/preview/MetaInfoCard";
import { useSearch } from "../src/hooks/useSearch";

const mocks = vi.hoisted(() => ({ invoke: vi.fn(), listen: vi.fn() }));
vi.mock("@tauri-apps/api/core", () => ({ invoke: mocks.invoke }));
vi.mock("@tauri-apps/api/event", () => ({ listen: mocks.listen }));
vi.mock("@tauri-apps/api/webview", () => ({
  getCurrentWebview: () => ({ onDragDropEvent: async () => () => undefined }),
}));
vi.mock("@tanstack/react-virtual", () => ({
  useVirtualizer: ({ count }: { count: number }) => ({
    getTotalSize: () => count,
    getVirtualItems: () => Array.from({ length: count }, (_, index) => ({ key: index, index, size: 1, start: index })),
    scrollToIndex: vi.fn(),
  }),
}));
vi.mock("../src/i18n", () => ({ useTranslation: () => (key: string) => key }));

const shapeResult: SearchMatch = {
  id: 1,
  file_name: "book.xlsx",
  full_path: "/books/book.xlsx",
  sheet_name: "Sheet1",
  cell_address: "",
  row_index: 0,
  col_index: 0,
  col_name: "",
  match_type: "Shape",
  shape_name: "Group / Label",
  sheet_hidden: false,
  snippet: "A <mark>needle</mark>",
  full_content: "A needle in a shape",
  formula: null,
  sheets_in_workbook: ["Sheet1"],
};

describe("Shape search results", () => {
  afterEach(() => {
    cleanup();
    vi.clearAllMocks();
  });

  it("shows the Shape type, name, and full text", () => {
    render(<ResultTable items={[shapeResult]} selectedId={null} onSelectItem={() => undefined} />);
    render(<MetaInfoCard match={shapeResult} />);
    expect(screen.getByText("ui.RESULT_MATCH_TYPE_SHAPE")).toBeTruthy();
    expect(screen.getAllByText("Group / Label").length).toBeGreaterThan(0);
    expect(screen.getByText("A needle in a shape")).toBeTruthy();
  });

  it("renders escaped Shape text without creating markup", () => {
    const escapedResult = {
      ...shapeResult,
      snippet: "&lt;img src=x onerror=alert(1)&gt;<mark>needle</mark>",
    };
    const { container } = render(
      <ResultTable items={[escapedResult]} selectedId={null} onSelectItem={() => undefined} />,
    );
    expect(container.querySelector("img")).toBeNull();
    expect(container.textContent).toContain("<img src=x onerror=alert(1)>");
  });

  it("does not load cell preview when an unanchored Shape result is selected", () => {
    mocks.invoke.mockResolvedValue({});
    mocks.listen.mockResolvedValue(() => undefined);
    const { result } = renderHook(() => useSearch());
    act(() => result.current.handleSelectItem(shapeResult));
    expect(mocks.invoke).not.toHaveBeenCalled();
    expect(result.current.previewData).toBeNull();
    expect(result.current.selectedMatch?.shape_name).toBe("Group / Label");
    expect(result.current.formulaOrValue).toBe(shapeResult.full_content);
  });

  it("shows an actual Shape anchor and loads cell preview around anchor", () => {
    const anchored = { ...shapeResult, cell_address: "B3", row_index: 3, col_index: 2, col_name: "B" };
    render(<MetaInfoCard match={anchored} />);
    expect(screen.getByText("3")).toBeTruthy();
    expect(screen.getByText(/2 \(B\)/)).toBeTruthy();

    mocks.invoke.mockResolvedValue({
      file_path: "/books/book.xlsx",
      sheet_name: "Sheet1",
      sheet_names: ["Sheet1"],
      cells: [],
      target_row: 3,
      target_col: 2,
    });
    mocks.listen.mockResolvedValue(() => undefined);
    const { result } = renderHook(() => useSearch());
    act(() => result.current.handleSelectItem(anchored));
    expect(mocks.invoke).toHaveBeenCalledWith("get_cell_preview", {
      filePath: "/books/book.xlsx",
      sheetName: "Sheet1",
      rowIndex: 3,
      colIndex: 2,
    });
  });
});


