/**
 * @fileoverview フロントエンド共通型定義エクスポートモジュール (src/types/index.ts)
 *
 * ## 処理内容
 * フロントエンド全体で使用される型定義を集約・再エクスポートする。
 * 検索フェーズ型、検索入力状態型、およびプレビュー用モデルのインターフェースを提供する。
 * 憲章原則III（ヘッダコメント）に準拠。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 初版策定。search.ts からの再エクスポートおよび SearchPhase, SearchInputState の定義。
 */

export * from "./search";
export * from "./defaultOptions";

/**
 * 検索進行フェーズ型 (SearchPhase)
 */
export type SearchPhase =
  | "idle"
  | "scanning"
  | "processing"
  | "completed"
  | "cancelled"
  | "error";

/**
 * 検索入力状態インターフェース (SearchInputState)
 */
export interface SearchInputState {
  keyword: string;
  target_dir: string;
  extensions: string[];
  isComposing: boolean;
  compositionEndTime: number;
}

/**
 * 検索進行状態インターフェース (SearchProgressState)
 */
export interface SearchProgressState {
  phase: SearchPhase;
  targetDir: string;
  currentFile?: string;
  scannedFiles: number;
  totalFiles: number;
  matchCount: number;
  elapsedSeconds: number;
}
