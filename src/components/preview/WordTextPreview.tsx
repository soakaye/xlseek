/**
 * Copyright (c) 2026 soakaye
 *
 * @fileoverview Word Document Text Preview Component (src/components/preview/WordTextPreview.tsx)
 *
 * ## Description
 * Renders ordered document paragraphs from a Microsoft Word document (.docx, .docm, .doc).
 * Displays line/paragraph indices, highlights search keywords with <mark> tags,
 * and automatically scrolls to the matched target paragraph using `scrollIntoView`.
 * Complies with Constitution Principle I (English documentation), Principle II (Constant references),
 * and Principle III (Comprehensive header comments).
 */

import React, { useEffect, useRef } from "react";
import { FileText } from "lucide-react";
import { WordPreviewPayload } from "../../types/preview";
import { useTranslation } from "../../i18n";
import { WORD_UI_CONSTANTS } from "../../constants";


interface WordTextPreviewProps {
  payload: WordPreviewPayload | null;
  isLoading: boolean;
  keyword: string;
  useRegex?: boolean;
  matchCase?: boolean;
}

/**
 * ## Description
 * Word text document preview UI component with keyword highlighting and auto-scroll.
 *
 * ## Arguments
 * @param props - `WordTextPreviewProps` containing preview payload, loading state, and search keyword
 *
 * ## Returns
 * @returns Rendered text preview element
 *
 * ## Errors / Exceptions
 * Catches regex compilation errors safely and falls back to plain substring matching. Does not crash or throw.
 */
export const WordTextPreview: React.FC<WordTextPreviewProps> = ({
  payload,
  isLoading,
  keyword,
  useRegex = false,
  matchCase = false,
}) => {
  const t = useTranslation();
  const targetParagraphRef = useRef<HTMLDivElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);

  // Instantly position target paragraph into view without animation delay when payload updates
  useEffect(() => {
    if (!payload || isLoading) return;
    targetParagraphRef.current?.scrollIntoView({
      // Constant reference: WORD_UI_CONSTANTS.SCROLL_BEHAVIOR
      behavior: WORD_UI_CONSTANTS.SCROLL_BEHAVIOR,
      // Constant reference: WORD_UI_CONSTANTS.SCROLL_BLOCK
      block: WORD_UI_CONSTANTS.SCROLL_BLOCK,
    });
  }, [payload, isLoading]);



  if (isLoading) {
    return (
      <div className="flex-1 p-6 flex items-center justify-center text-xs text-zinc-500">
        <div className="flex items-center gap-2">
          <div className="w-4 h-4 border-2 border-emerald-500 border-t-transparent rounded-full animate-spin"></div>
          {/* Constant reference: t("ui.PREVIEW_LOADING") */}
          <span>{t("ui.PREVIEW_LOADING")}</span>
        </div>
      </div>
    );
  }

  if (!payload || payload.paragraphs.length === 0) {
    return (
      <div className="flex-1 p-6 flex flex-col items-center justify-center text-xs text-zinc-500 gap-2">
        <FileText className="w-8 h-8 text-zinc-600" />
        {/* Constant reference: t("ui.NO_PREVIEW_DATA") */}
        <span>{t("ui.NO_PREVIEW_DATA")}</span>
      </div>
    );
  }

  /**
   * Helper function to highlight keywords inside paragraph text safely.
   */
  const renderHighlightedContent = (text: string) => {
    if (!keyword.trim()) {
      return text;
    }

    try {
      let re: RegExp;
      if (useRegex) {
        re = new RegExp(`(${keyword})`, matchCase ? "g" : "gi");
      } else {
        const escaped = keyword.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
        re = new RegExp(`(${escaped})`, matchCase ? "g" : "gi");
      }

      const parts = text.split(re);
      return parts.map((part, index) => {
        if (re.test(part)) {
          return (
            <mark
              key={index}
              className="bg-yellow-500/30 text-yellow-200 px-0.5 rounded font-medium border border-yellow-500/40"
            >
              {part}
            </mark>
          );
        }
        return part;
      });
    } catch {
      return text;
    }
  };

  return (
    <div ref={containerRef} className="flex-1 overflow-y-auto bg-[#141416] p-4 font-mono text-xs">
      <div className="space-y-1 max-w-4xl mx-auto">
        {payload.paragraphs.map((p) => {
          const isTarget = p.index === payload.target_paragraph;
          return (
            <div
              key={p.index}
              ref={isTarget ? targetParagraphRef : undefined}
              className={`flex items-start gap-3 px-3 py-1.5 rounded transition-colors ${
                isTarget
                  ? "bg-emerald-950/60 border border-emerald-600/80 text-emerald-100 shadow-sm"
                  : "hover:bg-zinc-800/40 text-zinc-300"
              }`}
            >
              {/* 1-based paragraph sequence index */}
              <span className="w-10 shrink-0 text-right font-mono text-[11px] text-zinc-500 select-none pt-0.5">
                {p.index}
              </span>
              {/* Paragraph text */}
              <div className="flex-1 leading-relaxed whitespace-pre-wrap break-words">
                {renderHighlightedContent(p.text)}
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
};
