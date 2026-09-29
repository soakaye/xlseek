//! ## 処理内容
//! 検索入力を保護し、結果を一時保存して完成ファイルだけ出力先へ公開する。
//! ## 引数・戻り値
//! 出力先、上書き指定、結果一覧、形式、言語、カタログを受けて保存結果を返す。
//! ## エラー
//! 入力衝突、出力競合、エクスポート、公開、ファイルシステム操作はErrで返す。
//! ## 変更履歴
//! - v1.0.0 (2026-09-29, Codex): 安全なCLI結果公開を追加。

use crate::constants;
use exlgrep_core::models::{ExportFormat, SearchMatch};
use same_file::Handle;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// ## 処理内容
/// 実行開始時の出力ファイルidentityと上書き方針を保持する。
/// ## 引数・戻り値
/// 出力先とoverwriteを受け、検査可能なOutputGuardを返す。
/// ## エラー
/// 上書き禁止時の既存出力、symlinkや取得不能metadataはErrとする。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): 出力ガードを追加。
pub struct OutputGuard {
    output: PathBuf,
    initial_identity: Option<Handle>,
    overwrite: bool,
}

impl OutputGuard {
    /// ## 処理内容
    /// 出力先が安全に公開可能な状態か検証して初期identityを記録する。
    /// ## 引数・戻り値
    /// 絶対出力パスとoverwriteを受け、`Result<Self, String>` を返す。
    /// ## エラー
    /// symlink、非通常ファイル、既定拒否対象、metadata取得失敗ではErrを返す。
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-29, Codex): 出力先初期検証を追加。
    pub fn new(output: PathBuf, overwrite: bool) -> Result<Self, String> {
        let initial_identity = match std::fs::symlink_metadata(&output) {
            Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_file() => {
                return Err(format!(
                    "{}: {}",
                    constants::ERR_CLI_OUTPUT_PATH,
                    output.display()
                ));
            }
            Ok(_) if !overwrite => {
                return Err(format!(
                    "{}: {}",
                    constants::ERR_CLI_EXISTS,
                    output.display()
                ));
            }
            Ok(_) => Some(Handle::from_path(&output).map_err(|error| error.to_string())?),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
            Err(error) => return Err(error.to_string()),
        };
        Ok(Self {
            output,
            initial_identity,
            overwrite,
        })
    }

    /// ## 処理内容
    /// 発見した検索入力が出力先と同一ファイルか検査する。
    /// ## 引数・戻り値
    /// 入力`&Path`を受け、同一ならErr、異なるならunitを返す。
    /// ## エラー
    /// hardlinkを含む同一性、またはidentity比較失敗でErrを返す。
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-29, Codex): 入力と出力の同一性検査を追加。
    pub fn check_input(&self, input: &Path) -> Result<(), String> {
        if let Some(output_identity) = &self.initial_identity {
            let input_identity = Handle::from_path(input).map_err(|error| error.to_string())?;
            if output_identity == &input_identity {
                return Err(format!(
                    "{}: {}",
                    constants::ERR_CLI_INPUT_COLLISION,
                    input.display()
                ));
            }
        } else if std::fs::canonicalize(input).ok().as_deref() == Some(self.output.as_path()) {
            return Err(format!(
                "{}: {}",
                constants::ERR_CLI_INPUT_COLLISION,
                input.display()
            ));
        }
        Ok(())
    }

    /// ## 処理内容
    /// 検索結果を同一親内の一時ファイルへ保存し、競合検査後に完成品を公開する。
    /// ## 引数・戻り値
    /// SearchMatch一覧、形式、言語、翻訳カタログを受け、公開パスを返す。
    /// ## エラー
    /// 一時領域作成、書出し、競合検査、hard_link、rename失敗ではErrを返す。
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-29, Codex): 一時保存と原子的公開を追加。
    pub fn publish(
        &self,
        matches: &[SearchMatch],
        format: ExportFormat,
        language: &str,
        catalogs: &BTreeMap<String, BTreeMap<String, String>>,
    ) -> Result<PathBuf, String> {
        let parent = self
            .output
            .parent()
            .ok_or_else(|| constants::ERR_CLI_OUTPUT_PATH.to_string())?;
        let temporary_directory = TemporaryDirectory::create(parent)?;
        let temporary_file = temporary_directory.path.join(constants::CLI_TEMP_FILE_NAME);
        let temporary_file_text = temporary_file.to_string_lossy();
        match format {
            ExportFormat::Csv => exlgrep_core::export::export_to_csv(
                &temporary_file_text,
                matches,
                language,
                catalogs,
            )?,
            ExportFormat::Xlsx => exlgrep_core::export::export_to_xlsx(
                &temporary_file_text,
                matches,
                language,
                catalogs,
            )?,
        }
        self.verify_output_identity()?;
        if self.overwrite {
            std::fs::rename(&temporary_file, &self.output)
                .map_err(|error| format!("{}: {}", constants::ERR_CLI_OUTPUT, error))?;
        } else {
            std::fs::hard_link(&temporary_file, &self.output)
                .map_err(|error| format!("{}: {}", constants::ERR_CLI_OUTPUT, error))?;
        }
        Ok(self.output.clone())
    }

    /// ## 処理内容
    /// 公開直前に出力先identityが開始時から変わっていないことを検証する。
    /// ## 引数・戻り値
    /// OutputGuardを参照し、同一ならunitを返す。
    /// ## エラー
    /// 予期しない作成・置換・削除・metadata失敗時にErrを返す。
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-29, Codex): 公開直前の出力競合検査を追加。
    fn verify_output_identity(&self) -> Result<(), String> {
        match (
            &self.initial_identity,
            std::fs::symlink_metadata(&self.output),
        ) {
            (None, Err(error)) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            (Some(expected), Ok(metadata))
                if metadata.is_file() && !metadata.file_type().is_symlink() =>
            {
                let actual = Handle::from_path(&self.output).map_err(|error| error.to_string())?;
                if expected == &actual {
                    Ok(())
                } else {
                    Err(constants::ERR_CLI_OUTPUT_CHANGED.to_string())
                }
            }
            _ => Err(constants::ERR_CLI_OUTPUT_CHANGED.to_string()),
        }
    }
}

/// ## 処理内容
/// 検索完了後に出力親の中へ排他的な一時ディレクトリを作成し、スコープ終了時に削除する。
/// ## 引数・戻り値
/// 既存親パスを受け、作成したディレクトリを保持するガードを返す。
/// ## エラー
/// 親metadataまたは上限回数内のディレクトリ作成失敗でErrを返す。
/// ## 変更履歴
/// - v1.0.0 (2026-09-29, Codex): 一時ディレクトリ管理を追加。
struct TemporaryDirectory {
    path: PathBuf,
}

impl TemporaryDirectory {
    /// ## 処理内容
    /// 衝突しない一時ディレクトリを出力先の隣に作成する。
    /// ## 引数・戻り値
    /// 出力親パスを受け、RAII削除ガードを返す。
    /// ## エラー
    /// 作成試行上限到達、またはI/O失敗でErrを返す。
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-29, Codex): 排他的な一時領域作成を追加。
    fn create(parent: &Path) -> Result<Self, String> {
        for attempt in 0..constants::CLI_TEMP_CREATE_ATTEMPTS {
            let path = parent.join(format!(
                "{}{}-{}",
                constants::CLI_TEMP_DIRECTORY_PREFIX,
                std::process::id(),
                attempt
            ));
            match std::fs::create_dir(&path) {
                Ok(()) => return Ok(Self { path }),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(error.to_string()),
            }
        }
        Err(constants::ERR_CLI_TEMP_LIMIT.to_string())
    }
}

impl Drop for TemporaryDirectory {
    /// ## 処理内容
    /// 一時領域内のファイルとディレクトリをスコープ終了時に削除する。
    /// ## 引数・戻り値
    /// `&mut self`を受け、戻り値はない。
    /// ## エラー
    /// cleanup失敗はデストラクターから返せないため無視する。
    /// ## 変更履歴
    /// - v1.0.0 (2026-09-29, Codex): RAII一時領域清掃を追加。
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}
