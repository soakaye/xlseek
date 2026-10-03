//! # Standalone CLI Entrypoint (main.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Entrypoint for the standalone command-line executable (xlseek-cli) without initializing any GUI.
//!
//! ## Arguments / Returns
//! Receives operating system arguments and converts the CLI execution outcome into a process exit code.
//!
//! ## Errors
//! Argument parsing, search, and output errors are reported to stderr and returned as non-zero exit codes.

/// Passes CLI arguments to the shared CLI execution layer and terminates the process with the returned exit status.
///
/// ## Arguments / Returns
/// - Receives OS arguments; does not return upon normal process exit.
///
/// ## Errors
/// The execution layer handles converting I/O and argument errors into appropriate exit codes.
fn main() {
    std::process::exit(xlseek_cli::run(std::env::args_os().skip(1)));
}
