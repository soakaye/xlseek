import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { setActiveLanguage } from "../src/constants";
import { ResultTable } from "../src/components/results/ResultTable";
import { SearchBar } from "../src/components/search/SearchBar";
import { StatusBar } from "../src/components/common/StatusBar";
import { Toast } from "../src/components/common/Toast";

vi.mock("@tauri-apps/api/webview", () => ({
  getCurrentWebview: () => ({ onDragDropEvent: async () => () => undefined }),
}));

describe("localized application UI", () => {
  afterEach(() => {
    cleanup();
    setActiveLanguage("ja");
  });

  it("renders translated controls and status while preserving user data", () => {
    setActiveLanguage("en");
    const onChangeQuery = vi.fn();
    render(<SearchBar
      query={{ keyword: "customer", target_dir: "/tmp/books", extensions: [".xlsx"] }}
      onChangeQuery={onChangeQuery}
      onSearch={() => undefined}
      onCancel={() => undefined}
      isScanning={false}
    />);
    expect(screen.getByPlaceholderText("Enter text or a regular expression... (Enter to search)")).toBeTruthy();
    expect(screen.getByText("SEARCH")).toBeTruthy();
    cleanup();

    render(<ResultTable items={[]} selectedId={null} onSelectItem={() => undefined} />);
    expect(screen.getByText("Search results")).toBeTruthy();
    expect(screen.getByText("No matching results")).toBeTruthy();
    cleanup();

    render(<StatusBar
      progress={{ state: "Scanning", phase: "scanning", scanned_files: 1, total_files: 2, matches_found: 1, current_file: "report_日本.xlsx", elapsed_ms: 1200 }}
      items={[]}
      onShowToast={() => undefined}
      onOpenAbout={() => undefined}
      onOpenSettings={() => undefined}
      language="en"
    />);
    expect(screen.getByText(/Scanning: 1\/2 files/)).toBeTruthy();
    expect(screen.getByText(/report_日本\.xlsx/)).toBeTruthy();
    cleanup();

    render(<Toast message="Original cell value: 値" onClose={() => undefined} duration={10000} />);
    expect(screen.getByText("Original cell value: 値")).toBeTruthy();
  });
});
