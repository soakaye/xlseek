//! # Export Submodule Definition (export/mod.rs)
//!
//! ## Description
//! Exposes and manages CSV export (csv_export) and Excel (.xlsx) export (xlsx_export)
//! functionality for search results.

pub mod csv_export;
pub mod xlsx_export;

pub use csv_export::{export_to_csv, write_csv_to_writer};
pub use xlsx_export::export_to_xlsx;
