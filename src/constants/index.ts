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
export const UI_MESSAGES = {
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
  ROOT_ELEMENT_ID: "root",
} as const;

// ==============================================================================
// 7. Aboutダイアログおよびライセンス表示定数 (About Dialog & Package Licenses)
// ==============================================================================
export const ABOUT_DIALOG_CONSTANTS = {
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
} as const;
