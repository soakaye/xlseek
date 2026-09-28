# Bug Assessment: Preview Not Showing and Consecutive Emit Failed Events

- **Slug**: preview-not-showing-emit-failed
- **Created**: 2026-09-28T13:05:00+09:00
- **Source**: pasted text
- **Verdict**: valid
- **Severity**: high

## Report (verbatim or summarized)

```text
F:\Projects\Primary\FTEC\JRA着順制御システム\01_入手\20260611_テスト仕様書\08.各種テスト仕様書\01.各種テスト仕様書（８次着順システム　全体更新）\01.通信系制御PC\01_８次着順制御装置_通信系制御ＰＣ_ＰＴ項目_.xlsx　をプレビューしたとき表示されない。
Failed to emit Tauri event　が出続ける。
```

## Symptom

1. 大規模なExcelファイル（上記テスト仕様書等、700件以上のマッチが存在するファイル）を検索・プレビューしようとした際、プレビューペインにスプレッドシートや数式バーが表示されない（空欄になる）。
2. バックエンドの標準エラー出力（ターミナル）に `Failed to emit Tauri event` が連続して大量に出力され続ける。

## Reproduction

1. 検索対象ディレクトリに `F:\Projects\Primary\FTEC\JRA着順制御システム\01_入手\20260611_テスト仕様書\08.各種テスト仕様書\01.各種テスト仕様書（８次着順システム　全体更新）\01.通信系制御PC` を指定する。
2. キーワードに「通信」または「PT」を入力して検索を実行する。
3. 検索結果リストの上位に表示された項目（特に `match_type: Shape` の項目）を選択してプレビューを表示しようとする。
4. プレビューペインのグリッドが表示されず、ターミナルに `Failed to emit Tauri event` が出続けることを確認する。

## Suspected Code Paths

- `src-tauri/src/commands/search_cmd.rs:83-97` — `app_handle_match.emit` および `app_handle_prog.emit` でエラーが発生した際に `LOG_EVENT_EMIT_FAILED` を出力している箇所。1マッチごとに即座に個別 emit している。
- `src-tauri/src/search/engine.rs:376-380` — 1ファイル内で見つかった数百〜数千のマッチを同期ループ内で即座にコールバック呼び出ししている箇所。
- `src/hooks/useSearch.ts:217-224` — `handleSelectItem` にて `item.match_type === "Shape"` の場合に `setPreviewData(null)` となり、プレビュー読み込み（`loadPreview`）がスキップされる。
- `src/App.tsx:171-198` — `selectedMatch?.match_type !== "Shape"` のガードにより、Shape 選択時に `SpreadsheetGrid`（プレビューテーブル）、`FormulaBar`、`SheetTabs` がすべて非表示になる。

## Root Cause Hypothesis

信頼度: **High (高)**

原因は以下の2つの複合によるものである：

1. **WebView2 IPC メッセージキューの飽和による emit 失敗**:
   該当Excelファイルはシート数が多く（34シート）、キーワード「通信」等で検索した際に 1 ファイルから 700 件以上の大量マッチが発生する。
   `engine.rs` はファイル内の全マッチを一括で `on_match` に渡し、`search_cmd.rs` は各マッチに対して個別に `app_handle.emit("search-match", m)` を実行している。
   Tauri v2 の Windows 実装（WebView2）では `emit` のたびに内部で `ExecuteScriptAsync` (`eval`) を実行するため、短時間に数百回連続してスクリプト実行がキューイングされると WebView2 の IPC メッセージキューが飽和・破綻し、以降のすべての `emit` が失敗して `Failed to emit Tauri event` が出力され続ける。
2. **Shape 一致項目のプレビュー非表示仕様**:
   直近の PR #4（`012-search-shape-text`）にて、Shape 検索結果選択時は「セルプレビューを行わない」という仕様（`setPreviewData(null)` かつ `SpreadsheetGrid` 非表示）が追加された。
   該当ファイルでは表紙シート等のテキストボックス（Shape）にキーワードが含まれているため、検索結果の先頭に Shape の一致項目（例: 表紙 B9）が並ぶ。
   ユーザーがこの行を選択した際、グリッドやシートタブが一切描画されず、プレビューが表示されないと認識される。またアンカーが存在していても周辺セルのプレビューが確認できない。

## Proposed Remediation

**Preferred (推奨対応)**:
1. **イベント emit のバッチ化 / レートリミット導入**:
   - `search_cmd.rs` / `engine.rs` において、マッチ結果を 1 件ずつ個別に `emit` するのではなく、バッファリングして複数件（例: 20〜50件単位、または一定ミリ秒間隔）でバッチ送信（配列形式のペイロード）するか、フロントエンド側でバッファリングする前にバックエンド側で過剰なイベント連打を抑制する。
   - `app_handle.emit` の失敗時エラー（`e`）を握りつぶさず詳細にログ出力できるようにする。
2. **Shape 選択時のプレビュー体験の改善**:
   - アンカーセル（`cell_address`, `row_index`, `col_index`）が存在する Shape 一致項目については、そのアンカー位置を中心とした通常のセルプレビュー（`get_cell_preview`）を読み込んでグリッドを表示する。
   - アンカーセルが存在しない Shape の場合でも、プレビュー領域に「図形内テキストのためセル配置情報はありません」といった分かりやすいガイダンスと該当シートのプレビューまたはテキスト全文を表示し、真っ黒な空欄表示にならないようにする。

**Files likely to change**:
- `src-tauri/src/commands/search_cmd.rs`
- `src-tauri/src/search/engine.rs`
- `src/hooks/useSearch.ts`
- `src/App.tsx`
- `src/components/preview/SpreadsheetGrid.tsx`

**Tests to add or update**:
- `src-tauri/tests/shape_preview.rs` (または既存テスト): 大量マッチ発生時のバッチ emit 動作とアンカー付き Shape のセルプレビュー取得検証。
- `tests/shape-results-ui.test.tsx`: アンカー付き Shape でセルプレビューが正常に表示されることの検証。

## Risks & Considerations

- バッチ emit 化に伴い、フロントエンド側の `listen<SearchMatch[]>` への型対応が必要になる可能性がある（後方互換性またはバッチ/単一受信用のアダプタ）。
- 既存の Shape 仕様（T019: 「セルプレビューを呼ばない」とされていた仕様）との整合性を再設計し、アンカー付きの場合はプレビューを許容するように拡張する必要がある。

## Open Questions

- なし（原因・現象ともにコードベースおよび実ファイルで検証済み）。
