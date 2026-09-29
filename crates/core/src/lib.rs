//! # Excel Grep 共有コアライブラリ (exlgrep_core)
//!
//! ## 処理内容
//! Excelブック（.xlsx, .xlsm, .xls, .xlsb）の高速並列検索エンジン、セル・コメント・図形内テキストの解析、
//! 検索結果のCSV/Excelエクスポート、共通データモデル、埋め込み翻訳カタログ、および共通定数を提供する。
//! Tauriデスクトップアプリケーションおよび独立CLIツールの双方が共有する最下位基盤クレートであり、
//! GUIフレームワークへの依存を一切持たない。
//!
//! ## 引数・戻り値
//! モジュール群（search, export, models, i18n, constants）を公開するライブラリルート。
//!
//! ## エラー
//! 各モジュール内のResult型を通じてエラー情報を伝播し、予期しないパニックを排除する。
//!
//! ## 変更履歴
//! - v1.0.0 (2026-09-29, Antigravity): 初版策定。src-tauriから独立した共有クレートとして抽出。

pub mod constants;
pub mod export;
pub mod i18n;
pub mod models;
pub mod search;
