<!-- 処理内容: CLI要求、検索結果、失敗報告、保存状態のデータモデルを定義する。入力・出力: 機能仕様と調査結果を入力とし、実装・検証で共通利用する設計情報を出力する。エラー: 不正入力、部分失敗、保存失敗の扱いを明記する。変更履歴: v1.0.0 2026-09-29 Codex 初版作成。v1.0.1 2026-09-29 Codex 整合性分析の指摘を反映。 -->
# データモデル: コマンドライン検索・結果保存

## CliOptions（CLI内部の実行要求）

| フィールド | 型 | 意味・検証 |
|---|---|---|
| query | SearchQuery | 共通検索条件。空の検索語は禁止 |
| input_path | PathBuf | 正規化した既存のファイルまたはディレクトリ |
| output_path | PathBuf | 既存の親ディレクトリ内の通常ファイル保存先 |
| format | ExportFormat | Csv または Xlsx |
| language | String | ja または en、既定ja |
| overwrite | bool | 既定false。入力ファイルを保護する条件は解除しない |

`SearchQuery.target_dir` は互換性のため名前を保持し、詳細実行ではファイルとディレクトリの双方を受け付ける。対象拡張子は対応する4形式の空でない部分集合。値を含む検索種別がすべて無効なら入力エラーにする。引数文法は [CLI契約](contracts/cli.md) を参照。

## SearchQuery（既存型の互換拡張）

追加する `include_value: bool` は既定true。既存JSON要求では省略可能とし、既存GUIの値検索を維持する。CLIは `include_formula`、`include_comment`、`include_shape`、`include_hidden` を省略時も明示的に設定し、FR-006の既定値を保証する。既存のShapeのSerde既定値を無関係に変更しない。

## SearchMatch（既存結果型を維持）

`id`、ファイル名、絶対パス、シート名、セル番地、行列、列名、一致種別、Shape名、非表示状態、プレビュー、全文、任意の数式、ブック内シート一覧を維持する。新しい出力列は追加しない。

- コメント/メモは `MatchType::Comment`。Shape名と数式はNone、セル番地と所属シートを持つ。
- 行列は1始まり。値・数式はrangeの開始位置を加算して実セルの位置を表す。
- 現行コメントの各投稿・返信はセルと本文を持つ。互換用メモに同じ内容を重複して出さない。
- 実行時IDと順序は並列処理で変わり得る。結果比較はIDと行順を除いたファイル・シート・セル・一致種別・全文・数式・Shape名の多重集合で行う。

## CommentText（抽出用内部型）

`sheet_name: String`、`row: u32`、`col: u32`、`text: String`。行列は1始まり。投稿IDと親IDは抽出中の結合・重複回避にだけ使い、検索結果へ保存しない。関係がない場合は空一覧、参照部品の破損・境界超過・外部参照は検索問題として扱う。

## FileSearchResult / SearchIssue / SearchReport

| 型 | フィールドと役割 |
|---|---|
| FileSearchResult | matches: Vec<SearchMatch>、issues: Vec<SearchIssue>。ブックを開けない場合はErr、開けたブックの一部抽出失敗はissuesに保持して他の項目を返す |
| SearchIssue | path: PathBuf、stage: 探索/ブック/シート/数式/Shape/コメント、任意のsheet_name、cause: エラー分類と原因。panicもファイル問題へ変換 |
| SearchReport | discovered_files、scanned_files、readable_files、failed_files、matches_found、elapsed_ms、issues。件数はusize、時間はu64、重複問題があってもfailed_filesはブック単位 |

レポートは検索完了時に返す。問題メッセージはCLIが翻訳してstderrへ出す。問題の件数と読み取り可能だったブック数で [終了コード](contracts/cli.md) を判定する。ScanProgressの既存JSON契約は変更しない。

## OutputGuard（CLI保存処理内部）

正規化した保存先・親、開始時の既存出力identity、overwrite、入力との衝突状態を保持する。`same-file` のHandleでハードリンクも判定する。入力検出のコールバックで同一性を検査し、保持する情報を出力identityと衝突状態に絞る。全入力パスの別一覧は作らない。

入力衝突、出力先のシンボリックリンク、開始時・入力検出時・公開直前の検査で確認した出力の作成・置換は保存拒否。既定公開は競合時も既存出力を置き換えない。明示上書きは保存先と親を他プロセスが変更しない前提であり、公開直前の検査とrenameの間の変更を排除する保証はない。入力は読取り専用。保存の一時ディレクトリは検索完了後に出力親の中へ作成する。

## 状態遷移

`引数解析 → 検証 → 検索 → 一時保存 → 公開 → 終了`。

- ヘルプは引数解析から正常終了へ移る。
- 引数・対象・入出力保護の不備は失敗終了へ移り、既存出力を保持する。
- ファイル単位の問題は検索を継続し、完走時の結果と問題を保持する。
- 保存または公開が失敗した場合は失敗終了へ移り、一時領域を清掃して既存出力を保持する。
- 保存成功後、問題なしは0、回復可能な問題ありは1、全対象ブックが読み取り不能なら2で終了する。最後の場合も列見出しだけの結果を保存する。
