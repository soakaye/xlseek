//! Copyright (c) 2026 soakaye
//!
//! # Tauri Build Script (build.rs)
//!
//! ## Description
//! Build-time code generator script for the Tauri desktop application.
//!
//! ## Arguments & Returns
//! None. Invoked automatically by Cargo during build.
//!
//! ## Errors / Exceptions
//! Panics if code generation or manifest inspection fails.

fn main() {
    tauri_build::build()
}
