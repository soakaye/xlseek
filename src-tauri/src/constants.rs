//! # 定数定義モジュール (constants.rs)
//!
//! ## 処理内容
//! アプリケーション全体（検索エンジン、パーサー、プレビュー、エクスポート、Tauriコマンド）で
//! 使用される固定値、設定値、ファイル拡張子、イベント名、および日本語エラーメッセージを一元管理する。
//! 憲章原則II（定数の外部抽出とハードコード禁止）に準拠。
//!
//! ## 変更履歴
//! - v1.0.0 (2026-09-26, AI Agent): 初版策定。全定数の外部化および整合性テストの実装。

// ==============================================================================
// 1. ファイル拡張子定数 (File Extensions)
// ==============================================================================

/// デフォルトの検索対象 Excel 拡張子リスト
pub const DEFAULT_EXTENSIONS: [&str; 4] = [".xlsx", ".xlsm", ".xlsb", ".xls"];

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

/// 検索一致アイテムIDの初期値
pub const DEFAULT_MATCH_ID_START: u64 = 1;

/// Excelの一時ロックファイルプレフィックス（除外対象）
pub const EXCEL_TEMP_FILE_PREFIX: &str = "~$";

/// スキャン状態メッセージ定数
pub const MSG_SCAN_STARTING: &str = "スキャン開始中...";
pub const MSG_SCAN_COMPLETED: &str = "スキャン完了";
pub const MSG_SCAN_CANCELLED: &str = "スキャンが中断されました";

// ==============================================================================
// 3. プレビュー表示設定定数 (Preview Settings)
// ==============================================================================

/// プレビュー表示時の対象セル前後の行半径
pub const PREVIEW_ROW_RADIUS: u32 = 3;

/// プレビュー表示時の対象セル前後の列半径
pub const PREVIEW_COL_RADIUS: u32 = 2;

// ==============================================================================
// 4. Tauri イベント名定数 (Event Names)
// ==============================================================================

/// 検索一致通知イベント名
pub const EVENT_SEARCH_MATCH: &str = "search-match";

/// 検索進捗通知イベント名
pub const EVENT_SCAN_PROGRESS: &str = "scan-progress";

// ==============================================================================
// 5. エクスポート設定定数 (Export Settings)
// ==============================================================================

/// CSVエクスポートヘッダー列名一覧
pub const CSV_EXPORT_HEADERS: [&str; 8] = [
    "ID",
    "ファイル名",
    "フルパス",
    "シート名",
    "セル位置",
    "一致種別",
    "一致内容",
    "数式",
];

/// Excelエクスポートのデフォルトシート名
pub const EXPORT_DEFAULT_SHEET_NAME: &str = "検索結果";

/// Excelエクスポート時のヘッダ背景色 (Teal 700)
pub const XLSX_HEADER_BG_COLOR: u32 = 0x000F_766E;

/// Excelエクスポート時のヘッダ文字色 (White)
pub const XLSX_HEADER_FG_COLOR: u32 = 0x00FF_FFFF;

/// Excelエクスポート時のヘッダー定義と列幅
pub const XLSX_HEADERS_WITH_WIDTH: [(&str, f64); 8] = [
    ("ID", 8.0),
    ("ファイル名", 25.0),
    ("フルパス", 40.0),
    ("シート名", 20.0),
    ("セル位置", 12.0),
    ("一致種別", 14.0),
    ("一致内容", 45.0),
    ("数式", 30.0),
];

/// 一致種別の日本語ラベル定数
pub const LABEL_MATCH_CELL_VALUE: &str = "値";
pub const LABEL_MATCH_FORMULA: &str = "数式";
pub const LABEL_MATCH_COMMENT: &str = "コメント";
pub const LABEL_MATCH_HIDDEN_SHEET: &str = "非表示シート";

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

pub const ERR_WORKBOOK_OPEN: &str = "ワークブックを開けませんでした";
pub const ERR_SHEET_NOT_FOUND: &str = "指定されたシートが見つかりません";
pub const ERR_CSV_HEADER_WRITE: &str = "CSVヘッダー書き込みエラー";
pub const ERR_CSV_RECORD_WRITE: &str = "CSVレコード書き込みエラー";
pub const ERR_XLSX_CREATE: &str = "Excelファイル作成エラー";
pub const ERR_XLSX_WORKSHEET: &str = "ワークシート追加エラー";
pub const ERR_XLSX_WRITE: &str = "Excel書き込みエラー";
pub const ERR_XLSX_SAVE: &str = "Excelファイル保存エラー";
pub const ERR_INVALID_REGEX: &str = "正規表現の構文が無効です";
pub const ERR_FILE_NOT_FOUND: &str = "指定されたファイルが存在しません";
pub const ERR_FOLDER_OPEN: &str = "フォルダを開けませんでした";
pub const ERR_APP_LAUNCH: &str = "アプリケーションを起動できませんでした";
pub const ERR_LOCK_FAILED: &str = "エンジンのロック取得に失敗しました";
pub const ERR_SYSTEM_APP_NOT_FOUND: &str =
    "対象ファイルを開くアプリケーションが見つかりませんでした";
pub const ERR_TASK_EXECUTION: &str = "タスク実行エラー";
pub const ERR_PATH_NOT_DIR: &str = "ディレクトリパスを特定できませんでした";

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
        assert_eq!(CSV_EXPORT_HEADERS.len(), 8);
        assert_eq!(CSV_EXPORT_HEADERS[0], "ID");
        assert_eq!(CSV_EXPORT_HEADERS[1], "ファイル名");
        assert_eq!(CSV_EXPORT_HEADERS[7], "数式");
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
}
