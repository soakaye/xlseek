//! # Ignore Pattern Matcher Module (search/ignore.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Implements `.gitignore`-style wildcard pattern parsing, tokenization, and filesystem path
//! evaluation using `ignore::gitignore::GitignoreBuilder`. Provides early directory subtree
//! pruning and pre-parse file exclusion for the search discovery engine.
//! Conforms to Constitution Principle I (English comments/logs), Principle II (No Hardcoded Constants),
//! and Principle III (Comprehensive Header Comments).

use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::path::Path;

/// Tokenizes a comma-separated ignore pattern string into discrete pattern tokens,
/// respecting double-quoted segments and trimming surrounding whitespace.
///
/// ## Arguments
/// * `input` - Raw comma-delimited ignore pattern string (e.g., `"~$*, *.tmp, \"backup, archive/*\""`).
///
/// ## Returns
/// * `Vec<String>` - Cleaned, non-empty list of individual pattern expressions.
///
/// ## Errors / Panics
/// Does not panic. Malformed or unclosed quotes are gracefully consumed as literal content.
pub fn parse_ignore_patterns(input: &str) -> Vec<String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Vec::new();
    }

    let mut patterns = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = trimmed.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            // Constant reference: crate::constants::IGNORE_PATTERN_QUOTE
            crate::constants::IGNORE_PATTERN_QUOTE => {
                if in_quotes {
                    // Check for escaped quote ("" or \")
                    if chars.peek() == Some(&crate::constants::IGNORE_PATTERN_QUOTE) {
                        chars.next();
                        current.push(crate::constants::IGNORE_PATTERN_QUOTE);
                    } else {
                        in_quotes = false;
                    }
                } else {
                    in_quotes = true;
                }
            }
            '\\' if in_quotes && chars.peek() == Some(&crate::constants::IGNORE_PATTERN_QUOTE) => {
                chars.next();
                current.push(crate::constants::IGNORE_PATTERN_QUOTE);
            }
            // Constant reference: crate::constants::IGNORE_PATTERN_DELIMITER
            crate::constants::IGNORE_PATTERN_DELIMITER if !in_quotes => {
                let token = current.trim();
                if !token.is_empty() {
                    patterns.push(token.to_string());
                }
                current.clear();
            }
            _ => {
                current.push(c);
            }
        }
    }

    let token = current.trim();
    if !token.is_empty() {
        patterns.push(token.to_string());
    }

    patterns
}

/// Evaluates file and directory paths against compiled `.gitignore`-style wildcard expressions.
///
/// ## Description
/// Encapsulates `ignore::gitignore::Gitignore` compiled DFA. If no ignore patterns are provided
/// or all tokens are empty, `inner` is `None` and acts as a zero-overhead passthrough matcher.
#[derive(Debug, Clone)]
pub struct IgnoreMatcher {
    inner: Option<Gitignore>,
}

impl IgnoreMatcher {
    /// Compiles an `IgnoreMatcher` from an anchor root directory and an optional comma-separated pattern string.
    ///
    /// ## Arguments
    /// * `root` - Base path used by gitignore to resolve root-relative anchors (e.g. `/build`).
    /// * `raw_patterns` - Optional comma-delimited ignore patterns string.
    ///
    /// ## Returns
    /// * `Result<Self, String>` - `Ok(IgnoreMatcher)` on success, or `Err(String)` if pattern compilation fails.
    ///
    /// ## Errors / Panics
    /// Does not panic. Returns `Err` with descriptive error if gitignore pattern syntax is invalid.
    pub fn from_patterns(root: &Path, raw_patterns: Option<&str>) -> Result<Self, String> {
        let raw = match raw_patterns {
            Some(s) if !s.trim().is_empty() => s,
            _ => return Ok(Self::passthrough()),
        };

        let patterns = parse_ignore_patterns(raw);
        if patterns.is_empty() {
            return Ok(Self::passthrough());
        }

        let mut builder = GitignoreBuilder::new(root);
        for pattern in patterns {
            builder
                .add_line(None, &pattern)
                .map_err(|e| format!("{}: {}", e, pattern))?;
        }

        let gitignore = builder.build().map_err(|e| e.to_string())?;
        Ok(Self {
            inner: Some(gitignore),
        })
    }

    /// Creates a passthrough `IgnoreMatcher` that never excludes any file or directory.
    ///
    /// ## Arguments
    /// None.
    ///
    /// ## Returns
    /// * `Self` - Matcher instance with `inner: None`.
    ///
    /// ## Errors / Panics
    /// Does not panic or produce errors.
    pub fn passthrough() -> Self {
        Self { inner: None }
    }

    /// Evaluates whether a candidate path should be ignored according to active rules.
    ///
    /// ## Arguments
    /// * `path` - Candidate filesystem path to evaluate.
    /// * `is_dir` - `true` if candidate is a directory; `false` for regular files.
    ///
    /// ## Returns
    /// * `bool` - `true` if the path matches an ignore rule and should be skipped; `false` otherwise.
    ///
    /// ## Errors / Panics
    /// Does not panic.
    pub fn is_ignored(&self, path: &Path, is_dir: bool) -> bool {
        match &self.inner {
            None => false,
            Some(gitignore) => gitignore
                .matched_path_or_any_parents(path, is_dir)
                .is_ignore(),
        }
    }

    /// Returns `true` if active ignore rules are configured; `false` if passthrough.
    ///
    /// ## Arguments
    /// None.
    ///
    /// ## Returns
    /// * `bool` - `true` if inner matcher is populated.
    ///
    /// ## Errors / Panics
    /// Does not panic.
    pub fn is_active(&self) -> bool {
        self.inner.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_parse_ignore_patterns_basic() {
        let input = "~$*, *.tmp, backup/";
        let tokens = parse_ignore_patterns(input);
        assert_eq!(tokens, vec!["~$*", "*.tmp", "backup/"]);
    }

    #[test]
    fn test_parse_ignore_patterns_redundant_commas_and_whitespace() {
        let input = "  ,  ~$* , , *.bak,  ";
        let tokens = parse_ignore_patterns(input);
        assert_eq!(tokens, vec!["~$*", "*.bak"]);
    }

    #[test]
    fn test_parse_ignore_patterns_quoted_commas() {
        let input = r#"~$*, "backup, archive/*", *.tmp"#;
        let tokens = parse_ignore_patterns(input);
        assert_eq!(tokens, vec!["~$*", "backup, archive/*", "*.tmp"]);
    }

    #[test]
    fn test_passthrough_matcher() {
        let matcher = IgnoreMatcher::passthrough();
        assert!(!matcher.is_active());
        assert!(!matcher.is_ignored(Path::new("test.xlsx"), false));
        assert!(!matcher.is_ignored(Path::new("~$test.xlsx"), false));
        assert!(!matcher.is_ignored(Path::new("backup"), true));
    }

    #[test]
    fn test_office_lock_file_exclusion() {
        let root = PathBuf::from("C:\\Work");
        let matcher = IgnoreMatcher::from_patterns(&root, Some("~$*")).unwrap();
        assert!(matcher.is_active());

        // Lock files
        assert!(matcher.is_ignored(&root.join("~$Report.xlsx"), false));
        assert!(matcher.is_ignored(&root.join("subdir").join("~$Budget.xlsm"), false));

        // Regular files & directories
        assert!(!matcher.is_ignored(&root.join("Report.xlsx"), false));
        assert!(!matcher.is_ignored(&root.join("ReportFolder"), true));
    }

    #[test]
    fn test_directory_pruning_and_extension_matching() {
        let root = PathBuf::from("C:\\Work");
        let matcher =
            IgnoreMatcher::from_patterns(&root, Some("node_modules/, *.tmp, !important.tmp"))
                .unwrap();

        // Directory pruning
        assert!(matcher.is_ignored(&root.join("node_modules"), true));
        assert!(matcher.is_ignored(&root.join("node_modules").join("nested"), true));
        assert!(matcher.is_ignored(&root.join("node_modules").join("package.json"), false));

        // Regular directories not ignored
        assert!(!matcher.is_ignored(&root.join("src"), true));

        // File globs & negation
        assert!(matcher.is_ignored(&root.join("scratch.tmp"), false));
        assert!(!matcher.is_ignored(&root.join("important.tmp"), false));
    }
}
