<!-- 処理内容: CLIプログラムソース分離およびマルチクレート構成に関する調査・技術選定・アーキテクチャ意思決定を記録する。 引数・戻り値: 仕様書（spec.md）および現状コードベースを入力とし、調査結果（Markdown文書）を出力する。 エラー: 依存循環、Tauriランタイム結合、ロケール不整合などのリスクとその回避策を定義する。 変更履歴: v1.0.0 2026-09-29 Antigravity 初版作成。 -->
# Research & Architecture Decisions: CLIプログラムソースのTauriディレクトリからの分離・独立化

**Feature Branch**: `016-extract-cli-crate`  
**Date**: 2026-09-29  
**Spec**: [spec.md](file:///F:/source/orca/workspaces/exgrep/.orca/workspaces/exgrep/copepod/specs/016-extract-cli-crate/spec.md)

## 1. Cargo ワークスペースとクレート分割構造

### 決定事項 (Decision)
リポジトリルートの Cargo ワークスペース（`Cargo.toml`）を 3 メンバー構成に再編する。
```toml
[workspace]
members = [
    "crates/core",
    "crates/cli",
    "src-tauri",
]
resolver = "2"
```
各クレートの役割とパッケージ名:
1. **`crates/core`** (`exlgrep-core` / ライブラリクレート `exlgrep_core`):
   - 検索エンジン（セル、コメント、図形内テキスト）、ファイル走査、Excel/CSVエクスポート、共通データモデル、埋め込み翻訳カタログ、共通定数を提供する。
2. **`crates/cli`** (`exlgrep-cli` / バイナリクレート `exlgrep-cli`):
   - コマンドライン引数解析（`args.rs`）、出力ガード・フォーマッタ（`output.rs`）、CLI実行制御（`mod.rs`）、エントリポイント（`src/main.rs`）。
   - `exlgrep-core` に依存し、Tauriには一切依存しない。
3. **`src-tauri`** (`exlgrep` / GUIデスクトップバイナリおよびライブラリ `exlgrep_lib`):
   - Tauri IPCコマンドハンドラ、ウィンドウ管理、macOSシステムメニュー、Aboutダイアログイベント。
   - `exlgrep-core` に依存し、CLI専用コード（`src/bin/exlgrep-cli.rs`, `src/cli/`）を完全撤去する。

### 選定理由 (Rationale)
- **完全な関心の分離**: `src-tauri` はデスクトップGUI専用となり、CLIプログラムはTauriビルドチェーン（`tauri-build` やGUIプラグイン）から完全に独立する。
- **共有コードの二重管理防止**: 検索エンジンやエクスポートロジック、データモデルを `exlgrep-core` に一元化し、単一のソースから双方が再利用する。
- **標準的なRustプラクティス**: `crates/` 配下にライブラリとバイナリを配置するマルチクレート構成は、Cargoのエコシステムで最も広く採用されており可読性と保守性に優れる。

### 検討した代替案 (Alternatives Considered)
- **代替案 A**: `src-tauri` に共通ロジックを残し、`crates/cli` から `src-tauri` をライブラリとして参照する。
  - *却下理由*: CLIが `src-tauri` ディレクトリを参照・依存することになり、「tauriディレクトリ内に独立CLIソースがあるのは不適切」という根本課題の解決にならない。また、Tauriプラグイン等の依存を引き込むリスクが残る。
- **代替案 B**: リポジトリ直下に `cli/` と `core/` を配置する。
  - *却下理由*: `crates/` ディレクトリ配下に Rust クレートを集約する方が、フロントエンドの `src/` や設定ファイルと混ざらず、明確な構造となる。

---

## 2. 依存関係の境界とTauri依存の完全遮断

### 決定事項 (Decision)
`crates/core` および `crates/cli` の `Cargo.toml` には、`tauri` や `tauri-plugin-*` を一切含めない。
- `crates/core` の主要依存:
  - `calamine`, `rayon`, `regex`, `same-file`, `serde`, `serde_json`, `serde_yaml`, `walkdir`, `rust_xlsxwriter`, `chrono`, `encoding_rs`, `cfb`, `quick-xml`, `zip`
- `crates/cli` の主要依存:
  - `exlgrep-core = { path = "../core" }`
  - `same-file`, `walkdir`, `rayon`, `regex`
- `src-tauri` の主要依存:
  - `exlgrep-core = { path = "../crates/core" }`
  - `tauri`, `tauri-plugin-dialog`, `tauri-plugin-os`, `tauri-plugin-shell`, `tauri-plugin-i18n`, `open`, `winreg` (Windows)

### 選定理由 (Rationale)
- CLI単体でのビルド速度が向上し、ヘッドレス環境やCI環境でGUIライブラリ（WebKitやシステムダイアログ等）のネイティブ前提条件なしにビルド・テスト可能になる。

### 検討した代替案 (Alternatives Considered)
- **フィーチャーフラグによる切り分け**: 単一クレート内で `features = ["gui", "cli"]` で切り替える。
  - *却下理由*: `src-tauri` 配下にコードが残存し、ディレクトリ構造の不適切さが解消されない。

---

## 3. ロケール・カタログの一元管理と共有方式

### 決定事項 (Decision)
1. 翻訳カタログ原本（`ja.yml`, `en.yml`）を `crates/core/locales/` に配置する。
2. `crates/core/src/i18n.rs` 内で `include_str!("../locales/ja.yml")` および `include_str!("../locales/en.yml")` を使用してバイナリ埋め込みカタログをパース・提供する。
3. `src-tauri` 側で必要な `src-tauri/locales/` については、原本（`crates/core/locales/`）と同期維持する（`src-tauri/build.rs` またはコピー）。さらにテスト `crates/core/tests/catalog_sync.rs` を用意し、両者の完全一致を自動検証する。

### 選定理由 (Rationale)
- CLIおよび共通エクスポート（CSV/Excelの列ヘッダー翻訳）は `crates/core` 内で自己完結し、外部ディレクトリに依存せず単独でコンパイル・実行できる。
- `tauri-plugin-i18n` が要求する `src-tauri/locales/` の配置も満たし、GUIデスクトップアプリケーションに影響を与えない。

### 検討した代替案 (Alternatives Considered)
- **シンボリックリンクの利用**: `src-tauri/locales` を `crates/core/locales` へのシンボリックリンクとする。
  - *却下理由*: Windows環境におけるGitのシンボリックリンク権限制約や開発環境差異でビルドエラーを引き起こすリスクがあるため不採用。

---

## 4. 定数一元化とモジュール配置（憲章 原則 II）

### 決定事項 (Decision)
プロジェクト憲章 原則 II（No Hardcoded Constants）を遵守し、各クレートに責務に応じた定数ファイル `src/constants.rs` を配置する。
1. **`crates/core/src/constants.rs`**:
   - 検索ステージ名、デフォルト検索ID、拡張子プレフィックス、エクスポートヘッダーキー、ロケールキー、エラー文字列定数など、コアロジック共通の定数。
2. **`crates/cli/src/constants.rs`**:
   - CLI引数名（オプション文字列、ショートオプション文字）、終了コード（0, 1, 2）、CLIエラーセパレータ、CLI用フォーマット定数。
3. **`src-tauri/src/constants.rs`**:
   - ウィンドウサイズ/最小サイズ、メニューID、IPCイベント名、Tauri固有エラー定数。
4. **`src/constants/index.ts`**（既存フロントエンド定数）:
   - 変更なし。

### 選定理由 (Rationale)
- 各クレートが自身の内部で閉じた定数を管理でき、不要な外部依存や循環参照を防ぐ。
- コード内のマジックナンバーおよび固定文字列はすべて対応する `constants.rs` から参照される。

---

## 5. テストコードの再配置と品質検証

### 決定事項 (Decision)
- **CLI統合テスト**:
  - `src-tauri/tests/cli_search.rs` → `crates/cli/tests/cli_search.rs`
  - `src-tauri/tests/cli_output.rs` → `crates/cli/tests/cli_output.rs`
- **Excel解析・図形検索テスト**:
  - `src-tauri/tests/shape_*.rs` → `crates/core/tests/shape_*.rs`
- **検証コマンド**:
  - ワークスペース全体テスト: `cargo test --workspace`
  - ワークスペース全体Clippy: `cargo clippy --workspace --all-targets -- -D warnings`
  - ワークスペース全体フォーマット: `cargo fmt --check`
  - フロントエンドビルド: `npm run build`
  - 個別テスト: `cargo test -p exlgrep-cli`, `cargo test -p exlgrep-core`, `cargo test -p exlgrep`

### 選定理由 (Rationale)
- テストコードもテスト対象のクレートと同じ場所に配置することで、クレートごとの独立したCI検証と保守性が向上する。
