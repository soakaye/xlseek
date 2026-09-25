# Quickstart: 憲章準拠プログラム検証ガイド

**Feature Branch**: `004-align-with-constitution`  
**Date**: 2026-09-26  
**Status**: Ready  

本ガイドは、改修後のプログラムがプロジェクト憲章（[constitution.md](../../.specify/memory/constitution.md)）および機能仕様書（[spec.md](spec.md)）の全要件を充足しているかをエンドツーエンドで検証するための手順書である。

---

## 1. 前提条件 (Prerequisites)

- **Rust**: 1.80 以上 (`cargo`, `clippy`, `rustfmt`)
- **Node.js**: v18 以上 (`npm`)
- **Tauri CLI**: 2.0 以上

---

## 2. 検証シナリオ一覧 (Validation Scenarios)

### シナリオ 1: 静的解析およびスタイルガイド完全準拠の検証 (Principle IV)

Rustの公式静的解析およびフォーマットチェックを実行し、警告およびエラーが0件であることを確認する。

```bash
# Rust フォーマット検査
cd src-tauri
cargo fmt --check

# Rust Clippy 静的解析（警告ゼロを強制）
cargo clippy --all-targets -- -D warnings
```

**期待される結果**:
- エラーおよび警告が一切出力されず、終了コード `0` で完了すること。

---

### シナリオ 2: フロントエンド型検査およびビルド検証 (Principle IV)

TypeScriptの型整合性およびViteによるプロダクションビルドが正常に完了することを確認する。

```bash
# プロジェクトルートで実行
npm run build
```

**期待される結果**:
- `tsc` 型エラーが0件であり、Viteバンドルが正常終了すること。

---

### シナリオ 3: ユニットテストおよびエラーハンドリングの検証 (Principle V)

バックエンドの全テストを実行し、テストが100%成功することを確認する。

```bash
cd src-tauri
cargo test
```

**期待される結果**:
- すべてのテストケース（定数定義テスト、スニペットUTF-8境界安全性、キャンセル制御、実ファイル検索等）が `ok` となること。

---

### シナリオ 4: 定数抽出および参照コメントの網羅性検証 (Principle II)

コードベース内に未抽出のリテラルが残存しておらず、定数利用箇所に参照コメントが付与されていることを静的検証する。

```bash
# Rust コード内の定数参照コメント確認
grep -rn "// 定数参照:" src-tauri/src/

# フロントエンド コード内の定数参照コメント確認
grep -rn "// 定数参照:" src/
```

**期待される結果**:
- 各コンポーネントおよびロジックにおいて、`0` および `""` を除く定数値が `constants` から参照され、対応するコメントが存在すること。

---

### シナリオ 5: 日本語出力品質および不要トークンの排除検証 (Principle I)

ソースコードおよびリソース内に、文字化けや `<PAD>`、`<pad>` 等の不要特殊トークンが含まれていないことを検証する。

```bash
# <PAD> または <pad> の混入チェック
grep -rn -i "<pad>" src/ src-tauri/src/
```

**期待される結果**:
- ヒット件数が0件であること。
- UI上に表示されるすべての文言が自然な日本語で表示されること。

---

## 3. 関連仕様・契約ドキュメント

- [data-model.md](data-model.md) - 定数モデルおよびエラー契約
- [contracts/constants-contract.md](contracts/constants-contract.md) - 定数命名・参照規約
- [contracts/header-comment-contract.md](contracts/header-comment-contract.md) - 4要素ヘッダコメント規約
- [contracts/tauri-commands-contract.md](contracts/tauri-commands-contract.md) - Tauriコマンド通信規約
