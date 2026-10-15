/**
 * Copyright (c) 2026 soakaye
 *
 * @fileoverview Accessible dropdown menu for ignore pattern history selection (src/components/search/IgnoreHistoryDropdown.tsx)
 *
 * ## Description
 * Renders an accessible listbox displaying recent ignore pattern history entries.
 * Supports mouse selection, active item highlighting, and keyboard focus states.
 * Complies with Constitution Principle I (English documentation), Principle II (Centralized constants),
 * and Principle III (Comprehensive header comments).
 */

import React from "react";
import { History, Trash2 } from "lucide-react";
import { IGNORE_PATTERNS_CONSTANTS } from "../../constants";
import { useTranslation } from "../../i18n";

export interface IgnoreHistoryDropdownProps {
  isOpen: boolean;
  history: string[];
  activeOption: number | null;
  onSelect: (pattern: string) => void;
  onClose: () => void;
  onClearHistory?: () => void;
}

/**
 * ## Description
 * Accessible dropdown listbox displaying recent ignore patterns.
 *
 * ## Arguments
 * @param props - IgnoreHistoryDropdownProps
 *
 * ## Returns
 * @returns JSX Element or null if not open
 *
 * ## Errors / Exceptions
 * Does not throw exceptions.
 */
export const IgnoreHistoryDropdown: React.FC<IgnoreHistoryDropdownProps> = ({
  isOpen,
  history,
  activeOption,
  onSelect,
  onClose: _onClose,
  onClearHistory,
}) => {
  const t = useTranslation();

  if (!isOpen) {
    return null;
  }

  return (
    <div
      // Constant reference: IGNORE_PATTERNS_CONSTANTS.HISTORY_DROPDOWN_ID
      id={IGNORE_PATTERNS_CONSTANTS.HISTORY_DROPDOWN_ID}
      role="listbox"
      // Translation reference: ui.IGNORE_PATTERNS_LABEL
      aria-label={t("ui.IGNORE_PATTERNS_LABEL")}
      className="absolute z-30 top-full mt-1 w-full rounded-lg border border-zinc-700 bg-zinc-900 p-1 shadow-xl max-h-60 overflow-y-auto"
    >
      {history.length > 0 ? (
        <>
          <div className="space-y-0.5">
            {history.map((pattern, index) => {
              const isActive = activeOption === index;
              return (
                <button
                  key={`${index}-${pattern}`}
                  type="button"
                  role="option"
                  aria-selected={isActive}
                  onMouseDown={(e) => e.preventDefault()}
                  onClick={() => onSelect(pattern)}
                  className={`flex items-center w-full rounded px-2.5 py-1.5 text-left text-xs font-mono transition ${
                    isActive
                      ? "bg-zinc-800 text-emerald-300 ring-1 ring-emerald-500/50"
                      : "text-zinc-200 hover:bg-zinc-800 hover:text-zinc-100"
                  }`}
                >
                  <History className="w-3.5 h-3.5 mr-2 text-zinc-500 flex-shrink-0" />
                  <span className="truncate flex-1">{pattern}</span>
                </button>
              );
            })}
          </div>

          {onClearHistory && (
            <div className="mt-1 pt-1 border-t border-zinc-800 flex justify-end">
              <button
                type="button"
                onMouseDown={(e) => e.preventDefault()}
                onClick={onClearHistory}
                className="flex items-center gap-1 px-2 py-1 text-[11px] text-zinc-500 hover:text-red-400 rounded transition"
              >
                <Trash2 className="w-3 h-3" />
                {/* Translation reference: ui.IGNORE_PATTERNS_CLEAR */}
                <span>{t("ui.IGNORE_PATTERNS_CLEAR")}</span>
              </button>
            </div>
          )}
        </>
      ) : (
        <div className="px-3 py-2 text-xs text-zinc-500 text-center">
          {/* Translation reference: ui.HISTORY_EMPTY */}
          {t("ui.HISTORY_EMPTY")}
        </div>
      )}
    </div>
  );
};
