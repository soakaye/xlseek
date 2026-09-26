/**
 * @fileoverview サードパーティライセンスおよびAboutダイアログ型定義 (src/types/license.ts)
 *
 * ## 処理内容
 * アプリケーションで使用されているオープンソースソフトウェア（OSS）のライセンスレコード型、
 * アプリケーション基本情報型、およびAboutダイアログの表示・選択・検索状態を管理する型を定義する。
 * 憲章原則I（自然かつ正確な日本語ドキュメント）、原則III（網羅的なヘッダコメント）、原則IV（モジュール設計）に準拠。
 *
 * ## 型一覧
 * - `PackageLicenseRecord`: サードパーティ製パッケージ（Rust / npm）の個別ライセンス情報レコード型
 * - `AppMetaInfo`: アプリケーション自体のバージョン・概要・著作権表示メタデータ型
 * - `AboutTabType`: Aboutダイアログ内のアクティブタブ区分 ("about" | "licenses")
 * - `AboutDialogState`: Aboutダイアログ全体のUIおよび選択・検索状態管理型
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 初版策定。PackageLicenseRecord, AppMetaInfo, AboutDialogState等の型定義。
 */

/**
 * サードパーティ製パッケージの個別ライセンス情報レコード型
 *
 * ## フィールド定義
 * - `id`: 一意識別子 (`{name}@{version}`)
 * - `name`: パッケージ名称（クレート名またはnpmパッケージ名）
 * - `version`: パッケージのバージョン文字列
 * - `source`: パッケージの出自 ("rust": バックエンドクレート, "npm": フロントエンドライブラリ)
 * - `license`: SPDXライセンス識別子またはライセンス名称（例: "MIT", "Apache-2.0"）
 * - `author`: 著作者・著作権者表記（Copyright notice、存在しない場合は null）
 * - `repository`: リポジトリまたは公式サイトのURL（存在しない場合は null）
 * - `license_text`: 原著作者によって提供された正規のライセンス全文テキスト
 */
export interface PackageLicenseRecord {
  id: string;
  name: string;
  version: string;
  source: "rust" | "npm";
  license: string;
  author: string | null;
  repository: string | null;
  license_text: string;
}

/**
 * アプリケーション基本情報メタデータ型
 *
 * ## フィールド定義
 * - `name`: アプリケーション名称 ("Excel Grep")
 * - `version`: 現在のアプリケーションバージョン番号 (例: "0.1.0")
 * - `description`: アプリケーションの概要説明文
 * - `copyright`: アプリケーション全体の著作権表示文字列
 * - `license`: アプリケーション自体の配布ライセンス ("MIT License")
 */
export interface AppMetaInfo {
  name: string;
  version: string;
  description: string;
  copyright: string;
  license: string;
}

/**
 * Aboutダイアログのアクティブタブ区分型
 * - `"about"`: アプリ基本情報・概要・著作権表示
 * - `"licenses"`: オープンソースライセンス一覧および詳細閲覧
 */
export type AboutTabType = "about" | "licenses";

/**
 * AboutダイアログUI状態型
 *
 * ## フィールド定義
 * - `isOpen`: ダイアログのモーダル表示フラグ
 * - `activeTab`: 現在アクティブな表示タブ
 * - `searchKeyword`: ライセンス一覧のフィルタリング用検索文字列
 * - `selectedPackageId`: 2ペイン右側で詳細表示されているパッケージの id
 * - `copyFeedback`: クリップボードコピー成功時の一時的視覚フィードバック表示中フラグ
 */
export interface AboutDialogState {
  isOpen: boolean;
  activeTab: AboutTabType;
  searchKeyword: string;
  selectedPackageId: string | null;
  copyFeedback: boolean;
}
