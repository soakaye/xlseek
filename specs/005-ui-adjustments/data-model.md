# Data Model: UI項目調整 (UI Adjustments)

## 概要
本ドキュメントは、UI項目調整においてフロントエンド（React / Tauri）およびプロトタイプ（HTMLモック）が扱うデータモデル、状態エンティティ、および状態遷移ルールを定義する。

---

## エンティティ定義

### 1. 検索入力状態（SearchInputState）

検索バーコンポーネントが保持・制御する入力およびIME状態。

| フィールド名 | 型 | 必須 | 説明 | バリデーション / 制約 |
|---|---|---|---|---|
| `keyword` | `string` | 必須 | 検索対象キーワードまたは正規表現文字列 | 空文字列可、スペースのみの場合は検索トリガー抑止 |
| `target_dir` | `string` | 必須 | 検索対象フォルダの絶対パス | 空文字列可、存在しないパスはスキャン時にエラー通知 |
| `extensions` | `string[]` | 必須 | 検索対象とする拡張子一覧（例: `[".xlsx", ".xlsm"]`） | 最低1要素必須（全解除禁止） |
| `isComposing` | `boolean` | 必須 | 現在日本語IME等の文字変換中であるか | `compositionstart` で true、`compositionend` で false |
| `compositionEndTime` | `number` | 必須 | 直近の `compositionend` 発火タイムスタンプ（ミリ秒） | Date.now() による記録 |

#### 状態遷移ルール（IME & Enter）
```
[未入力 / 入力中]
  │
  ├─(文字入力 / compositionstart)─► [IME変換中 (isComposing: true)]
  │                                    │
  │                                    ├─(変換確定Enter)─► [IME確定 (compositionend)]
  │                                    │                    │ (タイムスタンプ記録、検索は行わない)
  │                                    │                    ▼
  │                                    └────────────────► [確定入力済 (isComposing: false)]
  │                                                         │
  ├─(keyword欄でEnter & !isScanning)────────────────────────┼─► [検索実行 (onSearch)]
  │                                                         │
  └─(target_dir欄でEnter)───────────────────────────────────┴─► [無視 (preventDefault & フォーカス維持)]
```

---

### 2. 検索進行状態（SearchProgressState）

ステータスバーに表示される検索フェーズおよび進行状況。

| フィールド名 | 型 | 必須 | 説明 | 許容値 / 形式 |
|---|---|---|---|---|
| `phase` | `SearchPhase` | 必須 | 現在の検索フェーズ | `'idle'`, `'scanning'`, `'processing'`, `'completed'`, `'cancelled'` |
| `targetDir` | `string` | 必須 | 探索中のフォルダパス | 探索中フェーズでステータス欄に表示 |
| `currentFile` | `string` | 任意 | 現在解析中のファイル名 | 解析フェーズで表示（例: `Auditing_2026.xlsm`） |
| `scannedFiles` | `number` | 必須 | 走査完了したファイル数 | 0 以上の整数 |
| `totalFiles` | `number` | 必須 | 検出された総対象ファイル数 | 0 以上の整数 |
| `matchCount` | `number` | 必須 | 一致したセル件数 | 0 以上の整数 |
| `elapsedSeconds` | `number` | 必須 | 検索開始からの経過時間（秒） | 小数点第2位まで表示 |

#### フェーズ遷移
```
[idle] ──(検索開始)──► [scanning (探索中)] ──(ファイル検出)──► [processing (解析中)]
  ▲                         │                                        │
  │                         ├────────────────────────────────────────┤
  │                         │ (キャンセル)                            │ (走査完了)
  │                         ▼                                        ▼
  └─────────────────── [cancelled (中断)]                   [completed (完了)]
```

---

### 3. セルプレビューグリッド表示モデル（CellPreviewGridView）

周辺セルプレビューグリッドの描画用マトリクスデータ。

| フィールド名 | 型 | 必須 | 説明 |
|---|---|---|---|
| `sheet_name` | `string` | 必須 | 表示中の一致セルが存在するシート名 |
| `sheets_in_workbook` | `string[]` | 必須 | ブック内に存在する全ワークシート名一覧 |
| `columns` | `PreviewColumn[]` | 必須 | 表示対象の列情報（列記号ラベル、キー、列番号） |
| `rows` | `PreviewRow[]` | 必須 | 表示対象の行データリスト（最低30行を確保） |

#### `PreviewRow`
| フィールド名 | 型 | 説明 |
|---|---|---|
| `row_number` | `number` | 行番号（1始まり） |
| `cells` | `Record<string, PreviewCell>` | 各列キーに対応するセル情報マップ |

#### `PreviewCell`
| フィールド名 | 型 | 説明 |
|---|---|---|
| `value` | `string` | セルの表示文字列（数値、日付、テキスト） |
| `formula` | `string \| null` | 数式文字列（例: `=SUM(C8:E8)`、数式がない場合はnull） |
| `is_target` | `boolean` | 検索キーワードと一致した目的のセルかどうかのフラグ |
