/**
 * @fileoverview パッケージ一覧リストコンポーネント (src/components/about/PackageList.tsx)
 *
 * ## 処理内容
 * Aboutダイアログ「オープンソースライセンス」タブの左ペイン（幅40%相当）を担う。
 * リアルタイムインクリメンタル検索バーおよび全利用パッケージの行リストを表示し、
 * 各アイテムのパッケージ名、バージョン、ライセンス種別、利用元（Rust/npm）バッジを描画する。
 * 選択中アイテムのアクティブハイライトおよびアイテムクリック時の選択イベント通知を行う。
 * 憲章原則I（自然かつ正確な日本語）、原則II（定数の一元化）、原則III（網羅的なヘッダコメント）、原則IV（モジュール設計）に準拠。
 *
 * ## プロパティ (PackageListProps)
 * - `packages`: PackageLicenseRecord[] - フィルタリング適用後のパッケージ一覧
 * - `totalPackageCount`: number - 全パッケージ総数
 * - `selectedPackageId`: string | null - 現在選択されているパッケージの識別子 (`{name}@{version}`)
 * - `onSelectPackage`: (id: string) => void - パッケージ行選択時のコールバック
 * - `searchKeyword`: string - 検索キーワード
 * - `onSearchChange`: (keyword: string) => void - 検索キーワード変更時のコールバック
 *
 * ## 戻り値
 * - `React.ReactElement`: パッケージ検索および一覧リスト要素
 *
 * ## エラー・例外条件
 * - 0件ヒット時は定数メッセージ `NO_PACKAGES_FOUND` を表示する。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 初版策定。マスターリストおよびインクリメンタル検索UIの実装。
 */

import React from "react";
import { X } from "lucide-react";
import { PackageLicenseRecord } from "../../types/license";
import { useTranslation } from "../../i18n";


export interface PackageListProps {
  packages: PackageLicenseRecord[];
  totalPackageCount: number;
  selectedPackageId: string | null;
  onSelectPackage: (id: string) => void;
  searchKeyword: string;
  onSearchChange: (keyword: string) => void;
}

/**
 * パッケージ一覧リストコンポーネント
 *
 * ## 処理詳細
 * 検索キーワード入力ボックスおよびヒットしたパッケージ群のスクロール一覧を描画し、
 * 選択行に応じたハイライト表示およびクリック選択イベントを制御する。
 *
 * ## 引数
 * @param props - PackageListProps
 *
 * ## 戻り値
 * @returns レンダリング要素
 */
export const PackageList: React.FC<PackageListProps> = ({
  packages,
  totalPackageCount,
  selectedPackageId,
  onSelectPackage,
  searchKeyword,
  onSearchChange,
}) => {
  const t = useTranslation();
  return (
    <div className="w-[340px] flex-shrink-0 border-r border-zinc-800 flex flex-col bg-[#141416]">
      {/* 検索入力バー */}
      <div className="p-3 border-b border-zinc-800 flex-shrink-0">
        <div className="relative">
          <input
            type="text"
            value={searchKeyword}
            onChange={(e) => onSearchChange(e.target.value)}
            placeholder={t("about.SEARCH_PLACEHOLDER")}
            className="w-full pl-3 pr-8 py-1.5 bg-[#202024] border border-zinc-700/80 rounded-md text-xs text-zinc-100 placeholder-zinc-500 focus:outline-none focus:border-excel-light"
          />
          {searchKeyword && (
            <button
              type="button"
              onClick={() => onSearchChange("")}
              title={t("about.SEARCH_CLEAR_TOOLTIP")}
              className="absolute inset-y-0 right-1 px-1.5 text-zinc-400 hover:text-zinc-200 flex items-center cursor-pointer"
            >
              <X className="w-3.5 h-3.5" />
            </button>
          )}
        </div>
        <div className="flex justify-between items-center text-[10px] text-zinc-500 mt-2 px-1">
          <span>
            {packages.length} / {totalPackageCount} {t("about.PACKAGE_COUNT_UNIT")}
          </span>
          {searchKeyword && <span>{t("about.FILTER_ACTIVE")}</span>}
        </div>
      </div>

      {/* パッケージリスト */}
      <div className="flex-1 overflow-y-auto divide-y divide-zinc-800/60">
        {packages.length === 0 ? (
          <div className="p-6 text-center text-xs text-zinc-500">
            {t("about.NO_PACKAGES_FOUND")}
          </div>
        ) : (
          packages.map((pkg) => {
            const isSelected = pkg.id === selectedPackageId;
            return (
              <div
                key={pkg.id}
                onClick={() => onSelectPackage(pkg.id)}
                className={`p-3 text-left transition cursor-pointer ${
                  isSelected
                    ? "bg-zinc-800/90 border-l-2 border-l-emerald-400"
                    : "hover:bg-zinc-800/40"
                }`}
              >
                <div className="flex items-center justify-between gap-1 mb-1">
                  <span
                    className={`text-xs font-medium truncate ${
                      isSelected ? "text-emerald-300 font-semibold" : "text-zinc-200"
                    }`}
                  >
                    {pkg.name}
                  </span>
                  <span className="text-[10px] font-mono px-1.5 py-0.2 rounded bg-zinc-900 border border-zinc-800 text-zinc-400 flex-shrink-0">
                    v{pkg.version}
                  </span>
                </div>
                <div className="flex items-center gap-1.5 text-[10px]">
                  <span className="px-1.5 py-0.2 rounded bg-emerald-950/40 text-emerald-400 border border-emerald-900/60 font-medium">
                    {pkg.license}
                  </span>
                  <span className="text-zinc-500 font-mono">
                    {pkg.source === "rust" ? "crate" : "npm"}
                  </span>
                </div>
              </div>
            );
          })
        )}
      </div>
    </div>
  );
};
