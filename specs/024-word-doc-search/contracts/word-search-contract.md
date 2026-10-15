# Phase 1 インターフェース契約: MS Word ドキュメント検索

**機能ブランチ**: `024-word-doc-search`  
**日付**: 2026-10-15  
**仕様書**: [spec.md](../spec.md) | **データモデル**: [data-model.md](../data-model.md)

---

## 1. Tauri IPC コマンド契約

### 1.1 `get_word_preview` (新設コマンド)

Word 文書（`.doc`, `.docx`, `.docm`）の全文テキストおよび段落メタデータを取得します。

- **コマンド名**: `get_word_preview`
- **ハンドラー定義 (Rust)**:
  ```rust
  #[tauri::command]
  pub async fn get_word_preview(
      file_path: String,
      target_paragraph: u32,
  ) -> Result<WordPreviewPayload, CommandError>
  ```
- **リクエスト引数 (JSON)**:
  ```json
  {
    "filePath": "C:\\Documents\\Sample.docx",
    "targetParagraph": 12
  }
  ```
- **レスポンスペイロード (200 OK / Result::Ok)**:
  ```json
  {
    "file_path": "C:\\Documents\\Sample.docx",
    "file_name": "Sample.docx",
    "target_paragraph": 12,
    "total_paragraphs": 45,
    "paragraphs": [
      {
        "index": 1,
        "text": "第1章 プロジェクト概要",
        "is_target": false
      },
      {
        "index": 12,
        "text": "本仕様書における検索対象キーワードに関する説明...",
        "is_target": true
      }
    ]
  }
  ```
- **エラーレスポンス (Result::Err)**:
  ```json
  {
    "code": "PreviewFailed",
    "message": "Failed to extract Word document preview: File is corrupted or protected"
  }
  ```

---

### 1.2 `search_excel` / `execute_search` (既存コマンド契約の拡張)

- **拡張点**:  
  `SearchQuery.extensions` に `"docx"`, `"docm"`, `"doc"` が含まれる場合、検索エンジンは Word 解析パイプラインへルーティングする。
- **リクエスト引数例**:
  ```json
  {
    "keyword": "売上計画",
    "target_dir": "D:\\Projects",
    "extensions": ["xlsx", "docx", "docm", "doc"],
    "match_case": false,
    "use_regex": false,
    "include_comment": true,
    "include_shape": true,
    "include_formula": false,
    "include_hidden": false
  }
  ```

---

## 2. Tauri IPC イベント契約

### 2.1 `search-match` (イベント更新)

Word 文書内でキーワードが検出された際にフロントエンドへ発火されるイベント。

- **イベント名**: `search-match`
- **ペイロード (SearchMatch)**:
  ```json
  {
    "id": 104,
    "file_name": "企画書_2026.docx",
    "full_path": "D:\\Projects\\企画書_2026.docx",
    "sheet_name": "Word",
    "cell_address": "-",
    "row_index": 15,
    "col_index": 0,
    "col_name": "",
    "match_type": "WordText",
    "shape_name": null,
    "sheet_hidden": false,
    "snippet": "...当期の<mark>売上計画</mark>における重点項目...",
    "full_content": "当期の売上計画における重点項目は以下の通りとする。",
    "formula": null,
    "sheets_in_workbook": ["Word"]
  }
  ```

---

## 3. 定数定義契約（憲法 Principle II 準拠）

すべての固定値・拡張子・コマンド文字列は定数として一元管理する。

### Rust (`src-tauri/src/constants.rs` & `crates/core/src/constants.rs`)
```rust
// コマンド名
pub const CMD_GET_WORD_PREVIEW: &str = "get_word_preview";

// 拡張子
pub const EXT_DOCX: &str = "docx";
pub const EXT_DOCM: &str = "docm";
pub const EXT_DOC: &str = "doc";
pub const WORD_EXTENSIONS: &[&str] = &[EXT_DOCX, EXT_DOCM, EXT_DOC];

// シート・セル固定表示ラベル
pub const WORD_SHEET_LABEL: &str = "Word";
pub const WORD_CELL_DEFAULT_ADDRESS: &str = "-";
```

### TypeScript (`src/constants/index.ts`)
```typescript
// コマンド
export const COMMANDS = {
  // ...既存
  GET_WORD_PREVIEW: "get_word_preview",
} as const;

// 拡張子
export const WORD_EXTENSIONS = {
  DOCX: "docx",
  DOCM: "docm",
  DOC: "doc",
  DEFAULT_LIST: ["docx", "docm", "doc"],
} as const;

// UI 表示
export const WORD_UI_CONSTANTS = {
  SHEET_NAME: "Word",
  CELL_ADDRESS_PLACEHOLDER: "-",
} as const;
```

---

## 4. UI コンポーネント契約

### `WordTextPreview` コンポーネント契約
- **Props**:
  ```typescript
  interface WordTextPreviewProps {
    payload: WordPreviewPayload | null;
    isLoading: boolean;
    searchKeyword: string;
    onSelectParagraph?: (index: number) => void;
  }
  ```
- **振る舞い**:
  1. `isLoading` が `true` の場合、スケルトンまたはローディングスピナーを表示。
  2. `payload` の `paragraphs` を仮想スクロールまたはリスト表示し、各段落の左端に段落番号 `¶ {index}` を表示。
  3. `is_target` が `true` の段落へマウント時または選択変更時に自動スクロール。
  4. `searchKeyword` を含むテキスト部分を `<mark className="bg-amber-400 text-black px-0.5 rounded">` でハイライト。
