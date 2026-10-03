//! # Tauri IPC Commands Module (commands/mod.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Exposes Tauri commands (search, preview, export, system) and shared application state (AppState)
//! communicating with the frontend.

pub mod export_cmd;
pub mod preview_cmd;
pub mod search_cmd;
pub mod system_cmd;

pub use export_cmd::*;
pub use preview_cmd::*;
pub use search_cmd::*;
pub use system_cmd::*;
