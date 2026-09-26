/**
 * @fileoverview パッケージ詳細ビューコンポーネント (src/components/about/PackageDetail.tsx)
 *
 * ## 処理内容
 * Aboutダイアログ「オープンソースライセンス」タブの右ペイン（幅60%相当）を担う。
 * 選択中パッケージのメタヘッダー情報（名称、バージョン、SPDXライセンス、著作者、リポジトリURL）
 * および独立スクロール可能な正規ライセンス全文を描画する。
 * ライセンス本文のクリップボードコピーボタンと視覚フィードバック表示を統合する。
 * 未選択時（検索0件等）は案内プレースホルダーを表示する。
 * 憲章原則I（自然かつ正確な日本語）、原則II（定数の一元化）、原則III（網羅的なヘッダコメント）、原則IV（モジュール設計）に準拠。
 *
 * ## プロパティ (PackageDetailProps)
 * - `package`: PackageLicenseRecord | null - 表示対象のパッケージレコード
 * - `onCopyLicense`: (text: string) => void - ライセンス本文コピー要求コールバック
 * - `isCopied`: boolean - コピー完了フィードバック表示中フラグ
 *
 * ## 戻り値
 * - `React.ReactElement`: パッケージ詳細およびライセンス本文要素
 *
 * ## エラー・例外条件
 * - パッケージが null の場合は定数案内 `SELECT_PACKAGE_PROMPT` を中央表示する。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 初版策定。詳細ヘッダー、コピー操作、および全文表示UIの実装。
 */

import React from "react";
import { Copy, Check } from "lucide-react";
import { PackageLicenseRecord } from "../../types/license";
import { ABOUT_DIALOG_CONSTANTS } from "../../constants";

export interface PackageDetailProps {
  package: PackageLicenseRecord | null;
  onCopyLicense: (text: string) => void;
  isCopied: boolean;
}

/**
 * パッケージ詳細ビューコンポーネント
 *
 * ## 処理詳細
 * 選択パッケージの詳細属性およびライセンス全文を表示し、ワンクリックでのコピー実行と
 * 状態変化フィードバックを提供する。
 *
 * ## 引数
 * @param props - PackageDetailProps
 *
 * ## 戻り値
 * @returns レンダリング要素
 */
export const PackageDetail: React.FC<PackageDetailProps> = ({
  package: pkg,
  onCopyLicense,
  isCopied,
}) => {
  if (!pkg) {
    return (
      <div className="flex-1 flex items-center justify-center p-8 text-center text-xs text-zinc-500 bg-[#18181b]">
        {ABOUT_DIALOG_CONSTANTS.SELECT_PACKAGE_PROMPT}
      </div>
    );
  }

  const lineCount = pkg.license_text.split("\n").length;

  return (
    <div className="flex-1 flex flex-col bg-[#18181b] min-w-0 overflow-hidden">
      {/* 詳細ヘッダー */}
      <div className="p-4 border-b border-zinc-800 flex-shrink-0 bg-[#161618]">
        <div className="flex items-start justify-between gap-3 mb-2">
          <div>
            <div className="flex items-center gap-2">
              <h4 className="text-base font-semibold text-zinc-100">{pkg.name}</h4>
              <span className="text-xs font-mono text-zinc-400 bg-zinc-800 px-2 py-0.5 rounded border border-zinc-700">
                v{pkg.version}
              </span>
              <span className="text-[11px] font-medium px-2 py-0.5 rounded bg-emerald-950/60 text-emerald-300 border border-emerald-800">
                {pkg.license}
              </span>
            </div>
          </div>

          {/* コピーボタン */}
          <button
            type="button"
            onClick={() => onCopyLicense(pkg.license_text)}
            className={`px-3 py-1.5 rounded-lg border text-xs font-medium flex items-center gap-1.5 transition cursor-pointer flex-shrink-0 ${
              isCopied
                ? "bg-emerald-950 text-emerald-300 border-emerald-600 shadow"
                : "bg-zinc-800 hover:bg-zinc-700 text-zinc-200 hover:text-white border-zinc-700"
            }`}
          >
            {isCopied ? (
              <>
                <Check className="w-3.5 h-3.5 text-emerald-400" />
                <span>{ABOUT_DIALOG_CONSTANTS.BUTTON_COPIED}</span>
              </>
            ) : (
              <>
                <Copy className="w-3.5 h-3.5 text-zinc-400" />
                <span>{ABOUT_DIALOG_CONSTANTS.BUTTON_COPY_LICENSE}</span>
              </>
            )}
          </button>
        </div>

        {/* メタ情報（著作者・リポジトリ） */}
        <div className="space-y-1 text-xs text-zinc-400">
          {pkg.author && (
            <div className="flex items-baseline gap-2">
              <span className="text-zinc-500 font-medium w-28 flex-shrink-0">
                {ABOUT_DIALOG_CONSTANTS.AUTHOR_LABEL}
              </span>
              <span className="text-zinc-300 truncate">{pkg.author}</span>
            </div>
          )}
          {pkg.repository && (
            <div className="flex items-baseline gap-2">
              <span className="text-zinc-500 font-medium w-28 flex-shrink-0">
                {ABOUT_DIALOG_CONSTANTS.REPOSITORY_LABEL}
              </span>
              <span className="text-zinc-300 font-mono text-[11px] truncate flex items-center gap-1">
                {pkg.repository}
              </span>
            </div>
          )}
        </div>
      </div>

      {/* ライセンス本文スクロールビュー */}
      <div className="flex-1 flex flex-col p-4 min-h-0 bg-[#141416]">
        <div className="text-[11px] text-zinc-400 font-medium mb-1.5 flex items-center justify-between">
          <span>{ABOUT_DIALOG_CONSTANTS.LICENSE_TEXT_LABEL}</span>
          <span className="text-[10px] text-zinc-500 font-mono">
            {lineCount} 行
          </span>
        </div>
        <div className="flex-1 overflow-y-auto bg-[#1a1a1d] border border-zinc-800 rounded-lg p-3.5">
          <pre className="font-mono text-xs text-zinc-300 whitespace-pre-wrap leading-relaxed select-text">
            {pkg.license_text}
          </pre>
        </div>
      </div>
    </div>
  );
};
