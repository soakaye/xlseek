# 実装計画書: マルチプラットフォーム対応 (macOS対応・システム標準タイトルバー・OS固有機能の同等実装) (Implementation Plan)

**ブランチ**: `003-macos-support` | **作成日**: 2026-09-25 | **仕様書**: [spec.md](./spec.md)

**入力**: 機能仕様書 `/specs/003-macos-support/spec.md` (技術スタック: Tauri v2 + Rust + React / TypeScript)

---

## 概要 (Summary)

本機能は、アプリケーションのマルチプラットフォーム化を推進し、macOS への正式対応を実現するとともに、全プラットフォーム（macOS および Windows）において OS ネイティブの標準ウィンドウ装飾（タイトルバーおよびウィンドウコントロール）への移行を行います。
フロントエンドの独自カスタムタイトルバーとエミュレートされた操作ボタンを廃止し、OS ネイティブのウィンドウ枠（macOS では左側トラフィックライト、Windows では右側標準ボタン）に一本化します。
また、macOS 固有のシステム連携（Finder でのファイル選択表示、LaunchServices / アプリバンドル走査による表計算アプリ検出、`/Applications` アプリピッカーダイアログ、macOS システムメニューバーと標準ショートカット `⌘C`/`⌘V`/`⌘Q`）を Rust の条件コンパイル（`#[cfg(target_os = ...)]`）により安全に実装し、既存の Windows 向け実装（機能 001/002）に一切のリグレッションを与えることなく完全な機能同等性を提供します。

---

## 技術コンテキスト (Technical Context)

- **言語 / バージョン**: Rust 1.77+ (1.98.1), TypeScript 5.5, Node.js 18+ (v24.16.0)
- **主要依存ライブラリ**:
  - *Backend (Rust)*: `tauri` (v2), `tauri-plugin-dialog` (v2), `tauri-plugin-shell` (v2), `open` (v5.3), `serde` / `serde_json`, `calamine`, `walkdir`, `rust_xlsxwriter`, `winreg` (Windows専用: `[target.'cfg(windows)'.dependencies]`)
  - *Frontend (Web)*: `react` (v18), `lucide-react`, `tailwindcss` (v3), `@tauri-apps/api` (v2)
- **ストレージ**: なし（OS アプリケーションバンドルおよびファイルシステムを参照）
- **テスト**: `cargo test`, `cargo check`, フロントエンドコンポーネント動作確認
- **ターゲット環境**: macOS 12 Monterey 以降 (Apple Silicon & Intel), Windows 10 / 11 (WebView2)
- **プロジェクト種別**: マルチプラットフォーム・デスクトップアプリケーション (Tauri v2)
- **パフォーマンス目標**:
  - macOS サポートアプリ一覧の検出およびメニュー表示: 300 ミリ秒以内
  - 「フォルダを開く」による Finder 選択表示: 1 秒以内
  - ウィンドウのドラッグ・リサイズ・ズーム: OS ネイティブ描画による 60fps / 遅延なし
- **制約事項**:
  - Windows 既存機能（レジストリ読み出し、エクスプローラー選択表示等）の動作と互換性を 100% 保持（リグレッション 0 件）
  - macOS 特有のパス表現（POSIX、UTF-8、空白文字等）における安全なファイル・フォルダ操作
  - macOS WKWebView における標準キーボードショートカット（`⌘C`, `⌘V`, `⌘A`, `⌘Z`, `⌘W`, `⌘Q`）の完全動作
- **規模 / スコープ**:
  - `tauri.conf.json` での `decorations: true` 有効化
  - `WindowFrame.tsx` の独自タイトルバー撤去および `StatusBar.tsx` への補助情報再配置
  - `lib.rs` での macOS 向けシステムメニューバー構築
  - `system_cmd.rs` における macOS 向け `open_in_folder`, `get_supported_apps`, `launch_associated_app`, `show_open_with_dialog` の条件コンパイル実装

---

## 憲章・原則チェック (Constitution Check)

*GATE: Phase 0 の調査前に検証済み。Phase 1 設計完了後も再評価済み。*

1. **マルチプラットフォーム統一性とプラットフォーム最適化**:
   - フロントエンドとの IPC インターフェース（契約）は共通化し、OS 固有の実装は Rust バックエンドの `system_cmd.rs` にカプセル化。
2. **OS ネイティブ作法への適合**:
   - 独自の擬似タイトルバーを全廃し、OS 標準のウィンドウマネージャーに委譲することで、最高水準の安定性と操作感を実現。
3. **安全性とクラッシュ防止**:
   - アプリケーション起動やファイルマネージャー呼び出しの失敗は Result 型で補足し、エラー時もアプリが異常終了せずトースト通知で通知。
4. **設計ゲート**: すべてのゲートをパス（違反・例外なし）。

---

## プロジェクト構造 (Project Structure)

### ドキュメント構成 (この機能)

```text
specs/003-macos-support/
├── spec.md              # 機能仕様書 (基本要件・Clarifications Q&A 3問合意済み)
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
│   ├── layout/
│   │   └── WindowFrame.tsx   # 独自タイトルバー・独自ボタン撤去、レイアウト調整
│   ├── common/
│   │   └── StatusBar.tsx     # Calamine Engine バッジおよびバージョン表記の再配置
│   └── preview/
│       └── PreviewHeader.tsx # Numbers 向けアイコンヒント対応
src-tauri/                   # Rust バックエンド
├── tauri.conf.json          # decorations: true 有効化
└── src/
    ├── lib.rs               # macOS 向けネイティブメニューバー登録
    └── commands/
        └── system_cmd.rs    # macOS 向け Finder 選択、アプリ列挙、アプリ選択ダイアログの追加
```

**構造決定**:
既存の Tauri v2 アーキテクチャをそのまま活用し、`system_cmd.rs` に OS 条件コンパイル（`#[cfg(target_os = "windows")]` / `#[cfg(target_os = "macos")]`）を集約。フロントエンドは `WindowFrame.tsx` と `StatusBar.tsx` の更新にとどめることで最小限の差分で高信頼なマルチプラットフォーム対応を実現します。

---

## 複雑性トラッキング (Complexity Tracking)

> 憲章違反や不要な複雑性はありません。

| 決定事項 | なぜ必要か | 代替案を却下した理由 |
|:---|:---|:---|
| `decorations: true` への統一 | macOS/Windows 双方で OS ネイティブの操作感と描画品質を担保 | Webview 内での各 OS 風ボタンの自作エミュレーションは保守性が低く不自然さが残るため |
| `open -R` による Finder 連携 | Windows のエクスプローラー選択表示と同等の体験を提供 | AppleScript（`osascript`）は実行オーバーヘッドと権限警告のリスクがあるため |
| アプリバンドル走査によるアプリ検出 | macOS での表計算ソフトを高速かつ確実に列挙 | LaunchServices の Objective-C C-API 直接呼び出しは unsafe コードや依存が増加するため |
| macOS 標準メニューバーの有効化 | WKWebView での `⌘C`/`⌘V` などの標準編集ショートカット動作を保証 | JavaScript レベルのキー監視では `⌘Q` やシステム統合ショートカットを補完できないため |
| 補助情報のステータスバー移動 | ネイティブタイトルバー化後もエンジン名・バージョンを視認可能にする | タイトルバーを無理に透過（Overlay）させるとレイアウトの破綻リスクが高まるため |
