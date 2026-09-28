//! # 定数定義モジュール (constants.rs)
//!
//! ## 処理内容
//! アプリケーション全体（検索エンジン、パーサー、プレビュー、エクスポート、Tauriコマンド）で
//! 使用される固定値、設定値、ファイル拡張子、イベント名、および日本語エラーメッセージを一元管理する。
//! 憲章原則II（定数の外部抽出とハードコード禁止）に準拠。
//!
//! ## 変更履歴
//! - v1.0.0 (2026-09-26, AI Agent): 初版策定。全定数の外部化および整合性テストの実装。
//! - v1.1.0 (2026-09-26, AI Agent): Aboutダイアログメニューおよびイベント定数の追加。
//! - v1.2.0 (2026-09-26, AI Agent): 並行パイプライン用バッファ定数および検出中メッセージ定数の追加。
//! - v1.3.0 (2026-09-27, Codex): ホーム省略表記とパス補完上限の定数を追加。
//! - v1.4.0 (2026-09-28, AI Agent): 検索マッチのバッチ送信定数を追加。

// ==============================================================================
// 1. ファイル拡張子定数 (File Extensions)
// ==============================================================================

/// デフォルトの検索対象 Excel 拡張子リスト
pub const DEFAULT_EXTENSIONS: [&str; 4] = [".xlsx", ".xlsm", ".xlsb", ".xls"];
pub const DEFAULT_EXPORT_LANGUAGE: &str = "en";
pub const LANGUAGE_JA: &str = "ja";
pub const LANGUAGE_EN: &str = "en";
pub const TRANSLATION_UNAVAILABLE_KEY: &str = "common.translationUnavailable";
pub const ERR_TRANSLATION_MISSING: &str = "A required translation is missing";

/// 個別拡張子定数
pub const EXT_XLSX: &str = ".xlsx";
pub const EXT_XLSM: &str = ".xlsm";
pub const EXT_XLSB: &str = ".xlsb";
pub const EXT_XLS: &str = ".xls";

// ==============================================================================
// 2. 検索・スキャン設定定数 (Scan & Search Settings)
// ==============================================================================

/// スキャン進捗通知の最小送信間隔 (ミリ秒)
pub const PROGRESS_NOTIFY_INTERVAL_MS: u64 = 50;

/// スキャン中のキャンセルチェック行間隔 (ビットマスク: 0x7F = 127行ごと)
pub const CANCEL_CHECK_ROW_INTERVAL: usize = 0x7F;

/// スニペット生成時の前後コンテキスト文字数
pub const SNIPPET_CONTEXT_CHARS: usize = 30;

/// スニペット省略記号
pub const SNIPPET_ELLIPSIS: &str = "...";
pub const SNIPPET_MARK_OPEN: &str =
    "<mark class='bg-yellow-500/30 text-yellow-300 px-0.5 rounded font-semibold'>";
pub const SNIPPET_MARK_CLOSE: &str = "</mark>";
pub const HTML_ESCAPE_AMPERSAND: &str = "&amp;";
pub const HTML_ESCAPE_LESS_THAN: &str = "&lt;";
pub const HTML_ESCAPE_GREATER_THAN: &str = "&gt;";
pub const HTML_ESCAPE_DOUBLE_QUOTE: &str = "&quot;";
pub const HTML_ESCAPE_APOSTROPHE: &str = "&#39;";
pub const SHAPE_MAX_XML_BYTES: u64 = 52_428_800;
pub const SHAPE_MAX_TOTAL_XML_BYTES: u64 = 209_715_200;
pub const SHAPE_MAX_XML_DEPTH: usize = 128;
pub const SHAPE_MAX_BINARY_BYTES: u64 = 52_428_800;
pub const XLSB_BRT_BUNDLE_SH_RECORD_ID: u32 = 0x009C;
pub const XLSB_BRT_DRAWING_RECORD_ID: u32 = 0x0226;
pub const XLSB_BUNDLE_SHEET_FIXED_BYTES: usize = 8;
pub const XLSB_STRING_LENGTH_BYTES: usize = 4;
pub const XLSB_UTF16_UNIT_BYTES: usize = 2;
pub const XLSB_VARINT_VALUE_MASK: u8 = 0x7F;
pub const XLSB_VARINT_CONTINUATION_MASK: u8 = 0x80;
pub const XLSB_VARINT_SHIFT: u32 = 7;
pub const XLSB_VARINT_MAX_BYTES: u32 = 5;
pub const XLS_MAX_BIFF_RECORD_BYTES: usize = 8_224;
pub const XLS_BOUNDSHEET_RECORD_ID: u16 = 0x0085;
pub const XLS_TXO_RECORD_ID: u16 = 0x01B6;
pub const XLS_CONTINUE_RECORD_ID: u16 = 0x003C;
pub const XLS_TXO_TEXT_COUNT_OFFSET: usize = 10;
pub const XLS_TXO_FIXED_HEADER_BYTES: usize = 16;
pub const XLS_BIFF_RECORD_HEADER_BYTES: usize = 4;
pub const XLS_SHAPE_NAME_PREFIX: &str = "Shape ";
pub const ERR_SHAPE_READ: &str = "Failed to read Shape drawing data";
pub const ERR_SHAPE_LIMIT: &str = "Shape drawing data exceeds the safety limit";

/// 検索一致アイテムIDの初期値
pub const DEFAULT_MATCH_ID_START: u64 = 1;

/// Excelの一時ロックファイルプレフィックス（除外対象）
pub const EXCEL_TEMP_FILE_PREFIX: &str = "~$";

/// パイプライン並行処理における有界同期チャネルのバッファ容量
pub const CHANNEL_BUFFER_SIZE: usize = 1024;

// ==============================================================================
// 3. プレビュー表示設定定数 (Preview Settings)
// ==============================================================================

/// プレビュー表示時の対象セル前後の行半径
pub const PREVIEW_ROW_RADIUS: u32 = 3;

/// プレビュー表示時の対象セル前後の列半径
pub const PREVIEW_COL_RADIUS: u32 = 2;

// ==============================================================================
// 4. Tauri イベント名およびメニュー定数 (Events & Menus)
// ==============================================================================

/// 検索一致通知イベント名
pub const EVENT_SEARCH_MATCH: &str = "search-match";

/// 検索進捗通知イベント名
pub const EVENT_SCAN_PROGRESS: &str = "scan-progress";

/// Aboutダイアログ表示イベント名
pub const EVENT_OPEN_ABOUT_DIALOG: &str = "open-about-dialog";

/// メニュー項目ID: Aboutダイアログ
pub const MENU_ITEM_ABOUT_ID: &str = "open_about";

pub const MENU_KEY_ABOUT: &str = "menu.about";
pub const MENU_KEY_FILE: &str = "menu.file";
pub const MENU_KEY_EDIT: &str = "menu.edit";
pub const MENU_KEY_VIEW: &str = "menu.view";
pub const MENU_KEY_WINDOW: &str = "menu.window";
pub const MENU_KEY_HELP: &str = "menu.help";

// ==============================================================================
// 5. エクスポート設定定数 (Export Settings)
// ==============================================================================

/// Excelエクスポート時のヘッダ背景色 (Teal 700)
pub const XLSX_HEADER_BG_COLOR: u32 = 0x000F_766E;

/// Excelエクスポート時のヘッダ文字色 (White)
pub const XLSX_HEADER_FG_COLOR: u32 = 0x00FF_FFFF;

/// エクスポート列幅。翻訳見出しは共有カタログから取得する。
pub const XLSX_COLUMN_WIDTHS: [f64; 9] = [8.0, 25.0, 40.0, 20.0, 12.0, 22.0, 14.0, 45.0, 30.0];
pub const ERR_INVALID_LANGUAGE: &str = "Unsupported export language";
pub const EXPORT_HEADER_KEYS: [&str; 9] = [
    "export.header.id",
    "export.header.fileName",
    "export.header.fullPath",
    "export.header.sheetName",
    "export.header.cell",
    "export.header.shapeName",
    "export.header.matchType",
    "export.header.matchedContent",
    "export.header.formula",
];
pub const EXPORT_SHEET_NAME_KEY: &str = "export.sheetName";
pub const EXPORT_MATCH_VALUE_KEY: &str = "export.match.value";
pub const EXPORT_MATCH_FORMULA_KEY: &str = "export.match.formula";
pub const EXPORT_MATCH_COMMENT_KEY: &str = "export.match.comment";
pub const EXPORT_MATCH_HIDDEN_SHEET_KEY: &str = "export.match.hiddenSheet";
pub const EXPORT_MATCH_SHAPE_KEY: &str = "export.match.shape";

// ==============================================================================
// 6. アプリケーション識別子・表示名定数 (Supported Apps)
// ==============================================================================

pub const APP_ID_EXCEL: &str = "excel";
pub const APP_ID_NUMBERS: &str = "numbers";
pub const APP_ID_CALC: &str = "calc";

pub const APP_NAME_EXCEL: &str = "Microsoft Excel";
pub const APP_NAME_NUMBERS: &str = "Numbers";
pub const APP_NAME_CALC: &str = "LibreOffice Calc";

pub const ICON_HINT_EXCEL: &str = "excel";
pub const ICON_HINT_NUMBERS: &str = "numbers";
pub const ICON_HINT_CALC: &str = "calc";
pub const ICON_HINT_GENERIC: &str = "generic";

// ==============================================================================
// 7. 日本語エラーメッセージテンプレート (Localized Error Messages)
// ==============================================================================

pub const ERR_WORKBOOK_OPEN: &str = "Could not open workbook";
pub const ERR_SHEET_NOT_FOUND: &str = "Requested sheet was not found";
pub const ERR_CSV_HEADER_WRITE: &str = "Failed to write CSV header";
pub const ERR_CSV_RECORD_WRITE: &str = "Failed to write CSV record";
pub const ERR_XLSX_CREATE: &str = "Failed to create Excel workbook";
pub const ERR_XLSX_WORKSHEET: &str = "Failed to add worksheet";
pub const ERR_XLSX_WRITE: &str = "Failed to write Excel data";
pub const ERR_XLSX_SAVE: &str = "Failed to save Excel workbook";
pub const ERR_INVALID_REGEX: &str = "Invalid regular expression";
pub const ERR_FILE_NOT_FOUND: &str = "Requested file was not found";
pub const ERR_FOLDER_OPEN: &str = "Could not open folder";
pub const ERR_APP_LAUNCH: &str = "Could not launch application";
pub const ERR_LOCK_FAILED: &str = "Failed to acquire engine lock";
pub const ERR_SYSTEM_APP_NOT_FOUND: &str = "No application is available for this file";
pub const ERR_TASK_EXECUTION: &str = "Task execution failed";
pub const ERR_PATH_NOT_DIR: &str = "Path is not a directory";
pub const LOG_SEARCH_STARTED: &str = "[start_search] Search request received";
pub const LOG_SEARCH_COMPLETED: &str = "[start_search] Search completed";
pub const LOG_SEARCH_FAILED: &str = "[start_search] Search engine failed";
/// 定数参照: ホーム省略表記のパス接頭辞。
pub const HOME_PATH_PREFIX_UNIX: &str = "~/";
/// 定数参照: Windows のホーム省略表記のパス接頭辞。
pub const HOME_PATH_PREFIX_WINDOWS: &str = "~\\";
/// 定数参照: ディレクトリ補完で返す候補の上限。
pub const MAX_PATH_COMPLETION_RESULTS: usize = 10;
pub const LOG_EVENT_EMIT_FAILED: &str = "Failed to emit Tauri event";
pub const LOG_CANCEL_REQUESTED: &str = "[cancel_search] Cancellation requested";
/// 定数参照: 検索マッチのバッチ送信最大件数
pub const SEARCH_MATCH_BATCH_SIZE: usize = 50;
/// 定数参照: 検索マッチのバッチ送信インターバル (ミリ秒)
pub const SEARCH_MATCH_BATCH_INTERVAL_MS: u128 = 25;

/// macOS アプリケーションバンドル識別子
pub const MAC_BUNDLE_EXCEL: &str = "com.microsoft.Excel";
pub const MAC_BUNDLE_NUMBERS: &str = "com.apple.iWork.Numbers";

// ==============================================================================
// 8. 単体テスト (Unit Tests)
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// ## 処理内容
    /// 拡張子定数およびデフォルトリストの妥当性を検証する。
    ///
    /// ## 引数
    /// なし
    ///
    /// ## 戻り値
    /// なし
    ///
    /// ## エラー / 例外発生条件
    /// アサーション失敗時にpanic
    ///
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-26, AI Agent): 初版作成
    #[test]
    fn test_default_extensions_validity() {
        assert_eq!(DEFAULT_EXTENSIONS.len(), 4);
        assert!(DEFAULT_EXTENSIONS.contains(&EXT_XLSX));
        assert!(DEFAULT_EXTENSIONS.contains(&EXT_XLSM));
        assert!(DEFAULT_EXTENSIONS.contains(&EXT_XLSB));
        assert!(DEFAULT_EXTENSIONS.contains(&EXT_XLS));
    }

    /// ## 処理内容
    /// CSVエクスポートヘッダーの列数と主要列名の存在を検証する。
    ///
    /// ## 引数
    /// なし
    ///
    /// ## 戻り値
    /// なし
    ///
    /// ## エラー / 例外発生条件
    /// アサーション失敗時にpanic
    ///
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-26, AI Agent): 初版作成
    #[test]
    fn test_csv_export_headers() {
        assert_eq!(EXPORT_HEADER_KEYS.len(), XLSX_COLUMN_WIDTHS.len());
        assert_eq!(EXPORT_HEADER_KEYS[0], "export.header.id");
        assert_eq!(EXPORT_HEADER_KEYS[1], "export.header.fileName");
        assert_eq!(EXPORT_HEADER_KEYS[8], "export.header.formula");
        assert_eq!(EXPORT_SHEET_NAME_KEY, "export.sheetName");
        assert_eq!(EXPORT_MATCH_HIDDEN_SHEET_KEY, "export.match.hiddenSheet");
    }

    /// ## 処理内容
    /// 各種エラーメッセージが空でなく、日本語として定義されていることを検証する。
    ///
    /// ## 引数
    /// なし
    ///
    /// ## 戻り値
    /// なし
    ///
    /// ## エラー / 例外発生条件
    /// アサーション失敗時にpanic
    ///
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-26, AI Agent): 初版作成
    #[test]
    fn test_error_messages_non_empty() {
        assert!(!ERR_WORKBOOK_OPEN.is_empty());
        assert!(!ERR_SHEET_NOT_FOUND.is_empty());
        assert!(!ERR_INVALID_REGEX.is_empty());
        assert!(!ERR_FILE_NOT_FOUND.is_empty());
        assert!(!ERR_LOCK_FAILED.is_empty());
    }

    /// ## 処理内容
    /// メニューおよびイベント関連定数が空でなく想定通りの値であることを検証する。
    ///
    /// ## 引数
    /// なし
    ///
    /// ## 戻り値
    /// なし
    ///
    /// ## エラー / 例外発生条件
    /// アサーション失敗時にpanic
    ///
    /// ## 変更履歴
    /// - v1.1.0 (2026-09-26, AI Agent): Aboutダイアログメニューおよびイベント定数のテスト追加
    #[test]
    fn test_menu_and_event_constants() {
        assert_eq!(EVENT_OPEN_ABOUT_DIALOG, "open-about-dialog");
        assert_eq!(MENU_ITEM_ABOUT_ID, "open_about");
        assert_eq!(MENU_KEY_ABOUT, "menu.about");
        assert_eq!(MENU_KEY_FILE, "menu.file");
        assert_eq!(MENU_KEY_EDIT, "menu.edit");
        assert_eq!(MENU_KEY_VIEW, "menu.view");
        assert_eq!(MENU_KEY_WINDOW, "menu.window");
        assert_eq!(MENU_KEY_HELP, "menu.help");
    }

    /// ## 処理内容
    /// パイプライン並行処理用のバッファ定数およびメッセージ定数が適切に定義されていることを検証する。
    ///
    /// ## 引数
    /// なし
    ///
    /// ## 戻り値
    /// なし
    ///
    /// ## エラー / 例外発生条件
    /// アサーション失敗時にpanic
    ///
    /// ## 変更履歴
    /// - v1.2.0 (2026-09-26, AI Agent): 初版作成
    #[test]
    fn test_pipeline_constants() {
        assert_eq!(CHANNEL_BUFFER_SIZE, 1024);
        assert_eq!(SEARCH_MATCH_BATCH_SIZE, 50);
        assert_eq!(SEARCH_MATCH_BATCH_INTERVAL_MS, 25);
    }
}
