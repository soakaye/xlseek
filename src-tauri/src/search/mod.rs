//! # 検索サブモジュール定義 (search/mod.rs)
//!
//! ## 処理内容
//! Excel検索エンジン（engine）、ブックパーサー（parser）、およびセルプレビュー抽出（preview）
//! の各サブモジュールを公開・管理する。
//!
//! ## 変更履歴
//! - v1.0.0 (2026-09-26, AI Agent): 初版策定。憲章原則III準拠ヘッダコメントの付与。
//! - v1.1.0 (2026-09-29, Codex): 共通コメント抽出モジュールを公開。

pub mod comments;
pub mod engine;
pub mod parser;
pub mod path;
pub mod preview;
pub mod shape;
