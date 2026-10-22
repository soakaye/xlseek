/**
 * Copyright (c) 2026 soakaye
 *
 * @fileoverview Unit and component tests for MetaInfoCard (tests/meta-info-card.test.tsx)
 *
 * ## Description
 * Verifies that MetaInfoCard correctly renders search match metadata, applies the scrollable
 * viewport classes (`max-h-36`, `overflow-y-auto`, `whitespace-pre-wrap`, `overscroll-contain`),
 * preserves newline formatting, and safely handles null matches and empty values.
 * Complies with Constitution Principle I (English documentation), Principle II (constant references),
 * Principle III (comprehensive documentation), and Principle V (robust error handling and testing).
 */

import React from "react";
import { render, screen, cleanup } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { MetaInfoCard } from "../src/components/preview/MetaInfoCard";
import type { SearchMatch } from "../src/types/search";
import { LAYOUT_CONSTANTS } from "../src/constants";

vi.mock("../src/i18n", () => ({
  useTranslation: () => (key: string) => key,
}));

/**
 * ## Description
 * Factory helper creating a mock SearchMatch entity for testing.
 *
 * ## Arguments
 * @param overrides - Partial SearchMatch attributes to override defaults
 *
 * ## Returns
 * @returns Fully populated SearchMatch entity
 *
 * ## Errors / Exceptions
 * None. Always returns a valid SearchMatch object.
 */
const createMockMatch = (overrides?: Partial<SearchMatch>): SearchMatch => ({
  id: 1,
  file_name: "test.xlsx",
  full_path: "C:\\data\\test.xlsx",
  sheet_name: "Sheet1",
  cell_address: "A16",
  row_index: 16,
  col_index: 1,
  col_name: "A",
  match_type: "CellValue",
  shape_name: null,
  sheet_hidden: false,
  snippet: "target content",
  full_content: "Default full content for testing",
  formula: null,
  sheets_in_workbook: ["Sheet1"],
  ...overrides,
});

describe("MetaInfoCard Component", () => {
  afterEach(() => {
    cleanup();
    vi.clearAllMocks();
  });

  describe("Phase 2 & User Story 1: Scrollable Viewport and Grid Preservation", () => {
    it("renders with the centralized test id and scrollable classes (max-h-36 and overflow-y-auto)", () => {
      // Constant reference: LAYOUT_CONSTANTS.META_DETAIL_TEST_ID, LAYOUT_CONSTANTS.META_DETAIL_MAX_HEIGHT_CLASS
      const match = createMockMatch({ full_content: "Short test content" });
      render(<MetaInfoCard match={match} />);

      const contentBox = screen.getByTestId(LAYOUT_CONSTANTS.META_DETAIL_TEST_ID);
      expect(contentBox).toBeDefined();
      expect(contentBox.className).toContain(LAYOUT_CONSTANTS.META_DETAIL_MAX_HEIGHT_CLASS);
      expect(contentBox.className).toContain("overflow-y-auto");
    });

    it("renders long multi-line cell content inside the bounded scroll container", () => {
      const longText = Array.from({ length: 25 }, (_, i) => `Line ${i + 1}: detailed explanation text`).join("\n");
      const match = createMockMatch({ full_content: longText });
      render(<MetaInfoCard match={match} />);

      // Constant reference: LAYOUT_CONSTANTS.META_DETAIL_TEST_ID
      const contentBox = screen.getByTestId(LAYOUT_CONSTANTS.META_DETAIL_TEST_ID);
      expect(contentBox.textContent).toContain("Line 1:");
      expect(contentBox.textContent).toContain("Line 25:");
      expect(contentBox.className).toContain(LAYOUT_CONSTANTS.META_DETAIL_MAX_HEIGHT_CLASS);
      expect(contentBox.className).toContain("overflow-y-auto");
    });
  });

  describe("User Story 2: Newline Preservation and Text Selection", () => {
    it("applies whitespace-pre-wrap and select-text to preserve explicit newline formatting", () => {
      const multiLine = "Item 1: Start\nItem 2: Middle\r\nItem 3: End";
      const match = createMockMatch({ full_content: multiLine });
      render(<MetaInfoCard match={match} />);

      // Constant reference: LAYOUT_CONSTANTS.META_DETAIL_TEST_ID
      const contentBox = screen.getByTestId(LAYOUT_CONSTANTS.META_DETAIL_TEST_ID);
      expect(contentBox.className).toContain("whitespace-pre-wrap");
      expect(contentBox.className).toContain("select-text");
      expect(contentBox.className).toContain("break-words");
    });
  });

  describe("User Story 3: Scroll Containment and Empty / Null Fallbacks", () => {
    it("applies overscroll-contain to prevent parent scroll chaining", () => {
      const match = createMockMatch();
      render(<MetaInfoCard match={match} />);

      // Constant reference: LAYOUT_CONSTANTS.META_DETAIL_TEST_ID
      const contentBox = screen.getByTestId(LAYOUT_CONSTANTS.META_DETAIL_TEST_ID);
      expect(contentBox.className).toContain("overscroll-contain");
    });

    it("falls back to empty content label when full_content is empty", () => {
      const match = createMockMatch({ full_content: "" });
      render(<MetaInfoCard match={match} />);

      // Constant reference: LAYOUT_CONSTANTS.META_DETAIL_TEST_ID
      const contentBox = screen.getByTestId(LAYOUT_CONSTANTS.META_DETAIL_TEST_ID);
      expect(contentBox.textContent).toBe("ui.EMPTY_CONTENT");
    });

    it("returns null safely without error when match is null", () => {
      const { container } = render(<MetaInfoCard match={null} />);
      expect(container.firstChild).toBeNull();
    });

    it("renders coordinate metadata correctly", () => {
      const match = createMockMatch({ row_index: 42, col_index: 3, col_name: "C", sheet_hidden: true });
      render(<MetaInfoCard match={match} />);

      expect(screen.getByText("42")).toBeDefined();
      expect(screen.getByText("3 (C)")).toBeDefined();
      expect(screen.getByText("ui.STATUS_HIDDEN")).toBeDefined();
    });

    it("renders shape name when match_type is Shape", () => {
      const match = createMockMatch({ match_type: "Shape", shape_name: "FlowChart_Decision_1" });
      render(<MetaInfoCard match={match} />);

      expect(screen.getByText("FlowChart_Decision_1")).toBeDefined();
    });
  });
});
