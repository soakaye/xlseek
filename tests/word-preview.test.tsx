/**
 * Copyright (c) 2026 soakaye
 *
 * @fileoverview Unit and component tests for WordTextPreview (tests/word-preview.test.tsx)
 *
 * ## Description
 * Verifies that WordTextPreview renders paragraphs, highlights matching keywords,
 * and immediately calls scrollIntoView with instant behavior ("auto") and center block
 * positioning without artificial delay or smooth animation.
 * Complies with Constitution Principle I (English documentation) and Principle V (robust error handling and testing).
 */

import React from "react";
import { render, screen, cleanup } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { WordTextPreview } from "../src/components/preview/WordTextPreview";
import { WORD_UI_CONSTANTS } from "../src/constants";
import type { WordPreviewPayload } from "../src/types/preview";

vi.mock("../src/i18n", () => ({
  useTranslation: () => (key: string) => key,
}));

describe("WordTextPreview", () => {
  let scrollIntoViewMock: ReturnType<typeof vi.fn>;

  beforeEach(() => {
    scrollIntoViewMock = vi.fn();
    window.HTMLElement.prototype.scrollIntoView = scrollIntoViewMock;
  });

  afterEach(() => {
    cleanup();
    vi.clearAllMocks();
  });

  const createPayload = (targetParagraph: number): WordPreviewPayload => ({
    file_path: "C:\\docs\\report.docx",
    file_name: "report.docx",
    target_paragraph: targetParagraph,
    total_paragraphs: 5,
    paragraphs: [
      { index: 1, text: "First introductory line", is_target: targetParagraph === 1 },
      { index: 2, text: "Second line discussing requirements", is_target: targetParagraph === 2 },
      { index: 3, text: "Target keyword hit here in paragraph 3", is_target: targetParagraph === 3 },
      { index: 4, text: "Fourth line with additional notes", is_target: targetParagraph === 4 },
      { index: 5, text: "Fifth line concluding the document", is_target: targetParagraph === 5 },
    ],
  });

  it("calls scrollIntoView instantly with auto behavior and center block without delay", () => {
    const payload = createPayload(3);
    render(
      <WordTextPreview
        payload={payload}
        isLoading={false}
        keyword="keyword"
      />
    );

    expect(scrollIntoViewMock).toHaveBeenCalledTimes(1);
    expect(scrollIntoViewMock).toHaveBeenCalledWith({
      behavior: WORD_UI_CONSTANTS.SCROLL_BEHAVIOR,
      block: WORD_UI_CONSTANTS.SCROLL_BLOCK,
    });
  });

  it("updates scroll position immediately when target paragraph changes", () => {
    const payload1 = createPayload(2);
    const { rerender } = render(
      <WordTextPreview
        payload={payload1}
        isLoading={false}
        keyword="requirements"
      />
    );

    expect(scrollIntoViewMock).toHaveBeenCalledTimes(1);
    expect(scrollIntoViewMock).toHaveBeenCalledWith({
      behavior: WORD_UI_CONSTANTS.SCROLL_BEHAVIOR,
      block: WORD_UI_CONSTANTS.SCROLL_BLOCK,
    });

    const payload2 = createPayload(4);
    rerender(
      <WordTextPreview
        payload={payload2}
        isLoading={false}
        keyword="notes"
      />
    );

    expect(scrollIntoViewMock).toHaveBeenCalledTimes(2);
  });

  it("does not call scrollIntoView when preview is loading or payload is empty", () => {
    render(
      <WordTextPreview
        payload={null}
        isLoading={true}
        keyword="test"
      />
    );
    expect(scrollIntoViewMock).not.toHaveBeenCalled();
  });

  it("highlights search keywords inside matching paragraphs", () => {
    const payload = createPayload(3);
    render(
      <WordTextPreview
        payload={payload}
        isLoading={false}
        keyword="keyword"
      />
    );

    const mark = screen.getByText("keyword");
    expect(mark.tagName.toLowerCase()).toBe("mark");
  });
});
