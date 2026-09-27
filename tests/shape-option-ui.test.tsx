/**
 * 処理内容: Shape 検索トグルの既定状態と検索条件の独立更新を検証する。
 * 引数・戻り値: Vitest が SearchBar の操作を実行する。値の受け渡しはモックで確認する。
 * エラー: 表示、既定値、トグル反映が契約と異なる場合にテストが失敗する。
 * 変更履歴: v1.0.0 (2026-09-28, Codex): Shape オプション UI テストを追加。
 */
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { SearchBar } from "../src/components/search/SearchBar";

vi.mock("@tauri-apps/api/core", () => ({ invoke: vi.fn().mockResolvedValue([]) }));
vi.mock("@tauri-apps/plugin-dialog", () => ({ open: vi.fn() }));
vi.mock("@tauri-apps/api/webview", () => ({
  getCurrentWebview: () => ({ onDragDropEvent: async () => () => undefined }),
}));
vi.mock("../src/i18n", () => ({ useTranslation: () => (key: string) => key }));

describe("Shape search option", () => {
  afterEach(cleanup);

  it("defaults on and changes independently from formula search", () => {
    const onChangeQuery = vi.fn();
    render(
      <SearchBar
        query={{ keyword: "needle", target_dir: "/books", include_formula: false }}
        onChangeQuery={onChangeQuery}
        onSearch={() => undefined}
        onCancel={() => undefined}
        isScanning={false}
        history={{ keywords: [], directories: [] }}
        onSelectHistory={() => undefined}
      />,
    );

    const shapeOption = screen.getByLabelText("ui.OPTION_INCLUDE_SHAPE") as HTMLInputElement;
    expect(shapeOption.checked).toBe(true);
    fireEvent.click(shapeOption);
    expect(onChangeQuery).toHaveBeenCalledWith({ include_shape: false });
  });
});
