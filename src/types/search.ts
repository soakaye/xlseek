/**
 * @fileoverview 検索・プレビュー・エクスポート共通型定義 (src/types/search.ts)
 *
 * ## 処理内容
 * フロントエンド全体で使用される検索クエリ、検索結果一致アイテム、
 * スキャン進捗、セルプレビューデータ、エクスポート要求、および外部連携アプリの型を定義する。
 * 憲章原則III（ヘッダコメント）に準拠。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 初版策定。JSDocによる4要素ドキュメントの網羅。
 */

/**
 * 一致箇所の種別
 */
export type MatchType = "CellValue" | "Formula" | "Comment" | "HiddenSheet";

/**
 * 検索条件パラメータインターフェース
 */
export interface SearchQuery {
  keyword: string;
  target_dir: string;
  match_case?: boolean;
  use_regex?: boolean;
  include_formula?: boolean;
  include_comment?: boolean;
  include_hidden?: boolean;
  extensions?: string[];
}

/**
 * 検索一致アイテムインターフェース
 */
export interface SearchMatch {
  id: number;
  file_name: string;
  full_path: string;
  sheet_name: string;
  cell_address: string;
  row_index: number;
  col_index: number;
  col_name: string;
  match_type: MatchType;
  snippet: string;
  full_content: string;
  formula: string | null;
  sheets_in_workbook: string[];
}

/**
 * スキャン状態列挙型
 */
export type ScanState = "Scanning" | "Completed" | "Cancelled" | "Error";
export type ErrorCode = "invalid_regex" | "path_not_found" | "workbook_open_failed" | "preview_failed" | "export_failed" | "app_launch_failed" | "folder_open_failed" | "permission_denied" | "search_failed" | "internal_error";

/**
 * スキャン進捗イベント情報インターフェース
 */
export interface ScanProgress {
  state: ScanState;
  phase?: "preparing" | "discovering" | "scanning" | "finished";
  error_code?: string | null;
  scanned_files: number;
  total_files: number;
  matches_found: number;
  current_file: string;
  elapsed_ms: number;
}

/**
 * プレビュー列ヘッダーインターフェース
 */
export interface PreviewColumn {
  key: string;
  label: string;
}

/**
 * プレビュー単一セル情報インターフェース
 */
export interface CellValueInfo {
  value: string;
  is_target: boolean;
  formula?: string | null;
}

/**
 * プレビュー行データインターフェース
 */
export interface PreviewRow {
  row_number: number;
  cells: Record<string, CellValueInfo>;
}

/**
 * 周辺セルプレビュー全体データインターフェース
 */
export interface CellPreviewData {
  target_row: number;
  target_col: number;
  columns: PreviewColumn[];
  rows: PreviewRow[];
  sheets_in_workbook: string[];
}

/**
 * エクスポート出力フォーマット
 */
export type ExportFormat = "csv" | "xlsx";

/**
 * エクスポート要求パラメータインターフェース
 */
import type { DisplayLanguage } from "../locale-core";

export interface ExportRequest {
  format: ExportFormat;
  language: DisplayLanguage;
  output_path: string;
  items: SearchMatch[];
}

/**
 * 連携対象アプリケーション情報インターフェース
 */
export interface SupportedApp {
  id: string;
  name: string;
  executable_path: string;
  is_default: boolean;
  icon_hint?: "excel" | "numbers" | "calc" | "generic" | string | null;
}
