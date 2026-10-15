# Bug Assessment: プレビュー画面の場所表示における Windows Verbatim パスプレフィックス (//?/ または \\?\) の除去

- **Slug**: preview-verbatim-path-prefix
- **Created**: 2026-10-15T20:03:00+09:00
- **Source**: pasted text
- **Verdict**: valid
- **Severity**: medium

## Report (verbatim or summarized)

> 検索結果プレビューの場所に//?/ がWindows時についている。ローカルパスの場合は表示しない。

Windows環境において、検索結果プレビューの「場所:」欄に表示されるファイルパスの先頭に `//?/`（または Windows 拡張プレフィックス `\\?\`）が付加されて表示されてしまう事象が報告されました。ローカルファイルパス（`C:\...` 等）を表示する場合は、このプレフィックスを除去して標準的なパスとして表示することが求められています。

## Symptom

- **観測される振る舞い**:
  Windows 環境で検索を実行し、検索結果を選択してプレビュー画面（`PreviewHeader.tsx`）を表示すると、「場所:」行およびツールチップ・クリップボードコピー機能において、`\\?\C:\Users\...\file.xlsx` や `//?/C:/Users/.../file.xlsx` のように Windows 拡張表記（Verbatim path prefix）が含まれたまま表示・コピーされる。
- **期待される振る舞い**:
  ローカルファイルパスの場合、`\\?\` や `//?/` などの拡張プレフィックスを除去し、通常の標準パス（例: `C:\Users\...\file.xlsx`）としてプレビュー欄に表示およびコピーされる。

## Reproduction

1. Windows 環境で Excel Seek (`xlseek`) を起動する。
2. ローカルドライブ（例: `C:\work` など）を検索対象フォルダに指定して検索を実行する。
3. 検索結果テーブルで任意の行をクリックして選択する。
4. 画面右側のプレビューヘッダー上部にある「場所:」行を確認する。
5. パスの先頭に `\\?\` または `//?/` が付いていることを確認する。

## Suspected Code Paths

- `src/components/preview/PreviewHeader.tsx:326-331`:
  - プレビュー画面ヘッダーの「場所:」行（`ui.LOCATION_LABEL`）で、`selectedMatch.full_path` を直接そのまま JSX および `title` 属性にバインドして描画している。
- `src/components/preview/PreviewHeader.tsx:172-184`:
  - `handleCopyPath` において、`navigator.clipboard.writeText(selectedMatch.full_path)` を呼び出し、未加工の `full_path` をクリップボードにコピーしている。
- `crates/core/src/search/path.rs:120-126`:
  - `resolve_and_partition_paths` 内で `resolved.canonicalize()` が呼び出され、Windows 上で `std::fs::canonicalize` により `\\?\` プレフィックスが付与されたパスが生成され、探索エンジン・`SearchMatch` にそのまま渡されている。
- `crates/core/src/export/mod.rs:30-45`:
  - 既にエクスポート（CSV/XLSX）用として `normalize_export_path` が実装されており、`\\?\` および `\\?\UNC\` の正規化ロジックが存在しているが、GUIプレビューの表示パスには適用されていない。

## Root Cause Hypothesis

**確信度: 高 (High)**

Windows において、検索対象ディレクトリの検証時（`crates/core/src/search/path.rs`）に `Path::canonicalize()` を実行すると、Win32 の Verbatim パス表現である `\\?\`（UNC パスの場合は `\\?\UNC\`）が先頭に付与されます。
このパスをそのまま `discover_files` および `SearchMatch.full_path` に格納してフロントエンドへ Tauri イベント経由で配信しています。
フロントエンドのプレビューヘッダーコンポーネント（`src/components/preview/PreviewHeader.tsx`）は、受け取った `selectedMatch.full_path` をそのまま表示およびクリップボードコピーに使用しているため、ユーザーから見ると `//?/` または `\\?\` が付いた不自然なパスとして表示されてしまいます。

## Proposed Remediation

**Preferred (推奨方針)**:
1. **フロントエンド側のパス正規化ユーティリティの実装**:
   - `src/utils/pathDisplay.ts`（または既存の `src/utils/pathTokenizer.ts` に準拠したユーティリティ）に、ユーザー向け表示用パス正規化関数 `normalizeDisplayPath(path: string): string` を実装する。
   - 以下のプレフィックスを処理する:
     - `\\?\UNC\` または `//?/UNC/`（大文字小文字不問）: `\\`（標準 UNC プレフィックス）に置換。
     - `\\?\` または `//?/`: ローカルドライブパス（例: `C:\...`）の前に付いている場合は除去。
     - それ以外の UNIX パスや通常のパスはそのまま保持。
   - `src/constants/index.ts` に Verbatim プレフィックスに関する定数（`PATH_DISPLAY_CONSTANTS` など）を定義し、ハードコードを排除（憲法 Principle II を遵守）。
2. **`PreviewHeader.tsx` への適用**:
   - `selectedMatch.full_path` から `displayPath = normalizeDisplayPath(selectedMatch.full_path)` を計算。
   - 「場所:」の表示テキストおよび `title` 属性に `displayPath` を使用。
   - クリップボードコピー（`handleCopyPath`）時にも `displayPath` をコピー。
3. **バックエンド側（補完措置・整合性）**:
   - 必要に応じて、`src-tauri/src/commands/search_cmd.rs` 等で `SearchMatch` をフロントエンドに配信する際、あるいは `crates/core/src/export/mod.rs` の `normalize_export_path` と仕様を合致させ、GUI / CLI / エクスポート間でのパス表現の一貫性を確保する。
4. **テストの追加**:
   - `tests/path-display.test.ts` を新規作成し、Windows Verbatim プレフィックス（`\\?\C:\...`, `//?/C:/...`, `\\?\UNC\...`）や UNIX パス、エスケープ・通常パスが期待通りに正規化されることをユニットテストで担保する。

**Alternatives (代替案)**:
- *バックエンドの `canonicalize` 直後に除去する*:
  - トレードオフ: 260文字を超えるロングパス（MAX_PATH制約）を処理する場合、Windows API 内部では `\\?\` プレフィックスが必要となる場合があるため、探索やファイルアクセス等の内部処理では `canonicalize` 結果のパスを維持し、表示・出力レイヤーでのみ正規化を行う方が安全かつ責務の分離に適している。

**Files likely to change**:
- `src/constants/index.ts`
- `src/utils/pathDisplay.ts` (新規作成)
- `src/components/preview/PreviewHeader.tsx`
- `tests/path-display.test.ts` (新規作成)

**Tests to add or update**:
- `tests/path-display.test.ts`:
  - `\\?\C:\folder\file.xlsx` -> `C:\folder\file.xlsx`
  - `//?/C:/folder/file.xlsx` -> `C:/folder/file.xlsx`
  - `\\?\UNC\server\share\file.xlsx` -> `\\server\share\file.xlsx`
  - `//?/UNC/server/share/file.xlsx` -> `\\server\share\file.xlsx`
  - 通常の Windows パス（`C:\test.xlsx`）および UNIX パス（`/tmp/test.xlsx`）が変更されないことの検証。

## Risks & Considerations

- 外部アプリ起動（`open_in_folder`, `launch_associated_app`, `get_cell_preview` など）に渡すパスとの整合性:
  - Windows の Explorer（`open_in_folder`）は `explorer.exe /select,\\?\C:\...` のような Verbatim パスを渡すと正しくフォルダ選択できない場合があるため、正規化されたパスを使用するか、既存のハンドラとの互換性を確認する。
- パス区切り文字（`/` と `\`）の両方に対応し、ユーザーが指摘した `//?/` 形式と標準の `\\?\` 形式の双方を漏れなくカバーすること。

## Open Questions

- なし。要件および修正対象箇所は明確です。
