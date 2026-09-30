//! # Search Submodule Definition (search/mod.rs)
//!
//! ## Description
//! Exposes and manages Excel search engine (engine), workbook parser (parser),
//! path resolution (path), cell preview extraction (preview), and comment/shape extractors.

pub mod comments;
pub mod discovery;
pub mod engine;
pub mod parser;
pub mod path;
pub mod preview;
pub mod shape;
