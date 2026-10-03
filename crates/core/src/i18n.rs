//! # Internationalization & Translation Catalogs
//!
//! Copyright (c) 2026 soakaye
//!
//! ## Description
//! Provides language-specific translation resolution and fallback to English catalogs on the Rust side.
//!
//! ## Arguments / Returns
//! Accepts per-locale key maps, requested language code, and translation key, returning translated string or None.
//!
//! ## Errors
//! Returns `None` if catalog or key is missing, without panicking.

use std::collections::BTreeMap;

/// Loads bundled Japanese and English YAML catalogs for CLI translations at compile time.
///
/// ## Arguments / Returns
/// Takes no arguments; returns `Result<BTreeMap<String, BTreeMap<String, String>>, String>`.
///
/// ## Errors
/// Returns `Err` if YAML format, string values, or mandatory translation keys are invalid or missing.
pub fn load_embedded_catalogs() -> Result<BTreeMap<String, BTreeMap<String, String>>, String> {
    let japanese = parse_embedded_catalog(include_str!("../locales/ja.yml"))?;
    let english = parse_embedded_catalog(include_str!("../locales/en.yml"))?;
    for key in crate::constants::CLI_REQUIRED_TRANSLATION_KEYS {
        if !japanese.contains_key(key) || !english.contains_key(key) {
            return Err(format!(
                "{}: {key}",
                crate::constants::ERR_TRANSLATION_MISSING
            ));
        }
    }
    Ok(BTreeMap::from([
        (crate::constants::LANGUAGE_JA.to_string(), japanese),
        (crate::constants::LANGUAGE_EN.to_string(), english),
    ]))
}

/// Converts top-level YAML catalog string entries into a key-value translation map, excluding version metadata.
///
/// ## Arguments / Returns
/// Accepts YAML text `&str` and returns a `BTreeMap` of translation keys and string values.
///
/// ## Errors
/// Returns `Err` on YAML parse failure, non-mapping root, or non-string keys/values.
fn parse_embedded_catalog(source: &str) -> Result<BTreeMap<String, String>, String> {
    let value: serde_yaml::Value =
        serde_yaml::from_str(source).map_err(|error| error.to_string())?;
    let entries = value
        .as_mapping()
        .ok_or_else(|| crate::constants::ERR_CATALOG_INVALID.to_string())?;
    let mut catalog = BTreeMap::new();
    for (key, value) in entries {
        let key = key
            .as_str()
            .ok_or_else(|| crate::constants::ERR_CATALOG_INVALID.to_string())?;
        if key == crate::constants::LOCALE_METADATA_VERSION_KEY {
            continue;
        }
        let value = value
            .as_str()
            .ok_or_else(|| crate::constants::ERR_CATALOG_INVALID.to_string())?;
        catalog.insert(key.to_string(), value.to_string());
    }
    Ok(catalog)
}

/// Resolves translation text in order: requested language key, English catalog key, and finally English unavailable fallback.
///
/// ## Arguments / Returns
/// Accepts double map of locale and key, language code, and translation key, returning cloned string if found.
///
/// ## Errors
/// Returns `None` without panicking if catalogs or keys are missing.
pub fn resolve_catalog_text(
    catalogs: &BTreeMap<String, BTreeMap<String, String>>,
    language: &str,
    key: &str,
) -> Option<String> {
    let english_catalog = catalogs.get(crate::constants::LANGUAGE_EN);
    catalogs
        .get(language)
        .and_then(|catalog| catalog.get(key))
        .or_else(|| english_catalog.and_then(|catalog| catalog.get(key)))
        .or_else(|| {
            english_catalog
                .and_then(|catalog| catalog.get(crate::constants::TRANSLATION_UNAVAILABLE_KEY))
        })
        .cloned()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    /// Verifies that embedded catalogs are loaded without mixing version metadata into the translation dictionary.
    ///
    /// ## Arguments / Returns
    /// None.
    ///
    /// ## Errors
    /// Panics on assertion failure.
    #[test]
    fn loads_embedded_locales_without_version_metadata() {
        let catalogs = super::load_embedded_catalogs().unwrap();
        assert!(catalogs.contains_key(crate::constants::LANGUAGE_JA));
        assert!(catalogs.contains_key(crate::constants::LANGUAGE_EN));
        assert!(!catalogs[crate::constants::LANGUAGE_JA]
            .contains_key(crate::constants::LOCALE_METADATA_VERSION_KEY));
        assert!(catalogs[crate::constants::LANGUAGE_EN]
            .contains_key(crate::constants::CLI_TRANSLATION_HELP_KEY));
    }

    /// Verifies that a key missing in requested locale catalog falls back to English catalog.
    ///
    /// ## Arguments / Returns
    /// None.
    ///
    /// ## Errors
    /// Panics on assertion failure.
    #[test]
    fn missing_requested_locale_key_uses_english_without_changing_locale() {
        let active_locale = crate::constants::LANGUAGE_JA;
        let catalogs = BTreeMap::from([(
            crate::constants::LANGUAGE_EN.to_string(),
            BTreeMap::from([
                ("menu.about".to_string(), "About Excel Seek".to_string()),
                (
                    crate::constants::TRANSLATION_UNAVAILABLE_KEY.to_string(),
                    "Some text could not be translated.".to_string(),
                ),
            ]),
        )]);

        assert_eq!(
            super::resolve_catalog_text(&catalogs, active_locale, "menu.about"),
            Some("About Excel Seek".to_string())
        );
        assert_eq!(active_locale, crate::constants::LANGUAGE_JA);
    }

    /// Verifies that when key is missing in English too, the required fallback translation text is returned.
    ///
    /// ## Arguments / Returns
    /// None.
    ///
    /// ## Errors
    /// Panics on assertion failure.
    #[test]
    fn missing_key_uses_required_english_fallback() {
        let catalogs = BTreeMap::from([(
            crate::constants::LANGUAGE_EN.to_string(),
            BTreeMap::from([(
                crate::constants::TRANSLATION_UNAVAILABLE_KEY.to_string(),
                "Some text could not be translated.".to_string(),
            )]),
        )]);

        assert_eq!(
            super::resolve_catalog_text(&catalogs, crate::constants::LANGUAGE_EN, "menu.missing"),
            Some("Some text could not be translated.".to_string())
        );
    }
}
