/**
 * @fileoverview Search, preview, and export common type definitions (src/types/search.ts)
 *
 * ## Description
 * Defines search queries, search result matches, scan progress,
 * cell preview data, export requests, and external application integration types.
 * Complies with Constitution Principle III (comprehensive documentation).
 */

/**
 * Match type enumeration
 */
export type MatchType = "CellValue" | "Formula" | "Comment" | "HiddenSheet" | "Shape";

/**
 * Search query parameter interface
 */
export interface SearchQuery {
  keyword: string;
  target_dir: string;
  match_case?: boolean;
  use_regex?: boolean;
  include_formula?: boolean;
  include_comment?: boolean;
  include_shape?: boolean;
  include_hidden?: boolean;
  extensions?: string[];
}

/**
 * Search match item interface
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
  shape_name: string | null;
  sheet_hidden: boolean;
  snippet: string;
  full_content: string;
  formula: string | null;
  sheets_in_workbook: string[];
}

/**
 * Scan state union type
 */
export type ScanState = "Scanning" | "Completed" | "Cancelled" | "Error";
export type ErrorCode =
  | "invalid_regex"
  | "path_not_found"
  | "workbook_open_failed"
  | "preview_failed"
  | "export_failed"
  | "app_launch_failed"
  | "folder_open_failed"
  | "permission_denied"
  | "search_failed"
  | "internal_error";

/**
 * Scan progress event information interface
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
 * Preview column header interface
 */
export interface PreviewColumn {
  key: string;
  label: string;
}

/**
 * Preview cell information interface
 */
export interface CellValueInfo {
  value: string;
  is_target: boolean;
  formula?: string | null;
}

/**
 * Preview row data interface
 */
export interface PreviewRow {
  row_number: number;
  cells: Record<string, CellValueInfo>;
}

/**
 * Full surrounding cell preview data interface
 */
export interface CellPreviewData {
  target_row: number;
  target_col: number;
  columns: PreviewColumn[];
  rows: PreviewRow[];
  sheets_in_workbook: string[];
}

/**
 * Export output format union type
 */
export type ExportFormat = "csv" | "xlsx";

/**
 * Export request parameter interface
 */
import type { DisplayLanguage } from "../locale-core";

export interface ExportRequest {
  format: ExportFormat;
  language: DisplayLanguage;
  output_path: string;
  items: SearchMatch[];
}

/**
 * Supported external application info interface
 */
export interface SupportedApp {
  id: string;
  name: string;
  executable_path: string;
  is_default: boolean;
  icon_hint?: "excel" | "numbers" | "calc" | "generic" | string | null;
}
