# Implementation Plan: 並行ファイル走査・即時検索パイプラインと高速キャンセル応答 (007-parallel-scan-search)

**Branch**: `007-parallel-scan-search` | **Date**: 2026-09-26 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/007-parallel-scan-search/spec.md`

---

## Summary

ディレクトリスキャン開始から検索開始までの待機時間（TTFR: Time-to-First-Result）を最小化するため、ファイルリスト取得（`collect_files`）とファイル内容検索（`execute_search`）が順次実行されていた従来の構造を刷新する。

Rust標準ライブラリの有界同期チャネル（`std::sync::mpsc::sync_channel`）によるプロデューサー・コンシューマー・パイプラインを構築し、ディレクトリスキャナースレッドが対象Excelファイルを検出した瞬間に、`rayon` 並列ワーカー群が即時解析・検索を開始できるようにする。

また、`WalkDir` 反復ループにおいて各エントリごとにアトミック中断フラグ（`cancel_flag`）を評価し、キャンセル指定時は走査およびファイル検索の双方をミリ秒オーダーで即時中断可能にする。

フロントエンドのステータスバー（`StatusBar.tsx`）およびスタンドアローンHTMLモック（`design/mainui/index.html`）において、ディレクトリスキャン進行中の未確定期間は検出中アニメーションおよび走査済ファイル数を表示し、スキャン完了後に確定総数に基づく進捗率表示へとスムーズに移行するUI連携を実装する。

---

## Technical Context

**Language/Version**: Rust 2021 (Tauri v2), TypeScript 5.5 (React 18.3)

**Primary Dependencies**:
- バックエンド: `walkdir` (2.5), `rayon` (1.10), `calamine` (0.36), `std::sync::mpsc` (Rust標準ライブラリ)
- フロントエンド: React 18.3, Tailwind CSS 3.4, `@tauri-apps/api` (v2), Lucide React

**Storage**: N/A (メモリ上でのストリーミングパイプライン処理)

**Testing**:
- バックエンド単体テスト: `cargo test` (`src-tauri` 配下、パイプライン並行動作テストおよび即時中断テスト)
- バックエンド静的解析: `cargo clippy --all-targets -- -D warnings`, `cargo fmt --check`
- フロントエンド型検証 & ビルド: `npm run build` (`tsc --noEmit` & `vite build`)
- スタンドアローンHTMLモック検証: `design/mainui/index.html` 手動動作確認

**Target Platform**: macOS (WebKit), Windows (WebView2), Linux

**Project Type**: デスクトップGUIアプリケーション（Tauri v2）およびスタンドアローンプロトタイプ

**Performance Goals**:
- 大規模フォルダ（10,000+ ファイル）での初回結果表示時間（TTFR）80%以上短縮 (SC-001)
- 検索中断要求（キャンセル）の応答時間 < 1.0 秒（通常 500ms 未満） (SC-002)
- 並行スキャン中のUIスレッド描画 60fps 維持 (SC-003)

**Constraints**:
- 憲章原則I（自然かつ正確な日本語ログ・メッセージ・UI通知）
- 憲章原則II（定数値の完全外部抽出。バッファサイズやステータス文言の一元管理）
- 憲章原則III（網羅的な4要素ヘッダコメントの付与）
- 憲章原則IV（責務に応じたモジュール分割、標準ライブラリ優先の設計）
- 憲章原則V（パニックフリー設計、`catch_unwind` による保護、単体テスト網羅）
- スタンドアローンHTMLモック（`design/mainui/index.html`）との完全同期（Iron Law）

**Scale/Scope**:
- 単一フォルダから10万件以上のファイルを含む深い階層構造まで安定動作
- 同時オープンファイル数の制御（有界チャネル容量: 1024）

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| 憲章原則 | 準拠状況 | 評価詳細 |
|---|---|---|
| **I. 自然かつ正確な日本語出力** | **PASS** | UIメッセージ（「検出・走査中」「スキャン中」「中断」等）、エラー通知、ログ出力、コメントはすべて自然かつ正確な日本語で統一。不要トークンを排除。 |
| **II. 定数の外部抽出とハードコードの禁止** | **PASS** | チャネルバッファサイズ（`CHANNEL_BUFFER_SIZE`）やUI表示定数はすべて `constants.rs` および `src/constants/index.ts` へ定義し、利用箇所で定数参照コメントを明記。 |
| **III. 厳格なヘッダコメントとドキュメンテーション** | **PASS** | 変更・新設するすべての関数（`execute_search`, `collect_files`, `StatusBar` 等）に処理内容、引数/戻り値型、エラー/例外、変更履歴を含む4要素ヘッダコメントを付与。 |
| **IV. 責務に応じたモジュール分割と標準スタイル準拠** | **PASS** | スキャン（プロデューサー）と検索（コンシューマー）の責務を綺麗に分離し、Rust公式スタイルガイドおよび `clippy` に完全準拠。新規の不要クレートを追加せず標準ライブラリを活用。 |
| **V. 堅牢なエラーハンドリングとテスト検証** | **PASS** | チャネル切断、ファイル読み取り権限エラー、破損ファイルによるパニックを安全に捕捉・スキップし、パイプライン並行テストとキャンセル即時中断テストを実装。 |

---

## Project Structure

### Documentation (this feature)

```text
specs/007-parallel-scan-search/
├── spec.md              # 仕様書（/speckit-specify および /speckit-clarify 出力）
├── plan.md              # 実装計画書（本ファイル: /speckit-plan 出力）
├── research.md          # Phase 0: 技術調査と並行アーキテクチャ選定
├── data-model.md        # Phase 1: データモデル・状態遷移図
├── quickstart.md        # Phase 1: テスト検証シナリオと実行手順
├── contracts/           # Phase 1: インターフェース契約仕様
│   └── search-progress-contract.md
└── checklists/
    └── requirements.md  # 仕様品質検証チェックリスト
```

### Source Code Layout

```text
src-tauri/
├── src/
│   ├── constants.rs             # CHANNEL_BUFFER_SIZE, メッセージ定数追加
│   └── search/
│       └── engine.rs            # collect_files のキャンセル対応 & execute_search パイプライン化

src/
├── constants/
│   └── index.ts                 # STATUS_DISCOVERING_FILES 等の定数追加
└── components/
    └── common/
        └── StatusBar.tsx        # total_files===0 時の検出中アニメーション & テキスト表示更新

design/
└── mainui/
    └── index.html               # スタンドアローンHTMLモック同期（ステータスバー挙動）
```

---

## Complexity Tracking

*Constitution Check において違反なし（該当項目なし）。*
