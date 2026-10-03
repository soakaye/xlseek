/**
 * Copyright (c) 2026 soakaye
 *
 * Description: Unit and integration tests for shape-option-ui.test.tsx.
 * Arguments & Returns: Vitest runs test suites; no public arguments or return values.
 * Errors: Test assertions fail if behavior deviates from requirements.
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
