//! # CSVエクスポートモジュール (export/csv_export.rs)
//!
//! ## 処理内容
//! 検索一致アイテムの一覧をBOM付きUTF-8のCSVファイルとして書き出す。
//! Excelでの日本語文字化け防止のためBOMを付与し、列ヘッダーおよびデータ行を整形出力する。
//! 憲章原則I（日本語出力）、原則II（定数参照）、原則III（ヘッダコメント）、原則IV（Clippy完全準拠）に準拠。
//!
//! ## 変更履歴
//! - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化、Clippy借用指摘修正、4要素ヘッダコメント付与。

use crate::models::{MatchType, SearchMatch};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;

/// ## 処理内容
/// 検索結果アイテムのスライスをBOM付きUTF-8のCSVファイルに出力する。
///
/// ## 引数
/// - `path`: `&str` - 出力先CSVファイルのファイルパス
/// - `items`: `&[SearchMatch]` - エクスポート対象の検索一致アイテムスライス
/// - `language`: `&str` - `ja` または `en` の出力言語
/// - `catalogs`: `&BTreeMap<String, BTreeMap<String, String>>` - プラグインから取得した翻訳カタログ
///
/// ## 戻り値
/// - `Result<(), String>`: 成功時は `Ok(())`、言語・翻訳・ファイル操作の失敗時は英語エラーコード
///
/// ## エラー / 例外発生条件
/// - 対応外言語、必要な翻訳の欠落、ファイル作成、BOM書き込み、レコード書き込み、またはフラッシュの失敗時に `Err` を返却する。
/// - panicは発生しない。
///
/// ## 変更履歴
/// - v1.1.0 (2026-09-26, Codex): 要求言語のカタログから列見出しと一致種別を取得。
pub fn export_to_csv(
    path: &str,
    items: &[SearchMatch],
    language: &str,
    catalogs: &BTreeMap<String, BTreeMap<String, String>>,
) -> Result<(), String> {
    if language != crate::constants::LANGUAGE_JA && language != crate::constants::LANGUAGE_EN {
        return Err(crate::constants::ERR_INVALID_LANGUAGE.to_string());
    }
    let file = File::create(path).map_err(|e| {
        // 定数参照: crate::constants::ERR_CSV_RECORD_WRITE を使用
        format!("{}: {}", crate::constants::ERR_CSV_RECORD_WRITE, e)
    })?;
    let mut writer = std::io::BufWriter::new(file);

    // Excelで日本語が文字化けしないようにUTF-8 BOMを付与
    writer.write_all(b"\xEF\xBB\xBF").map_err(|e| {
        // 定数参照: crate::constants::ERR_CSV_HEADER_WRITE を使用
        format!("{}: {}", crate::constants::ERR_CSV_HEADER_WRITE, e)
    })?;

    let mut csv_writer = csv::Writer::from_writer(writer);

    // 定数参照: constants::EXPORT_HEADER_KEYS を使用し、要求言語のカタログから見出しを取得する。
    let headers = crate::constants::EXPORT_HEADER_KEYS
        .iter()
        .map(|key| {
            crate::i18n::resolve_catalog_text(catalogs, language, key)
                .ok_or_else(|| crate::constants::ERR_TRANSLATION_MISSING.to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;
    csv_writer.write_record(&headers).map_err(|e| {
        // 定数参照: crate::constants::ERR_CSV_HEADER_WRITE を使用
        format!("{}: {}", crate::constants::ERR_CSV_HEADER_WRITE, e)
    })?;

    // データ行
    for item in items {
        // 定数参照: constants::EXPORT_MATCH_*_KEY を使用。
        let match_key = match item.match_type {
            MatchType::CellValue => crate::constants::EXPORT_MATCH_VALUE_KEY,
            MatchType::Formula => crate::constants::EXPORT_MATCH_FORMULA_KEY,
            MatchType::Comment => crate::constants::EXPORT_MATCH_COMMENT_KEY,
            MatchType::HiddenSheet => crate::constants::EXPORT_MATCH_HIDDEN_SHEET_KEY,
        };
        let match_type_str = crate::i18n::resolve_catalog_text(catalogs, language, match_key)
            .ok_or_else(|| crate::constants::ERR_TRANSLATION_MISSING.to_string())?;

        // Clippy: needless_borrows_for_generic_args 回避のため配列を直接渡す
        csv_writer
            .write_record([
                item.id.to_string(),
                item.file_name.clone(),
                item.full_path.clone(),
                item.sheet_name.clone(),
                item.cell_address.clone(),
                match_type_str,
                item.full_content.clone(),
                item.formula.clone().unwrap_or_default(),
            ])
            .map_err(|e| {
                // 定数参照: crate::constants::ERR_CSV_RECORD_WRITE を使用
                format!("{}: {}", crate::constants::ERR_CSV_RECORD_WRITE, e)
            })?;
    }

    csv_writer.flush().map_err(|e| {
        // 定数参照: crate::constants::ERR_CSV_RECORD_WRITE を使用
        format!("{}: {}", crate::constants::ERR_CSV_RECORD_WRITE, e)
    })?;
    Ok(())
}

#[cfg(test)]
mod localization_tests {
    use std::collections::BTreeMap;

    const TEST_OUTPUT_PREFIX: &str = "exlgrep-i18n-test";

    /// ## 処理内容
    /// CSV の列見出しが指定されたカタログ言語になることを検証する。
    /// ## 引数・戻り値
    /// 引数なし。日英の出力ファイルを読み、2列目の見出しを比較する。
    /// ## エラー
    /// ファイル操作、CSV解析、または見出し不一致時にテストが失敗する。
    /// ## 変更履歴
    /// - v1.2.0 (2026-09-26, Codex): カタログによるCSV見出し選択テストを追加。
    #[test]
    fn uses_requested_catalog_for_headers() {
        let mut english = BTreeMap::from([(
            crate::constants::TRANSLATION_UNAVAILABLE_KEY.to_string(),
            "Some text could not be translated.".to_string(),
        )]);
        for key in crate::constants::EXPORT_HEADER_KEYS {
            english.insert(key.to_string(), key.to_string());
        }
        english.insert(
            "export.header.fileName".to_string(),
            "File name".to_string(),
        );
        let mut japanese = BTreeMap::new();
        for key in crate::constants::EXPORT_HEADER_KEYS {
            japanese.insert(key.to_string(), key.to_string());
        }
        japanese.insert(
            "export.header.fileName".to_string(),
            "ファイル名".to_string(),
        );
        let catalogs = BTreeMap::from([
            (crate::constants::LANGUAGE_EN.to_string(), english),
            (crate::constants::LANGUAGE_JA.to_string(), japanese),
        ]);

        for (language, expected) in [
            (crate::constants::LANGUAGE_EN, "File name"),
            (crate::constants::LANGUAGE_JA, "ファイル名"),
        ] {
            // 定数参照: TEST_OUTPUT_PREFIX を使用
            let path = std::env::temp_dir().join(format!("{TEST_OUTPUT_PREFIX}-{language}.csv"));
            super::export_to_csv(path.to_str().unwrap(), &[], language, &catalogs).unwrap();
            let mut reader = csv::Reader::from_path(&path).unwrap();
            assert_eq!(reader.headers().unwrap().get(1), Some(expected));
            std::fs::remove_file(path).unwrap();
        }
    }

    /// ## 処理内容
    /// 未対応言語の CSV 出力を、ファイル作成前に拒否する。
    /// ## 引数・戻り値
    /// 引数なし。アサーションのみを実行する。
    /// ## エラー
    /// 想定外の出力結果でテストが失敗する。
    /// ## 変更履歴
    /// - v1.1.0 (2026-09-26, AI Agent): 不正な出力言語の検証を追加。
    #[test]
    fn rejects_unsupported_language() {
        assert_eq!(
            super::export_to_csv("", &[], "fr", &BTreeMap::new()),
            Err(crate::constants::ERR_INVALID_LANGUAGE.to_string())
        );
    }
}
