/**
 * @fileoverview フロントエンド定数一元定義モジュール (src/constants/index.ts)
 *
 * ## 処理内容
 * フロントエンド全体で使用されるIPCコマンド名、Tauriイベント名、UIレイアウト寸法、
 * タイミング設定、デフォルト拡張子、および表示言語に依存しないアプリ定数を一元管理する。
 * 憲章原則I（自然かつ正確な日本語）、原則II（定数の外部抽出とハードコード禁止）に準拠。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 初版策定。全定数の集約とJSDocドキュメンテーションの付与。
 * - v1.1.0 (2026-09-26, AI Agent): UI文言・レイアウト・スクロール寸法の定数を拡充。
 * - v1.2.0 (2026-09-26, AI Agent): 拡張子トグル選択機能のUI文言定数を追加。
 * - v1.3.0 (2026-09-26, AI Agent): 探索中表示、フォルダスキャン、待機中等のUI文言定数を拡充。
 * - v1.4.0 (2026-09-26, AI Agent): Aboutダイアログおよびパッケージライセンス表示定数 (ABOUT_DIALOG_CONSTANTS) を追加。
 * - v1.5.0 (2026-09-26, AI Agent): システムメニュー連携用Aboutダイアログ表示イベント定数 (OPEN_ABOUT_DIALOG) を追加。
 * - v1.6.0 (2026-09-26, AI Agent): 著作権者表記 (ABOUT_DIALOG_CONSTANTS.COPYRIGHT) を soakaye に更新。
 * - v1.9.0 (2026-09-26, Codex): 翻訳文言をプラグインカタログへ移し、定数モジュールから日英辞書と Proxy を削除。
 * - v1.10.0 (2026-09-27, Codex): 検索履歴・パス補完の設定値と IPC 名を追加。
 * - v1.11.0 (2026-09-27, Codex): 履歴操作で使うキー名を追加。
 */

// ==============================================================================
// 0. 言語設定値
// ===============================================================================
export const LANGUAGE_PREFERENCES = { DEFAULT: "default", JA: "ja", EN: "en" } as const;
export const DISPLAY_LANGUAGES = { JA: "ja", EN: "en" } as const;
export const LANGUAGE_STORAGE_KEY = "exlgrep.language";
export const APP_CONSTANTS = { ROOT_ELEMENT_ID: "root", APP_VERSION: "0.1.0" } as const;
export const I18N_CONSTANTS = {
  TRANSLATION_UNAVAILABLE_KEY: "common.translationUnavailable",
  TRANSLATION_UNAVAILABLE_TEXT: "Some text could not be translated.",
  PLUGIN_LOAD_FAILED_LOG: "[i18n] Failed to load translation catalogs; using bundled English strings.",
  PLUGIN_LOCALE_FAILED_LOG: "[i18n] Failed to change plugin locale; using bundled English strings where needed.",
} as const;
export const APP_LOGS = {
  MENU_UPDATE_FAILED: "[App] Failed to update application menu language.",
  ABOUT_LISTENER_FAILED: "[App] Failed to register About dialog event listener.",
} as const;

// ===============================================================================
// 1. Tauri イベント名定数
// ==============================================================================
export const EVENT_NAMES = {
  SEARCH_MATCH: "search-match",
  SCAN_PROGRESS: "scan-progress",
  OPEN_ABOUT_DIALOG: "open-about-dialog",
} as const;

// ==============================================================================
// 2. Tauri IPC コマンド名定数
// ==============================================================================
export const COMMANDS = {
  START_SEARCH: "start_search",
  COMPLETE_DIRECTORY_PATH: "complete_directory_path",
  CANCEL_SEARCH: "cancel_search",
  GET_CELL_PREVIEW: "get_cell_preview",
  OPEN_IN_EXCEL: "open_in_excel",
  OPEN_IN_FOLDER: "open_in_folder",
  EXPORT_RESULTS: "export_results",
  RESOLVE_DROPPED_PATH: "resolve_dropped_path",
  GET_SUPPORTED_APPS: "get_supported_apps",
  LAUNCH_ASSOCIATED_APP: "launch_associated_app",
  SHOW_OPEN_WITH_DIALOG: "show_open_with_dialog",
  SET_MENU_LOCALE: "set_menu_locale",
} as const;
export const SEARCH_LABELS = {
  INCLUDE_SHAPE: "ui.OPTION_INCLUDE_SHAPE",
  SHAPE_TYPE: "ui.MATCH_TYPE_SHAPE",
  SHAPE_RESULT_TYPE: "ui.RESULT_MATCH_TYPE_SHAPE",
  SHAPE_NAME: "ui.SHAPE_NAME_LABEL",
} as const;

// 定数参照: 検索履歴の保存値と入力補完の上限・待機時間を一元管理する。
export const SEARCH_HISTORY_CONSTANTS = {
  STORAGE_KEY: "exlgrep.searchHistory",
  DEFAULT_MAX_ENTRIES: 20,
  MIN_ENTRIES: 0,
  MAX_ENTRIES: 50,
  STEP: 1,
  INPUT_ID: "search-history-limit",
} as const;

// 定数参照: パス補完の応答負荷と入力待機時間を制限する。
export const PATH_COMPLETION_CONSTANTS = {
  DEBOUNCE_MS: 200,
  MAX_RESULTS: 10,
  KEYBOARD_STEP: 1,
} as const;

// 定数参照: 履歴一覧のキー操作で使用するキー名を一元管理する。
export const KEYBOARD_KEYS = {
  ARROW_DOWN: "ArrowDown",
  ARROW_UP: "ArrowUp",
  ENTER: "Enter",
  ESCAPE: "Escape",
  IME_COMPOSITION: "Process",
  IME_COMPOSITION_KEY_CODE: 229,
  TAB: "Tab",
} as const;

// ==============================================================================
// 3. UIレイアウトおよび仮想化寸法定数 (Layout Dimensions)
// ==============================================================================
export const LAYOUT_CONSTANTS = {
  /** 検索結果テーブルの1行あたりの高さ (px) */
  RESULT_ROW_HEIGHT_PX: 36,
  /** 検索結果テーブルのヘッダー高さ (px) */
  HEADER_HEIGHT_PX: 40,
  /** プレビューセルの標準幅 (px) */
  PREVIEW_CELL_WIDTH_PX: 100,
  /** プレビューセルの標準高さ (px) */
  PREVIEW_CELL_HEIGHT_PX: 24,
  /** 仮想スクロールのオーバースキャン行数 */
  VIRTUAL_OVERSCAN: 10,
  /** シートタブの横スクロール量 (px) */
  SHEET_SCROLL_OFFSET_PX: 100,
  /** コピー成功アイコン表示タイマー (ms) */
  COPY_FEEDBACK_DURATION_MS: 2000,
} as const;

// ==============================================================================
// 4. タイミング設定定数 (Timings)
// ==============================================================================
export const TIMING_CONSTANTS = {
  /** トースト通知の自動非表示時間 (ミリ秒) */
  TOAST_DURATION_MS: 3000,
  /** 進捗UI更新のスロットル時間 (ミリ秒) */
  PROGRESS_THROTTLE_MS: 50,
} as const;

// ==============================================================================
// 5. 検索対象拡張子定数 (File Extensions)
// ==============================================================================
export const FILE_EXTENSIONS = {
  XLSX: ".xlsx",
  XLSM: ".xlsm",
  XLSB: ".xlsb",
  XLS: ".xls",
  DEFAULT_LIST: [".xlsx", ".xlsm", ".xlsb", ".xls"],
} as const;
