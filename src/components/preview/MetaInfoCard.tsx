/**
 * Copyright (c) 2026 soakaye
 *
 * @fileoverview Search match metadata details card component (src/components/preview/MetaInfoCard.tsx)
 *
 * ## Description
 * Displays detailed match attributes for the active search match (SearchMatch), including match type
 * (cell value, formula, comment, hidden sheet, shape), full content string, row/col coordinates,
 * and hidden sheet status in a card layout.
 * Complies with Constitution Principle I (English documentation), Principle II (constant reference), and Principle III (comprehensive documentation).
 */

import React from "react";
import { Info } from "lucide-react";
import { SearchMatch } from "../../types/search";
import { useTranslation } from "../../i18n";
import { SEARCH_LABELS } from "../../constants";

interface MetaInfoCardProps {
  match: SearchMatch | null;
}

/**
 * ## Description
 * Search match metadata card component.
 *
 * ## Arguments
 * @param props - MetaInfoCardProps
 *
 * ## Returns
 * @returns Rendered card element or null if no match is selected
 *
 * ## Errors / Exceptions
 * Safe exit without rendering when match is null.
 */
export const MetaInfoCard: React.FC<MetaInfoCardProps> = ({ match }) => {
  const t = useTranslation();
  if (!match) return null;

  const matchTypeLabel = (() => {
    switch (match.match_type) {
      case "CellValue":
        // Constant reference: t("ui.MATCH_TYPE_CELL_VALUE")
        return t("ui.MATCH_TYPE_CELL_VALUE");
      case "Formula":
        // Constant reference: t("ui.MATCH_TYPE_FORMULA")
        return t("ui.MATCH_TYPE_FORMULA");
      case "Comment":
        // Constant reference: t("ui.MATCH_TYPE_COMMENT")
        return t("ui.MATCH_TYPE_COMMENT");
      case "HiddenSheet":
        // Constant reference: t("ui.MATCH_TYPE_HIDDEN_SHEET")
        return t("ui.MATCH_TYPE_HIDDEN_SHEET");
      case "Shape":
        // Constant reference: SEARCH_LABELS.SHAPE_TYPE
        return t(SEARCH_LABELS.SHAPE_TYPE);
    }
  })();

  return (
    <div className="mt-3 p-3 bg-zinc-900/90 rounded-lg border border-zinc-800 text-xs space-y-2 mr-2 flex-shrink-0">
      <div className="flex items-center justify-between text-zinc-400">
        <span className="font-medium text-zinc-300 flex items-center gap-1.5">
          {/* Constant reference: t("ui.MATCH_DETAIL_TITLE") */}
          <Info className="w-3.5 h-3.5 text-emerald-400" /> {t("ui.MATCH_DETAIL_TITLE")}
        </span>
        <span className="text-[11px] text-zinc-500">
          {/* Constant reference: t("ui.TYPE_LABEL") */}
          {t("ui.TYPE_LABEL")} <strong className="text-zinc-300">{matchTypeLabel}</strong>
        </span>
      </div>

      <div className="text-zinc-300 bg-zinc-950/80 p-2 rounded border border-zinc-800/80 font-mono text-[11px] select-text break-words">
        {/* Constant reference: t("ui.EMPTY_CONTENT") */}
        {match.full_content || t("ui.EMPTY_CONTENT")}
      </div>

      {match.match_type === "Shape" && (
        <div className="text-zinc-300 text-[11px]">
          {/* Constant reference: SEARCH_LABELS.SHAPE_NAME */}
          {t(SEARCH_LABELS.SHAPE_NAME)} <strong className="text-zinc-100">{match.shape_name}</strong>
        </div>
      )}

      <div className="flex items-center gap-4 text-[11px] text-zinc-400 pt-1">
        {(match.match_type !== "Shape" || match.cell_address !== "") && (
          <>
        <span>
          {/* Constant reference: t("ui.ROW_NUMBER_LABEL") */}
          {t("ui.ROW_NUMBER_LABEL")} <strong className="text-zinc-200">{match.row_index}</strong>
        </span>
        <span>
          {/* Constant reference: t("ui.COL_NUMBER_LABEL") */}
          {t("ui.COL_NUMBER_LABEL")}{" "}
          <strong className="text-zinc-200">
            {match.col_index} ({match.col_name})
          </strong>
        </span>
          </>
        )}
        <span>
          {/* Constant reference: t("ui.HIDDEN_STATUS_LABEL"), STATUS_HIDDEN, STATUS_VISIBLE */}
          {t("ui.HIDDEN_STATUS_LABEL")}{" "}
          <strong className="text-zinc-200">
            {match.sheet_hidden ? t("ui.STATUS_HIDDEN") : t("ui.STATUS_VISIBLE")}
          </strong>
        </span>
      </div>
    </div>
  );
};
