//! # エクスポートサブモジュール定義 (export/mod.rs)
//!
//! ## 処理内容
//! 検索結果のCSVエクスポート（csv_export）およびExcel（.xlsx）エクスポート（xlsx_export）
//! の各機能を公開・管理する。
//!
//! ## 変更履歴
//! - v1.0.0 (2026-09-26, AI Agent): 初版策定。憲章原則III準拠ヘッダコメントの付与。

pub mod csv_export;
pub mod xlsx_export;

pub use csv_export::export_to_csv;
pub use xlsx_export::export_to_xlsx;
