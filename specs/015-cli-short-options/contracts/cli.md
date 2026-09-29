<!-- 処理内容: CLI短縮オプション、位置引数、複数パス、フォーマット自動推論、標準出力CSV出力のインターフェース契約を定義する。入力・出力: 機能仕様とデータモデルを入力とし、CLI契約（Markdown）を出力する。エラー: 不正引数、競合、終了コードの仕様を明記する。変更履歴: v1.0.0 2026-09-29 AI Agent 初版作成。v1.1.0 2026-09-29 AI Agent 出力先省略時のstdout仕様を追加。 -->
# CLI契約 (015-cli-short-options)

## コマンド構文 (Command Syntax)

```text
exlgrep-cli [OPTIONS] [QUERY] [PATH]...
```

利用可能な呼び出しパターン：
1. **標準出力パイプ形式（-o省略時: 最も簡潔でパイプに最適）**:
   ```bash
   exlgrep-cli "売上" ./data/branch1 ./data/branch2 | grep "重点"
   ```
2. **位置引数＋ファイル保存形式**:
   ```bash
   exlgrep-cli "売上" ./data/branch1 ./data/branch2 -o results.csv
   ```
3. **短縮オプション形式**:
   ```bash
   exlgrep-cli -q "売上" -p ./data -o results.csv -r true -c false
   ```
4. **従来のロングオプション形式（後方互換性）**:
   ```bash
   exlgrep-cli --query "売上" --path ./data --format csv --output results.csv
   ```
5. **ヘルプ表示**:
   ```bash
   exlgrep-cli -h
   exlgrep-cli --help --language en
   ```

---

## 引数およびオプション一覧 (Arguments & Options Reference)

### 1. 位置引数 (Positional Arguments)

| 順序 | 項目 | 説明 | 必須・備考 |
|---|---|---|---|
| 第1引数 | `QUERY` | 検索対象文字列または正規表現 | `-q`/`--query` 未指定時に必須。空文字不可。 |
| 第2引数以降 | `PATH...` | 検索対象ファイルまたはフォルダー（1個以上） | `-p`/`--path` 未指定時に必須。複数指定可能。 |

### 2. オプション一覧 (Named Options)

| 短縮 | ロング | 値 | 必須・既定値 | 説明 |
|---|---|---|---|---|
| `-q` | `--query` | 文字列 | 必須 (位置引数と排他) | 検索文字列 |
| `-p` | `--path` | パス | 必須 (位置引数と排他) | 検索対象パス |
| `-o` | `--output` | パス | **省略可** (省略時はstdout) | 出力先ファイルパス。省略時は標準出力へCSV出力 |
| `-f` | `--format` | `csv` \| `xlsx` | 省略可 (自動推論) | 出力形式。省略時: `-o`省略ならCSV、`-o`ありなら拡張子推論 |
| `-c` | `--match-case` | `true` \| `false` | `false` | 大文字/小文字を区別して検索 |
| `-r` | `--regex` | `true` \| `false` | `false` | 正規表現として解釈 |
| `-w` | `--overwrite` | (フラグ・値なし) | 指定なし=拒否 | 既存の出力先ファイルを上書き保存 |
| `-l` | `--language` | `ja` \| `en` | `ja` | ヘルプ・メッセージ・ヘッダーの出力言語 |
| `-e` | `--extensions` | カンマ区切りリスト | `.xlsx,.xlsm,.xlsb,.xls` | 走査対象のExcel拡張子 |
| - | `--shapes` | `true` \| `false` | `false` | 図形・テキストボックス内を検索 |
| - | `--comments` | `true` \| `false` | `true` | コメント・メモ内を検索 |
| - | `--values` | `true` \| `false` | `true` | セル値を検索 |
| - | `--formulas` | `true` \| `false` | `true` | 数式を検索 |
| - | `--hidden-sheets` | `true` \| `false` | `false` | 非表示シートを対象に含める |
| `-h` | `--help` | (フラグ・値なし) | - | ヘルプ表示（検索は実行しない） |

---

## 標準出力（stdout）とサマリーの仕様

- **`-o` / `--output` が指定されている場合**:
  - 検索結果は指定ファイルへ出力される（CSVはBOM付きUTF-8、Excelはxlsx）。
  - 標準出力（stdout）へ完了サマリー（「検索完了: {scanned} ファイル、{matches} 件一致、保存先 {path}」）を出力する。
- **`-o` / `--output` が省略されている場合（標準出力モード）**:
  - 検索結果は標準出力（stdout）へ直接 CSV 形式（プレーン UTF-8、BOMなし）で出力される。
  - **標準出力へサマリー文言は出力しない**（UNIXパイプラインを壊さないため）。
  - エラーや診断情報（issue）は、発生の都度リアルタイムで標準エラー出力（stderr）へ出力される。
