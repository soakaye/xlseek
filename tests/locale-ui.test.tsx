import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { parse } from "yaml";
import enSource from "../src-tauri/locales/en.yml?raw";
import jaSource from "../src-tauri/locales/ja.yml?raw";
import { initializeI18n, LocaleProvider, setI18nLocale } from "../src/i18n";
import { ResultTable } from "../src/components/results/ResultTable";
import { SearchBar } from "../src/components/search/SearchBar";
import { StatusBar } from "../src/components/common/StatusBar";
import { SettingsDialog } from "../src/components/settings/SettingsDialog";
import { AboutDialog } from "../src/components/about/AboutDialog";
import { MetaInfoCard } from "../src/components/preview/MetaInfoCard";
import { Toast } from "../src/components/common/Toast";
import type { SearchMatch } from "../src/types/search";

const plugin = vi.hoisted(() => ({
  language: "en",
  catalogs: {} as Record<string, Record<string, string>>,
}));
vi.mock("@razein97/tauri-plugin-i18n", () => ({
  default: {
    getInstance: () => ({ load: async () => undefined, translate: (key: string) => plugin.catalogs[plugin.language]?.[key] ?? key }),
    setLocale: async (language: string) => { plugin.language = language; },
  },
}));
vi.mock("@tauri-apps/api/webview", () => ({
  getCurrentWebview: () => ({ onDragDropEvent: async () => () => undefined }),
}));
vi.mock("@tanstack/react-virtual", () => ({
  useVirtualizer: ({ count }: { count: number }) => ({
    getTotalSize: () => count * 36,
    getVirtualItems: () => Array.from({ length: count }, (_, index) => ({ key: index, index, size: 36, start: index * 36 })),
    scrollToIndex: () => undefined,
  }),
}));

describe("localized application UI", () => {
  beforeEach(async () => {
    plugin.language = "en";
    plugin.catalogs = { en: parse(enSource) as Record<string, string>, ja: parse(jaSource) as Record<string, string> };
    await initializeI18n("en");
  });
  afterEach(cleanup);

  it("renders translated controls and status while preserving user data", () => {
    const onChangeQuery = vi.fn();
    render(<LocaleProvider value="en"><SearchBar
        query={{ keyword: "customer", target_dir: "/tmp/books", extensions: [".xlsx"] }}
        onChangeQuery={onChangeQuery}
        onSearch={() => undefined}
        onCancel={() => undefined}
        isScanning={false}
      /></LocaleProvider>);
    expect(screen.getByPlaceholderText("Enter text or a regular expression... (Enter to search)")).toBeTruthy();
    expect(screen.getByText("SEARCH")).toBeTruthy();
    cleanup();

    render(<LocaleProvider value="en"><ResultTable items={[]} selectedId={null} onSelectItem={() => undefined} /></LocaleProvider>);
    expect(screen.getByText("Search results")).toBeTruthy();
    expect(screen.getByText("No matching results")).toBeTruthy();
    cleanup();

    render(<LocaleProvider value="en"><StatusBar
      progress={{ state: "Scanning", phase: "scanning", scanned_files: 1, total_files: 2, matches_found: 1, current_file: "report_日本.xlsx", elapsed_ms: 1200 }}
      items={[]}
      onShowToast={() => undefined}
      onOpenAbout={() => undefined}
      onOpenSettings={() => undefined}
      language="en"
    /></LocaleProvider>);
    expect(screen.getByText(/Scanning: 1\/2 files/)).toBeTruthy();
    expect(screen.getByText(/report_日本\.xlsx/)).toBeTruthy();
    cleanup();

    render(<LocaleProvider value="en"><Toast message="Original cell value: 値" onClose={() => undefined} duration={10000} /></LocaleProvider>);
    expect(screen.getByText("Original cell value: 値")).toBeTruthy();
  });

  it("renders settings and About labels from the Japanese plugin catalog", async () => {
    plugin.language = "ja";
    await setI18nLocale("ja");
    render(<LocaleProvider value="ja"><SettingsDialog isOpen preference="default" language="ja" onSelect={() => undefined} onClose={() => undefined} /></LocaleProvider>);
    expect(screen.getByRole("dialog", { name: "言語設定" })).toBeTruthy();
    expect(screen.getByRole("radio", { name: "日本語" })).toBeTruthy();
    cleanup();

    render(<LocaleProvider value="ja"><AboutDialog isOpen onClose={() => undefined} onShowToast={() => undefined} /></LocaleProvider>);
    expect(screen.getByText("Excel Grep について")).toBeTruthy();
    expect(screen.getAllByRole("button", { name: "閉じる" }).length).toBeGreaterThan(0);
  });

  it.each([
    { locale: "en" as const, matchType: "CellValue" as const, label: "Value", detail: "Cell value (Text / Number)", header: "Match type", detailHeading: "Match details" },
    { locale: "ja" as const, matchType: "CellValue" as const, label: "値", detail: "セル値 (Text / Number)", header: "一致種別", detailHeading: "一致の詳細情報" },
    { locale: "en" as const, matchType: "Formula" as const, label: "Formula", detail: "Formula", header: "Match type", detailHeading: "Match details" },
    { locale: "ja" as const, matchType: "Formula" as const, label: "数式", detail: "数式 (Formula)", header: "一致種別", detailHeading: "一致の詳細情報" },
    { locale: "en" as const, matchType: "Comment" as const, label: "Comment / Note", detail: "Comment / Note", header: "Match type", detailHeading: "Match details" },
    { locale: "ja" as const, matchType: "Comment" as const, label: "コメント / メモ", detail: "コメント / メモ", header: "一致種別", detailHeading: "一致の詳細情報" },
    { locale: "en" as const, matchType: "HiddenSheet" as const, label: "Hidden sheet match", detail: "Hidden sheet match", header: "Match type", detailHeading: "Match details" },
    { locale: "ja" as const, matchType: "HiddenSheet" as const, label: "非表示シート一致", detail: "非表示シート一致", header: "一致種別", detailHeading: "一致の詳細情報" },
  ])("shows the $locale $matchType label in results and preserves its detail label", async ({ locale, matchType, label, detail, header, detailHeading }) => {
    plugin.language = locale;
    await setI18nLocale(locale);
    const match: SearchMatch = {
      id: 1,
      file_name: "report.xlsx",
      full_path: "/tmp/report.xlsx",
      sheet_name: "Summary",
      cell_address: "A1",
      row_index: 1,
      col_index: 1,
      col_name: "A",
      match_type: matchType,
      snippet: "match",
      full_content: "match",
      formula: null,
      sheets_in_workbook: ["Summary"],
    };

    render(<LocaleProvider value={locale}>
      <ResultTable items={[match]} selectedId={1} onSelectItem={() => undefined} />
      <MetaInfoCard match={match} />
    </LocaleProvider>);

    const badge = screen.getByLabelText(label);
    expect(badge.textContent).toBe(label);
    expect(badge.getAttribute("class")).toContain("whitespace-nowrap");
    expect(badge.getAttribute("class")).toContain("truncate");
    expect(badge.getAttribute("title")).toBe(label);
    expect(badge.getAttribute("aria-label")).toBe(label);
    const detailTypeLabel = screen.getByText(detailHeading).parentElement?.querySelector("strong");
    expect(detailTypeLabel?.textContent).toBe(detail);

    const headerColumn = screen.getByRole("button", { name: header });
    const rowColumn = badge.parentElement;
    const headerWidth = headerColumn.className.match(/w-\[\d+%\]/)?.[0];
    expect(headerWidth).toBe("w-[15%]");
    expect(rowColumn?.className).toContain(headerWidth);
  });
});
