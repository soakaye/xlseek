# Research & Technical Decisions: 並行ファイル走査・即時検索パイプラインと高速キャンセル応答 (007-parallel-scan-search)

本ドキュメントは、ディレクトリスキャンとファイル内容検索のパイプライン並行化、およびキャンセル即時応答化に関する技術調査、アーキテクチャ選定、および意思決定内容を記録する。

---

## 1. 並行パイプラインアーキテクチャの選定 (Producer-Consumer パターン)

### 課題
従来の検索エンジン（`SearchEngine::execute_search`）では、`WalkDir` による `collect_files` を完全に実行し終えて全ファイルリスト（`Vec<PathBuf>`）が確定した後に初めて `rayon` の並列ループが起動していた。これにより、10,000件以上のファイルが存在する巨大フォルダやネットワークドライブでは、検索結果が1件も表示されない待ち時間（数秒〜数十秒）が発生していた。

### 決定事項 (Decision)
ディレクトリスキャンを行うプロデューサースレッドと、`rayon` スレッドプールによる並列コンシューマーを、Rust標準ライブラリの有界同期チャネル `std::sync::mpsc::sync_channel(CHANNEL_BUFFER_SIZE)`（定数: 1024）で接続するプロデューサー・コンシューマー・パイプライン構造を採用する。

```text
[Producer Thread: Directory Scanner]
   WalkDir 巡回 (毎エントリ cancel_flag チェック)
         │  PathBuf (有界同期チャネル: buffer=1024)
         ▼
  std::sync::mpsc::sync_channel
         │
         ├── Worker 1 (rayon) ── parse_and_search_file ──> on_match / on_progress
         ├── Worker 2 (rayon) ── parse_and_search_file ──> on_match / on_progress
         └── Worker N (rayon) ── parse_and_search_file ──> on_match / on_progress
```

### 選定理由 (Rationale)
1. **メモリ安全性とバックプレッシャー**: 有界チャネル（bounded channel: 1024）を採用することで、超巨大ディレクトリ（10万件以上）の探索時でもメモリ上に未処理パスが無制限に溜まることを防ぎ、コンシューマーの消費速度に合わせてプロデューサーに適度なバックプレッシャーがかかる。
2. **依存関係の極小化 (YAGNI / 憲章準拠)**: 外部クレート（`crossbeam-channel` 等）を新規追加せず、Rust標準ライブラリ（`std::sync::mpsc`）のみで完結するため、バイナリサイズやビルド時間の増加を招かない。
3. **即時処理開始**: 最初のExcelファイルが検出された瞬間（ミリ秒オーダー）にワーカースレッドが取得して解析を開始するため、初回結果表示時間（TTFR）が劇的に短縮される。

### 検討した代替案 (Alternatives Considered)
- **代替案A: `crossbeam-channel` クレートの導入**:
  - 評価: MPMC（マルチプロデューサー・マルチコンシューマー）をネイティブサポートするが、本要件はシングルプロデューサー（スキャナー）であり、`Arc<Mutex<Receiver<PathBuf>>>` で十分高速（ロック時間はマイクロ秒未満、Excel解析はミリ秒単位のため競合オーバーヘッドは無視可能）。外部依存を増やさない標準ライブラリ採用が優位。
- **代替案B: `tokio` 非同期チャネルと非同期タスク**:
  - 評価: Excel解析はCPUバウンドおよびブロッキングI/O（`calamine`、ZIP伸長）であるため、`tokio` のグリーンスレッドではなく `rayon` のOSネイティヴスレッドプールで実行するのが最適。非同期ランタイムとの混合による不要な複雑化を回避。

---

## 2. ディレクトリ走査のキャンセル即時応答化 (Fast Cancellation)

### 課題
従来の `SearchEngine::collect_files`（`engine.rs:141`）は `WalkDir` のイテレータを最後まで回し切る実装となっており、ユーザーがUIから「キャンセル」を押下しても走査ループが即座に中断されず、数秒以上フリーズしたように見える問題があった。

### 決定事項 (Decision)
1. `WalkDir` の反復ループ内部で、毎エントリごとに `cancel_flag.load(Ordering::Relaxed)` をチェックし、キャンセル指示を検知した場合は直ちに `break` してループを終了する。
2. チャネル送信 `tx.send(path)` がエラーを返した場合（レシーバー側が破棄・キャンセルされた場合）も直ちに走査ループを脱出する。
3. `collect_files` 関数にも `cancel_flag: Option<&AtomicBool>` を引数として受け取れるようシグネチャを改修し、単独呼び出し時でも即時中断可能とする。

### 選定理由 (Rationale)
- `AtomicBool::load(Ordering::Relaxed)` は極めて低コスト（数CPUサイクル）であり、毎エントリ評価してもスキャン性能への悪影響は皆無である。
- ユーザーがキャンセルボタンを押した際、即座（1ms未満）に走査スレッドが終了するため、SC-002（1.0秒以内の完全停止）を確実に達成できる。

---

## 3. 並行進捗通知とUI連携 (Progress Dynamics)

### 課題
ファイル検出と検索が並行動作する場合、全ファイル検出が完了するまでは分母となる「総ファイル数」が未確定である。この未確定期間中に進捗バーをどのように描画するかの合意が必要。

### 決定事項 (Decision - Option A 採用)
1. **未確定時の進捗イベント**:
   - ディレクトリスキャン完了前（`scan_completed == false`）は、バックエンドから送信する `ScanProgress.total_files` を `0` とする。
   - `scanned_files`（走査済みファイル数）および `matches_found`（一致件数）は通常どおりリアルタイムにカウントアップ通知する。
2. **確定時の進捗イベント**:
   - ディレクトリスキャンスレッドが正常に探索を完了した時点で、確定した発見ファイル数（`total_discovered`）を `total_files` にセットし、進捗通知を送信する。
3. **フロントエンド描画 (`StatusBar.tsx` & `design/mainui/index.html`)**:
   - `isScanning && progress.total_files === 0` の場合:
     - プログレスバー: `animate-pulse` による検出中アニメーション（固定幅インジケータ）。
     - テキスト表示: `検出・走査中 (${progress.scanned_files} ファイル)`
   - `isScanning && progress.total_files > 0` の場合:
     - プログレスバー: `(scanned_files / total_files) * 100%` の確定進捗率バー。
     - テキスト表示: `${percent}% (${progress.scanned_files}/${progress.total_files})`

### 選定理由 (Rationale)
- ファイルが見つかるたびに分母を動的更新すると進捗率が「50% → 15% → 25%」のように逆戻りして不快な印象を与えるが、Option A では「探索・走査並行フェーズ」から「残余ファイル消化フェーズ」へと自然にUIが遷移するため、利用者に直感的な安心感を与える。

---

## 4. 定数一元管理設計 (憲章原則II準拠)

本改修で必要となる定数を整理し、ハードコードを一切排除する。

- **Rust バックエンド (`src-tauri/src/constants.rs`)**:
  - `CHANNEL_BUFFER_SIZE`: 有界チャネルのバッファ容量（1024）
  - `MSG_SCAN_DISCOVERING`: スキャン中の進捗メッセージ（"ファイルを検出・走査中..."）
- **TypeScript フロントエンド (`src/constants/index.ts`)**:
  - `STATUS_DISCOVERING_FILES`: "ファイルを検出・走査中..."
  - `STATUS_DISCOVERING_PREFIX`: "検出・走査中"
