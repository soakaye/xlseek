# AGENTS.md — Excel Grep 開発ガイドライン & エージェント行動規範

本ファイルは、Excel Grep リポジトリで作業するすべての AI エージェント（Antigravity, Claude, Codex, Cursor 等）が遵守すべき最上位の行動指針、コーディング規約、アーキテクチャ、および開発手順を定義します。

---

## 1. プロジェクト概要

- **名称**: Excel Grep (`exlgrep`)
- **目的**: 大量の Excel ファイル（`.xlsx`, `.xlsm`, `.xls`, `.xlsb`）から、指定キーワードや正規表現に一致するセル、シート、コメントを高速かつセキュアに横断検索するデスクトップアプリケーション。
- **アーキテクチャ**:
  - **GUI / フロントエンド**: React 18, TypeScript, Tailwind CSS, Lucide React, Tauri v2 API
  - **バックエンド / コアエンジン**: Rust (2021 edition), `calamine` (高速 Excel 解析), `rayon` (マルチスレッド並列走査), `rust_xlsxwriter` (Excel 出力), `regex` (正規表現)
  - **デスクトップ基盤**: Tauri v2 (`@tauri-apps/api` v2, `tauri-plugin-dialog`, `tauri-plugin-shell`)

---

## 2. 最重要原則（プロジェクト憲章準拠）

すべてのエージェントは、`.specify/memory/constitution.md`（プロジェクト憲章）に定められた以下の 5 原則を例外なく厳格に遵守しなければなりません（**MUST**）。

### 原則 I. 指定言語の優先と自然な出力（Language-Directed Quality）
- ユーザーへの回答、コミットメッセージ、生成ドキュメント、UI 文言、エラーメッセージ、ログは、利用者の指示または承認済みの仕様で言語が明示されている場合、その言語で記述すること。
- 言語の指定がないユーザーへの回答、コミットメッセージ、生成ドキュメントは、自然かつ正確な日本語で記述すること。指定言語が日本語以外の場合は、日本語への翻訳を強制しないこと。
- 文字化けや `<PAD>`, `<pad>` などの不要・不明な特殊トークンが混入していないことを提出前に検査し、指定言語に合う自然な表現へ補正すること。

### 原則 II. 定数の外部抽出とハードコードの禁止（No Hardcoded Constants）
- `0` と空文字列（`""`）を除き、定数値（数値、マジックナンバー、固定文字列、イベント名、コマンド名、UI ラベル等）をコードへ直接ハードコードしてはならない（**MUST NOT**）。
- **フロントエンド定数**: `src/constants/index.ts` へ定義・一元管理。
- **Rust バックエンド定数**: `src-tauri/src/constants.rs` へ定義・一元管理。
- 定数を利用する箇所には、必ず該当定数を参照している旨のコメント（例: `// 定数参照: constants::MENU_ITEM_ABOUT_ID`）を明記すること。

### 原則 III. 厳格な 4 要素ヘッダコメントとドキュメンテーション（Comprehensive Header Comments）
すべてのファイル、モジュール、構造体、クラス、関数、およびメソッドには、以下の 4 要素を漏れなく含むヘッダコメントを必ず記載すること（**MUST**）：
1. **処理内容の詳細な説明**
2. **引数・戻り値の型と各説明**
3. **起こり得るエラー、Result/Option の扱い、または panic / 例外の発生条件**
4. **変更履歴（バージョン、作成日、作成者、修正内容）**

### 原則 IV. 責務に応じたモジュール分割と標準スタイル準拠（Modular Design & Code Standards）
- 単一責任の原則（SRP）を守り、責務に応じたモジュール分割を行うこと。
- Rust: 公式スタイルガイドおよび `clippy`, `rustfmt` に完全準拠すること。
- TypeScript: 型エラー（`tsc --noEmit`）および ESLint 警告をゼロに維持すること。

### 原則 V. 堅牢なエラーハンドリングとテスト検証（Robust Error Handling & Testing）
- 予期しないパニックや未処理例外を排除し、`Result` や適切なエラー型を用いて安全に処理すること。
- モジュールや関数を変更・追加した際は、必ず対応する単体テスト（`cargo test` / フロントエンドテスト）を作成・更新して動作を検証すること。

---

## 3. ディレクトリ構成と役割

```text
exlgrep/
├── AGENTS.md                  # 本ファイル（AI エージェント行動規範）
├── README.md                  # プロジェクト概要・機能説明
├── Cargo.toml                 # Cargo ワークスペース定義 (crates/core, crates/cli, src-tauri)
├── package.json               # フロントエンド依存関係 & npm スクリプト
├── crates/                    # バックエンド共有コア & 独立CLIクレート
│   ├── core/                  # 検索エンジン・パーサー・エクスポート・共有モデル (exlgrep-core)
│   │   ├── Cargo.toml
│   │   ├── locales/           # 翻訳カタログ原本 (ja.yml, en.yml)
│   │   ├── src/               # lib.rs, constants.rs, models/, search/, export/, i18n.rs
│   │   └── tests/             # shape_*.rs 統合テスト
│   └── cli/                   # 独立CLIバイナリ (exlgrep-cli)
│       ├── Cargo.toml
│       ├── src/               # main.rs, lib.rs, constants.rs, args.rs, output.rs
│       └── tests/             # cli_search.rs, cli_output.rs 統合テスト
├── src/                       # フロントエンドソースコード (React / TypeScript)
│   ├── App.tsx                # ルートコンポーネント (状態バインディング & ダイアログ統合)
│   ├── main.tsx               # エントリポイント
│   ├── constants/             # 定数一元定義モジュール (index.ts, licenses.json)
│   ├── types/                 # TypeScript 型定義 (search.ts, license.ts)
│   ├── hooks/                 # カスタムフック (useSearch.ts 等)
│   └── components/            # UI コンポーネント群
│       ├── about/             # About & OSS ライセンスダイアログ (AboutDialog.tsx 等)
│       ├── common/            # 共通パーツ (StatusBar.tsx, Toast.tsx)
│       ├── layout/            # ウィンドウ枠・ドラッグ領域 (WindowFrame.tsx)
│       ├── preview/           # Excel セルプレビューグリッド・数式バー
│       ├── results/           # 検索結果テーブル (仮想スクロール表示)
│       └── search/            # 検索入力バー・拡張子トグル・コントロール
├── src-tauri/                 # Tauri v2 デスクトップGUIアプリケーション (exlgrep)
│   ├── Cargo.toml             # Rust クレート依存定義 (exlgrep-core に依存)
│   ├── tauri.conf.json        # Tauri 設定ファイル (ウィンドウ設定, 権限設定)
│   ├── capabilities/          # Tauri v2 セキュリティケーパビリティ (default.json)
│   └── src/
│       ├── lib.rs             # アプリケーション初期化, メニュー構築, イベントハンドラ
│       ├── main.rs            # 実行バイナリエントリポイント
│       ├── constants.rs       # GUI専用定数一元定義モジュール & 単体テスト
│       └── commands/          # Tauri IPC コマンドハンドラ群 (search, preview, export, system)
├── specs/                     # Spec Kit 機能仕様・計画・タスク管理ディレクトリ
├── design/                    # UI モックアップ & スタンドアローン HTML プロトタイプ
└── .specify/                  # Spec Kit ワークフロー基盤
    ├── memory/constitution.md # プロジェクト憲章（最優先ガバナンス文書）
    ├── bugs/                  # バグ評価・修正・検証レポート記録場所
    └── extensions/            # Spec Kit 拡張（git, bug 等）
```

---

## 4. 必須検証コマンド（Quality Gates）

コード変更や修正作業を実施した後は、完了報告を行う前に**必ず以下の検証コマンドを実行し、エラーや警告がゼロであることを確認してください**。

| レイヤー | 検証コマンド | 目的・合格基準 |
| :--- | :--- | :--- |
| **Frontend** | `npm run build` | TypeScript 型チェック (`tsc`) と Vite バンドルが正常に exit 0 で完了すること |
| **Rust Test** | `cargo test --workspace` | 全クレートの単体・統合テストが 1 件の失敗もなくパスすること |
| **Rust Clippy** | `cargo clippy --workspace --all-targets -- -D warnings` | 全クレートでコンパイラおよび Clippy の警告が 0 件であること |
| **Rust Format** | `cargo fmt --check` | ワークスペース全体のコードが Rust 公式フォーマット規約に完全に適合していること |

---

## 5. Spec Kit & バグ対応ワークフロー

本プロジェクトでは Spec Kit ツールチェーンおよび関連スキルが導入されています。

### バグ対応手順（Bug Triage & Fix Workflow）
1. **バグ評価 (`/speckit-bug-assess`)**:
   - 症状のヒアリング、コードパス特定、原因仮説の立案、および改修案を策定。
   - 成果物: `.specify/bugs/<slug>/assessment.md`
2. **バグ修正 (`/speckit-bug-fix`)**:
   - `assessment.md` に基づき最小限かつ憲章準拠のコード修正・テスト追加を実施。
   - 成果物: `.specify/bugs/<slug>/fix.md`
3. **バグ検証 (`/speckit-bug-test`)**:
   - 自動テスト、ビルド検証、再現防止の確認を行い、検証レポートを記録。
   - 成果物: `.specify/bugs/<slug>/test.md`
4. **コミット (`/speckit-git-commit`)**:
   - 自動コミットフックまたはコマンドにより変更をコミット。

### UI モックアップの同期維持
- `src/` 配下のコンポーネント構造、デザインスタイル、UI オプションを変更した場合は、`syncing-mainui-mock` スキルを活用し、`design/mainui/index.html` に配置されているスタンドアローン HTML プロトタイプとの整合性を維持すること。

---

## 6. エージェントのコーディング禁止事項（Anti-Patterns）

- ❌ **マジックナンバー・文字列の直書き**: `0` と `""` 以外の定数は必ず定数定義ファイルへ外出しすること。
- ❌ **ヘッダコメントの省略**: 自動生成や簡易修正であっても 4 要素コメントを省略してはならない。
- ❌ **型チェックやテストの未実行での完了報告**: `npm run build` や `cargo test` のエビデンスなく成功を主張しないこと。
- ❌ **過剰なリファクタリング**: バグ修正や機能実装時に対象範囲外の無関係なコードを変更しないこと（YAGNI 原則）。
