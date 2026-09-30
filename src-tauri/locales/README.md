<!-- Description: Explains editing conventions and scope for the Rust-side CLI translation catalogs. Arguments & Returns: Documents how to add and maintain translation keys using locales/ja.yml and locales/en.yml as inputs. Errors / Exceptions: Invalid YAML syntax and key mismatches between languages are caught during CLI catalog load verification. -->
# Rust CLI 翻訳カタログ

`ja.yml` と `en.yml` はGUIの翻訳とCLIの翻訳を共有します。CLI用のキーを追加・変更するときは両ファイルのキーを揃え、`_version` を除く項目に文字列値を設定してください。CLIはビルド時に両YAMLを実行ファイルへ埋め込むため、配布先にカタログファイルは不要です。

`cargo test` はカタログを読み込み、必須CLIキーと翻訳の欠落を検証します。コメント抽出の現状対応は `.xlsx` / `.xlsm` の従来メモです。スレッドコメント、`.xls` / `.xlsb` のメモは未対応です。
