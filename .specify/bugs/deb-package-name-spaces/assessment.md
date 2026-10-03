# Bug Assessment: Debianパッケージ名に空白が含まれる問題

- **Slug**: deb-package-name-spaces
- **Created**: 2026-10-02T17:15:30+09:00
- **Source**: pasted text
- **Verdict**: valid
- **Severity**: medium

## Report (verbatim or summarized)

> パッケージ名に`Excel Seek_0.1.0_amd64.deb`の様な空白を含まない

## Symptom

Tauri v2 による Linux Debian パッケージ（`.deb`）ビルド時、出力ファイル名およびパッケージ識別名が `Excel Seek_0.1.0_amd64.deb` のようにスペースを含む名前として生成される。
Debian パッケージ命名規則や Linux の CLI 環境（`dpkg`, `apt`, スクリプトによる自動配布・CI/CD）において、ファイル名やパッケージ名に空白文字が含まれると、引用符処理漏れによる誤動作やパッケージマネージャの構文エラーを招く原因となる。
期待される動作としては、パッケージファイル名や Linux 向けパッケージ名に空白を含まない形式（`xlseek_0.1.0_amd64.deb` 等）でビルド・生成されること。

## Reproduction

1. `src-tauri/tauri.conf.json` を確認する。
   - `"productName": "Excel Seek"` と定義されている。
2. Tauri CLI（`npx tauri build --bundles deb` 等）を実行して Linux Debian パッケージをビルドする。
3. 生成されるアーティファクト名が `target/release/bundle/deb/Excel Seek_0.1.0_amd64.deb` となり、ファイル名に半角スペースが含まれることを確認する。

## Suspected Code Paths

- `src-tauri/tauri.conf.json:3` — `"productName": "Excel Seek"` がトップレベルに設定されている。Tauri の bundler はデフォルトで `productName` をそのままパッケージファイル名フォーマット（`{productName}_{version}_{arch}.deb`）に使用する。
- `src-tauri/tauri.conf.json:16` — `"title": "Excel Seek"` ウィンドウタイトル。

## Root Cause Hypothesis

**確信度: High（高）**

Tauri v2 のバンドラ（`tauri-bundler` / `@tauri-apps/cli`）は、生成する Debian パッケージ（`.deb`）のファイル名およびインストーラ出力名に `tauri.conf.json` の `productName` を参照します。
現在 `tauri.conf.json` の `"productName"` に空白文字を含む `"Excel Seek"` が設定されているため、生成されるアーティファクト名が `Excel Seek_0.1.0_amd64.deb` となり空白が含まれてしまいます。

Tauri v2 ではプラットフォーム固有の設定ファイル（`src-tauri/tauri.linux.conf.json`）を作成するか、あるいはプロジェクト全体の `productName` を空白なしの `xlseek`（またはハイフン繋ぎの `excel-seek`）に設定し、ウィンドウタイトルや表示名を維持することで、他プラットフォームや GUI 表示を損なわずに Linux パッケージ名から空白を排除できます。
特に、本プロジェクトは仕様 021 においてプロジェクト名・バイナリ名・パッケージ名を `xlseek` に統一する方針をとっており、Cargo パッケージ名（`xlseek` / `xlseek-cli` / `xlseek-core`）やリポジトリ名も `xlseek` となっています。

## Proposed Remediation

**Preferred（推奨）**:
1. `src-tauri/tauri.linux.conf.json` または `src-tauri/tauri.conf.json` における設定の調整:
   - 最もクリーンかつプロジェクトの命名統一方針（`xlseek`）に合致する方法として、Linux 向けビルド時にパッケージ名が `xlseek`（または `excel-seek`）となるよう設定する。
   - Tauri v2 では、プラットフォーム別設定ファイル `src-tauri/tauri.linux.conf.json` を配置することで Linux 専用設定をマージ可能：
     ```json
     {
       "productName": "xlseek"
     }
     ```
     または、`src-tauri/tauri.conf.json` の `"productName"` 自体を `"xlseek"` に設定し、ウィンドウタイトル（`app.windows[0].title`）を `"Excel Seek"` のまま維持する。
   - これにより、Debian パッケージ名が `xlseek_0.1.0_amd64.deb` となり、空白が完全に排除される。

**Alternatives（代替案）**:
- *代替案1: `tauri.linux.conf.json` のみで `productName` を上書きする*:
  - Windows や macOS 向けのインストーラ・`.app` バンドル名には `"Excel Seek"` を維持しつつ、Linux（deb/rpm/appimage）ビルド時のみ `xlseek`（または `excel-seek`）を適用する。
  - *トレードオフ*: プラットフォーム別の設定ファイルを管理する必要がある。
- *代替案2: `productName` を `excel-seek` とする*:
  - 空白をハイフンに置換する。

**Files likely to change**:
- `src-tauri/tauri.conf.json` (または新規 `src-tauri/tauri.linux.conf.json`)

**Tests to add or update**:
- `tauri.conf.json` (および Linux 向け設定) のバリデーションテスト。
- パッケージビルド設定の検証。

## Risks & Considerations

- `productName` を変更した場合、デスクトップ上のショートカット名やメニューに影響する可能性があるため、ウィンドウタイトルやメタデータ（`.desktop` ファイル内の `Name` 等）が適切に維持されるか確認する。
- 既存の定数やコードロジック（`src-tauri/src/constants.rs` 等）との整合性に問題がないことを確認する。

## Open Questions

- なし（原因および改修方法は明確）。
