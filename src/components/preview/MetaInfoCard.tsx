/**
 * @fileoverview 検索一致詳細メタ情報表示カードコンポーネント (src/components/preview/MetaInfoCard.tsx)
 *
 * ## 処理内容
 * 選択された検索一致結果（SearchMatch）の一致種別（セル値、数式、コメント、非表示シート）、
 * 完全なセル内容文字列、行番号、列番号、および非表示ステータスをカード形式で詳細表示する。
 * 憲章原則I（自然かつ正確な日本語）、原則II（定数の外部抽出とハードコード禁止）、原則III（網羅的なヘッダコメント）に準拠。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 初版策定。
 * - v1.1.0 (2026-09-26, AI Agent): 憲章原則に準拠し、UIメッセージの定数参照化および4要素ヘッダコメントを追加。
 */

import React from "react";
import { Info } from "lucide-react";
import { SearchMatch } from "../../types/search";
import { useTranslation } from "../../i18n";


/**
 * 検索一致詳細メタ情報カードコンポーネントのプロパティ定義
 *
 * ## プロパティ一覧
 * - `match`: SearchMatch | null - 表示対象の検索一致データオブジェクト
 */
interface MetaInfoCardProps {
  match: SearchMatch | null;
}

/**
 * 検索一致詳細メタ情報カードコンポーネント
 *
 * ## 処理詳細
 * 一致種別に応じた日本語ラベルの判定、セル内容の表示、行列座標のレンダリングを行う。
 *
 * ## 引数
 * - `props`: MetaInfoCardProps - コンポーネントプロパティ
 *
 * ## 戻り値
 * - `React.ReactElement | null`: カードUI要素。一致結果が未選択の場合は null。
 *
 * ## エラー・例外条件
 * - `match` が null の場合は何も描画せず安全に終了する。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 初版作成。
 * - v1.1.0 (2026-09-26, AI Agent): 憲章原則に準拠し、定数参照と4要素コメントを追加。
 */
export const MetaInfoCard: React.FC<MetaInfoCardProps> = ({ match }) => {
  const t = useTranslation();
  if (!match) return null;

  const matchTypeLabel = (() => {
    switch (match.match_type) {
      case "CellValue":
        // 定数参照: t("ui.MATCH_TYPE_CELL_VALUE")
        return t("ui.MATCH_TYPE_CELL_VALUE");
      case "Formula":
        // 定数参照: t("ui.MATCH_TYPE_FORMULA")
        return t("ui.MATCH_TYPE_FORMULA");
      case "Comment":
        // 定数参照: t("ui.MATCH_TYPE_COMMENT")
        return t("ui.MATCH_TYPE_COMMENT");
      case "HiddenSheet":
        // 定数参照: t("ui.MATCH_TYPE_HIDDEN_SHEET")
        return t("ui.MATCH_TYPE_HIDDEN_SHEET");
    }
  })();

  return (
    <div className="mt-3 p-3 bg-zinc-900/90 rounded-lg border border-zinc-800 text-xs space-y-2 mr-2 flex-shrink-0">
      <div className="flex items-center justify-between text-zinc-400">
        <span className="font-medium text-zinc-300 flex items-center gap-1.5">
          {/* 定数参照: t("ui.MATCH_DETAIL_TITLE") */}
          <Info className="w-3.5 h-3.5 text-emerald-400" /> {t("ui.MATCH_DETAIL_TITLE")}
        </span>
        <span className="text-[11px] text-zinc-500">
          {/* 定数参照: t("ui.TYPE_LABEL") */}
          {t("ui.TYPE_LABEL")} <strong className="text-zinc-300">{matchTypeLabel}</strong>
        </span>
      </div>

      <div className="text-zinc-300 bg-zinc-950/80 p-2 rounded border border-zinc-800/80 font-mono text-[11px] select-text break-words">
        {/* 定数参照: t("ui.EMPTY_CONTENT") */}
        {match.full_content || t("ui.EMPTY_CONTENT")}
      </div>

      <div className="flex items-center gap-4 text-[11px] text-zinc-400 pt-1">
        <span>
          {/* 定数参照: t("ui.ROW_NUMBER_LABEL") */}
          {t("ui.ROW_NUMBER_LABEL")} <strong className="text-zinc-200">{match.row_index}</strong>
        </span>
        <span>
          {/* 定数参照: t("ui.COL_NUMBER_LABEL") */}
          {t("ui.COL_NUMBER_LABEL")}{" "}
          <strong className="text-zinc-200">
            {match.col_index} ({match.col_name})
          </strong>
        </span>
        <span>
          {/* 定数参照: t("ui.HIDDEN_STATUS_LABEL"), STATUS_HIDDEN, STATUS_VISIBLE */}
          {t("ui.HIDDEN_STATUS_LABEL")}{" "}
          <strong className="text-zinc-200">
            {match.match_type === "HiddenSheet" ? t("ui.STATUS_HIDDEN") : t("ui.STATUS_VISIBLE")}
          </strong>
        </span>
      </div>
    </div>
  );
};
