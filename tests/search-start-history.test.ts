/**
 * Copyright (c) 2026 soakaye
 *
 * Description: Unit and integration tests for search-start-history.test.ts.
 * Arguments & Returns: Vitest runs test suites; no public arguments or return values.
 * Errors: Test assertions fail if behavior deviates from requirements.
 */
import { act, renderHook } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
  invoke: vi.fn(),
  listen: vi.fn(),
}));
vi.mock("@tauri-apps/api/core", () => ({ invoke: mocks.invoke }));
vi.mock("@tauri-apps/api/event", () => ({ listen: mocks.listen }));

import { useSearch } from "../src/hooks/useSearch";

describe("search history acceptance", () => {
  beforeEach(() => {
    mocks.invoke.mockReset().mockResolvedValue(undefined);
    mocks.listen.mockReset().mockResolvedValue(() => undefined);
  });
  afterEach(() => vi.restoreAllMocks());

  it("records the original query only after the start command succeeds", async () => {
    const onSearchAccepted = vi.fn();
    const { result } = renderHook(() => useSearch({ onSearchAccepted }));
    act(() => result.current.updateQuery({
      keyword: "invoice",
      target_dir: "~/reports",
    }));

    await act(async () => result.current.startSearch());

    expect(onSearchAccepted).toHaveBeenCalledWith(expect.objectContaining({
      keyword: "invoice",
      target_dir: "~/reports",
      directory_mode: "sequential",
      burst_workers: null,
    }));
  });

  it("does not record a rejected regex and shows its specific error", async () => {
    mocks.invoke.mockRejectedValue({ code: "invalid_regex" });
    const onSearchAccepted = vi.fn();
    const onShowToast = vi.fn();
    const { result } = renderHook(() => useSearch({ onSearchAccepted, onShowToast }));
    act(() => result.current.updateQuery({
      keyword: "(",
      target_dir: "/reports",
      use_regex: true,
    }));

    await act(async () => result.current.startSearch());

    expect(onSearchAccepted).not.toHaveBeenCalled();
    expect(onShowToast).toHaveBeenCalledWith("ui.INVALID_REGEX");
  });
});
