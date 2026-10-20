/**
 * Copyright (c) 2026 soakaye
 *
 * @fileoverview Search results right-click context menu component (src/components/results/ContextMenu.tsx)
 *
 * ## Description
 * Renders a floating context menu at the user's cursor position when right-clicking search result rows.
 * Provides granular AI prompt copying options (Markdown table context, header & cell only,
 * or Word document headings vs plain paragraph modes) with viewport boundary clamping and outside-click dismissal.
 *
 * Conforms to Constitution Principle I (English documentation), Principle II (Constant references),
 * Principle III (Comprehensive Header Comments), and Principle V (Robust error handling).
 */

import React, { useEffect, useRef } from "react";
import { Sparkles, Table, FileText, AlignLeft, Copy } from "lucide-react";
import { SearchMatch } from "../../types/search";
import { LlmCopyType, WordExtractionMode } from "../../types/llm";
import { WORD_FILE_EXTENSIONS } from "../../constants";
import { useTranslation } from "../../i18n";

/**
 * Context menu component properties.
 */
export interface ContextMenuProps {
  x: number;
  y: number;
  targetMatch: SearchMatch;
  selectedMatches: SearchMatch[];
  onClose: () => void;
  onCopyLlm: (matches: SearchMatch[], copyType: LlmCopyType, wordMode?: WordExtractionMode) => void;
}

/**
 * ## Description
 * Determines if a given match represents a Word document.
 *
 * ## Arguments
 * - `match`: `SearchMatch` - The search match item to evaluate
 *
 * ## Returns
 * - `boolean`: `true` if Word document, `false` otherwise
 *
 * ## Errors / Exceptions
 * Does not throw.
 */
const isWordMatch = (match: SearchMatch): boolean => {
  if (match.match_type === "WordText") return true;
  const lower = match.file_name.toLowerCase();
  // Constant reference: WORD_FILE_EXTENSIONS.DEFAULT_LIST
  return (WORD_FILE_EXTENSIONS.DEFAULT_LIST as readonly string[]).some((ext) => lower.endsWith(ext));
};

/**
 * ## Description
 * Floating context menu offering copy type actions tailored to Excel or Word search results.
 * Clamps coordinates to stay within viewport bounds and automatically dismisses on outside clicks or Escape.
 *
 * ## Arguments
 * - `props`: `ContextMenuProps` - Position coordinates, target match, multi-selection, and callback handlers
 *
 * ## Returns
 * - `React.ReactElement`: Rendered floating context menu
 *
 * ## Errors / Exceptions
 * Does not panic; invalid coordinates are safely clamped.
 */
export const ContextMenu: React.FC<ContextMenuProps> = ({
  x,
  y,
  targetMatch,
  selectedMatches,
  onClose,
  onCopyLlm,
}) => {
  const t = useTranslation();
  const menuRef = useRef<HTMLDivElement>(null);

  // Determine which matches to copy (if target is in multi-selection, use all selected, otherwise just target)
  const matchesToProcess =
    selectedMatches.length > 1 && selectedMatches.some((m) => m.id === targetMatch.id)
      ? selectedMatches
      : [targetMatch];

  const isWord = isWordMatch(targetMatch);

  // Outside click and Escape key listeners
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
        onClose();
      }
    };

    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        onClose();
      }
    };

    document.addEventListener("mousedown", handleClickOutside);
    document.addEventListener("keydown", handleKeyDown);
    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
      document.removeEventListener("keydown", handleKeyDown);
    };
  }, [onClose]);

  // Viewport clamping
  const menuWidth = 260;
  const menuHeight = 160;
  const clampedX = Math.max(8, Math.min(x, window.innerWidth - menuWidth - 8));
  const clampedY = Math.max(8, Math.min(y, window.innerHeight - menuHeight - 8));

  return (
    <div
      ref={menuRef}
      style={{ top: clampedY, left: clampedX }}
      className="fixed z-50 w-64 bg-zinc-900 border border-zinc-700 rounded-md shadow-2xl py-1 text-xs animate-in fade-in zoom-in-95 duration-100"
    >
      <div className="px-3 py-1.5 border-b border-zinc-800 text-[11px] text-zinc-400 font-semibold flex items-center gap-1.5">
        <Sparkles className="w-3.5 h-3.5 text-purple-400" />
        <span>{t("ui.COPY_FOR_LLM")}</span>
        {matchesToProcess.length > 1 && (
          <span className="ml-auto bg-purple-950/80 text-purple-300 border border-purple-800/60 px-1.5 py-0.2 rounded font-mono text-[10px]">
            {matchesToProcess.length}
          </span>
        )}
      </div>

      {isWord ? (
        <>
          <button
            onClick={() => {
              onCopyLlm(matchesToProcess, "table", "headings_and_paragraph");
              onClose();
            }}
            className="w-full text-left px-3 py-2 hover:bg-zinc-800 text-zinc-200 hover:text-white flex items-center gap-2 transition"
          >
            <FileText className="w-3.5 h-3.5 text-blue-400 flex-shrink-0" />
            <div className="flex flex-col">
              <span className="font-medium">{t("ui.COPY_FOR_LLM_WORD_HEADING")}</span>
            </div>
          </button>

          <button
            onClick={() => {
              onCopyLlm(matchesToProcess, "table", "paragraph_only");
              onClose();
            }}
            className="w-full text-left px-3 py-2 hover:bg-zinc-800 text-zinc-200 hover:text-white flex items-center gap-2 transition"
          >
            <AlignLeft className="w-3.5 h-3.5 text-zinc-400 flex-shrink-0" />
            <div className="flex flex-col">
              <span className="font-medium">{t("ui.COPY_FOR_LLM_WORD_PARAGRAPH")}</span>
            </div>
          </button>
        </>
      ) : (
        <>
          <button
            onClick={() => {
              onCopyLlm(matchesToProcess, "table");
              onClose();
            }}
            className="w-full text-left px-3 py-2 hover:bg-zinc-800 text-zinc-200 hover:text-white flex items-center gap-2 transition"
          >
            <Table className="w-3.5 h-3.5 text-emerald-400 flex-shrink-0" />
            <div className="flex flex-col">
              <span className="font-medium">{t("ui.COPY_FOR_LLM_TABLE")}</span>
            </div>
          </button>

          <button
            onClick={() => {
              onCopyLlm(matchesToProcess, "header_cell_only");
              onClose();
            }}
            className="w-full text-left px-3 py-2 hover:bg-zinc-800 text-zinc-200 hover:text-white flex items-center gap-2 transition"
          >
            <Copy className="w-3.5 h-3.5 text-zinc-400 flex-shrink-0" />
            <div className="flex flex-col">
              <span className="font-medium">{t("ui.COPY_FOR_LLM_HEADER_ONLY")}</span>
            </div>
          </button>
        </>
      )}
    </div>
  );
};
