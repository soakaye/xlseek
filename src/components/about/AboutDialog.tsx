/**
 * @fileoverview About（アプリについて）ダイアログ最上位コンポーネント (src/components/about/AboutDialog.tsx)
 *
 * ## 処理内容
 * アプリケーションの基本情報（名称、バージョン、概要、著作権）および利用している全サードパーティ製
 * パッケージのオープンソースライセンス情報をモーダル表示する。
 * タブ切り替え（アプリ情報 / オープンソースライセンス）、ESCキー押下やバックドロップクリックによる
 * 安全な閉鎖処理、および子コンポーネント（PackageList, PackageDetail）の統合制御を行う。
 * 憲章原則I（自然かつ正確な日本語）、原則II（定数の一元化）、原則III（網羅的なヘッダコメント）、原則IV（モジュール設計）に準拠。
 *
 * ## プロパティ (AboutDialogProps)
 * - `isOpen`: boolean - ダイアログの表示状態
 * - `onClose`: () => void - ダイアログを閉じるコールバック
 * - `onShowToast`: (message: string) => void - コピー完了等のトースト通知コールバック
 * - `licenses`?: PackageLicenseRecord[] - 表示対象のライセンスデータ（省略時はバンドルJSONを使用）
 *
 * ## 戻り値
 * - `React.ReactElement | null`: ダイアログUI（非表示時は null）
 *
 * ## エラー・例外条件
 * - クリップボードコピー失敗時は例外を捕捉し、日本語のエラートーストを通知する。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 初版策定。モーダルシェルおよびアプリ情報タブの実装。
 * - v1.1.0 (2026-09-26, AI Agent): PackageListおよびPackageDetailコンポーネントの分離・統合。
 */

import React, { useState, useEffect, useMemo, useCallback } from "react";
import { X, FileCode2, ShieldCheck } from "lucide-react";
import { PackageLicenseRecord, AboutTabType } from "../../types/license";
import { ABOUT_DIALOG_CONSTANTS, LAYOUT_CONSTANTS } from "../../constants";
import defaultLicensesJson from "../../constants/licenses.json";
import { PackageList } from "./PackageList";
import { PackageDetail } from "./PackageDetail";

export interface AboutDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onShowToast: (message: string) => void;
  licenses?: PackageLicenseRecord[];
}

/**
 * Aboutダイアログモーダルコンポーネント
 *
 * ## 処理詳細
 * モーダル表示状態、アクティブタブ、キーワード検索、および選択中のパッケージを管理し、
 * キーボードESC操作やバックドロップクリックによる安全な閉鎖イベントを処理する。
 *
 * ## 引数
 * @param props - AboutDialogProps
 *
 * ## 戻り値
 * @returns レンダリング要素またはnull
 */
export const AboutDialog: React.FC<AboutDialogProps> = ({
  isOpen,
  onClose,
  onShowToast,
  licenses = defaultLicensesJson as PackageLicenseRecord[],
}) => {
  const [activeTab, setActiveTab] = useState<AboutTabType>("about");
  const [searchKeyword, setSearchKeyword] = useState<string>("");
  const [selectedPackageId, setSelectedPackageId] = useState<string | null>(null);
  const [copyFeedback, setCopyFeedback] = useState<boolean>(false);

  // ダイアログが開かれた際に初期選択を設定
  useEffect(() => {
    if (isOpen) {
      if (licenses.length > 0 && !selectedPackageId) {
        setSelectedPackageId(licenses[0].id);
      }
    }
  }, [isOpen, licenses, selectedPackageId]);

  // ESCキー押下によるダイアログ閉鎖の購読
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape" && isOpen) {
        onClose();
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [isOpen, onClose]);

  // 検索キーワードに基づくパッケージ一覧のフィルタリング
  const filteredPackages = useMemo(() => {
    const keyword = searchKeyword.trim().toLowerCase();
    if (!keyword) {
      return licenses;
    }
    return licenses.filter(
      (pkg) =>
        pkg.name.toLowerCase().includes(keyword) ||
        pkg.license.toLowerCase().includes(keyword) ||
        pkg.version.toLowerCase().includes(keyword)
    );
  }, [licenses, searchKeyword]);

  // フィルタリング結果に応じた選択アイテムの同期
  useEffect(() => {
    if (filteredPackages.length > 0) {
      const exists = filteredPackages.some((p) => p.id === selectedPackageId);
      if (!exists) {
        setSelectedPackageId(filteredPackages[0].id);
      }
    } else {
      setSelectedPackageId(null);
    }
  }, [filteredPackages, selectedPackageId]);

  // 現在選択中のパッケージレコード
  const selectedPackage = useMemo(() => {
    if (!selectedPackageId) return null;
    return filteredPackages.find((p) => p.id === selectedPackageId) || null;
  }, [filteredPackages, selectedPackageId]);

  // ライセンス本文のクリップボードコピー処理
  const handleCopyLicense = useCallback(
    async (text: string) => {
      try {
        await navigator.clipboard.writeText(text);
        setCopyFeedback(true);
        const pkgName = selectedPackage ? selectedPackage.name : "";
        onShowToast(`${pkgName}${ABOUT_DIALOG_CONSTANTS.TOAST_COPIED_SUFFIX}`);
        setTimeout(() => {
          setCopyFeedback(false);
        }, LAYOUT_CONSTANTS.COPY_FEEDBACK_DURATION_MS);
      } catch {
        console.error("[AboutDialog] Failed to copy license text");
        onShowToast(ABOUT_DIALOG_CONSTANTS.TOAST_COPY_FAILED);
      }
    },
    [onShowToast, selectedPackage]
  );

  if (!isOpen) return null;

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm select-none p-4"
      onClick={onClose}
    >
      {/* モーダルウィンドウ本体 */}
      <div
        className="w-full max-w-4xl h-[620px] bg-[#18181b] border border-zinc-700/80 rounded-xl shadow-2xl flex flex-col overflow-hidden text-zinc-100 animate-in fade-in zoom-in-95 duration-150"
        onClick={(e) => e.stopPropagation()}
      >
        {/* モーダル上部ヘッダーバー */}
        <div className="h-14 px-5 bg-[#141416] border-b border-zinc-800 flex items-center justify-between flex-shrink-0">
          <div className="flex items-center gap-3">
            <div className="w-8 h-8 rounded-lg bg-excel/20 border border-excel/40 flex items-center justify-center text-excel-light">
              <ShieldCheck className="w-5 h-5" />
            </div>
            <div>
              <h2 className="text-base font-semibold text-zinc-100 flex items-center gap-2">
                {ABOUT_DIALOG_CONSTANTS.TITLE}
                <span className="text-[11px] font-mono px-2 py-0.5 rounded-full bg-zinc-800 text-zinc-300 border border-zinc-700">
                  v{ABOUT_DIALOG_CONSTANTS.APP_VERSION}
                </span>
              </h2>
            </div>
          </div>

          {/* タブ切り替えボタン */}
          <div className="flex items-center gap-1 bg-[#202024] p-1 rounded-lg border border-zinc-800">
            <button
              type="button"
              onClick={() => setActiveTab("about")}
              className={`px-3 py-1 text-xs font-medium rounded-md transition-all cursor-pointer ${
                activeTab === "about"
                  ? "bg-zinc-800 text-zinc-100 shadow-sm border border-zinc-700/60"
                  : "text-zinc-400 hover:text-zinc-200"
              }`}
            >
              {ABOUT_DIALOG_CONSTANTS.TAB_ABOUT}
            </button>
            <button
              type="button"
              onClick={() => setActiveTab("licenses")}
              className={`px-3 py-1 text-xs font-medium rounded-md transition-all cursor-pointer flex items-center gap-1.5 ${
                activeTab === "licenses"
                  ? "bg-zinc-800 text-zinc-100 shadow-sm border border-zinc-700/60"
                  : "text-zinc-400 hover:text-zinc-200"
              }`}
            >
              <FileCode2 className="w-3.5 h-3.5" />
              <span>{ABOUT_DIALOG_CONSTANTS.TAB_LICENSES}</span>
              <span className="text-[10px] font-mono px-1.5 py-0.2 rounded-full bg-zinc-700/80 text-zinc-300">
                {licenses.length}
              </span>
            </button>
          </div>

          {/* 閉じるボタン (✕) */}
          <button
            type="button"
            onClick={onClose}
            title={ABOUT_DIALOG_CONSTANTS.BUTTON_CLOSE}
            className="p-1.5 text-zinc-400 hover:text-zinc-100 hover:bg-zinc-800 rounded-lg transition cursor-pointer"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* モーダルコンテンツ領域 */}
        <div className="flex-1 overflow-hidden flex flex-col bg-[#161618]">
          {activeTab === "about" ? (
            /* タブ 1: アプリケーション基本情報 */
            <div className="flex-1 overflow-y-auto p-8 flex flex-col items-center justify-center text-center">
              <div className="w-20 h-20 rounded-2xl bg-gradient-to-br from-emerald-500/20 to-excel/10 border border-emerald-500/30 flex items-center justify-center mb-5 shadow-lg shadow-emerald-950/20">
                <ShieldCheck className="w-10 h-10 text-emerald-400" />
              </div>

              <h3 className="text-2xl font-bold text-zinc-100 mb-1">
                {ABOUT_DIALOG_CONSTANTS.APP_NAME}
              </h3>
              <p className="text-xs font-mono text-zinc-400 mb-4 bg-zinc-900/80 px-3 py-1 rounded-full border border-zinc-800">
                Version {ABOUT_DIALOG_CONSTANTS.APP_VERSION}
              </p>

              <p className="text-sm text-zinc-300 max-w-lg mb-6 leading-relaxed">
                {ABOUT_DIALOG_CONSTANTS.APP_DESCRIPTION}
              </p>

              <div className="w-full max-w-md bg-[#1c1c20] border border-zinc-800 rounded-lg p-4 text-xs text-zinc-400 space-y-2 mb-6">
                <div className="flex justify-between items-center py-1 border-b border-zinc-800/80">
                  <span className="text-zinc-500">{ABOUT_DIALOG_CONSTANTS.COPYRIGHT_LABEL}</span>
                  <span className="font-mono text-zinc-300">{ABOUT_DIALOG_CONSTANTS.COPYRIGHT}</span>
                </div>
                <div className="flex justify-between items-center py-1 border-b border-zinc-800/80">
                  <span className="text-zinc-500">{ABOUT_DIALOG_CONSTANTS.LICENSE_LABEL}</span>
                  <span className="font-medium text-emerald-400">{ABOUT_DIALOG_CONSTANTS.APP_LICENSE_LABEL}</span>
                </div>
                <div className="flex justify-between items-center py-1">
                  <span className="text-zinc-500">{ABOUT_DIALOG_CONSTANTS.THIRD_PARTY_LABEL}</span>
                  <span className="font-mono text-zinc-300">
                    {licenses.length}
                    {ABOUT_DIALOG_CONSTANTS.PACKAGE_COUNT_SUFFIX}
                  </span>
                </div>
              </div>

              <button
                type="button"
                onClick={() => setActiveTab("licenses")}
                className="px-4 py-2 bg-zinc-800 hover:bg-zinc-700 text-zinc-200 hover:text-white rounded-lg border border-zinc-700 text-xs font-medium transition flex items-center gap-2 cursor-pointer"
              >
                <FileCode2 className="w-4 h-4 text-emerald-400" />
                <span>{ABOUT_DIALOG_CONSTANTS.PACKAGE_LIST_LINK}</span>
              </button>
            </div>
          ) : (
            /* タブ 2: オープンソースライセンス一覧 (2ペイン Master-Detail) */
            <div className="flex-1 flex overflow-hidden min-h-0">
              <PackageList
                packages={filteredPackages}
                totalPackageCount={licenses.length}
                selectedPackageId={selectedPackageId}
                onSelectPackage={setSelectedPackageId}
                searchKeyword={searchKeyword}
                onSearchChange={setSearchKeyword}
              />
              <PackageDetail
                package={selectedPackage}
                onCopyLicense={handleCopyLicense}
                isCopied={copyFeedback}
              />
            </div>
          )}
        </div>

        {/* モーダル下部フッター */}
        <div className="h-12 px-5 bg-[#141416] border-t border-zinc-800 flex items-center justify-end flex-shrink-0">
          <button
            type="button"
            onClick={onClose}
            className="px-4 py-1.5 bg-zinc-800 hover:bg-zinc-700 text-zinc-200 hover:text-white text-xs font-medium rounded-lg border border-zinc-700 transition cursor-pointer"
          >
            {ABOUT_DIALOG_CONSTANTS.BUTTON_CLOSE}
          </button>
        </div>
      </div>
    </div>
  );
};
