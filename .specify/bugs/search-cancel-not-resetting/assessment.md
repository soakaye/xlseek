# Bug Assessment: CANCELボタンによる中断要求を行ってもSEARCH(START)ボタン表示に戻らない

- **Slug**: search-cancel-not-resetting
- **Created**: 2026-09-25T17:40:00+09:00
- **Source**: pasted text
- **Verdict**: valid
- **Severity**: high

## Report (verbatim or summarized)

CANCELボタンによる中断要求を行ってもSTARTボタン表示にならない

## Symptom

- 検索実行中（Scanning状態）にユーザーが「CANCEL」ボタンをクリックして検索中断を要求しても、ボタン表示が「SEARCH」（ユーザーの呼称: STARTボタン）に戻らず、「CANCEL」ボタンの表示のまま停止または固まる。
- 検索処理が即座に中断されず、バックグラウンドで走査が継続されるか、中断後のUI状態が更新されない。

## Reproduction

1. ファイル数の多いフォルダまたは大容量のExcelファイルが存在するディレクトリを対象に検索を実行する（「SEARCH」ボタンをクリック）。
2. スキャン実行中（プログレスバーが進行し、ボタンが赤色の「CANCEL」に切り替わっている状態）に、「CANCEL」ボタンをクリックする。
3. ボタンが緑色の「SEARCH」ボタン表示に戻らず、「CANCEL」ボタンのまま留まる。

## Suspected Code Paths

- `src/hooks/useSearch.ts:226-233`: `cancelSearch` 関数
  - `cancelSearch` 内で `invoke("cancel_search")` を呼び出しているが、フロントエンドの `progress` 状態を即時に更新していない。
- `src/hooks/useSearch.ts:78-95`: `scan-progress` イベントリスナー
  - キャンセル要求後も、バックエンドのワーカースレッドから遅延して届く `ScanState::Scanning` イベントをそのまま `setProgress` に反映してしまい、`isScanning` が `true` に再上書きされる。
- `src-tauri/src/search/engine.rs:133-187`: 並列走査ループ
  - `cancel_flag` のチェックがファイル単位（`par_iter` の先頭）でのみ行われており、現在パース処理中のスレッドは中断されない。
  - キャンセル後も遅延しているワーカースレッドが `ScanState::Scanning` の進捗通知を発行し続ける。
- `src-tauri/src/search/parser.rs:63-197`: `parse_and_search_file` 関数
  - キャンセルフラグが渡されておらず、巨大なワークシートや多数のシート・数式を含むブックのパース中に中断できない。

## Root Cause Hypothesis

**確信度: high (極めて高い)**

本障害には、フロントエンドの状態管理とバックエンドの並列処理キャンセルの両面に起因する複数の要因が存在します：

1. **フロントエンドの状態更新遅延と遅延イベントによる上書き (`useSearch.ts`)**:
   - `cancelSearch()` はバックエンドコマンド `cancel_search` を呼び出すのみで、ローカルの `progress` 状態を中断中/中断済みに即時遷移させていません。
   - バックエンドのワーカースレッドから遅延して届く `state: "Scanning"` の進捗イベントを無条件に受け取るため、中断操作後も `isScanning`（`progress?.state === "Scanning"`）が `true` に戻され、ボタンが「CANCEL」のままになります。
2. **バックエンドでのファイル内部パースの中断不能 (`parser.rs`, `engine.rs`)**:
   - `parse_and_search_file` にキャンセルフラグが渡されていないため、1ファイルあたりの読み込み（セル・数式の全件走査）が完了するまで各ワーカースレッドが停止しません。
   - Rayon の `par_iter().for_each` は実行中の全スレッドが完了するまで待機するため、中断処理全体の完了（`ScanState::Cancelled` の通知）が著しく遅延します。

## Proposed Remediation

**Preferred**:

1. **フロントエンドの即時中断反映と遅延イベント遮断 (`src/hooks/useSearch.ts`)**:
   - `isCancellingRef`（または中断中フラグ）を設け、`cancelSearch()` 実行時に即座に `progress.state` を `"Cancelled"` に更新し、`isScanning` を `false` に切り替えてボタンを「SEARCH」表示に戻す。
   - `scan-progress` イベントリスナーにて、キャンセル要求後に受信した `state === "Scanning"` の遅延イベントを破棄（無視）し、状態の巻き戻りを防ぐ。
2. **バックエンドでのきめ細やかなキャンセル判定 (`src-tauri/src/search/parser.rs`, `engine.rs`)**:
   - `parse_and_search_file` に `cancel_flag: Option<&AtomicBool>` を渡し、シートループおよびセル/数式走査ループの内部で定期的に中断を検知して即座に関数を抜ける。
   - `engine.rs` の進捗通知送信部で、`cancel_flag` が立っている場合は `ScanState::Scanning` の通知を抑止する。

**Alternatives**:
- フロントエンドのみで `cancelSearch` 時に強制的にボタン表示を切り替える案もあるが、バックエンドで走査が動き続けるとCPUリソースを浪費し、次の検索実行時にスレッドの競合が発生するため、バックエンドの早期中断と組み合わせることが必須である。

**Files likely to change**:
- `src/hooks/useSearch.ts`
- `src-tauri/src/search/engine.rs`
- `src-tauri/src/search/parser.rs`

**Tests to add or update**:
- `src-tauri/src/search/engine.rs`: キャンセルフラグを設定した際に走査が即座に中断される単体テスト。
- フロントエンドビルド（`npm run build`）およびRust単体テスト（`cargo test`）。

## Risks & Considerations

- `parse_and_search_file` のループ内で毎回アトミックフラグを読み出すと軽微なオーバーヘッドが生じる可能性があるため、数百行ごとのサンプリングまたはシート単位＋行インデックスの適度な間隔でチェックを行う。
- キャンセル直後に新規検索が開始された場合、前回のキャンセルスレッドと新しい検索スレッドが交錯しないよう、`SearchEngine` のキャンセル状態管理を整合させる。

## Open Questions

- なし（原因・再現経路・対策方針ともに特定完了）。
