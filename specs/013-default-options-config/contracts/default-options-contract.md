# 契約仕様書 (Interface Contracts): デフォルト動作設定 & 出力

## 1. フロントエンド内部契約 (TypeScript)

### 1.1 デフォルト検索オプション型 & 検証関数 (`src/types/defaultOptions.ts` または `src/default-options-core.ts`)

```typescript
export interface DefaultSearchOptions {
  match_case: boolean;
  use_regex: boolean;
  include_formula: boolean;
  include_shape: boolean;
  include_comment: boolean;
  include_hidden: boolean;
  extensions: string[];
}

export function isDefaultSearchOptions(value: unknown): value is DefaultSearchOptions;
export function loadDefaultSearchOptions(): DefaultSearchOptions;
export function saveDefaultSearchOptions(options: DefaultSearchOptions): boolean;
export function resetDefaultSearchOptions(): DefaultSearchOptions;
```

### 1.2 設定ダイアログ プロパティ拡張 (`SettingsDialogProps`)

```typescript
interface SettingsDialogProps {
  isOpen: boolean;
  onClose: () => void;
  // 言語設定
  preference: LanguagePreference;
  language: DisplayLanguage;
  onSelect: (value: LanguagePreference) => void;
  // 履歴上限設定
  maxEntries: number;
  onSetMaxEntries: (value: number) => void;
  // デフォルト検索オプション設定（新規追加）
  defaultOptions: DefaultSearchOptions;
  onSaveDefaultOptions: (newOptions: DefaultSearchOptions) => void;
}
```

---

## 2. エクスポート IPC 契約 (Tauri Command)

### 2.1 コマンド名: `export_results`

- **呼び出し側**: フロントエンド (`StatusBar.tsx`)
- **受取側**: Rust バックエンド (`commands::export_results`)

#### リクエストペイロード (`ExportRequest`)

```json
{
  "request": {
    "format": "csv" | "xlsx",
    "output_path": "C:\\Users\\example\\Desktop\\results.csv",
    "language": "ja" | "en",
    "items": [
      {
        "id": 1,
        "file_name": "Report.xlsx",
        "full_path": "C:\\Data\\Report.xlsx",
        "sheet_name": "Sheet1",
        "cell_address": "B12",
        "row_index": 11,
        "col_index": 1,
        "col_name": "B",
        "match_type": "CellValue",
        "shape_name": null,
        "sheet_hidden": false,
        "snippet": "売上合計",
        "full_content": "売上合計",
        "formula": null,
        "sheets_in_workbook": ["Sheet1"]
      }
    ]
  }
}
```

#### レスポンス
- **成功時**: `Result<(), AppError>` → `()`
- **失敗時**: `Result::Err(AppError)` → フロントエンドでトーストエラー通知
