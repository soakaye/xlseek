# 実装計画書: xlsx/xls 登録アプリおよびサポートアプリ起動・フォルダDnD対応 (Implementation Plan)

**ブランチ**: `002-launch-associated-app` | **作成日**: 2026-09-25 | **仕様書**: [spec.md](./spec.md)

**入力**: 機能仕様書 `/specs/002-launch-associated-app/spec.md` (技術スタック: Tauri v2 + Rust + React / TypeScript)

---

## 概要 (Summary)

本機能は、プレビュー領域に配置された従来の「Excel で開く」固定ボタンを刷新し、OS に登録された既定の表計算アプリケーションで起動するスプリットボタン（固定ラベル: 「アプリで開く」＋ツールチップでの既定アプリ名表示）へ変更するとともに、ドロップダウンメニューからシステムにインストールされている `.xlsx` / `.xls` サポートアプリケーション一覧の選択起動および Windows 標準「別のプログラムを選択...」の呼び出しを可能にします。
あわせて、検索バーのフォルダ選択入力枠に対するドラッグ＆ドロップ（DnD）操作を受け入れ、エクスプローラーからドロップされたフォルダ（またはファイルが属する親フォルダ）を即座に設定できる直感的な操作性を提供します。

---

## 技術コンテキスト (Technical Context)

- **言語 / バージョン**: Rust 1.77+ (1.98.1), TypeScript 5.5, Node.js 18+ (v24.16.0)
- **主要依存ライブラリ**:
  - *Backend (Rust)*: `tauri` (v2), `winreg` (v0.55), `open` (v5.3), `serde` / `serde_json`, `std::process::Command`
  - *Frontend (Web)*: `react` (v18), `lucide-react`, `tailwindcss` (v3), `@tauri-apps/api` (v2: `webview`, `core`)
- **ストレージ**: なし（OS レジストリおよびシェル関連付け情報を参照）
- **テスト**: `cargo test` (レジストリ読み出し・パス解決ロジック), コンポーネント検証
- **ターゲット環境**: Windows 10 / 11 (WebView2 ランタイム)
- **プロジェクト種別**: デスクトップアプリケーション (Tauri v2)
- **パフォーマンス目標**:
  - サポートアプリ一覧の取得およびポップアップメニューの表示: 300 ミリ秒以内
  - フォルダDnD時のドラッグオーバー・ドロップ検出: 即時（遅延なし）
- **制約事項**:
  - ユーザー環境に Office/Excel がインストールされていなくても安全にフォールバック表示・トースト通知
  - ポップアップメニューのウィンドウ端での見切れ防止
  - フォルダドロップ時はパス設定のみにとどめ、意図しない重い検索の自動開始を防止
- **規模 / スコープ**: スプリットボタンUI刷新、レジストリ走査によるアプリ列挙、OpenWithダイアログ呼び出し、ネイティブDnD連携

---

## 憲章・原則チェック (Constitution Check)

*GATE: Phase 0 の調査前に検証済み。Phase 1 設計完了後も再評価済み。*

1. **モジュール性と疎結合**: Windows 固有のレジストリ走査・アプリ起動ロジックは `system_cmd` モジュール内にカプセル化し、UIコンポーネント（`PreviewHeader`, `SearchBar`）からは Tauri IPC 経由でクリーンに呼び出す。
2. **パフォーマンス・ファースト**: レジストリ走査は軽量なキー読み出しのみとし、メニュー表示の遅延を 300ms 以内に抑制。
3. **安全性とクラッシュ防止**: アプリ起動失敗や権限エラーは Result 型で補足し、アプリ全体がクラッシュせずトースト通知でユーザーにフィードバック。
4. **設計ゲート**: すべてのゲートをパス（違反・例外なし）。

---

## プロジェクト構造 (Project Structure)

### ドキュメント構成 (この機能)

```text
specs/002-launch-associated-app/
├── spec.md              # 機能仕様書 (基本要件・明確化事項 Q&A)
├── checklists/
│   └── requirements.md  # 仕様品質検証チェックリスト (全16項目パス)
├── plan.md              # 実装計画書 (本ドキュメント)
├── research.md          # Phase 0: 技術調査・設計決定・代替案比較
├── data-model.md        # Phase 1: データ構造・エンティティ・状態遷移
├── contracts/           # Phase 1: Tauri IPC API コマンド契約
│   └── tauri-ipc-api.json
└── quickstart.md        # Phase 1: E2E 検証・クイックスタートガイド
```

### ソースコード配置 (変更対象)

```text
src/                         # React / TypeScript フロントエンド
├── components/
│   ├── preview/
│   │   └── PreviewHeader.tsx # スプリットボタン化（アプリで開く + ▼ドロップダウン）、ポップアップメニュー実装
│   └── search/
│       └── SearchBar.tsx     # フォルダ選択領域への DnD イベント購読 & 破線ハイライト演出
└── types/
    └── search.ts            # SupportedApp 型定義の追加

src-tauri/                   # Rust バックエンド
├── Cargo.toml               # winreg 依存クレートの追加（Windows用）
└── src/
    ├── commands/
    │   ├── system_cmd.rs    # get_supported_apps, launch_associated_app, show_open_with_dialog, resolve_dropped_path
    │   └── mod.rs
    └── lib.rs               # invoke_handler への新規コマンド登録
```

**構造決定**: 既存の Tauri v2 アーキテクチャに従い、`system_cmd.rs` にバックエンド処理を集約し、フロントエンドは `PreviewHeader` と `SearchBar` の更新で完結させます。

---

## 複雑性トラッキング (Complexity Tracking)

> 憲章違反や不要な複雑性はありません。

| 決定事項 | なぜ必要か | 代替案を却下した理由 |
|:---|:---|:---|
| スプリットボタンUI | 1クリックで既定起動し、▼で任意アプリを選べる | 通常のドロップダウンのみにすると毎回2クリック必要になり操作性が低下するため |
| `winreg` による直接走査 | 300ms以内の高速なサポートアプリ列挙 | PowerShell 起動方式は 500ms〜1秒の遅延が生じるため |
| `rundll32 shell32.dll` 呼び出し | Windows標準の「プログラムから開く」UI表示 | サードパーティライブラリ導入を避け、OS標準機能を安全に活用するため |
| フォルダDnD時の自動検索抑制 | 誤ったフォルダ投入による検索暴走の防止 | 即時開始は巨大フォルダ指定時にユーザーの意図に反して重い処理が走るため |
