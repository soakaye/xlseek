<!-- 処理内容: CLI短縮オプション、位置引数、複数パス、非同期パイプライン、標準出力CSVストリーミングの実装タスクをユーザーストーリー別・依存順に定義する。入力・出力: spec.md、plan.md、research.md、data-model.md、contracts/cli.md、quickstart.mdを入力とし、実行可能なタスクと完了条件を出力する。エラー: 未達の検証を完了扱いにせず、依存関係と環境上の未確認事項を記録する。変更履歴: v1.0.0 2026-09-29 AI Agent 初版作成。 -->
# Tasks: CLI短縮オプション・位置引数・複数パス・非同期パイプライン・標準出力対応 (015-cli-short-options)

**入力**: `specs/015-cli-short-options/` の設計文書。ブランチ: `015-cli-short-options`。

**前提**: [計画](plan.md)、[仕様](spec.md)、[調査](research.md)、[データモデル](data-model.md)、[CLI契約](contracts/cli.md)、[検証ガイド](quickstart.md)。

**テスト**: AGENTS.mdと憲章原則Vに従い、短縮オプション、位置引数、複数パス走査、非同期パイプライン、標準出力ストリーミングを検証する単体・結合テストを作成する。

---

## Phase 1: Setup（定数・翻訳・共通準備）

**目的**: 短縮オプション文字、出力ターゲット定数、更新された日英ヘルプ・エラー文言を一元定義する。

- [x] T001 [P] `src-tauri/src/constants.rs` に短縮オプション文字（`-p`, `-q`, `-f`, `-o`, `-c`, `-r`, `-w`, `-l`, `-h`, `-e`）、位置引数インデックス定数、標準出力モード用定数を定義し、憲章原則II（定数抽出）に準拠する。
- [x] T002 [P] `src-tauri/locales/ja.yml` および `src-tauri/locales/en.yml` の `cli.HELP` を更新し、位置引数構文（`<QUERY> <PATH>...`）、短縮オプション一覧、および出力先省略時の標準出力（stdout）動作を日英で記載する。

**チェックポイント**: 定数・翻訳カタログが更新され、引数解析とヘルプ表示の準備が整う。

---

## Phase 2: Foundational（共通基盤・データモデル・出力層）

**目的**: 複数パス対応と標準出力CSVストリーミングのデータモデル・低レベル書き出し基盤を整備する。

- [x] T003 [P] `src-tauri/src/cli/args.rs` に `CliOutputTarget`（`File(PathBuf)` / `Stdout`）列挙型を定義し、`CliOptions` の `input_path: PathBuf` を `input_paths: Vec<PathBuf>` へ、`output_path: PathBuf` を `output_target: CliOutputTarget` へ変更する。
- [x] T004 [P] `src-tauri/src/export/csv_export.rs` に `std::io::Write` を受け取れる汎用CSV書き出し関数 `write_csv_to_writer<W: std::io::Write>` を追加し、BOM付与有無フラグ（stdout時はBOMなし、ファイル時はBOMあり）をサポートする。既存 `export_to_csv` はその互換ラッパーとする。

**チェックポイント**: 複数パスおよび標準出力ターゲットを保持する型定義と、汎用CSVストリーミング機能が完成。

---

## Phase 3: User Story 1 & User Story 2 - 位置引数・複数パス & 標準出力CSVストリーミング (Priority: P1) 🎯 MVP

**Goal**: 位置引数（第1=検索語、第2以降=複数パス）による直感的な検索と、`-o` 省略時の標準出力（stdout）への純粋なCSVストリーミング出力を実現する。

**Independent Test**: `exlgrep-cli "test" ./data1 ./data2` で標準出力にヘッダー付きCSVレコードが出力され、`-o out.csv` でファイルに保存されることを確認する。

### 先行テスト
- [x] T005 [P] [US1] `src-tauri/src/cli/args.rs` 内に位置引数抽出（第1=query, 第2以降=paths）および `-o` 省略時の `CliOutputTarget::Stdout` 判定の単体テストを追加する。
- [x] T006 [P] [US2] `src-tauri/tests/cli_search.rs` に実プロセス起動テストを追加し、位置引数での単一/複数パス指定、および標準出力モード（stdoutへ純粋CSV出力、サマリー文言抑制）を検証する。

### 実装
- [x] T007 [US1] `src-tauri/src/cli/args.rs` の `parse_args` を拡張し、`-` で始まらない独立引数および `--` 以降を位置引数として抽出し、第1引数を query、第2引数以降を input_paths へ設定するロジックを実装する。
- [x] T008 [US2] `src-tauri/src/cli/args.rs` で `-o` / `--output` が未指定の場合に `CliOutputTarget::Stdout` とし、フォーマットを `ExportFormat::Csv` に自動設定する。
- [x] T009 [US1] `src-tauri/src/cli/output.rs` を更新し、`CliOutputTarget::File` の場合のみファイル検証・上書き検査・一時保存を行い、`CliOutputTarget::Stdout` の場合はファイル保護処理をスキップする。
- [x] T010 [US1] `src-tauri/src/cli/mod.rs` の `execute` を更新し、`CliOutputTarget::Stdout` 時は `write_csv_to_writer` で標準出力にCSVを直接ストリーミング出力し、サマリー文言を非表示にする。`File` 時はサマリーを通常通りstdoutに出力する。
- [x] T011 [US1] T005およびT006のテストを実行し、位置引数検索および標準出力ストリーミングがパスすることを確認する。

**チェックポイント**: 位置引数（複数パス対応）および出力先省略時の標準出力ストリーミング（MVP）が完全に動作する。

---

## Phase 4: User Story 3 - 頻出オプションの短縮形式（ショートオプション）指定とフォーマット自動推論 (Priority: P1)

**Goal**: `-p`, `-q`, `-f`, `-o`, `-c`, `-r`, `-w`, `-l`, `-h`, `-e` の短縮オプション指定と、出力拡張子（`.csv` / `.xlsx`）からのフォーマット自動推論を実現する。

**Independent Test**: `-q "test" -p ./data -o out.xlsx -c true` で設定が反映され、`-f` を省略しても `.xlsx` として正常に出力されることを確認する。

### 先行テスト
- [x] T012 [P] [US3] `src-tauri/src/cli/args.rs` 内に短縮オプション（各1文字、空白区切り/イコール区切り）およびフォーマット自動推論（`.csv` → Csv, `.xlsx` → Xlsx）の単体テストを追加する。

### 実装
- [x] T013 [US3] `src-tauri/src/cli/args.rs` に短縮オプション文字（`-p`, `-q`, `-f`, `-o`, `-c`, `-r`, `-w`, `-l`, `-h`, `-e`）のマッピングと、空白区切り（`-o path`）・イコール区切り（`-o=path`）の双方を同一のロング名へ正規化して解釈する処理を実装する。
- [x] T014 [US3] `src-tauri/src/cli/args.rs` で、ファイル出力時に `-f` / `--format` が省略された場合、出力ファイルパスの拡張子（`.csv` または `.xlsx`）から `ExportFormat` を自動推論し、拡張子不一致や推論不能時はエラーとする処理を実装する。
- [x] T015 [US3] T012のテストを実行し、短縮オプションおよびフォーマット推論がすべてパスすることを確認する。

**チェックポイント**: 短縮オプション指定と拡張子からのフォーマット自動推論が正常に動作する。

---

## Phase 5: User Story 4 - ディレクトリ探索とファイル内検索の非同期パイプライン処理 (Priority: P1)

**Goal**: 複数パスのディレクトリ探索（プロデューサー）とファイル内検索（コンシューマー）を有界同期チャネル（`sync_channel`）と `rayon` で接続し、全探索完了を待たずに順次並行解析を実行し、エラー発生時は即座に stderr へ通知する。

**Independent Test**: 大量ファイルが存在する環境で探索と検索が並行動作すること、および破損・アクセス不可ファイルが検出された瞬間にリアルタイムで stderr に出力されることを確認する。

### 先行テスト
- [x] T016 [P] [US4] `src-tauri/tests/cli_search.rs` に非同期パイプラインの結合テストを追加し、複数パス（ディレクトリ＋ファイル混在）の走査、`same-file` による重複ファイル走査除外、およびエラー時の即時 stderr 出力を検証する。

### 実装
- [x] T017 [US4] `src-tauri/src/cli/mod.rs` に `sync_channel::<PathBuf>(constants::CHANNEL_BUFFER_SIZE)` を導入し、探索スレッド（プロデューサー）を別スレッドで起動して複数パスを順次走査（ファイル直通、ディレクトリ `WalkDir`）し、`same-file::Handle` によるユニーク化と出力衝突検査を行ってチャネルへ送信する。探索中のエラーは発生時に即時 `eprintln!` で stderr に出力する。
- [x] T018 [US4] `src-tauri/src/cli/mod.rs` の受信側（コンシューマー）で `rayon` のスレッドプールを活用し、チャネルから届いたパスを順次 `parse_and_search_file` で並行解析し、解析エラー発生時も即時 `eprintln!` で stderr に出力しつつ結果をスレッドセーフに収集する。
- [x] T019 [US4] T016のテストを実行し、非同期パイプライン処理およびリアルタイムエラー出力が正常に動作することを確認する。

**チェックポイント**: ディレクトリ探索とファイル解析の同時並行化およびリアルタイムエラー出力が完了。

---

## Phase 6: User Story 5 & User Story 6 - 重複・競合エラー通知とヘルプ表示 (Priority: P2)

**Goal**: 位置引数と名前付きオプションの重複指定（排他違反）や未知の短縮オプションのエラー検知、および更新されたヘルプ表示を完成させる。

**Independent Test**: `exlgrep-cli "query" ./dir -q "another"` で重複エラーとなり、`-h` で位置引数と短縮オプションの対比ヘルプが出力されることを確認する。

### 先行テスト
- [x] T020 [P] [US5] `src-tauri/src/cli/args.rs` 内に位置引数と名前付きオプション（`-q`, `-p`）の重複エラー、未知の短縮オプションエラー、必須クエリ欠落エラーの単体テストを追加する。

### 実装
- [x] T021 [US5] `src-tauri/src/cli/args.rs` で、位置引数で query が指定された場合の `-q` / `--query` 重複チェック、および位置引数で paths が指定された場合の `-p` / `--path` 重複チェックを実装し、違反時は `Duplicate CLI option` エラーとする。
- [x] T022 [US5] `src-tauri/src/cli/args.rs` で未知の短縮オプション（未定義の `-x` 等）が渡された場合に `Unknown CLI option: -x` を返して終了コード2とする。
- [x] T023 [US6] `src-tauri/src/cli/args.rs` で `-h` / `--help` が指定された場合、T002で定義した日英カタログから位置引数構文、短縮オプション、標準出力動作を含むヘルプメッセージを表示して終了コード0で即時終了する。
- [x] T024 [US5] T020のテストを実行し、すべてのエラーケースおよびヘルプ表示がパスすることを確認する。

**チェックポイント**: 引数の重複・競合・不正入力に対する安全保護とヘルプ案内が完成。

---

## Phase 7: Polish & Quality Gates（品質検証と仕上げ）

**目的**: 全体結合の検証、ドキュメントの整合性確認、および憲章必須の品質ゲート（Clippy、Fmt、テスト、Build）を通過する。

- [x] T025 `specs/015-cli-short-options/quickstart.md` に記載されている全シナリオ（位置引数、複数パス、短縮オプション、標準出力パイプ、ヘルプ）を手動・自動で実行し、期待結果と合致することを確認する。
- [x] T026 憲章原則III（4要素ヘッダコメント）および原則II（定数外部化）がすべての変更ファイルに漏れなく適用されていることを確認する。
- [x] T027 必須検証コマンドを実行し、エラーおよび警告がゼロであることを確認する：
  - `cargo test --manifest-path src-tauri/Cargo.toml`
  - `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings`
  - `cargo fmt --manifest-path src-tauri/Cargo.toml --check`
  - `npm run build`

---

## Dependencies & Execution Order

### Phase Dependencies
- **Phase 1 (Setup)**: 前提なし。即時開始可能。
- **Phase 2 (Foundational)**: Phase 1 完了後に開始。
- **Phase 3 (US1/US2 - MVP)**: Phase 2 完了後に開始。
- **Phase 4 (US3 - 短縮オプション & 推論)**: Phase 3 完了後に開始。
- **Phase 5 (US4 - 非同期パイプライン)**: Phase 3 完了後に開始（Phase 4 と並行作業可能）。
- **Phase 6 (US5/US6 - 排他検査 & ヘルプ)**: Phase 4 & Phase 5 完了後に開始。
- **Phase 7 (Polish)**: すべてのユーザーストーリー完了後に開始。

### Parallel Opportunities
- T001 と T002 は並行作業可能。
- T003 と T004 は並行作業可能。
- T005 と T006 は並行作業可能。
- Phase 4（引数解析の短縮・推論）と Phase 5（実行パイプラインの並列化）はモジュールが分かれているため並行作業可能。
