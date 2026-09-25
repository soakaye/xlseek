# 実装計画書: Excel Grep デスクトップアプリケーション (Implementation Plan)

**ブランチ**: `001-excel-grep-app` | **作成日**: 2026-09-25 | **仕様書**: [spec.md](./spec.md)

**入力**: 機能仕様書 `/specs/001-excel-grep-app/spec.md` (技術スタック指定: Tauri + Rust)

---

## 概要 (Summary)

本機能は、大量の Excel ブック（.xlsx, .xlsm, .xlsb, .xls）から指定したキーワードや正規表現に一致するセル・数式・コメントを瞬時に横断検索し、直感的な 2 ペイン画面でスプレッドシートプレビューや外部連携を行うデスクトップアプリケーションです。
バックエンドに **Rust**（`calamine`, `rayon`, `regex`）を採用して圧倒的な並列処理速度と安全な中断制御を実現し、フロントエンドに **Tauri v2** + **TypeScript** / **React** / **Tailwind CSS** + **仮想スクロール** を採用して、10,000 件規模の検索結果でも軽快に動作する UI を提供します。

---

## 技術コンテキスト (Technical Context)

- **言語 / バージョン**: Rust 1.77+ (1.98.1 動作確認済み), TypeScript 5.5, Node.js 18+ (v24.16.0 動作確認済み)
- **主要依存ライブラリ**:
  - *Backend (Rust)*: `tauri` (v2), `calamine` (v0.26+), `rayon` (v1.10+), `regex` (v1.10+), `walkdir` (v2.5+), `serde` / `serde_json`, `csv`, `rust_xlsxwriter`, `open`
  - *Frontend (Web)*: `react` (v18), `@tanstack/react-virtual` (v3), `tailwindcss` (v3), `lucide-react`
- **ストレージ**: なし（ステートレスなローカルファイル検索。結果エクスポート時に CSV/XLSX ファイルを直接書き込み）
- **テスト**: `cargo test` (Rust 側の Excel パース・検索・周辺抽出ロジック単体テスト), Vitest / React Testing Library (フロントエンドコンポーネント)
- **ターゲット環境**: Windows 10/11 (WebView2 ランタイム利用), 将来的なクロスプラットフォーム展開 (macOS/Linux) 容易な構造
- **プロジェクト種別**: デスクトップアプリケーション (Tauri v2)
- **パフォーマンス目標**:
  - 100 ファイル（合計 100MB 規模）の走査開始から初期結果表示まで数秒以内
  - 10,000 件規模の検索結果一覧でも仮想スクロールにより 60fps の滑らかな描画
  - セル選択から右ペインのプレビュー更新まで 100 ミリ秒以内
- **制約事項**:
  - 巨大なドライブ指定時でも UI がフリーズしない非同期並列処理
  - ユーザーによる「CANCEL」操作時の安全かつ即時なスレッド停止と結果保持
  - ウィンドウ幅縮小時にもボタン欠け・パス重なり・右辺ボーダー欠けを生じさせないレスポンシブ設計（`design/mainui/index.html` 準拠）
- **規模 / スコープ**: 4 種類の主要 Excel 拡張子走査、最大 10,000 件のストリーミング表示、前後 3 行・前後 2 列の周辺コンテキスト抽出

---

## 憲章・原則チェック (Constitution Check)

*GATE: Phase 0 の調査前に検証済み。Phase 1 設計完了後も再評価済み。*

1. **モジュール性と疎結合**: コア検索ロジック（`search` クレート/モジュール）は GUI から独立して単体テスト・ベンチマーク可能。
2. **パフォーマンス・ファースト**: メモリコピーを最小化し、シート全読み込みではなく周辺セルのバウンディングボックス抽出を採用。
3. **テスト駆動・品質保証**: テスト用 Excel フィクスチャ（.xlsx, .xlsm, .xlsb）を用いた受入テストを容易に実施可能。
4. **設計ゲート**: すべてのゲートをパス（違反・例外なし）。

---

## プロジェクト構造 (Project Structure)

### ドキュメント構成 (この機能)

```text
specs/001-excel-grep-app/
├── spec.md              # 機能仕様書 (基本要件・明確化事項 Q&A)
├── checklists/
│   └── requirements.md  # 仕様品質検証チェックリスト (全16項目パス)
├── plan.md              # 実装計画書 (本ドキュメント)
├── research.md          # Phase 0: 技術調査・設計決定・代替案比較
├── data-model.md        # Phase 1: データ構造・エンティティ・状態遷移
├── contracts/           # Phase 1: Tauri IPC API コマンド・イベント契約
│   └── tauri-ipc-api.json
└── quickstart.md        # Phase 1: E2E 検証・クイックスタートガイド
```

### ソースコード配置 (リポジトリルート)

```text
src/                         # React / TypeScript フロントエンド
├── components/
│   ├── layout/              # ウィンドウタイトルバー, 2ペインコンテナ, フッター
│   ├── search/              # キーワード入力, フォルダ指定, オプショントグル, SEARCH/CANCELボタン
│   ├── results/             # 仮想スクロール結果一覧テーブル, クイックフィルタ, 件数バッジ
│   └── preview/             # プレビューヘッダー (パス専用行含む), 数式バー, ワークシート (sticky), シートタブ
├── hooks/
│   ├── useSearch.ts         # 検索開始・キャンセル・ストリーミング状態管理
│   ├── useCellPreview.ts    # 選択セル周辺データの取得とキャッシュ
│   └── useExport.ts         # CSV / Excel エクスポート実行
├── types/
│   └── search.ts            # SearchQuery, SearchMatch, CellPreviewData 型定義
├── App.tsx                  # メイン画面組み立て
└── main.tsx                 # フロントエンドエントリポイント

src-tauri/                   # Rust バックエンド
├── src/
│   ├── search/              # Excel パース & Grep コア並列検索エンジン
│   │   ├── mod.rs
│   │   ├── engine.rs        # rayon 並列走査 & AtomicBool キャンセル制御
│   │   ├── parser.rs        # calamine によるブック読み込み・セル値/数式/コメント走査
│   │   └── preview.rs       # 前後3行・前後2列の周辺セル高速抽出ロジック
│   ├── export/              # CSV (BOM付き) / XLSX 出力処理
│   │   ├── mod.rs
│   │   ├── csv_export.rs
│   │   └── xlsx_export.rs
│   ├── commands/            # Tauri IPC コマンド群
│   │   ├── mod.rs
│   │   ├── search_cmd.rs    # start_search, cancel_search
│   │   ├── preview_cmd.rs   # get_cell_preview
│   │   ├── system_cmd.rs    # open_in_excel, open_in_folder
│   │   └── export_cmd.rs    # export_results
│   ├── models/              # データ構造体定義 (Serialize / Deserialize)
│   │   └── mod.rs
│   ├── lib.rs               # Tauri プラグイン初期化 & コマンド登録
│   └── main.rs              # デスクトップエントリポイント
├── Cargo.toml               # Rust 依存クレート定義
└── tauri.conf.json          # Tauri v2 アプリケーション設定 (ウィンドウサイズ・権限設定)

tests/                       # テストコード
├── fixtures/                # テスト用 Excel ファイル群 (.xlsx, .xlsm, .xlsb)
├── search_test.rs           # 検索エンジン単体・統合テスト
└── preview_test.rs          # 周辺セル抽出ロジックのテスト
```

**構造決定**: Tauri v2 の公式推奨構造（`src/` + `src-tauri/`）を採用し、UI ロジックと高負荷なファイルパースエンジンを明確に分離します。

---

## 複雑性トラッキング (Complexity Tracking)

> 憲章違反や不要な複雑性はありません（すべての決定が要件に直結）。

| 決定事項 | なぜ必要か | 代替案を却下した理由 |
|:---|:---|:---|
| `rayon` によるマルチスレッド | 大量・大容量 Excel の高速走査 | シングルスレッドでは数分以上要し Grep ツールとして実用不可 |
| 周辺セルの局所抽出 | プレビュー切り替えの即時性 (100ms以内) | シート全体の全セル転送は IPC と描画のボトルネックになる |
| 仮想スクロール (`@tanstack/react-virtual`) | 10,000 件規模の結果テーブル表示 | 通常の DOM 描画では数百件以上でスクロールがカクつく |
