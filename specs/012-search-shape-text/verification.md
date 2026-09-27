# 検証記録

## 実行環境

- 実施日: 2026-09-28
- 対象ブランチ: `012-search-shape-text`
- UI モック: `design/mainui/index.html` を Shape 検索トグル・一覧・詳細表示に同期

## 自動検証

| コマンド | 結果 |
| --- | --- |
| `npm test -- --run` | 成功、11 files / 65 tests |
| `npm run build` | 成功 |
| `npm run lint` | 成功 |
| `cd src-tauri && cargo test` | 成功、Rust unit / integration / doc tests |
| `cd src-tauri && cargo clippy --all-targets -- -D warnings` | 成功、警告なし |
| `cd src-tauri && cargo fmt --check` | 成功 |
| `git diff --check` | 成功 |

## 対象機能の自動確認

- Shape 検索の既定値と JSON 契約、 Shape 名、可視状態、アンカーなしの空セル番地・行列 0 を検証。
- 合成した OOXML と XLSB パッケージでテキスト・名前・シート・アンカー、および Shape 検索オフ時にセル検索を維持することを検証。
- CFB/BIFF 合成ストリームで `.xls` TxO テキスト復号と、抽出開始前のキャンセルを検証。
- CSV / Excel の Shape 名専用列を検証。
- UI で既定オン、独立トグル、Shape 詳細、アンカーあり・なし、セルプレビュー抑止、エスケープ済みテキスト表示を検証。

## 未実施・制限

- T001 と T009 に定義した実ファイル fixture 群（4形式の実ブック、edge ケース、由来・SHA-256・期待件数）は未作成。テストは合成 ZIP/CFB パッケージを使用。
- `.xls` は TxO/Continue からテキストを抽出する最小実装。OfficeArt の実 Shape 名、グループ階層、アンカーは読まず、安定した仮名を付与する。このため `.xls` の名前・アンカー要件は未達。
- 破損 Shape 単位のエラー隔離と過大入力専用テストは未確認。`.xls` はシート走査の境界でキャンセルを確認するため、単一シート内での即時中断は未対応。
- 実 Excel UI での4形式の手動操作、日英切替、破損入力、CSV/Excel出力、100ファイル/100 MB の10秒性能測定は未実施。
- 以上により T011、T016、T017、T027、T037、T038、T039 は未完了。全仕様の完了判定は保留。
