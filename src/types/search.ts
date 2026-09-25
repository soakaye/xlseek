export type MatchType = "CellValue" | "Formula" | "Comment" | "HiddenSheet";

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

export type ScanState = "Scanning" | "Completed" | "Cancelled" | "Error";

export interface ScanProgress {
  state: ScanState;
  scanned_files: number;
  total_files: number;
  matches_found: number;
  current_file: string;
  elapsed_ms: number;
}

export interface PreviewColumn {
  key: string;
  label: string;
}

export interface CellValueInfo {
  value: string;
  is_target: boolean;
  formula?: string | null;
}

export interface PreviewRow {
  row_number: number;
  cells: Record<string, CellValueInfo>;
}

export interface CellPreviewData {
  target_row: number;
  target_col: number;
  columns: PreviewColumn[];
  rows: PreviewRow[];
  sheets_in_workbook: string[];
}

export type ExportFormat = "csv" | "xlsx";

export interface ExportRequest {
  format: ExportFormat;
  output_path: string;
  items: SearchMatch[];
}
