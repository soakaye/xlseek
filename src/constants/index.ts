/**
 * @fileoverview Frontend centralized constants module (src/constants/index.ts)
 *
 * ## Description
 * Centralizes IPC command names, Tauri event names, UI layout dimensions,
 * timing configurations, default file extensions, and language-independent application constants.
 * Complies with Constitution Principle I (English documentation) and Principle II (external constants).
 */

// ==============================================================================
// 0. Language and General App Constants
// ==============================================================================
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

// ==============================================================================
// 1. Tauri Event Name Constants
// ==============================================================================
export const EVENT_NAMES = {
  SEARCH_MATCH: "search-match",
  SCAN_PROGRESS: "scan-progress",
  OPEN_ABOUT_DIALOG: "open-about-dialog",
} as const;

// ==============================================================================
// 2. Tauri IPC Command Name Constants
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

// Constant reference: Centralizes search history storage keys, limits, and timing.
export const SEARCH_HISTORY_CONSTANTS = {
  STORAGE_KEY: "exlgrep.searchHistory",
  DEFAULT_MAX_ENTRIES: 20,
  MIN_ENTRIES: 0,
  MAX_ENTRIES: 50,
  STEP: 1,
  INPUT_ID: "search-history-limit",
} as const;

// Constant reference: Limits path completion response overhead and debounce timing.
export const PATH_COMPLETION_CONSTANTS = {
  DEBOUNCE_MS: 200,
  MAX_RESULTS: 10,
  KEYBOARD_STEP: 1,
} as const;

// Constant reference: Standardizes keyboard key identifiers used across list navigation.
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
// 3. UI Layout and Virtualization Dimension Constants (Layout Dimensions)
// ==============================================================================
export const LAYOUT_CONSTANTS = {
  /** Search result table row height (px) */
  RESULT_ROW_HEIGHT_PX: 36,
  /** Search result table header height (px) */
  HEADER_HEIGHT_PX: 40,
  /** Preview grid cell default width (px) */
  PREVIEW_CELL_WIDTH_PX: 100,
  /** Preview grid cell default height (px) */
  PREVIEW_CELL_HEIGHT_PX: 24,
  /** Virtual scrolling overscan row count */
  VIRTUAL_OVERSCAN: 10,
  /** Sheet tab horizontal scroll offset (px) */
  SHEET_SCROLL_OFFSET_PX: 100,
  /** Clipboard copy success feedback display duration (ms) */
  COPY_FEEDBACK_DURATION_MS: 2000,
} as const;

// ==============================================================================
// 4. Timing Configuration Constants (Timings)
// ==============================================================================
export const TIMING_CONSTANTS = {
  /** Toast notification auto-dismiss duration (ms) */
  TOAST_DURATION_MS: 3000,
  /** Progress UI update throttle interval (ms) */
  PROGRESS_THROTTLE_MS: 50,
} as const;

// ==============================================================================
// 5. Target File Extension Constants (File Extensions)
// ==============================================================================
export const FILE_EXTENSIONS = {
  XLSX: ".xlsx",
  XLSM: ".xlsm",
  XLSB: ".xlsb",
  XLS: ".xls",
  DEFAULT_LIST: [".xlsx", ".xlsm", ".xlsb", ".xls"],
} as const;

// ==============================================================================
// 6. Default Search Options Constants (Default Search Options)
// ==============================================================================
export const DEFAULT_OPTIONS_STORAGE_KEY = "exlgrep.default_search_options";

export const DEFAULT_SEARCH_OPTIONS = {
  match_case: false,
  use_regex: false,
  include_formula: true,
  include_shape: false,
  include_comment: true,
  include_hidden: false,
  extensions: [".xlsx", ".xlsm", ".xlsb", ".xls"],
} as const;
