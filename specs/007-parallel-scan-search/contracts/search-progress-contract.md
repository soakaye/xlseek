# Interface Contract: 検索進捗イベントおよび検索エンジン仕様 (007-parallel-scan-search)

本ドキュメントは、バックエンド（Rust）とフロントエンド（React / TypeScript）間で交わされる検索進捗イベント契約、および検索エンジンの公開メソッド仕様を定義する。

---

## 1. Tauri IPC イベント契約: `scan-progress`

バックエンドからフロントエンドへ送信される進捗イベントペイロード仕様。

### イベント名
`scan-progress`（定数: `crate::constants::EVENT_SCAN_PROGRESS` / `src/constants/index.ts:EVENTS.SCAN_PROGRESS`）

### ペイロードスキーマ (TypeScript)

```typescript
export type ScanState = "Scanning" | "Completed" | "Cancelled" | "Error";

export interface ScanProgress {
  /** 検索の現在状態 */
  state: ScanState;
  /** これまでに検索走査が完了したファイル数 */
  scanned_files: number;
  /** 対象ファイル総数。ディレクトリスキャン完了前は 0、完了後は確定件数 */
  total_files: number;
  /** 発見された一致セルの総件数 */
  matches_found: number;
  /** 現在解析中のファイル名、またはステータスメッセージ */
  current_file: string;
  /** 検索開始からの経過時間（ミリ秒） */
  elapsed_ms: number;
}
```

### イベント送信タイミング契約

| 状況 | `state` | `total_files` | `scanned_files` | UI側の振る舞い |
| :--- | :--- | :--- | :--- | :--- |
| **検索開始直後** | `Scanning` | `0` | `0` | プログレスバーにアニメーションパルス表示、「ディレクトリを走査中...」 |
| **スキャン＆検索並行中** | `Scanning` | `0` | `1..N` | プログレスバーにアニメーションパルス表示、「検出・走査中 (N ファイル)」 |
| **スキャン完了時** | `Scanning` | `Total` | `N` | プログレスバーが正規進捗率（`N/Total`）に切り替わる |
| **検索完了時** | `Completed` | `Total` | `Total` | 進捗率 100%、完了メッセージと結果件数表示 |
| **キャンセル時** | `Cancelled` | 任意 | 中断時点の件数 | キャンセル状態表示、中断時点の結果を保持して入力復帰 |
| **エラー時** | `Error` | `0` | `0` | エラーメッセージ表示 |

---

### 1.1 Tauri IPC イベント契約: `search-match` (バッチ配信仕様)

大量マッチ発生時（1ファイルあたり数百〜数千件のヒット）に、同期的な単一イベント連打によって Windows (WebView2) の IPC メッセージキューが飽和し、`Failed to emit Tauri event` が連続発生して検索・プレビューが破綻することを防止するためのバッチ配信契約。

#### イベント名
`search-match`（定数: `crate::constants::EVENT_SEARCH_MATCH` / `src/constants/index.ts:EVENT_NAMES.SEARCH_MATCH`）

#### ペイロードスキーマ (TypeScript)
`SearchMatch | SearchMatch[]` (後方互換性のため単一および配列の双方を受容)

#### バックエンド送信制御仕様 (`src-tauri/src/commands/search_cmd.rs`)
- マッチ結果はスレッドセーフなバッファに蓄積される。
- 以下のいずれかの条件を満たした時点でバッチ送信（`app_handle.emit`）を実行する：
  1. バッファ内の件数が定数 `SEARCH_MATCH_BATCH_SIZE`（50件）に達したとき
  2. 前回の送信から定数 `SEARCH_MATCH_BATCH_INTERVAL_MS`（25ミリ秒）以上が経過したとき
- 検索処理完了時（正常完了またはエラー終了時）には、バッファ内に残存する全件を即座に強制フラッシュ（送信）する。

#### フロントエンド受信処理仕様 (`src/hooks/useSearch.ts`)
- リスナーコールバックにて `Array.isArray(event.payload)` を判定し、配列の場合は一括展開して結果状態（`results`, `resultCount`）へマージする。

---

## 2. 検索エンジン内部インターフェース仕様 (`src-tauri/src/search/engine.rs`)

### 2.1 `collect_files`

```rust
pub fn collect_files(
    target_dir: &str,
    extensions: &[String],
    cancel_flag: Option<&AtomicBool>,
) -> Vec<PathBuf>
```
- **処理内容**: 対象ディレクトリを再帰走査し、指定拡張子に合致するファイル一覧を収集する。`cancel_flag` が指定され、その値が `true` となった場合は即座に走査を終了して収集済みパスを返却する。
- **一時ファイル除外**: `~$` で始まるExcelロックファイルは除外する。

### 2.2 `execute_search`

```rust
pub fn execute_search<FMatch, FProgress>(
    &self,
    query: SearchQuery,
    on_match: FMatch,
    on_progress: FProgress,
) -> Result<ScanProgress, String>
where
    FMatch: FnMut(SearchMatch) + Send + Sync + 'static,
    FProgress: FnMut(ScanProgress) + Send + Sync + 'static,
```
- **処理内容**:
  1. ディレクトリスキャナースレッドを起動し、有界同期チャネル（`sync_channel(1024)`）へ検出パスを順次送信。
  2. Rayon スレッドプールにより並列ワーカーがチャネルからパスを受信し、即座に Excel ファイル解析・検索を開始。
  3. 各エントリ走査時および各ファイル/シート走査時に `cancel_flag` をチェックし、中断要求をミリ秒オーダーで即座に反映。
