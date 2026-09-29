<!-- 処理内容: CLI検索・結果保存の実装計画、技術構成、憲章適合性を定義する。入力・出力: 承認済み仕様と調査結果を入力とし、実装の進め方と受け入れ確認手順を出力する。エラー: 入力不備・部分失敗・保存失敗の検証条件を明記する。変更履歴: v1.0.0 2026-09-29 Codex 初版作成。v1.0.1 2026-09-29 Codex 整合性分析の指摘を反映。 -->
# Implementation Plan: コマンドライン検索・結果保存

**Branch**: `014-cli-search-export` | **Date**: 2026-09-29 | **Spec**: [spec.md](spec.md)

**Input**: `specs/014-cli-search-export/spec.md`

## Summary

同じRustパッケージへ `exlgrep-cli` バイナリを追加し、Tauriの起動処理を呼ばず、既存の検索・CSV/XLSX出力を直接使う。コマンドラインで検索条件と保存先を指定し、全成功・部分失敗・全体失敗を終了コード0・1・2で区別する。

既存実装には単一ファイルのengine受付、値検索の切替、コメント抽出、部分失敗の報告が不足している。この範囲を共有コアで補い、CLI専用の検索エンジンは作らない。安全な一時保存と公開をCLI側に置き、既存出力と検索入力を保護する。詳細は [research.md](research.md) と [CLI契約](contracts/cli.md) を参照。

## Technical Context

**Language/Version**: Rust 2021 edition。既存React 18 / TypeScriptフロントエンドは回帰確認の対象。

**Primary Dependencies**: 既存calamine 0.36（ロック済み0.36.1）、rayon、regex、walkdir、csv、rust_xlsxwriter、quick-xml、zip、cfb、encoding_rs、serde。既存Cargo.lockに含まれるserde_yaml 0.9.34とsame-file 1.0.6を直接依存へ宣言する。引数解析・一時領域・公開操作は標準ライブラリを使う。

**Storage**: ローカルExcel入力とCSV/XLSX出力。検索完了後に出力親ディレクトリ内の一時領域を使用する。設定・検索履歴の保存は不要。

**Testing**: Rustの既存単体・統合テストを拡張し、CLIを実プロセスで起動する統合テストを追加する。フロントエンドは既存Vitest、build、lintで回帰確認する。

**Target Platform**: 既存アプリが対象とするWindows・macOS。Windows版CLIはコンソールアプリとして起動する。

**Project Type**: 既存デスクトップアプリに同一パッケージのCLIバイナリを追加。

**Performance Goals**: 同一条件で既存コアと一致する結果を得る。既存の並列処理と容量制限付き探索キューを維持し、CLI用の事前全走査や再検索を追加しない。検索・保存時間と結果件数を記録するが、新たな固定秒数の合格基準は設けない。

**Constraints**: GUI・ダイアログ・IPCの初期化なし、ネットワーク送信なし、検索入力の上書き禁止、既存出力は明示指定のみ置換。保存失敗時は既存出力を保持する。入力とパス構成は実行中に他プロセスが変更しない前提。明示上書きは保存先と親にも同じ前提を適用し、検査とrenameの間の第三者による変更を排除する保証はない。既定公開は競合時も既存出力を上書きしない。

**Scale/Scope**: 1検索パス、4入力形式、CSV/XLSXの2出力形式、既存の検索対象種別。結果件数に比例するメモリとExcelライブラリの出力上限を受け継ぐ。上限に当たった結果を黙って切り捨てない。

## Constitution Check

Phase 0開始前とPhase 1設計後の判定を以下に示す。実装のテスト成功を意味するものではない。

| 原則・ゲート | 設計上の対応 | 調査前 / 設計後 |
|---|---|---|
| I. 指定言語と自然な出力 | 文書は日本語。CLIは既定ja、明示指定en。文言は既存翻訳カタログを共有 | PASS / PASS |
| II. 定数の外部抽出 | 引数名、既定値、終了コード、翻訳キー、固定パス、形式レコードID・上限はconstants.rsに定義。使用箇所へ定数参照コメント | PASS / PASS |
| III. 4要素ヘッダ | 新規・変更ファイル、型、関数・メソッドに処理説明、型付き入出力、エラー条件、履歴を付与 | PASS / PASS |
| IV. モジュール責務 | CLI引数・実行・保存を分割。共有検索と出力を再利用し、新規コアクレートや汎用フレームワークを作らない | PASS / PASS |
| V. 堅牢なエラーと検証 | ファイル・探索・抽出の問題を報告し、保存失敗を伝播。形式別の実データと実プロセスの終了状態を確認 | PASS / PASS |
| 性能とローカル処理 | 既存並列パイプラインを維持。外部relationshipsを取得せず、ファイル内容を送信しない | PASS / PASS |
| 品質ゲート | 実装後にnpm build/test/lint、cargo test、clippy、fmtを実施する計画 | PASS / PASS |

憲章の例外は不要。技術的な未決定事項は [調査結果](research.md) で解消した。

## Project Structure

### Documentation (this feature)

```text
specs/014-cli-search-export/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/cli.md
└── checklists/requirements.md
```

実装作業は [tasks.md](tasks.md) に生成済み。

### Source Code (repository root)

以下は実装時に追加・変更する予定の範囲。

```text
src-tauri/
├── Cargo.toml                  # default-run、既存依存の直接利用
├── locales/{ja,en}.yml         # CLI文言の追加
├── src/
│   ├── bin/exlgrep-cli.rs      # 新規: コンソールの入口
│   ├── cli/
│   │   ├── mod.rs             # 新規: 検証、検索、状態判定
│   │   ├── args.rs            # 新規: 引数解析とヘルプ
│   │   └── output.rs          # 新規: 入力保護、一時保存、公開
│   ├── lib.rs                 # CLIモジュールの公開
│   ├── constants.rs           # CLI・コメント用定数
│   ├── i18n.rs                # GUI不要の同梱カタログ読取
│   ├── models/mod.rs          # 値検索切替と詳細レポート
│   └── search/
│       ├── engine.rs          # ファイル入力、問題集計、入力通知
│       ├── parser.rs          # 対象切替、コメント、正しい座標
│       ├── mod.rs
│       ├── comments/
│       │   ├── mod.rs         # 新規: 共通コメント情報
│       │   ├── ooxml.rs       # 新規: xlsx/xlsm、現行コメントXML
│       │   ├── xls.rs         # 新規: BIFFメモ
│       │   └── xlsb.rs        # 新規: バイナリコメント
│       └── shape/{mod,ooxml,xls,xlsb}.rs # 部品読取の共有・コメント分類
└── tests/{cli_search,cli_output}.rs     # 新規: 実プロセスの契約検証
Cargo.lock                     # 直接依存の関係を更新
README.md                      # CLIビルドと実行の案内
src-tauri/locales/README.md     # CLI文言と同梱読取の案内
tests/fixtures/cli/            # 有効な4形式の追加フィクスチャ
```

**Structure Decision**: 新規ファイルはCLIと未実装のコメント抽出に必要な責務へ限定する。CSV/XLSX出力本体は既存関数を再利用する。GUIの公開JSON契約は互換を保つ。

## Phase 0: 調査

完了。[research.md](research.md) に採用方針、根拠、代案を記載した。`execute_search` のGUI呼び出し元、parserの統合テスト、exportコマンド、ロック済み依存とローカルソースを確認した。コメントの形式情報とファイル公開の挙動は一次資料で確認した。

## Phase 1: 設計

完了。[data-model.md](data-model.md)、[contracts/cli.md](contracts/cli.md)、[quickstart.md](quickstart.md) を生成した。詳細実行のレポートと入力通知を、既存関数の互換ラッパーから共有する。CLIは同梱翻訳、実行結果、保存ガードだけを追加し、コアを複製しない。

実装順序は、共有検索の不足解消 → CLI引数とカタログ → 保存保護と終了コード → 実プロセス検証・案内の順とする。コメント抽出は4形式の実データによる検証を完了させる必要がある。

## 検証方針

FR-001〜004/007〜009は単一ファイル・再帰検索・ヘルプ・両出力・0件で確認する。FR-005/006は検索種別と既定値、4形式と現行コメント・返信・非表示シートで確認する。FR-010/011は正常と破損ファイルの混在、全件読取失敗、部品単位の破損、探索失敗で確認する。FR-012は通常の既存出力、入力と同じパス、シンボリックリンク・ハードリンク、保存中の出力出現で確認する。FR-013は外部参照を取得しないコード経路とGUI非起動を確認する。

空ファイルと有効な空ブックを区別し、極小/極大セル値、Excelの行数・文字列長の上限境界と超過時の保存失敗を検証する。

同じコアへの呼び出し結果だけで正しさを判断せず、既知のセル番地・本文を照合する。ID・行順は固定せず、重複数を含む結果内容を比較する。実装後の必須ゲートと実行例はquickstartに記載した。本コマンドでは設計文書だけを作成し、実装テストはまだ実施していない。
