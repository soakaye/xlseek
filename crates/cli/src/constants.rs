//! # CLI Constants Module (constants.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Centralizes constant and configuration values for command line argument parsing,
//! option specifications, exit statuses, temporary file naming, and output formats
//! for the standalone CLI tool (xlseek-cli).
//! Re-exports shared core constants (`xlseek_core::constants`) while defining CLI-specific constants.
//! Conforms to Constitution Principle II (No Hardcoded Constants).
//!
//! ## Arguments / Returns
//! Exposes public CLI constant values.
//!
//! ## Errors
//! As a constant definition module, this does not produce runtime errors.

pub use xlseek_core::constants::*;
