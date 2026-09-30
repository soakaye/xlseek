/**
 * @fileoverview Frontend common types export module (src/types/index.ts)
 *
 * ## Description
 * Aggregates and re-exports type definitions used throughout the frontend.
 * Provides search phase types, search input state types, and preview model interfaces.
 * Complies with Constitution Principle III (comprehensive documentation).
 */

export * from "./search";
export * from "./defaultOptions";

/**
 * Search progress phase type (SearchPhase)
 */
export type SearchPhase =
  | "idle"
  | "scanning"
  | "processing"
  | "completed"
  | "cancelled"
  | "error";

/**
 * Search input state interface (SearchInputState)
 */
export interface SearchInputState {
  keyword: string;
  target_dir: string;
  extensions: string[];
  isComposing: boolean;
  compositionEndTime: number;
}

/**
 * Search progress state interface (SearchProgressState)
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
