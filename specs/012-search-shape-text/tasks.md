<!-- 処理内容: Shape 内テキスト検索を依存順に実装・検証するタスクを定義する。引数・戻り値: 入力は spec.md、plan.md と設計資料、出力は実行可能な作業一覧。エラー: 4形式の未検証や品質ゲート失敗を完了扱いしない。変更履歴: v1.0.0 2026-09-27 Codex 初版作成。v1.1.0 2026-09-27 Codex 残る分析指摘4件を修正。 -->
# Tasks: Excel Shape 内テキストの検索

**Input**: `specs/012-search-shape-text/` の [spec.md](spec.md)、[plan.md](plan.md)、[research.md](research.md)、[data-model.md](data-model.md)、[検索契約](contracts/shape-search-contract.md)、[quickstart.md](quickstart.md)

**Tests**: プロジェクト憲章が新機能の単体テストを必須とするため、各ストーリーの検証を実装より先に置く。

**Organization**: US1＝Shape の発見と表示、US2＝独立オプション、US3＝既存条件・出力。各タスクは着手条件を依存関係に従って満たす。

## Phase 1: Setup

**Goal**: 4形式の実物ブックと必要な読取依存を用意する。

- [ ] T001 `tests/fixtures/shapes/{sample,edge}.{xlsx,xlsm,xls,xlsb}` と `tests/fixtures/shapes/README.md` に、4形式共通の通常図形・テキストボックス・グループ内の子図形2個・空図形・画像・グラフ・非表示シート・セル値・数式を含む sample と、形式ごとに分割テキスト・アンカー欠損・破損図形を確認できる edge を用意する。README に各ファイルの作成元または入手元、作成・変換手順、SHA-256、図形名・文字列・アンカー・期待結果件数を記録する
- [X] T002 [P] `src-tauri/Cargo.toml` と `Cargo.lock` に ZIP・XML・CFB 読取に必要な直接依存だけを追加し、既存 Rust ビルドを確認する

**Checkpoint**: 4形式の fixture を実際に開け、各図形の名前・文字列・所在が README の期待値と一致する。

---

## Phase 2: Foundational

**Goal**: 全ストーリーが共有する要求・結果モデルと安全な表示経路を整える。

- [X] T003 `src-tauri/tests/shape_contract.rs` に `include_shape` の省略時既定値 `true`、`Shape` 種別、`shape_name`、`sheet_hidden`、アンカーなしの空セル番地と内部行列 0 を検証する失敗テストを追加する
- [X] T004 `src-tauri/src/models/mod.rs` に既定値 `true` の `SearchQuery.include_shape: bool`、`MatchType::Shape`、`SearchMatch.shape_name: Option<String>`、`sheet_hidden: bool` を追加し、Shape では名前必須・既存結果では `None` の契約を既存の生成箇所にも適用する
- [X] T005 [P] `src/types/search.ts` に Rust と同じ検索条件・一致種別・結果フィールドを追加し、アンカー不明時の空文字列・行列 0 の表現を一致させる
- [X] T006 `src-tauri/src/search/parser.rs` の単体テストに、検索対象文字列中の HTML 特殊文字が結果スニペットから実行可能な HTML にならない失敗ケースを追加する
- [X] T007 `src-tauri/src/search/parser.rs` と `src/components/results/ResultTable.tsx` の共通スニペット経路を安全に描画し、セル値・数式・Shape すべての未信頼文字列を HTML として解釈しないようにする
- [X] T008 [P] `src-tauri/src/constants.rs` に ZIP 展開量・XML 深さ・BIFF レコード長の上限と形式識別子を定義し、利用箇所の定数参照コメントと4要素ヘッダの規則を `src-tauri/src/search/shape/mod.rs` の入口に適用する

**Checkpoint**: T003 と T006 のテストが通り、Shape 結果を既存 IPC 契約で安全に表現できる。

---

## Phase 3: User Story 1 - Shape 内の文字を見つける (Priority: P1) 🎯 MVP

**Goal**: 4形式の通常図形・テキストボックス・グループ内の子図形を検索し、個別の結果と全文を表示する。

**Independent Test**: 4形式それぞれで Shape にだけある語を検索し、図形ごとに1件、正しいファイル・シート・Shape 名・全文が表示される。

### Tests

- [ ] T009 [P] [US1] `src-tauri/tests/shape_extract.rs` に4形式ごとのテキストボックス・通常図形・グループ内の子図形2個・同名図形・空図形・画像・グラフを検証する失敗テストを追加する
- [ ] T010 [P] [US1] `tests/shape-results-ui.test.tsx` に Shape 種別・名前・全文の表示、未信頼文字列の安全な表示、アンカーあり・なしの双方で `get_cell_preview` を呼ばず、実アンカーだけを補助表示する失敗テストを追加し、結果選択からファイル・シート・Shape 名・一致箇所の表示までが3操作以内であることを確認する

### Implementation

- [ ] T011 [US1] `src-tauri/src/search/shape/xls.rs` で T009 の `.xls` fixture の代表的テキストボックスとグループ内の子図形を先に抽出して実現性を確認し、CFB/BIFF の `MsoDrawing`・`TxO`・`Continue` と OfficeArt グループ階層の読取を完成させ、子図形ごとの名前・文字・アンカーを返す
- [X] T012 [US1] `src-tauri/src/search/shape/ooxml.rs` に `.xlsx`・`.xlsm` のシート名→シートパート→描画パート参照と `xdr:sp/txBody` の文字順序・段落境界・アンカー抽出を実装し、`xdr:grpSp` の子図形を個別に返す
- [X] T013 [US1] `src-tauri/src/search/shape/xlsb.rs` に `.xlsb` のシート名→バイナリシートの対応、`BrtDrawing` とリレーションの解決を実装し、T012 の描画 XML 抽出を再利用する
- [ ] T014 [US1] `src-tauri/src/search/shape/mod.rs` と `src-tauri/src/search/mod.rs` に4形式の抽出結果を共通の Shape テキストへ正規化する入口を接続し、グループ名から子図形名までの経路を保持し、名前欠損時は図形 ID に基づく安定名を付ける
- [X] T015 [US1] `src-tauri/src/search/parser.rs` に既存の一致判定を使う Shape 結果の合流を追加し、空文字は除外、同一図形の複数一致は1件、別図形は別件、`formula=None`、実アンカーは1始まり、アンカー不明ならセル番地・列名は空で行列は内部値 0 とする
- [ ] T016 [US1] `src-tauri/tests/shape_extract.rs` に破損図形・過大 ZIP/XML/BIFF 入力・検索中断の失敗テストを追加し、読取可能な他の図形とセル結果が残ることを検証する
- [ ] T017 [US1] `src-tauri/src/search/shape/{ooxml,xlsb,xls}.rs` と `src-tauri/src/search/parser.rs` に T016 を満たす境界検証、図形単位のエラー隔離、キャンセル確認を追加する
- [X] T018 [US1] `src/components/results/ResultTable.tsx`、`src-tauri/locales/ja.yml`、`src-tauri/locales/en.yml` に日英の Shape 種別・Shape 名ラベル、Shape 名の表示・絞り込み、実際のアンカーだけのセル位置表示を追加する
- [X] T019 [US1] `src/hooks/useSearch.ts`、`src/App.tsx`、`src/components/preview/MetaInfoCard.tsx` に Shape 選択時のセルプレビュー抑止と Shape 名・シート・全文の表示を追加し、アンカー不明時は行列とセル番地を隠す
- [X] T020 [US1] `design/mainui/index.html` に Shape 結果の一覧・詳細表示を反映し、`src-tauri/tests/shape_extract.rs` と `tests/shape-results-ui.test.tsx` を通して US1 を確認する

**Checkpoint**: 4形式の抽出テストと結果表示テストが通る。1形式でも欠けた場合は US1 未完了。

---

## Phase 4: User Story 2 - Shape 検索を切り替える (Priority: P1)

**Goal**: 数式検索と独立した Shape 検索トグルを提供し、オフの時は描画データを読まない。

**Independent Test**: 同じ語が Shape、セル、数式にあるブックで Shape の切替だけが Shape 結果を増減させ、数式の切替と互いに干渉しない。

### Tests

- [ ] T021 [P] [US2] `src-tauri/tests/shape_contract.rs` に `include_shape=false` では破損描画データも開かず Shape 0件、セル・数式結果は維持される失敗テストを追加する
- [ ] T022 [P] [US2] `tests/shape-option-ui.test.tsx` に既定オン、トグル操作、数式オプションとの独立、検索要求への値の反映を検証する失敗テストを追加する

### Implementation

- [X] T023 [US2] `src-tauri/src/search/parser.rs` に `include_shape=false` の場合は Shape 抽出入口を呼ばない条件を追加し、既存のセル値・数式経路をそのまま実行する
- [X] T024 [US2] `src/hooks/useSearch.ts` と `src/components/search/SearchBar.tsx` に既定オンの `include_shape` と独立トグルを追加し、変更を `start_search` 要求へ反映する
- [X] T025 [US2] `src/constants/index.ts`、`src-tauri/locales/ja.yml`、`src-tauri/locales/en.yml` に Shape 検索項目の定数と日英文言を追加する
- [X] T026 [US2] `design/mainui/index.html` に Shape 検索トグルを反映し、`src-tauri/tests/shape_contract.rs` と `tests/shape-option-ui.test.tsx` を通して US2 を確認する

**Checkpoint**: Shape オフで結果 0 件、数式オフでも Shape オンなら Shape 結果が得られる。

---

## Phase 5: User Story 3 - 既存条件と結果出力を使う (Priority: P2)

**Goal**: 大文字小文字・正規表現・非表示シートと日英表示を Shape に適用し、CSV / Excel 出力に Shape 名を残す。

**Independent Test**: 条件を変えた検索結果と CSV / Excel の行を照合し、Shape の全件と所在・種別・文字列が一致する。

### Tests

- [ ] T027 [P] [US3] `src-tauri/tests/shape_search_conditions.rs` に大文字小文字・正規表現・実際の非表示シート状態を4形式で検証する失敗テストを追加する
- [ ] T028 [P] [US3] `src-tauri/tests/shape_export.rs` に CSV / Excel の独立 Shape 名列、セル位置の空値、全結果件数、日英の列名と種別を検証する失敗テストを追加する
- [ ] T029 [P] [US3] `tests/shape-results-ui.test.tsx` に日英の Shape 種別と実際のシート可視状態を確認する失敗テストを追加する

### Implementation

- [X] T030 [US3] `src-tauri/src/search/parser.rs` に `Reader::sheets_metadata()` による Hidden と VeryHidden の判定を導入し、シート名に `hidden` を含むかで判定する旧処理を置き換える
- [X] T031 [US3] `src-tauri/src/search/parser.rs` に Shape とセル値・数式で同じ検索語、大文字小文字、正規表現の一致判定を使う処理を完成させ、`sheet_hidden` を実際の状態で埋める
- [X] T032 [US3] `src-tauri/src/constants.rs`、`src-tauri/locales/ja.yml`、`src-tauri/locales/en.yml` に出力 Shape 名列と Shape 種別の日英キー、列幅・列数の定数を定義する
- [X] T033 [P] [US3] `src-tauri/src/export/csv_export.rs` に Shape 名の独立列、Shape 種別、実際のアンカーだけのセル位置を出力する処理を追加する
- [X] T034 [P] [US3] `src-tauri/src/export/xlsx_export.rs` に T032 の共通定数で定めた列順と値を出力し、列幅・フィルター範囲を新列数に合わせる
- [X] T035 [US3] `src/components/results/ResultTable.tsx` と `src/components/preview/MetaInfoCard.tsx` に Shape 種別の日英表示と `sheet_hidden` による可視状態を反映する
- [ ] T036 [US3] `design/mainui/index.html` に Shape の種別・可視状態の最終表示を同期し、`src-tauri/tests/{shape_search_conditions,shape_export}.rs` と `tests/shape-results-ui.test.tsx` を通して US3 を確認する

**Checkpoint**: 条件別のヒット件数と CSV / Excel の Shape 行数が画面と一致する。

---

## Phase 6: Polish & Cross-Cutting Concerns

**Goal**: 仕様の成功基準と憲章の品質ゲートを全件確認する。

- [ ] T037 `specs/012-search-shape-text/quickstart.md` に従って4形式の画面操作、破損入力、キャンセル、日英表示、CSV / Excel 出力を確認し、アンカーあり・なしの各結果選択からファイル・シート・Shape 名・一致箇所の表示までの操作数が3以内か記録する。各ファイルに一致する Shape があるローカル100ファイル・合計100 MB で画面の初回結果が10秒以内かと検索中の操作可否を測り、端末環境と結果を `specs/012-search-shape-text/verification.md` に記録する
- [ ] T038 `src-tauri/tests/shape_performance.rs` に100ファイル・合計100 MB の測定用データと各ファイルの Shape ヒットを確認する手動実行の計測を追加し、`src-tauri/src/search/shape/{ooxml,xlsb,xls}.rs` の不要な重複読取を計測結果に基づいて解消する
- [ ] T039 `git status --short --untracked-files=all` で今回追加・変更した全テキストファイルを列挙し、`src-tauri/src/`、`src-tauri/tests/`、`src/`、`tests/`、`src-tauri/locales/`、`design/mainui/index.html`、設定ファイルの4要素ヘッダ・定数参照・文字化け・UI整合を漏れなく確認して `specs/012-search-shape-text/verification.md` に記録する
- [X] T040 `specs/012-search-shape-text/verification.md` に `npm test`、`npm run build`、`npm run lint`、`cd src-tauri && cargo test`、`cargo clippy --all-targets -- -D warnings`、`cargo fmt --check` の実行結果を記録し、失敗・警告があれば該当ソースを修正して再実行する

---

## Dependencies & Execution Order

| Phase | Dependency | Completion signal |
|---|---|---|
| Setup | なし | 4形式 fixture と読取依存が準備できる |
| Foundational | Setup | 共通モデルと安全な表示経路がテストで確認できる |
| US1 | Foundational | 4形式の抽出と Shape 結果表示が成立する |
| US2 | US1 の抽出入口と共通モデル | 独立トグルで Shape の走査だけが切り替わる |
| US3 | US1 の結果経路 | 既存条件、日英表示、両形式の出力が一致する |
| Polish | US1、US2、US3 | 成功基準・品質ゲートがすべて通る |

US2 の画面トグルと US3 の出力テスト準備は、US1 の形式別抽出実装と並行可能。ただし同じソースファイルを更新するタスクは順番に統合する。US1 を4形式未達のまま MVP 完了とはしない。

## Parallel Execution Examples

- **US1**: T009 (`src-tauri/tests/shape_extract.rs`) と T010 (`tests/shape-results-ui.test.tsx`) は別ファイルで並行可能。T011 で `.xls` の代表例を先に実証する。T012 の OOXML 抽出器を完成させた後に T013 の `.xlsb` 連携を進める。
- **US2**: T021 (Rust 契約テスト) と T022 (画面テスト) は別ファイルで並行可能。
- **US3**: T027 (検索条件テスト)、T028 (出力テスト)、T029 (画面テスト) は別ファイルで並行可能。T033 (CSV) と T034 (Excel) は共通定数 T032 が整った後、別ファイルで並行可能。

## Implementation Strategy

1. Setup と Foundational を終え、テストを失敗させてから実装する。
2. US1 は `.xls` の代表例と抽出経路を先に確認し、次に `.xlsx`・`.xlsm`、最後に `.xlsb` を実装して、4形式の結果表示までを MVP とする。
3. US2 でオプションの独立性と Shape オフ時の非読取を確認する。
4. US3 で既存条件・可視状態・出力・日英表示を完成させる。
5. quickstart と全品質ゲートを通し、4形式の未達を残さず完了判定する。

## Phase 7: Convergence

**Goal**: 実装と仕様・計画の残る差を解消し、未完の受け入れ条件を形式横断で検証する。

- [ ] T041 CRITICAL: `src-tauri/src/search/shape/{ooxml,xlsb,xls}.rs` の固定パート名・XML要素名・BIFFオフセット等を定数へ集約し、利用箇所に定数参照コメントを追加する per Constitution II (contradicts)
- [ ] T042 `.xls` の `MsoDrawing` / OfficeArt を解析して図形名・グループ内の子図形名・テキスト・アンカーを返し、名前と位置を `.xlsx` / `.xlsm` / `.xlsb` と同じ結果契約に合わせる per FR-002, FR-005, US1/AC1, research §1 (partial)
- [ ] T043 OOXML・XLSB・BIFF の解析失敗を図形またはパート単位で隔離し、読取可能な Shape とセル結果を保つ。破損 Shape を含む入力のテストを追加する per FR-008, Constitution V, research §4 (partial)
- [ ] T044 OOXML・XLSB・BIFF の図形走査中もキャンセルを確認し、取得済みの結果を保って速やかに終了するテストを追加する per FR-008, SC-006, plan constraints (partial)
- [ ] T045 `.xlsx`・`.xlsm`・`.xls`・`.xlsb` の実 fixture と README を作成し、テキストボックス・通常図形・グループ子図形・同名・空図形・画像/グラフ除外・分割テキスト・破損入力の期待結果を形式ごとに検証する per SC-001, T001, T009 (missing)
- [ ] T046 `include_shape=false` で破損描画を開かず、セル・数式結果を維持すること、Shape/数式オプションが独立すること、UI 操作が検索要求へ反映されることを Rust と UI のテストで確認する per US2/AC1–3, T021, T022 (partial)
- [ ] T047 4形式について大文字小文字・正規表現・Hidden/VeryHidden の条件別 Shape 検索を統合テストし、UI の日英種別と実際の可視状態も確認する per FR-003, FR-004, FR-009, US3/AC1–3, T027, T029 (partial)
- [ ] T048 CSV / Excel の Shape 名列、実アンカーまたは空位置、全結果件数、日英の見出し・種別を画面結果と照合する出力テストを追加する per FR-007, US3/AC4, T028 (partial)
- [ ] T049 Shape の結果選択から3操作以内にファイル・シート・Shape 名・一致箇所を確認でき、アンカー有無双方でセルプレビューを呼ばないことを UI テストで確認する per FR-005, FR-006, SC-003, T010 (partial)
- [ ] T050 各ファイルに Shape ヒットを含む100ファイル・合計100 MB の計測手順とベンチマークを用意し、初回結果10秒以内・検索中の操作性を同一端末で計測して環境と結果を記録する per SC-006, T037, T038 (missing)
