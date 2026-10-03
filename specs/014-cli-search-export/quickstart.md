<!-- 処理内容: CLI実装後のビルド、動作確認、品質ゲート、配布確認の手順を定義する。入力・出力: 承認済み仕様と調査結果を入力とし、実装の進め方と受け入れ確認手順を出力する。エラー: 入力不備・部分失敗・保存失敗の検証条件を明記する。変更履歴: v1.0.0 2026-09-29 Codex 初版作成。v1.0.1 2026-09-29 Codex 整合性分析の指摘を反映。 -->
# 検証ガイド: コマンドライン検索・結果保存

CLIバイナリと統合テストは実装済み。完了した確認と未対応範囲は [verification.md](verification.md) を参照する。オプションと終了コードの正本は [CLI契約](contracts/cli.md)、内部データは [data-model.md](data-model.md) を参照。

## 前提とビルド

既存アプリをビルドできるRust・Node/npm・各OSのTauriビルド環境を用意する。コマンドはリポジトリルートで実行する。Cargo workspaceの出力先はルートの `target/` である。

```sh
npm ci
npm run build
cargo build -p exlgrep --bin exlgrep-cli
cargo run -p exlgrep --bin exlgrep-cli -- --help
cargo run -p exlgrep --bin exlgrep-cli -- --help --language en
```

最初のヘルプは日本語、次は英語で表示され、ウィンドウ・保存ダイアログが開かず、終了コード0になる。既存GUIの起動も `default-run = "exlgrep"` により従来のバイナリを選ぶことを確認する。

以下はmacOSのsh/zshでの例。WindowsはPowerShellで `./target/debug/exlgrep-cli.exe` を呼び、直後の `$LASTEXITCODE` を確認する。

```sh
CLI_BIN="$PWD/target/debug/exlgrep-cli"
CLI_WORK_DIR="$(mktemp -d)"
```

## 1. 単一ファイルからCSVとExcelへ保存

```sh
"$CLI_BIN" --path tests/fixtures/sample_report.xlsx \
  --query 'Financial Report Q3' --comments false \
  --format csv --output "$CLI_WORK_DIR/result.csv"
"$CLI_BIN" --path tests/fixtures/sample_report.xlsx \
  --query 'Financial Report Q3' --comments false \
  --format xlsx --output "$CLI_WORK_DIR/result.xlsx"
```

各実行の終了コードは0。CSVとExcelに `Financial Report Q3` の一致が入り、セル番地は実ブックのA12となる。CSVはBOM付きUTF-8。GUIで同じ条件を検索・出力し、ID・行順を除いた内容と件数を比較する。

## 2. 0件・正規表現・大小文字

```sh
"$CLI_BIN" --path tests/fixtures/sample_report.xlsx \
  --query '__CLI_NO_MATCH_20260929__' \
  --format csv --output "$CLI_WORK_DIR/empty.csv"
"$CLI_BIN" --path tests/fixtures/sample_report.xlsx \
  --query '^financial report q3$' --regex true --match-case false \
  --comments false --format csv --output "$CLI_WORK_DIR/regex.csv"
"$CLI_BIN" --path tests/fixtures/sample_report.xlsx \
  --query '[' --regex true --format csv \
  --output "$CLI_WORK_DIR/invalid.csv"
```

最初は見出しだけのファイルを保存して0。2つ目はA12の値を検索して0。最後は不正な正規表現を表示して2、出力ファイルは作成しない。引数の省略、不正な真偽値、重複・未知オプションも2を返すことを統合テストで確認する。

## 3. 既存コメントと検索種別の切替

```sh
"$CLI_BIN" --path tests/fixtures/sample_report.xlsx \
  --query '財務報告レビュー対象セル' \
  --values false --formulas false --comments true --shapes false \
  --format csv --output "$CLI_WORK_DIR/comment.csv"
```

既存フィクスチャのA12のメモが `Comment` として1件出る。作者名を検索本文に足さず、Shapeとして二重計上しない。追加フィクスチャで数式、Shape、現行コメントと返信、非表示シート、各true/false設定、大小文字が異なる拡張子も確認する。

## 4. 再帰検索と部分失敗

```sh
mkdir -p "$CLI_WORK_DIR/input/sub"
cp tests/fixtures/sample_report.xlsx "$CLI_WORK_DIR/input/sub/valid.xlsx"
printf '破損データ\n' > "$CLI_WORK_DIR/input/broken.xlsx"
"$CLI_BIN" --path "$CLI_WORK_DIR/input" \
  --query 'Financial Report Q3' --format csv \
  --output "$CLI_WORK_DIR/partial.csv"
echo "$?"
```

終了コード1。有効ブックの結果を保存し、stderrにbroken.xlsxの問題が表示される。壊れたブックだけを別フォルダーに置いたケースは2となり、結果ファイルには見出しだけが保存される。空フォルダーは0になる。部品単位の破損とサブフォルダーの探索失敗も、問題を隠さず処理できる範囲を継続する。

## 5. 上書きと入力保護

```sh
"$CLI_BIN" --path tests/fixtures/sample_report.xlsx \
  --query 'Financial Report Q3' --format csv \
  --output "$CLI_WORK_DIR/result.csv"
"$CLI_BIN" --path tests/fixtures/sample_report.xlsx \
  --query 'Financial Report Q3' --format csv \
  --output "$CLI_WORK_DIR/result.csv" --overwrite
"$CLI_BIN" --path tests/fixtures/sample_report.xlsx \
  --query 'Financial Report Q3' --format xlsx \
  --output tests/fixtures/sample_report.xlsx --overwrite
```

順に2（既存出力を保持）、0（明示的な置換）、2（入力保護）。いずれも対話的確認なし。入力保護の前後でフィクスチャの内容が変わらないことを確認する。

統合テストでは入力へのハードリンク・出力シンボリックリンク、処理途中に出力が出現する競合、書込失敗、公開失敗を検証し、既存ファイルの内容が保持されることを確認する。変更検出のテストは公開直前の検査より前に競合を発生させる。既定の上書き禁止では検査後の出力作成も拒否して既存内容を保持する。明示上書きは保存先と親を他プロセスが変更しない前提であり、最終検査とrenameの間の変更は保証対象外とする。入力とパス構成も実行中に変更しない。

OSでリンク作成が許可されない場合は理由を明示して該当ケースを扱い、通常の入出力保護テストは必ず実行する。

## 6. 形式対応と回帰テスト

```sh
cargo test -p exlgrep --test cli_search
cargo test -p exlgrep --test cli_output
```

CLIの通常検索はxlsx・xlsm・xls・xlsbを対象にする。現在、コメント抽出はxlsx・xlsmの従来メモのみで、スレッドコメントとxls/xlsbのメモは未対応。追加実装時は有効な形式別フィクスチャで値・数式・コメント・Shape・非表示の結果を既知座標と照合する。拡張子だけを変更したファイルで形式対応を証明しない。XLSのメモと通常図形の区別、Unicodeや分割されたコメント本文、破損・保護されたブック、空ファイル、有効な空ブック、極小/極大セル値も含める。Excel出力の行数・文字列長の上限ちょうどと超過をexport関数の単体テストで検証し、超過時は保存失敗となって既存出力が保持されることを確認する。

## 必須品質ゲート

```sh
npm run build
npm test
npm run lint
```

Rust側はsrc-tauriで実行する。

```sh
cd src-tauri
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

全テスト成功、buildのexit 0、警告なし、書式一致を確認する。WindowsとmacOSでGUI非起動・コンソール出力・終了コード・既存出力の置換も確認する。

## リリース成果物

```sh
# フロントエンドおよび CLI の一括ビルド
npm run build:all

# または CLI 単体ビルド
npm run build:cli

# デスクトップアプリ & インストーラー生成 (GUI本体とCLIが自動同梱されます)
npm run tauri build
```

macOSは `target/release/exlgrep-cli`、Windowsは `target/release/exlgrep-cli.exe` をCLI成果物として使う。翻訳は同梱されるため実行時にlocalesフォルダーを置く必要はない。`npm run tauri build` によるインストーラー（MSI / NSIS）生成時には、GUI 本体（`exlgrep.exe`）とともに `exlgrep-cli.exe` も自動同梱される。

検索・保存時間、入力件数、結果件数、測定環境を記録し、CLI専用の全走査や再検索が発生していないことを確認する。測定結果に根拠なく固定秒数の合格を付けない。
