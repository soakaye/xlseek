//! # Excel Grep Shared Core Library (exlgrep_core)
//!
//! ## Description
//! Provides high-speed parallel search engine for Excel workbooks (.xlsx, .xlsm, .xls, .xlsb),
//! parsing of cell values, comments, and drawing shape text, CSV/Excel export facilities,
//! shared data models, embedded translation catalogs, and common constants.
//! Serves as the foundational crate shared by both the Tauri desktop application and the standalone CLI tool,
//! with zero dependencies on GUI frameworks.
//!
//! ## Arguments / Returns
//! Root module exposing submodules (`constants`, `export`, `i18n`, `models`, `search`).
//!
//! ## Errors
//! Propagates errors via module-specific `Result` types, eliminating unexpected panics.

pub mod constants;
pub mod export;
pub mod i18n;
pub mod models;
pub mod search;
