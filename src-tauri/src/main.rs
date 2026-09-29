//! # Excel Grep Desktop Application Entry Point (main.rs)
//!
//! ## Description
//! Application startup point for desktop binary. Suppresses console window on Windows
//! and calls core library `xlseek_lib::run()` to start the GUI event loop.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// ## Description
/// Application entry point function.
///
/// ## Arguments
/// None
///
/// ## Returns
/// None
///
/// ## Errors / Exceptions
/// Panics if application initialization fails.
fn main() {
    xlseek_lib::run();
}
