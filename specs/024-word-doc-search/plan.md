# 実装計画: MS Word ドキュメント検索 (.doc, .docx, .docm)

**機能ブランチ**: `024-word-doc-search` | **日付**: 2026-10-15 | **仕様書**: [spec.md](./spec.md)

**入力**: `/specs/024-word-doc-search/spec.md` より生成

---

## 概要

MS Word 文書（`.doc`, `.docx`, `.docm`）を検索対象としてサポートし、設定画面からの有効化（オプトイン）、メイン検索バーとの拡張子連動、および検索結果に対する段落番号・自動スクロール付きのテキスト形式プレビューを実装する。

- **コアエンジン (`xlseek-core`)**: 既存の `zip` + `quick-xml` を用いた OOXML (`.docx`, `.docm`) パーサーおよび `cfb` + `encoding_rs` を用いたバイナリ OLE2 (`.doc`) パーサーを `crates/core/src/search/word/` に追加し、マルチスレッド検索パイプラインに統合する。
- **デスクトップバックエンド (`src-tauri`)**: Word プレビュー取得 IPC コマンド `get_word_preview` を新設し、定数管理を行う。
- **フロントエンド (`src/`)**: 設定ダイアログに Word 検索オプションを追加し、メイン画面の拡張子バーに `.docx`, `.docm`, `.doc` ボタンを動的表示する。右ペインにテキストプレビューコンポーネント `WordTextPreview` を追加する。

---

## 技術コンテキスト

**言語/バージョン**:
- バックエンド/コア: Rust 2021 edition
- フロントエンド: React 18, TypeScript 5.5, Tailwind CSS, Vite 6
- デスクトップ基盤: Tauri v2

**主要依存関係**:
- `zip` (0.6.6) & `quick-xml` (0.41.0) - OOXML (`.docx`, `.docm`) の XML ストリーミングパース
- `cfb` (0.7.3) & `encoding_rs` (0.8.42) - レガシー OLE2 (`.doc`) のバイナリおよび日本語テキストデコード
- `rayon` (1.10) - 並列走査・マルチスレッドスキャン
- `regex` (1.10) - 正規表現検索
- Lucide React - アイコン表示

**ストレージ**:
- `localStorage` による設定（`DefaultSearchOptions` / `enable_word_search`）の永続化

**テスト基盤**:
- Rust: `cargo test --workspace` (単体テスト・結合テスト)
- フロントエンド: `npm run build` (`tsc` による型チェックと Vite バンドル検証)
- 静的解析: `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`

**対象プラットフォーム**:
- Windows, macOS, Linux (クロスプラットフォームデスクトップ)

**プロジェクト種別**:
- デスクトップアプリケーション (`xlseek`) およびコア検索エンジンライブラリ (`xlseek-core`)

**パフォーマンス目標**:
- 100件以上の Word 文書走査時も UI の応答性（60fps）を維持
- Word テキストプレビュー表示（段落抽出および該当行スクロール）を 300ms 以内に完了

**制約条件**:
- 憲法 原則 II: 定数ハードコードの完全禁止（0 と "" を除く全数値を定数化）
- 憲法 原則 III: すべての新規ファイル・構造体・関数に必須3項目を満たすヘッダコメントを付与
- 憲法 原則 IV: Clippy 警告ゼロ、型エラーゼロ

**規模/スコープ**:
- コア解析モジュール 3ファイル追加 (`word/mod.rs`, `word/docx.rs`, `word/doc.rs`)
- フロントエンド UI コンポーネント 1ファイル追加 (`WordTextPreview.tsx`)
- 設定画面・検索バー・プレビューペインの連動改修

---

## 憲章チェック (Constitution Check)

*GATE: Phase 0 研究前にパス確認。Phase 1 設計後にも再検証済み。*

- [x] **原則 I: 言語指示の遵守と品質 (Language-Directed Quality)**:
  - ユーザー指示に従い、本アーティファクト（計画書・設計書）を日本語で作成。
  - ソースコード内のコメント・Docstring・コミットログは憲章に基づき自然で正確な英語を使用。
  - 不正トークン（`<PAD>` 等）や文字化けがないことを確認。
- [x] **原則 II: 定数の外部抽出とハードコード禁止 (No Hardcoded Constants)**:
  - すべての拡張子（`"docx"`, `"docm"`, `"doc"`）、IPC コマンド名（`"get_word_preview"`）、UI固定ラベル（`"Word"`, `"-"`）は定数ファイル（`src/constants/index.ts`, `src-tauri/src/constants.rs`, `crates/core/src/constants.rs`）へ抽出して定義。
  - 利用箇所には `// Constant reference: ...` コメントを徹底。
- [x] **原則 III: 厳格なヘッダコメントとドキュメンテーション (Comprehensive Header Comments)**:
  - 新設するすべてのモジュール、関数、コンポーネントに (1) 処理内容、(2) 引数と戻り値、(3) エラー・パニックハンドリングを記載。
- [x] **原則 IV: 責務に応じたモジュール分割と標準スタイル準拠 (Modular Design & Code Standards)**:
  - 単一責任の原則に従い、OOXML 処理と OLE2 処理を別ファイルに分離。
  - `cargo clippy`, `cargo fmt`, `npm run build` を遵守。
- [x] **原則 V: 堅牢なエラーハンドリングとテスト検証 (Robust Error Handling & Testing)**:
  - 破損ファイル、暗号化ファイル、不正な文字エンコーディング遭遇時もパニックせず `Result` で安全にスキップ・処理。
  - 単体テストを追加して動作を担保。

---

## プロジェクト構造

### ドキュメント構成 (本機能)

```text
specs/024-word-doc-search/
├── spec.md              # 仕様書（確定版）
├── plan.md              # 本ファイル（実装計画）
├── research.md          # Phase 0 技術調査レポート
├── data-model.md        # Phase 1 データモデル設計書
├── contracts/           # Phase 1 インターフェース契約書
│   └── word-search-contract.md
├── quickstart.md        # Phase 1 クイックスタート検証ガイド
└── checklists/
    └── requirements.md  # 仕様品質チェックリスト
```

### ソースコード配置構造

```text
crates/core/
├── src/
│   ├── constants.rs             # Word 拡張子・ラベル定数追加
│   ├── models/
│   │   └── mod.rs               # MatchType::WordText 追加
│   └── search/
│       ├── mod.rs               # word モジュール公開
│       ├── engine.rs            # Word 拡張子ルーティング
│       └── word/                # 新設: Word 文書解析
│           ├── mod.rs           # エントリポイント・パーサーディスパッチ
│           ├── docx.rs          # OOXML (.docx, .docm) テキスト抽出
│           └── doc.rs           # OLE2 (.doc) バイナリテキスト抽出
│
src-tauri/
├── src/
│   ├── constants.rs             # コマンド名定数追加
│   └── commands/
│       ├── mod.rs
│       └── preview_cmd.rs       # get_word_preview コマンド追加
│
src/
├── constants/
│   └── index.ts                 # フロントエンド定数追加
├── types/
│   ├── search.ts                # MatchType に WordText 追加
│   ├── defaultOptions.ts        # enable_word_search 追加
│   └── preview.ts               # WordPreviewPayload 型定義追加
├── components/
│   ├── preview/
│   │   ├── WordTextPreview.tsx  # 新設: テキスト形式プレビュー
│   │   └── PreviewHeader.tsx    # Word アイコン・タイトル対応
│   ├── search/
│   │   └── SearchBar.tsx        # Word 拡張子トグルの動的表示
│   └── settings/
│       └── SettingsDialog.tsx   # Word 検索トグル設定項目追加
└── App.tsx                      # プレビュー表示の条件分岐 (Word プレビューへルーティング)
```

---

## 複雑性追跡 (Complexity Tracking)

憲章違反や不当な複雑性はありません（該当なし）。

| 項目 | 理由 | 代替案を棄却した理由 |
|---|---|---|
| 該当なし | - | - |
