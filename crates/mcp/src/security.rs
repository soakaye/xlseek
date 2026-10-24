//! # Security & Path Sandboxing Module (crates/mcp/src/security.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Implements filesystem boundary validation and path whitelisting for MCP tool calls.
//! When allowed directories are configured, verifies that requested target files
//! or directories reside strictly within permitted boundaries.
//! Conforms to Constitution Principle I (English text), Principle II (No Hardcoded Constants),
//! Principle III (Comprehensive Header Comments), and Principle V (Robust Error Handling).

use std::path::{Path, PathBuf};

/// Security context maintaining allowed directory boundaries.
///
/// ## Description
/// Encapsulates the whitelist of permitted root directories.
/// When the whitelist is empty, unrestricted local filesystem access (bounded by OS user permissions) is granted.
#[derive(Debug, Clone, Default)]
pub struct McpSecurityContext {
    /// Canonicalized paths of directories permitted for tool inspection.
    allowed_directories: Vec<PathBuf>,
}

impl McpSecurityContext {
    /// Creates a new security context with designated allowed directory paths.
    ///
    /// ## Arguments
    /// - `allowed_dirs`: `Vec<PathBuf>` - List of candidate directory roots.
    ///
    /// ## Returns
    /// An initialized `McpSecurityContext` with normalized, canonical paths.
    ///
    /// ## Errors
    /// Invalid or non-existent directories in the initial list are canonicalized where possible
    /// or retained in standard absolute form without causing a panic.
    pub fn new(allowed_dirs: Vec<PathBuf>) -> Self {
        let canonical_dirs = allowed_dirs
            .into_iter()
            .map(|dir| {
                let abs_dir = if dir.is_absolute() {
                    dir
                } else {
                    std::env::current_dir()
                        .map(|cwd| cwd.join(&dir))
                        .unwrap_or(dir)
                };
                xlseek_core::search::path::normalize_canonical_path(&abs_dir)
            })
            .collect();

        Self {
            allowed_directories: canonical_dirs,
        }
    }

    /// Initializes a security context by inspecting the `XLSEEK_ALLOWED_DIRS` environment variable.
    ///
    /// ## Arguments
    /// None.
    ///
    /// ## Returns
    /// Initialized `McpSecurityContext`.
    ///
    /// ## Errors
    /// If the environment variable is unset or empty, returns an unrestricted context.
    pub fn from_env() -> Self {
        // Constant reference: crate::constants::ENV_ALLOWED_DIRS
        match std::env::var(crate::constants::ENV_ALLOWED_DIRS) {
            Ok(val) if !val.trim().is_empty() => {
                let paths = val
                    .split(',')
                    .map(|segment| segment.trim())
                    .filter(|segment| !segment.is_empty())
                    .map(PathBuf::from)
                    .collect();
                Self::new(paths)
            }
            _ => Self::default(),
        }
    }

    /// Checks if a whitelist boundary is actively enforced.
    ///
    /// ## Arguments
    /// None.
    ///
    /// ## Returns
    /// `true` if restricted to designated directories; `false` if unrestricted.
    ///
    /// ## Errors
    /// Does not panic.
    pub fn is_restricted(&self) -> bool {
        !self.allowed_directories.is_empty()
    }

    /// Returns a slice of active allowed directories.
    ///
    /// ## Arguments
    /// None.
    ///
    /// ## Returns
    /// Reference to slice of permitted `PathBuf` entries.
    ///
    /// ## Errors
    /// Does not panic.
    pub fn allowed_directories(&self) -> &[PathBuf] {
        &self.allowed_directories
    }

    /// Validates that a requested path is permitted under active security constraints.
    ///
    /// ## Arguments
    /// - `target`: `P` - Path requested by client tool invocation.
    ///
    /// ## Returns
    /// - `Ok(PathBuf)`: Validated canonical or absolute path.
    /// - `Err(String)`: Human-readable error message explaining access refusal.
    ///
    /// ## Errors
    /// - Returns `Err` if the path falls outside permitted root directories when whitelist mode is active.
    /// - Returns `Err` if path resolution fails.
    /// - Does not panic.
    pub fn validate_path<P: AsRef<Path>>(&self, target: P) -> Result<PathBuf, String> {
        let path_ref = target.as_ref();

        let abs_target = if path_ref.is_absolute() {
            path_ref.to_path_buf()
        } else {
            std::env::current_dir()
                .map(|cwd| cwd.join(path_ref))
                .unwrap_or_else(|_| path_ref.to_path_buf())
        };

        let resolved_path = xlseek_core::search::path::normalize_canonical_path(&abs_target);

        if !self.is_restricted() {
            return Ok(resolved_path);
        }

        let is_allowed = self
            .allowed_directories
            .iter()
            .any(|allowed| resolved_path.starts_with(allowed) || resolved_path == *allowed);

        if is_allowed {
            Ok(resolved_path)
        } else {
            // Constant reference: crate::constants::ERR_ACCESS_DENIED_PREFIX
            let allowed_list_str = self
                .allowed_directories
                .iter()
                .map(|p| p.display().to_string())
                .collect::<Vec<_>>()
                .join(", ");
            Err(format!(
                "{}: '{}' (Permitted: [{}])",
                crate::constants::ERR_ACCESS_DENIED_PREFIX,
                resolved_path.display(),
                allowed_list_str
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that an empty security context permits arbitrary valid paths.
    #[test]
    fn test_unrestricted_mode_permits_path() {
        let ctx = McpSecurityContext::default();
        assert!(!ctx.is_restricted());

        let cwd = std::env::current_dir().expect("cwd exists");
        let validated = ctx.validate_path(&cwd);
        assert!(validated.is_ok());
    }

    /// Tests that whitelist mode rejects paths outside permitted directory roots.
    #[test]
    fn test_restricted_mode_rejects_foreign_path() {
        let temp_dir = std::env::temp_dir();
        let ctx = McpSecurityContext::new(vec![temp_dir.clone()]);
        assert!(ctx.is_restricted());

        // Target inside whitelist
        let canonical_base = &ctx.allowed_directories()[0];
        let valid_target = canonical_base.join("subfolder");
        assert!(ctx.validate_path(&valid_target).is_ok());

        // Target outside whitelist
        let foreign_target = if cfg!(windows) {
            PathBuf::from("Z:\\NonExistentForeignDirectory")
        } else {
            PathBuf::from("/etc/forbidden_directory")
        };
        let err = ctx.validate_path(&foreign_target);
        assert!(err.is_err());
        assert!(err
            .unwrap_err()
            .contains(crate::constants::ERR_ACCESS_DENIED_PREFIX));
    }
}
