/**
 * @fileoverview フロントエンド定数一元定義モジュール (src/constants/index.ts)
 *
 * ## 処理内容
 * フロントエンド全体で使用されるIPCコマンド名、Tauriイベント名、UIレイアウト寸法、
 * タイミング設定、デフォルト拡張子、および日本語UI文言定数を一元管理する。
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
 * - v1.7.0 (2026-09-26, AI Agent): 並行パイプライン用検出・走査中メッセージ定数およびステータス・エクスポート用定数を追加。
 */

// ==============================================================================
// 0. 言語設定値
// ===============================================================================
export const LANGUAGE_PREFERENCES = { DEFAULT: "default", JA: "ja", EN: "en" } as const;
export const DISPLAY_LANGUAGES = { JA: "ja", EN: "en" } as const;
export const LANGUAGE_STORAGE_KEY = "exlgrep.language";

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

// ==============================================================================
// 6. UI表示用日本語メッセージ定数 (UI Text & Messages)
// ==============================================================================
const JA_UI_MESSAGES = {
  APP_TITLE: "Excel Grep",
  SEARCHING: "検索中...",
  SEARCHING_DIR: "探索中",
  STATUS_IDLE: "検索待機中",
  STATUS_WAITING: "待機中",
  STATUS_SCAN_PREPARING: "スキャン開始準備中...",
  STATUS_DISCOVERING_FILES: "ファイルを検出・走査中...",
  STATUS_DISCOVERING_PREFIX: "検出・走査中",
  STATUS_SCANNING_PREFIX: "スキャン中: ",
  STATUS_COMPLETED_PREFIX: "完了: ",
  FOLDER_SCANNING_PREFIX: "フォルダスキャン中: ",
  FOLDER_SEARCHING_DEFAULT: "対象フォルダを探索しています...",
  SCAN_CANCELLED_MSG: "スキャンが中断されました",
  EXPORT_NO_RESULTS_MSG: "エクスポート対象の結果がありません",
  EXPORT_ERROR_PREFIX: "エクスポート失敗: ",
  EXPORT_SAVED_PREFIX: "ファイルを保存しました: ",
  EXPORT_CSV_FILTER_NAME: "CSVファイル",
  EXPORT_XLSX_FILTER_NAME: "Excelブック",
  EXPORT_DEFAULT_FILENAME_PREFIX: "ExcelGrep_Results_",
  COMPLETED: "検索完了",
  CANCELLED: "検索中断",
  ERROR: "エラー",
  NO_RESULTS: "検索条件に一致する結果は見つかりませんでした",
  SELECT_FOLDER_PROMPT: "検索対象フォルダを選択してください",
  SELECT_FOLDER_DIALOG_TITLE: "検索対象フォルダを選択",
  SELECT_FOLDER_TOOLTIP: "フォルダを選択",
  KEYWORD_PLACEHOLDER: "検索キーワードを入力...",
  KEYWORD_INPUT_PLACEHOLDER: "検索するテキストまたは正規表現を入力... (Enterで検索)",
  CLEAR_TOOLTIP: "クリア",
  FOLDER_DROP_PLACEHOLDER: "ここにフォルダをドロップ...",
  FOLDER_INPUT_PLACEHOLDER: "フォルダを選択またはドラッグ＆ドロップ...",
  FOLDER_DROP_PROMPT: "フォルダをここにドロップ",
  BUTTON_CANCEL: "CANCEL",
  BUTTON_SEARCH: "SEARCH",
  OPTIONS_LABEL: "オプション:",
  OPTION_MATCH_CASE: "大文字/小文字を区別",
  OPTION_USE_REGEX: "正規表現 (Regex)",
  OPTION_INCLUDE_FORMULA: "数式 (Formula)",
  OPTION_INCLUDE_COMMENT: "コメント / メモ",
  OPTION_INCLUDE_HIDDEN: "非表示シート",
  LABEL_TARGET_EXTENSIONS: "対象拡張子:",
  SELECT_EXTENSION_PROMPT: "検索対象の拡張子を1つ以上選択してください",
  EXTENSION_TOGGLE_EXCLUDE_SUFFIX: " を検索対象から除外",
  EXTENSION_TOGGLE_INCLUDE_SUFFIX: " を検索対象に追加",
  PREVIEW_LOADING: "プレビュー読み込み中...",
  PREVIEW_EMPTY: "プレビューデータがありません",
  PREVIEW_GUIDANCE: "周辺セルプレビュー (前後3行・前後2列)",
  PREVIEW_SCROLL_GUIDANCE: "縦横スクロール可能 (固定見出し)",
  PREVIEW_SELECT_ITEM_PROMPT: "項目を選択するとプレビューが表示されます",
  OPEN_IN_APP_DEFAULT: "アプリで開く",
  OPEN_WITH_APP_TOOLTIP: "開くアプリケーションを選択",
  OPEN_WITH_SPECIFIC_APP_PREFIX: " で開く",
  DEFAULT_APP_LABEL: "既定",
  NO_SUPPORTED_APPS: "利用可能なアプリが見つかりません",
  OPEN_WITH_OTHER_APP: "別のプログラムを選択...",
  OPEN_LOCATION_TOOLTIP: "ファイルの保存場所を開く",
  LOCATION_LABEL: "場所:",
  COPY_PATH_TOOLTIP: "パスをコピー",
  EXPORT_SUCCESS: "エクスポートが完了しました",
  EXPORT_FAILED: "エクスポートに失敗しました",
  OPEN_FILE_FAILED: "ファイルを開けませんでした",
  OPEN_FOLDER_FAILED: "フォルダを開けませんでした",
  LAUNCH_APP_FAILED: "アプリケーションを起動できませんでした",
  SHOW_OPEN_WITH_FAILED: "「プログラムから開く」ダイアログを起動できませんでした",
  COPIED_TO_CLIPBOARD: "クリップボードにコピーしました",
  COPIED_FILE_PATH: "ファイルパスをコピーしました",
  COPY_TO_CLIPBOARD_FAILED: "クリップボードへのコピーに失敗しました",
  MATCH_TYPE_CELL_VALUE: "セル値 (Text / Number)",
  MATCH_TYPE_FORMULA: "数式 (Formula)",
  MATCH_TYPE_COMMENT: "コメント / メモ",
  MATCH_TYPE_HIDDEN_SHEET: "非表示シート一致",
  MATCH_DETAIL_TITLE: "一致の詳細情報",
  TYPE_LABEL: "タイプ:",
  ROW_NUMBER_LABEL: "行番号:",
  COL_NUMBER_LABEL: "列番号:",
  HIDDEN_STATUS_LABEL: "非表示状態:",
  STATUS_HIDDEN: "非表示",
  STATUS_VISIBLE: "表示",
  EMPTY_CONTENT: "(空)",
  PREV_SHEET_TOOLTIP: "前のシートへ",
  NEXT_SHEET_TOOLTIP: "次のシートへ",
  SHEET_TABS_BADGE: "Sheet Tabs",
  SETTINGS: "設定",
  SETTINGS_TITLE: "言語設定",
  DISPLAY_LANGUAGE: "表示言語",
  LANGUAGE_DEFAULT: "デフォルト",
  LANGUAGE_JA: "日本語",
  LANGUAGE_EN: "English",
  SAVE_FAILED: "設定を保存できませんでした。次回起動時は保存済み設定を使用します。",
  CURRENT_LANGUAGE: "現在の表示言語",
  SETTINGS_DESCRIPTION: "言語を選択してください。デフォルトはシステム言語を使用します。",
  EXPORT_CSV_BUTTON: "CSV 出力",
  EXPORT_XLSX_BUTTON: "Excel 出力",
  RESULTS_TITLE: "検索結果リスト",
  FILTER_RESULTS_PLACEHOLDER: "結果内を絞り込み...",
  COLUMN_FILE: "ファイル名",
  COLUMN_SHEET: "シート",
  COLUMN_CELL: "セル",
  COLUMN_MATCH_TYPE: "一致種別",
  COLUMN_PREVIEW: "一致内容 (プレビュー)",
  WORKSHEET_TITLE: "検索一致が発生したワークシート",
  LABEL_SHEET: "シート:",
  UNIT_FILES: "ファイル",
  UNIT_MATCHES: "一致",
  RESULT_COUNT_SUFFIX: "件",
  NO_FILTERED_RESULTS: "一致する結果がありません",
  FORMULA_TOOLTIP_PREFIX: "数式:",
  UNIT_ROWS: "行",
  TOAST_SEARCH_KEYWORD: "検索キーワードを入力してください",
  SEARCH_START_ERROR: "検索を開始できませんでした",
  PREVIEW_LOAD_ERROR: "プレビューを読み込めませんでした",
  SEARCH_ERROR: "検索中にエラーが発生しました",
  SELECT_FOLDER_FAILED: "フォルダ選択に失敗しました",
  SUPPORTED_APPS_FAILED: "サポートアプリ一覧を取得できませんでした",
  MENU_UPDATE_FAILED: "アプリメニューの言語を更新できませんでした",
  UNIT_SECONDS_SUFFIX: "秒",
  STATUS_DISCOVERING_DETAIL: "検出・走査中",
  STATUS_SCANNING_DETAIL: "スキャン中",
  STATUS_CANCELLED_DETAIL: "検索中断",
  STATUS_ERROR_DETAIL: "エラーが発生しました",
  ROOT_ELEMENT_ID: "root",
} as const;

/**
 * ## 処理内容
 * 英語 UI 文言を日本語定数と同じキーで管理する。
 * ## 引数・戻り値
 * 引数なし。キーと英語文字列の対応表を返す。
 * ## エラー
 * 欠落キーは取得側でキー名を返し、未処理例外は発生しない。
 * ## 変更履歴
 * - v1.8.0 (2026-09-26, AI Agent): 英語表示とキー単位のフォールバックを追加。
 */
const EN_UI_MESSAGES: Partial<Record<keyof typeof JA_UI_MESSAGES, string>> = {
  APP_TITLE: "Excel Grep", SEARCHING: "Searching...", SEARCHING_DIR: "Discovering",
  STATUS_IDLE: "Ready", STATUS_WAITING: "Waiting", STATUS_SCAN_PREPARING: "Preparing scan...",
  STATUS_DISCOVERING_FILES: "Discovering and scanning files...", STATUS_DISCOVERING_PREFIX: "Discovering",
  STATUS_SCANNING_PREFIX: "Scanning: ", STATUS_COMPLETED_PREFIX: "Completed: ",
  FOLDER_SCANNING_PREFIX: "Scanning folder: ", FOLDER_SEARCHING_DEFAULT: "Discovering target folder...",
  SCAN_CANCELLED_MSG: "Scan cancelled", EXPORT_NO_RESULTS_MSG: "There are no results to export",
  EXPORT_ERROR_PREFIX: "Export failed: ", EXPORT_SAVED_PREFIX: "Saved file: ",
  EXPORT_CSV_FILTER_NAME: "CSV file", EXPORT_XLSX_FILTER_NAME: "Excel workbook",
  EXPORT_DEFAULT_FILENAME_PREFIX: "ExcelGrep_Results_", COMPLETED: "Search completed",
  CANCELLED: "Search cancelled", ERROR: "Error", NO_RESULTS: "No results matched the search criteria",
  SELECT_FOLDER_PROMPT: "Select a folder to search", SELECT_FOLDER_DIALOG_TITLE: "Select search folder",
  SELECT_FOLDER_TOOLTIP: "Select folder", KEYWORD_PLACEHOLDER: "Enter a search keyword...",
  KEYWORD_INPUT_PLACEHOLDER: "Enter text or a regular expression... (Enter to search)",
  CLEAR_TOOLTIP: "Clear", FOLDER_DROP_PLACEHOLDER: "Drop a folder here...",
  FOLDER_INPUT_PLACEHOLDER: "Select or drop a folder...", FOLDER_DROP_PROMPT: "Drop folder here",
  BUTTON_CANCEL: "CANCEL", BUTTON_SEARCH: "SEARCH", OPTIONS_LABEL: "Options:",
  OPTION_MATCH_CASE: "Match case", OPTION_USE_REGEX: "Regular expression (Regex)",
  OPTION_INCLUDE_FORMULA: "Formula", OPTION_INCLUDE_COMMENT: "Comments / Notes",
  OPTION_INCLUDE_HIDDEN: "Hidden sheets", LABEL_TARGET_EXTENSIONS: "File types:",
  SELECT_EXTENSION_PROMPT: "Select one or more file types to search",
  EXTENSION_TOGGLE_EXCLUDE_SUFFIX: " excluded from search", EXTENSION_TOGGLE_INCLUDE_SUFFIX: " included in search",
  PREVIEW_LOADING: "Loading preview...", PREVIEW_EMPTY: "No preview data",
  PREVIEW_GUIDANCE: "Nearby cells (3 rows and 2 columns around)",
  PREVIEW_SCROLL_GUIDANCE: "Scroll vertically and horizontally (fixed headers)",
  PREVIEW_SELECT_ITEM_PROMPT: "Select a result to show its preview", OPEN_IN_APP_DEFAULT: "Open in app",
  OPEN_WITH_APP_TOOLTIP: "Choose an application", OPEN_WITH_SPECIFIC_APP_PREFIX: " - Open with",
  DEFAULT_APP_LABEL: "Default", NO_SUPPORTED_APPS: "No supported applications found",
  OPEN_WITH_OTHER_APP: "Choose another application...", OPEN_LOCATION_TOOLTIP: "Open file location",
  LOCATION_LABEL: "Location:", COPY_PATH_TOOLTIP: "Copy path", EXPORT_SUCCESS: "Export completed",
  EXPORT_FAILED: "Export failed", OPEN_FILE_FAILED: "Could not open file", OPEN_FOLDER_FAILED: "Could not open folder",
  LAUNCH_APP_FAILED: "Could not launch application", SHOW_OPEN_WITH_FAILED: "Could not open the application chooser",
  COPIED_TO_CLIPBOARD: "Copied to clipboard", COPIED_FILE_PATH: "Copied file path",
  COPY_TO_CLIPBOARD_FAILED: "Could not copy to clipboard", MATCH_TYPE_CELL_VALUE: "Cell value (Text / Number)",
  MATCH_TYPE_FORMULA: "Formula", MATCH_TYPE_COMMENT: "Comment / Note", MATCH_TYPE_HIDDEN_SHEET: "Hidden sheet match",
  MATCH_DETAIL_TITLE: "Match details", TYPE_LABEL: "Type:", ROW_NUMBER_LABEL: "Row:", COL_NUMBER_LABEL: "Column:",
  HIDDEN_STATUS_LABEL: "Visibility:", STATUS_HIDDEN: "Hidden", STATUS_VISIBLE: "Visible", EMPTY_CONTENT: "(empty)",
  PREV_SHEET_TOOLTIP: "Previous sheet", NEXT_SHEET_TOOLTIP: "Next sheet", SHEET_TABS_BADGE: "Sheet Tabs",
  SETTINGS: "Settings", SETTINGS_TITLE: "Language Settings", DISPLAY_LANGUAGE: "Display language",
  LANGUAGE_DEFAULT: "Default", LANGUAGE_JA: "日本語", LANGUAGE_EN: "English",
  SAVE_FAILED: "Could not save settings. The saved preference will be used next time.",
  CURRENT_LANGUAGE: "Current display language",
  SETTINGS_DESCRIPTION: "Choose a language. Default follows the system language.",
  ROOT_ELEMENT_ID: "root",
  EXPORT_CSV_BUTTON: "Export CSV",
  EXPORT_XLSX_BUTTON: "Export Excel",
  RESULTS_TITLE: "Search results",
  FILTER_RESULTS_PLACEHOLDER: "Filter results...",
  COLUMN_FILE: "File name",
  COLUMN_SHEET: "Sheet",
  COLUMN_CELL: "Cell",
  COLUMN_MATCH_TYPE: "Match type",
  COLUMN_PREVIEW: "Matched content (preview)",
  WORKSHEET_TITLE: "Worksheet containing the match",
  LABEL_SHEET: "Sheet:",
  UNIT_FILES: "files",
  UNIT_MATCHES: "matches",
  RESULT_COUNT_SUFFIX: "matches",
  NO_FILTERED_RESULTS: "No matching results",
  FORMULA_TOOLTIP_PREFIX: "Formula:",
  UNIT_ROWS: "rows",
  TOAST_SEARCH_KEYWORD: "Enter a search keyword",
  SEARCH_START_ERROR: "Could not start search",
  PREVIEW_LOAD_ERROR: "Could not load preview",
  SEARCH_ERROR: "An error occurred during search",
  SELECT_FOLDER_FAILED: "Could not select folder",
  SUPPORTED_APPS_FAILED: "Could not load supported applications",
  MENU_UPDATE_FAILED: "Could not update application menu language",
  UNIT_SECONDS_SUFFIX: "s",
  STATUS_DISCOVERING_DETAIL: "Discovering",
  STATUS_SCANNING_DETAIL: "Scanning",
  STATUS_CANCELLED_DETAIL: "Search cancelled",
  STATUS_ERROR_DETAIL: "An error occurred",
};

type DisplayLanguage = (typeof DISPLAY_LANGUAGES)[keyof typeof DISPLAY_LANGUAGES];
let activeLanguage: DisplayLanguage = DISPLAY_LANGUAGES.JA;
/**
 * ## 処理内容
 * 文言 Proxy が参照するアプリ全体の表示言語を更新する。
 * ## 引数・戻り値
 * 対応済み表示言語コードを受け取り、戻り値なし。
 * ## エラー
 * 型で許可した言語のみを受けるため例外は発生しない。
 * ## 変更履歴
 * - v1.8.0 (2026-09-26, AI Agent): 実行中の文言切り替えに対応。
 */
export const setActiveLanguage = (language: DisplayLanguage) => { activeLanguage = language; };

/**
 * ## 処理内容
 * 指定言語の辞書から文言を取得し、対象言語に未登録なら英語へフォールバックする。
 * ## 引数・戻り値
 * 言語、キー、対象言語辞書、英語辞書を受け取り、表示文字列を返す。
 * ## エラー
 * 辞書にキーがない場合も例外を送出せず、キー文字列を返す。
 * ## 変更履歴
 * - v1.8.0 (2026-09-26, AI Agent): キー単位の英語フォールバックを追加。
 */
export const resolveTranslation = (
  language: DisplayLanguage,
  key: string,
  targetMessages: Record<string, string | undefined>,
  englishMessages: Record<string, string | undefined>,
): string => language === DISPLAY_LANGUAGES.JA
  ? targetMessages[key] ?? englishMessages[key] ?? key
  : englishMessages[key] ?? key;

/**
 * ## 処理内容
 * UI 文言辞書から指定言語の文字列を検索する。
 * ## 引数・戻り値
 * 言語コードとキーを受け取り、翻訳済み文字列を返す。
 * ## エラー
 * キー欠落時は安全なフォールバック値を返す。
 * ## 変更履歴
 * - v1.8.0 (2026-09-26, AI Agent): UI 文言取得関数を追加。
 */
export const translate = (language: DisplayLanguage, key: string): string => resolveTranslation(
  language,
  key,
  JA_UI_MESSAGES,
  EN_UI_MESSAGES,
);

/**
 * ## 処理内容
 * 表示中の通知を新しい言語に変換し、既知文言以外の差し込み値を保持する。
 * ## 引数・戻り値
 * 元メッセージ、元言語、新言語を受け取り、再翻訳した通知文字列を返す。
 * ## エラー
 * キーを特定できない場合は元メッセージを返す。
 * ## 変更履歴
 * - v1.8.0 (2026-09-26, AI Agent): 言語切替時の通知再翻訳を追加。
 */
export const retranslateMessage = (
  message: string,
  sourceLanguage: DisplayLanguage,
  targetLanguage: DisplayLanguage,
): string => {
  if (sourceLanguage === targetLanguage) return message;
  const sourceUi = sourceLanguage === DISPLAY_LANGUAGES.JA ? JA_UI_MESSAGES : EN_UI_MESSAGES;
  for (const key of Object.keys(sourceUi) as Array<keyof typeof JA_UI_MESSAGES>) {
    const sourceText = sourceUi[key];
    if (sourceText && message.includes(sourceText)) {
      return message.replace(sourceText, translate(targetLanguage, key));
    }
  }
  const sourceAbout = sourceLanguage === DISPLAY_LANGUAGES.JA
    ? JA_ABOUT_DIALOG_CONSTANTS
    : EN_ABOUT_DIALOG_CONSTANTS;
  for (const key of Object.keys(sourceAbout) as Array<keyof typeof JA_ABOUT_DIALOG_CONSTANTS>) {
    const sourceText = sourceAbout[key];
    if (sourceText && message.includes(sourceText)) {
      const targetText = targetLanguage === DISPLAY_LANGUAGES.JA
        ? JA_ABOUT_DIALOG_CONSTANTS[key]
        : EN_ABOUT_DIALOG_CONSTANTS[key] ?? JA_ABOUT_DIALOG_CONSTANTS[key];
      return message.replace(sourceText, targetText);
    }
  }
  return message;
};
export const UI_MESSAGES = new Proxy(JA_UI_MESSAGES, {
  get: (_target, key: string | symbol) => typeof key === "string" ? translate(activeLanguage, key) : undefined,
}) as typeof JA_UI_MESSAGES;

// ==============================================================================
// 7. Aboutダイアログおよびライセンス表示定数 (About Dialog & Package Licenses)
// ==============================================================================
const JA_ABOUT_DIALOG_CONSTANTS = {
  TITLE: "Excel Grep について",
  APP_NAME: "Excel Grep",
  APP_VERSION: "0.1.0",
  APP_DESCRIPTION: "高速・セキュアなExcel専用ファイル内検索デスクトップアプリケーション",
  COPYRIGHT: "Copyright © 2026 soakaye",
  APP_LICENSE_LABEL: "配布ライセンス: MIT License",

  TAB_ABOUT: "アプリ情報",
  TAB_LICENSES: "オープンソースライセンス",

  SEARCH_PLACEHOLDER: "パッケージ名・ライセンスで検索...",
  SEARCH_CLEAR_TOOLTIP: "検索キーワードをクリア",
  NO_PACKAGES_FOUND: "一致するパッケージが見つかりません",
  PACKAGE_COUNT_SUFFIX: " 件のパッケージ",

  BUTTON_COPY_LICENSE: "ライセンス本文をコピー",
  BUTTON_COPIED: "コピー完了",
  BUTTON_CLOSE: "閉じる",
  BUTTON_ABOUT_TOOLTIP: "Excel Grep について",

  AUTHOR_LABEL: "著作者 / 著作権表記:",
  LICENSE_TYPE_LABEL: "ライセンス種別:",
  REPOSITORY_LABEL: "リポジトリ / 公式サイト:",
  SOURCE_RUST_LABEL: "Rust (バックエンド)",
  SOURCE_NPM_LABEL: "npm (フロントエンド)",
  LICENSE_TEXT_LABEL: "ライセンス条項全文:",
  SELECT_PACKAGE_PROMPT: "左側の一覧からパッケージを選択するとライセンス詳細が表示されます",

  TOAST_COPIED_PREFIX: "",
  TOAST_COPIED_SUFFIX: " のライセンス本文をクリップボードにコピーしました",
  TOAST_COPY_FAILED: "クリップボードへのコピーに失敗しました",
  COPYRIGHT_LABEL: "著作権:",
  LICENSE_LABEL: "ライセンス:",
  THIRD_PARTY_LABEL: "サードパーティパッケージ:",
  PACKAGE_LIST_LINK: "オープンソースライセンス一覧を確認する",
  PACKAGE_COUNT_UNIT: "件",
  FILTER_ACTIVE: "フィルタ適用中",
} as const;

const EN_ABOUT_DIALOG_CONSTANTS: Partial<Record<keyof typeof JA_ABOUT_DIALOG_CONSTANTS, string>> = {
  TITLE: "About Excel Grep", APP_NAME: "Excel Grep", APP_DESCRIPTION: "Fast and secure desktop search for Excel files",
  COPYRIGHT: "Copyright © 2026 soakaye", APP_LICENSE_LABEL: "Distribution license: MIT License",
  TAB_ABOUT: "About", TAB_LICENSES: "Open source licenses", SEARCH_PLACEHOLDER: "Search package or license...",
  SEARCH_CLEAR_TOOLTIP: "Clear search", NO_PACKAGES_FOUND: "No matching packages found", PACKAGE_COUNT_SUFFIX: " packages",
  BUTTON_COPY_LICENSE: "Copy license text", BUTTON_COPIED: "Copied", BUTTON_CLOSE: "Close",
  BUTTON_ABOUT_TOOLTIP: "About Excel Grep", AUTHOR_LABEL: "Author / Copyright:", LICENSE_TYPE_LABEL: "License type:",
  REPOSITORY_LABEL: "Repository / Website:", SOURCE_RUST_LABEL: "Rust (backend)", SOURCE_NPM_LABEL: "npm (frontend)",
  LICENSE_TEXT_LABEL: "Full license text:", SELECT_PACKAGE_PROMPT: "Select a package from the list to view its license",
  TOAST_COPIED_SUFFIX: " license text copied to clipboard", TOAST_COPY_FAILED: "Could not copy license text",
  COPYRIGHT_LABEL: "Copyright:", THIRD_PARTY_LABEL: "Third-party packages:",
  LICENSE_LABEL: "License:",
  PACKAGE_LIST_LINK: "View open-source licenses", PACKAGE_COUNT_UNIT: "packages", FILTER_ACTIVE: "Filter active",
};

export const ABOUT_DIALOG_CONSTANTS = new Proxy(JA_ABOUT_DIALOG_CONSTANTS, {
  get: (_target, key: string | symbol) => {
    if (typeof key !== "string") return undefined;
    return activeLanguage === "ja"
      ? JA_ABOUT_DIALOG_CONSTANTS[key as keyof typeof JA_ABOUT_DIALOG_CONSTANTS]
      : EN_ABOUT_DIALOG_CONSTANTS[key as keyof typeof EN_ABOUT_DIALOG_CONSTANTS]
        ?? JA_ABOUT_DIALOG_CONSTANTS[key as keyof typeof JA_ABOUT_DIALOG_CONSTANTS];
  },
}) as typeof JA_ABOUT_DIALOG_CONSTANTS;
