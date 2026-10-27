//! # File Pattern Matcher Module (search/pattern.rs)
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Implements file search pattern parsing, extension complement, and glob evaluation
//! using `ignore::gitignore::GitignoreBuilder`. Evaluates candidate files during directory
//! traversal before ignore patterns are evaluated. Conforms to Constitution Principle I
//! (English comments/logs), Principle II (No Hardcoded Constants), and Principle III
//! (Comprehensive Header Comments).
//!
//! ## Arguments / Returns
//! Exposes `FilePatternMatcher`, `parse_file_patterns`, `has_explicit_extension`, and `complement_tokens`.
//!
//! ## Errors
//! Returns descriptive error strings when pattern compilation fails; does not panic.

use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::path::Path;

/// Tokenizes a comma-separated file pattern string into discrete pattern tokens,
/// respecting double-quoted segments and trimming surrounding whitespace.
///
/// ## Arguments
/// * `input` - Raw comma-delimited file pattern string (e.g., `"*budget*, quarterly_*, \"annual, q1*\""`).
///
/// ## Returns
/// * `Vec<String>` - Cleaned, non-empty list of individual pattern expressions.
///
/// ## Errors / Panics
/// Does not panic. Malformed or unclosed quotes are gracefully consumed as literal content.
pub fn parse_file_patterns(input: &str) -> Vec<String> {
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
            // Constant reference: crate::constants::FILE_PATTERN_QUOTE
            crate::constants::FILE_PATTERN_QUOTE => {
                if in_quotes {
                    if chars.peek() == Some(&crate::constants::FILE_PATTERN_QUOTE) {
                        chars.next();
                        current.push(crate::constants::FILE_PATTERN_QUOTE);
                    } else {
                        in_quotes = false;
                    }
                } else {
                    in_quotes = true;
                }
            }
            '\\' if in_quotes && chars.peek() == Some(&crate::constants::FILE_PATTERN_QUOTE) => {
                chars.next();
                current.push(crate::constants::FILE_PATTERN_QUOTE);
            }
            // Constant reference: crate::constants::FILE_PATTERN_DELIMITER
            crate::constants::FILE_PATTERN_DELIMITER if !in_quotes => {
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

/// Determines whether a pattern token explicitly specifies a supported document file extension.
///
/// ## Arguments
/// * `pattern` - Candidate pattern string (e.g. `report.xlsx`, `*.xls`, `budget*`).
///
/// ## Returns
/// * `bool` - `true` if the pattern ends with a recognized supported extension; `false` otherwise.
///
/// ## Errors / Panics
/// Does not panic or produce errors.
pub fn has_explicit_extension(pattern: &str) -> bool {
    let lower = pattern.to_ascii_lowercase();
    // Constant reference: crate::constants::SUPPORTED_ALL_EXTENSIONS
    for ext in crate::constants::SUPPORTED_ALL_EXTENSIONS {
        if lower.ends_with(ext) {
            return true;
        }
    }
    false
}

/// Complements extensionless pattern tokens with active extension options while preserving explicit extensions.
///
/// ## Arguments
/// * `tokens` - Parsed pattern tokens without surrounding whitespace.
/// * `enabled_extensions` - Slices of currently active extension options (e.g., `[".xlsx", ".xlsm"]`).
///
/// ## Returns
/// * `Vec<String>` - Complemented list of pattern strings ready for glob compilation.
///
/// ## Errors / Panics
/// Does not panic or produce errors.
pub fn complement_tokens(tokens: &[String], enabled_extensions: &[String]) -> Vec<String> {
    let mut complemented = Vec::new();

    for token in tokens {
        if has_explicit_extension(token) {
            complemented.push(token.clone());
        } else {
            // Append each enabled extension to the token
            for ext in enabled_extensions {
                let normalized_ext = if ext.starts_with('.') {
                    ext.clone()
                } else {
                    format!(".{}", ext)
                };

                if token.ends_with('.') {
                    let suffix = &normalized_ext[1..];
                    complemented.push(format!("{}{}", token, suffix));
                } else {
                    complemented.push(format!("{}{}", token, normalized_ext));
                }
            }
        }
    }

    complemented
}

/// Evaluates file paths against compiled inclusion file search patterns.
///
/// ## Description
/// Encapsulates `ignore::gitignore::Gitignore` compiled DFA for inclusion rules.
/// If no patterns are provided or all tokens are empty, `inner` is `None` and acts as a passthrough.
#[derive(Debug, Clone)]
pub struct FilePatternMatcher {
    inner: Option<Gitignore>,
    complemented_patterns: Vec<String>,
    has_explicit_extension: bool,
}

impl FilePatternMatcher {
    /// Compiles a `FilePatternMatcher` from root directory, raw patterns, and active extensions.
    ///
    /// ## Arguments
    /// * `root` - Base anchor directory for gitignore matching.
    /// * `raw_patterns` - Optional comma-delimited inclusion pattern string.
    /// * `enabled_extensions` - List of active file extension filters.
    ///
    /// ## Returns
    /// * `Result<Self, String>` - `Ok(FilePatternMatcher)` or `Err(String)` if pattern compilation fails.
    ///
    /// ## Errors / Panics
    /// Returns `Err` with descriptive message if gitignore syntax is invalid; does not panic.
    pub fn from_patterns(
        root: &Path,
        raw_patterns: Option<&str>,
        enabled_extensions: &[String],
    ) -> Result<Self, String> {
        let raw = match raw_patterns {
            Some(s) if !s.trim().is_empty() => s,
            _ => return Ok(Self::passthrough()),
        };

        let tokens = parse_file_patterns(raw);
        if tokens.is_empty() {
            return Ok(Self::passthrough());
        }

        let has_explicit = tokens.iter().any(|t| has_explicit_extension(t));
        let complemented = complement_tokens(&tokens, enabled_extensions);
        if complemented.is_empty() {
            return Ok(Self::passthrough());
        }

        let mut builder = GitignoreBuilder::new(root);
        for pattern in &complemented {
            builder
                .add_line(None, pattern)
                .map_err(|e| format!("{}: {}", e, pattern))?;
        }

        let gitignore = builder.build().map_err(|e| e.to_string())?;
        Ok(Self {
            inner: Some(gitignore),
            complemented_patterns: complemented,
            has_explicit_extension: has_explicit,
        })
    }

    /// Creates a passthrough matcher when no file search patterns are specified.
    ///
    /// ## Arguments
    /// None.
    ///
    /// ## Returns
    /// * `Self` - Passthrough instance with `inner: None`.
    ///
    /// ## Errors / Panics
    /// Does not panic or produce errors.
    pub fn passthrough() -> Self {
        Self {
            inner: None,
            complemented_patterns: Vec::new(),
            has_explicit_extension: false,
        }
    }

    /// Checks whether the matcher operates in passthrough mode (no custom file search patterns).
    ///
    /// ## Arguments
    /// None.
    ///
    /// ## Returns
    /// * `bool` - `true` if passthrough; `false` if active patterns exist.
    ///
    /// ## Errors / Panics
    /// Does not panic.
    pub fn is_passthrough(&self) -> bool {
        self.inner.is_none()
    }

    /// Returns a slice of all complemented pattern strings.
    ///
    /// ## Arguments
    /// None.
    ///
    /// ## Returns
    /// * `&[String]` - Slice of complemented patterns.
    ///
    /// ## Errors / Panics
    /// Does not panic.
    pub fn complemented_patterns(&self) -> &[String] {
        &self.complemented_patterns
    }

    /// Evaluates whether a candidate file satisfies active inclusion criteria.
    ///
    /// ## Arguments
    /// * `path` - Candidate filesystem path to evaluate.
    /// * `enabled_extensions` - Slice of enabled file extensions.
    ///
    /// ## Returns
    /// * `bool` - `true` if candidate is included; `false` otherwise.
    ///
    /// ## Errors / Panics
    /// Does not panic.
    pub fn is_match(&self, path: &Path, enabled_extensions: &[String]) -> bool {
        if let Some(ref gitignore) = self.inner {
            let matched = gitignore.matched(path, false).is_ignore();
            if !matched {
                return false;
            }

            // Verify file has a supported format
            if self.has_explicit_extension {
                let path_str = path.to_string_lossy().to_ascii_lowercase();
                // Constant reference: crate::constants::SUPPORTED_ALL_EXTENSIONS
                crate::constants::SUPPORTED_ALL_EXTENSIONS
                    .iter()
                    .any(|ext| path_str.ends_with(ext))
            } else {
                let path_str = path.to_string_lossy().to_ascii_lowercase();
                enabled_extensions
                    .iter()
                    .any(|ext| path_str.ends_with(&ext.to_ascii_lowercase()))
            }
        } else {
            // Passthrough mode: match active enabled extensions
            let path_str = path.to_string_lossy().to_ascii_lowercase();
            enabled_extensions
                .iter()
                .any(|ext| path_str.ends_with(&ext.to_ascii_lowercase()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// Tests tokenizing basic comma-separated file search patterns.
    ///
    /// ## Arguments / Returns
    /// None.
    ///
    /// ## Errors
    /// Panics on assertion failure.
    #[test]
    fn test_parse_file_patterns_basic() {
        let input = "*budget*, quarterly_*, report.xlsx";
        let tokens = parse_file_patterns(input);
        assert_eq!(tokens, vec!["*budget*", "quarterly_*", "report.xlsx"]);
    }

    /// Tests tokenizing with redundant commas and surrounding whitespace.
    ///
    /// ## Arguments / Returns
    /// None.
    ///
    /// ## Errors
    /// Panics on assertion failure.
    #[test]
    fn test_parse_file_patterns_redundant_commas_and_whitespace() {
        let input = "  ,  *budget* , ,   quarterly_* , ";
        let tokens = parse_file_patterns(input);
        assert_eq!(tokens, vec!["*budget*", "quarterly_*"]);
    }

    /// Tests tokenizing patterns with quoted commas.
    ///
    /// ## Arguments / Returns
    /// None.
    ///
    /// ## Errors
    /// Panics on assertion failure.
    #[test]
    fn test_parse_file_patterns_quoted_commas() {
        let input = r#""2024, Annual Report*", *audit*"#;
        let tokens = parse_file_patterns(input);
        assert_eq!(tokens, vec!["2024, Annual Report*", "*audit*"]);
    }

    /// Tests explicit extension detection across supported document formats.
    ///
    /// ## Arguments / Returns
    /// None.
    ///
    /// ## Errors
    /// Panics on assertion failure.
    #[test]
    fn test_has_explicit_extension() {
        assert!(has_explicit_extension("report.xlsx"));
        assert!(has_explicit_extension("REPORT.XLSX"));
        assert!(has_explicit_extension("*.xls"));
        assert!(has_explicit_extension("summary.xlsm"));
        assert!(has_explicit_extension("notes.docx"));
        assert!(!has_explicit_extension("budget*"));
        assert!(!has_explicit_extension("report.v1.0*"));
        assert!(!has_explicit_extension("release.2024"));
    }

    /// Tests extension complement for tokens lacking extensions.
    ///
    /// ## Arguments / Returns
    /// None.
    ///
    /// ## Errors
    /// Panics on assertion failure.
    #[test]
    fn test_complement_tokens() {
        let tokens = vec![
            "budget*".to_string(),
            "report.xlsx".to_string(),
            "summary.".to_string(),
        ];
        let exts = vec![".xlsx".to_string(), ".xlsm".to_string()];
        let complemented = complement_tokens(&tokens, &exts);

        assert_eq!(
            complemented,
            vec![
                "budget*.xlsx",
                "budget*.xlsm",
                "report.xlsx",
                "summary.xlsx",
                "summary.xlsm",
            ]
        );
    }

    /// Tests FilePatternMatcher matching and filtering.
    ///
    /// ## Arguments / Returns
    /// None.
    ///
    /// ## Errors
    /// Panics on assertion failure.
    #[test]
    fn test_file_pattern_matcher_matching() {
        let root = PathBuf::from("/dummy/root");
        let exts = vec![".xlsx".to_string(), ".xlsm".to_string()];
        let matcher =
            FilePatternMatcher::from_patterns(&root, Some("*budget*, quarterly_*"), &exts)
                .expect("Valid patterns must compile");

        assert!(matcher.is_match(&PathBuf::from("/dummy/root/annual_budget.xlsx"), &exts));
        assert!(matcher.is_match(&PathBuf::from("/dummy/root/quarterly_report.xlsm"), &exts));
        assert!(!matcher.is_match(&PathBuf::from("/dummy/root/inventory.xlsx"), &exts));
        assert!(!matcher.is_match(&PathBuf::from("/dummy/root/annual_budget.csv"), &exts));
    }

    /// Tests passthrough matcher behavior when raw pattern is empty or None.
    ///
    /// ## Arguments / Returns
    /// None.
    ///
    /// ## Errors
    /// Panics on assertion failure.
    #[test]
    fn test_file_pattern_matcher_passthrough() {
        let root = PathBuf::from("/dummy/root");
        let exts = vec![".xlsx".to_string(), ".xlsm".to_string()];
        let matcher = FilePatternMatcher::from_patterns(&root, Some(""), &exts)
            .expect("Empty pattern produces passthrough");

        assert!(matcher.is_passthrough());
        assert!(matcher.is_match(&PathBuf::from("/dummy/root/any_file.xlsx"), &exts));
        assert!(!matcher.is_match(&PathBuf::from("/dummy/root/any_file.pdf"), &exts));
    }

    /// Tests explicit extension override when extension is not in active options.
    ///
    /// ## Arguments / Returns
    /// None.
    ///
    /// ## Errors
    /// Panics on assertion failure.
    #[test]
    fn test_file_pattern_matcher_explicit_extension_override() {
        let root = PathBuf::from("/dummy/root");
        // Only .xlsx is enabled in toggles
        let exts = vec![".xlsx".to_string()];
        let matcher = FilePatternMatcher::from_patterns(&root, Some("*.xls"), &exts)
            .expect("Explicit pattern must compile");

        // .xls is not in exts, but explicit *.xls pattern allows it for supported format
        assert!(matcher.is_match(&PathBuf::from("/dummy/root/legacy_data.xls"), &exts));
        assert!(!matcher.is_match(&PathBuf::from("/dummy/root/legacy_data.xlsx"), &exts));
    }
}
