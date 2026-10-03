# Quickstart Validation Guide: アプリ利用パッケージの著作権・ライセンス表示 (About Dialog & Package Licenses)

**Feature Branch**: `006-about-dialog-licenses`  
**Date**: 2026-09-26  
**Status**: Completed  

---

## 1. 前提条件と環境準備 (Prerequisites)

- Node.js >= 18, npm >= 9
- Rust >= 1.75 (`cargo`, `rustc`)
- Python 3.10+ (ライセンス収集スクリプト実行用)
- 依存関係のインストールが完了していること:
  ```bash
  npm install
  ```

---

## 2. ライセンスデータの生成検証 (Scenario 1: License Generation)

### 実行コマンド
```bash
python3 scripts/generate-licenses.py
```

### 期待される結果 (Expected Outcome)
- スクリプトが正常終了（終了コード 0）すること。
- `src/constants/licenses.json` が生成または更新されること。
- 生成されたJSONが [license-schema.json](./contracts/license-schema.json) のスキーマに完全準拠していること。
- Rust実行時クレート（`calamine`, `rayon`, `regex`, `tauri` 等）および npmプロダクション依存パッケージ（`react`, `lucide-react`, `@tauri-apps/api` 等）が漏れなく網羅されていること。
- 各レコードに `id` (`{name}@{version}`), `name`, `version`, `source`, `license`, `license_text` が正しく含まれていること。

---

## 3. Aboutダイアログ起動と基本情報確認 (Scenario 2: About Dialog & Meta Info)

### 実行手順
1. アプリケーション開発サーバーを起動:
   ```bash
   npm run tauri dev
   ```
   （またはスタンドアローンモック検証: `open design/mainui/index.html`）
2. メイン画面の最下部ステータスバー最右端（CSV/Excel出力ボタンの右隣）にある `Info` アイコンボタンをクリックする。

### 期待される結果 (Expected Outcome)
- 画面中央に半透明暗転バックドロップとともにAboutダイアログが即座に（100ms以内）モーダル表示されること。
- 初期タブ「アプリ情報」において、アプリ名（Excel Grep）、バージョン（0.1.0）、概要説明文、および著作権表記（Copyright notice）が明瞭に表示されること。
- ESCキー、右上の閉じるボタン（✕）、またはバックドロップをクリックした際、ダイアログがスムーズに閉じ、メイン画面の検索条件や選択状態が維持されていること。

---

## 4. オープンソースライセンス閲覧・検索・コピー (Scenario 3: Licenses Tab, Search & Copy)

### 実行手順
1. Aboutダイアログを開き、「オープンソースライセンス」タブをクリックする。
2. 左ペインのパッケージ一覧に全利用ライブラリが一覧表示され、先頭アイテムが選択されていることを確認する。
3. 任意のパッケージ（例: `calamine` または `react`）をクリックして選択する。
4. 右ペインのヘッダーにパッケージ名、バージョン、ライセンス種別、著作者名が表示され、下部のスクロールビューに正規ライセンス本文が改行・書式を維持して表示されることを確認する。
5. 上部のアクションバーにある「ライセンス本文をコピー」ボタンをクリックする。
6. 左ペイン上部の検索入力欄に「mit」や「regex」などのキーワードを入力する。

### 期待される結果 (Expected Outcome)
- パッケージ選択時、右ペインの内容が瞬時に切り替わり、選択中行が左ペインでハイライトされること。
- 長文のライセンス本文（Apache-2.0など）が右ペイン内でスムーズにスクロールできること。
- 「ライセンス本文をコピー」をクリックした際、ボタンが一時的に「コピー完了」に変化し、画面右上にトースト通知（「{package} のライセンス本文をクリップボードにコピーしました」）が表示されること。
- クリップボードの内容をエディタ等に貼り付けた際、選択したパッケージの正確なライセンス全文が貼り付けられること。
- 検索キーワードの入力により、200ms以内に対象パッケージのみがフィルタリングされ、存在しないキーワードを入力した場合は「一致するパッケージが見つかりません」が表示されること。

---

## 5. 静的解析およびコード品質検証 (Scenario 4: Quality & Style Check)

### 実行コマンド
```bash
# TypeScript型チェック & フロントエンドビルド
npm run build

# Rust静的解析 & フォーマットチェック
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
cargo fmt --manifest-path src-tauri/Cargo.toml --check
```

### 期待される結果 (Expected Outcome)
- `npm run build` が警告・エラーなしで正常終了すること（`tsc` 型エラーゼロ）。
- `clippy` および `fmt` が警告・エラーなしで正常終了すること。
- 新規・変更されたすべてのファイルに憲章原則IIIに準拠した4要素ヘッダコメントが記載されていること。
- ハードコードされた固定文字列が残存せず、すべて `src/constants/index.ts` を参照していること（憲章原則II）。
