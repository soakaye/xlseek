//! ## 処理内容
//! Rust 側の要求言語別翻訳検索と英語カタログへのフォールバックを提供する。
//! ## 入力・出力
//! ロケール別キー対応表、要求言語、翻訳キーを受け取り、翻訳文または欠落を返す。
//! ## エラー
//! カタログやキーが欠けている場合は `None` とし、panic を起こさない。
//! ## 変更履歴
//! - v1.0.0 (2026-09-26, Codex): 翻訳フォールバックテストを追加。

use std::collections::BTreeMap;

/// ## 処理内容
/// コンパイル時に同梱した日本語・英語YAMLをCLI用翻訳カタログへ読み込む。
/// ## 引数・戻り値
/// 引数なし。`Result<BTreeMap<String, BTreeMap<String, String>>, String>` を返す。
/// ## エラー
/// YAML形式、文字列値、必須翻訳キーが不正・欠落した場合はErrを返す。
/// ## 変更履歴
/// - v1.1.0 (2026-09-29, Codex): GUIなしCLI向け埋込カタログ読込を追加。
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

/// ## 処理内容
/// YAMLカタログのトップレベル文字列項目を翻訳辞書へ変換し、版メタデータを除く。
/// ## 引数・戻り値
/// YAMLテキスト `&str` を受け取り、キーと文言の `BTreeMap` を返す。
/// ## エラー
/// YAML解析失敗、辞書形式でない値、非文字列キー・文言でErrを返す。
/// ## 変更履歴
/// - v1.1.0 (2026-09-29, Codex): 同梱YAML解析を追加。
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

/// ## 処理内容
/// 要求言語のキー、英語の同一キー、必須英語フォールバックの順に翻訳文を探す。
/// ## 引数・戻り値
/// locale と key の二重マップ、言語コード、翻訳キーを受け取り、見つかった翻訳を複製して返す。
/// ## エラー
/// カタログまたは必要なキーが欠けている場合は `None` を返し、panic を起こさない。
/// ## 変更履歴
/// - v1.0.0 (2026-09-26, Codex): Rust 側の英語フォールバックを実装。
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

    /// ## 処理内容
    /// 埋込カタログを読み込み、版メタデータを翻訳辞書へ混入させないことを検証する。
    /// ## 引数・戻り値
    /// 引数なし。ja/enキーとCLI必須文言をアサートする。
    /// ## エラー
    /// カタログ解析失敗または期待キー不一致でテストが失敗する。
    /// ## 変更履歴
    /// - v1.1.0 (2026-09-29, Codex): 同梱カタログの回帰テストを追加。
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

    /// ## 処理内容
    /// 日本語カタログにないキーが英語カタログから返ることを検証する。
    /// ## 引数・戻り値
    /// 引数なし。翻訳結果が英語文言と一致することをアサートする。
    /// ## エラー
    /// 期待値が異なる場合はテストが失敗する。
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-26, Codex): 英語フォールバック検証を追加。
    #[test]
    fn missing_requested_locale_key_uses_english_without_changing_locale() {
        let active_locale = crate::constants::LANGUAGE_JA;
        let catalogs = BTreeMap::from([(
            crate::constants::LANGUAGE_EN.to_string(),
            BTreeMap::from([
                ("menu.about".to_string(), "About Excel Grep".to_string()),
                (
                    crate::constants::TRANSLATION_UNAVAILABLE_KEY.to_string(),
                    "Some text could not be translated.".to_string(),
                ),
            ]),
        )]);

        assert_eq!(
            super::resolve_catalog_text(&catalogs, active_locale, "menu.about"),
            Some("About Excel Grep".to_string())
        );
        assert_eq!(active_locale, crate::constants::LANGUAGE_JA);
    }

    /// ## 処理内容
    /// 英語にも翻訳キーがない場合、必須の英語フォールバック文を返すことを検証する。
    /// ## 引数・戻り値
    /// 引数なし。解決結果が必須フォールバック文と一致することをアサートする。
    /// ## エラー
    /// 期待値が異なる場合はテストが失敗する。
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-26, Codex): 最終フォールバック検証を追加。
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
