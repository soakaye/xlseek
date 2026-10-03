<!-- 処理内容: CLIの構成と既存実装との差分を調査し、採用する技術方針を記録する。入力・出力: 承認済み機能仕様と既存コードを入力とし、実装時の設計判断と検証基準を出力する。エラー: 未解決の判断や実装済みとの誤認を避け、制約を明記する。変更履歴: v1.0.0 2026-09-29 Codex 初版作成。v1.0.1 2026-09-29 Codex 整合性分析の指摘を反映。 -->
# 調査結果: コマンドライン検索・結果保存

## 1. GUIを起動しない実行構成

- **決定**: 同じRustパッケージに `src/bin/exlgrep-cli.rs` を追加し、公開済み `exlgrep_lib` の検索・出力モジュールを直接呼ぶ。`Cargo.toml` に `default-run = "exlgrep"` を設定する。
- **理由**: `lib.rs` の `run()` を呼ばなければTauriのイベントループ・ウィンドウ・プラグインは初期化されない。CLIにはWindowsのコンソール抑制属性を付けない。
- **検討した代案**: 新しいコアクレートへの全移設、GUIバイナリの引数分岐、全依存のfeature分離。いずれも現要件には不要。
- **根拠**: `src-tauri/src/{lib.rs,main.rs}`、`src-tauri/Cargo.toml`。ビルド時は既存Tauri依存と `dist/` の準備が必要。実行時のGUI非起動とビルド依存の除去は別の条件である。

## 2. 引数解析と既定値

- **決定**: `std::env::args_os` と小さな引数パーサーを使用する。値を取るオプションは `--name VALUE` と `--name=VALUE`、真偽値は `true|false` とする。
- **理由**: サブコマンドや複雑な文法がなく、既存依存にCLI専用パーサーはない。新しい引数解析ライブラリは不要。非Unicodeの引数はpanicではなく入力エラーにする。
- **検討した代案**: clapの追加、設定ファイルやGUIの保存済み設定への依存。今回は引数と仕様の固定既定値だけで再現できる実行を優先する。
- **決定**: 言語は `--language ja|en`、既定は日本語。検索語・対象・出力形式・出力先は必須。その他の値とエラー契約は [CLI契約](contracts/cli.md) に一本化する。

## 3. 共通検索エンジンの不足と改修

- **決定**: ディレクトリ限定の `execute_search` の内部を、単一ファイルと再帰検索を扱う詳細実行処理にする。既存公開関数は互換ラッパーとして残し、既存GUIのコールバックと結果型を維持する。
- **理由**: `parse_and_search_file` は既に単一ファイルを処理できる。CLI側で検索処理を複製する必要はない。
- **決定**: 詳細実行処理で探索、ブック、シート、数式、Shape、コメントの問題を集計する。入力ファイル検出時のコールバックを追加し、CLIの入出力同一性検査に使う。検索ディレクトリを事前に全走査し直す方式は採らない。
- **理由**: 現在の `WalkDir` の `filter_map`、parserの `if let Ok`、Shapeの `unwrap_or_default`、engineのparse失敗・panic処理は問題を握り潰す。部分失敗の終了状態を実現するには共有処理で通知が必要。
- **決定**: `SearchQuery.include_value` を既定trueで追加する。既存GUIのJSONがフィールドを省略しても値検索を維持する。Shapeの拡張子比較を大小文字非依存に揃える。
- **決定**: 値・数式のセル座標にはrangeの開始位置を加算する。既存 `sample_report.xlsx` はA9開始であり、単なるGUIとの一致では正しいセル番地を証明できない。
- **検討した代案**: CLIだけのparser、画面の進捗イベントからの失敗推測。検索結果の差異とエラーの見落としを残すため不採用。

## 4. コメント・メモ検索

- **確認結果**: ローカルのcalamine 0.36.1にはセルコメント取得APIがない。`include_comment` と `MatchType::Comment` は存在するが、共通parserに抽出処理がない。FR-005/006を満たすため共有コアで実装する。
- **決定**: 形式別の小さな抽出処理を追加し、既存 `shape/` のZIP、relationships、CFB、レコード境界、文字列読み取りヘルパーを必要な範囲だけ共有する。

| 形式 | 抽出する情報・結合方法 |
|---|---|
| xlsx / xlsm | シートrelationshipsから従来のcomments XMLとthreadedComments XMLを解決。従来メモはセル参照と本文、現行コメントはid・parentIdから返信のセルを解決し本文を取得 |
| xlsb | workbook.binのシート対応とrelationshipsからコメントパーツを解決。BrtBeginCommentのセル座標とBrtCommentTextのRichStrを結合。現行コメントXMLはOOXMLの処理を共有 |
| xls | Workbook/Bookストリームのシート対応を取得し、NoteSh.idObj、Obj/FtCmoのID、TxO/Continueを結合 |

- **決定**: 現行コメントと互換用メモを二重計上しない。XLSのコメント由来TxOはShapeの一致から除外する。作者・日時・装飾は検索結果項目に追加しない。
- **決定**: コメント関係がない場合は正常な「コメントなし」。参照があるのにパーツが欠落・破損している場合は問題として集計する。外部relationshipsは読み込まない。
- **根拠**: [NoteSh](https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-xls/3a610bb3-9d35-435f-92ef-cdbc42974404)、[Obj](https://learn.microsoft.com/ja-jp/openspecs/office_file_formats/ms-xls/dd34df60-8250-40a9-83a3-911476a31ea7)、[BrtBeginComment](https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-xlsb/3ba13168-8341-42d8-bb64-6143c5ac4631)、[BrtCommentText](https://learn.microsoft.com/en-au/openspecs/office_file_formats/MS-XLSB/edf80558-ed1c-431c-ac2f-59742445cbce)、[RichStr](https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-xlsb/cbb5d08d-ea55-4184-aff0-2ea8967f918e)、[ThreadedComments Schema](https://learn.microsoft.com/en-us/openspecs/office_standards/ms-xlsx/adb84732-9fc8-48b6-bddc-6b0bcdaad940)。レコードID・オフセット・上限は公式仕様から確認して定数化する。
- **検討した代案**: 未実装の設定を受け付けるだけにする、CLIだけ別のExcel解析ライブラリを追加する。いずれも設定の実効性または結果の一致を満たさない。

## 5. 翻訳と出力の共有

- **決定**: 既存 `export_to_csv` / `export_to_xlsx` と `resolve_catalog_text` を再利用する。YAMLカタログを `include_str!` で同梱し、既にCargo.lockに含まれる `serde_yaml` を直接依存として宣言する。
- **理由**: export関数自体はTauri非依存だが、プラグインのカタログローダーは非公開で、公開コンストラクターにはAppHandleが必要。数値の `_version` は検証用メタデータとして除外し、翻訳キーを既存の二重BTreeMapへ変換する。
- **検討した代案**: カタログのJSON移行、Nodeによるビルド時変換、独自YAML解析。GUIと既存翻訳ソースの変更、追加のビルド環境、独自パーサーを避ける。
- **決定**: CLIの文言キーと固定値を `constants.rs`、日英文言を既存 `locales/*.yml` に追加する。GUI保存済み言語設定は参照しない。

## 6. 保存先と入力データの保護

- **決定**: 正規化済みパスと、Cargo.lockにある `same-file` を直接依存として使うファイル同一性検査を組み合わせる。入力と出力が別名ハードリンクでも保存を拒否する。出力先自身のシンボリックリンクは拒否する。
- **決定**: 検索開始時に既存出力のidentityを保持し、各入力の検出時に比較する。公開直前の検査で出力が第三者によって作成・置換されていたことを確認した場合も拒否する。検索対象ディレクトリ内の既存出力xlsxが実際に入力対象なら、`--overwrite` でも拒否する。
- **決定**: 検索完了後、出力親ディレクトリに排他的に作成した一時ディレクトリ内へ既存export関数で保存する。ファイルを閉じた後、既定保存は `std::fs::hard_link`、明示上書きは `std::fs::rename` で完成ファイルを公開する。既存ファイルを先に削除しない。
- **理由**: 既存export関数は最終パスへ直接書くと既存内容を破壊し得る。hard_linkによる公開は保存中に出力先が出現しても上書きしない。renameによる置換は同じファイルシステム内で行う。
- **制約**: 入力とパス構成は実行中に他プロセスが変更しない前提とする。明示上書きでは保存先と親ディレクトリにも同じ前提を適用する。同一性検査とrenameは原子的な比較・置換ではないため、検査後の第三者による変更を拒否する保証はない。標準ライブラリによる方式を維持し、同時変更を排除するロック機構は追加しない。既定のhard_link公開は出力が競合しても上書きしない。
- **制約**: hard_linkを提供しない保存先では既定保存をエラーとして通知し、危険なcopyによる代替はしない。実装コメントにこの上限と、必要になった際のOS別no-replace公開処理への移行を記録する。
- **根拠**: [Rust hard_link](https://doc.rust-lang.org/stable/std/fs/fn.hard_link.html)、[Rust rename](https://doc.rust-lang.org/std/fs/fn.rename.html)、[Rust OpenOptions](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html)。

## 7. 終了状態、規模、配布

- **決定**: 終了コードは全件成功 `0`、一部失敗 `1`、全体失敗 `2`。入力不備・保存失敗は2。処理を完走したが全対象ブックを開けなかった場合も2とし、0件一致の正常検索と区別する。この場合は問題を明示したうえで列見出しだけの結果を保存する。
- **決定**: 空フォルダー・正常な0件一致は結果を保存して0。フォルダーの一部に探索エラーがあった場合は、取得済みの結果を保存して1。ルートが検索不能な場合は2。
- **決定**: 既存rayonと容量制限付き探索キューを維持する。結果は既存export APIのため一度だけVecへ収集する。追加の結果ストリーミング層やキャッシュは導入しない。
- **制約**: 結果件数に比例するメモリと既存Excelライブラリの出力上限は残る。上限による保存失敗を通知し、切り捨てた結果を成功としない。固定秒数の新しい性能保証は設定せず、同じ条件で検索・保存時間と結果件数を記録する。
- **決定**: WindowsとmacOSの既存対応環境でCLIバイナリを別成果物としてビルド・配布する。GUIインストーラーへの統合やPATH自動登録は追加しない。

## 調査の完了条件

技術上の未決定事項は解消済み。これは設計判断であり、機能が実装済み・検証済みであることを意味しない。Phase 1ではこの判断をデータモデル・CLI契約・検証ガイドに反映する。
