# Phase 0 技術調査レポート: MS Word ドキュメント検索 (.doc, .docx, .docm)

**機能ブランチ**: `024-word-doc-search`  
**日付**: 2026-10-15  
**仕様書**: [spec.md](./spec.md)

---

## 1. 調査概要

本機能では、Excel Seek (`xlseek`) に MS Word 文書（`.doc`, `.docx`, `.docm`）の検索機能およびテキスト形式プレビューを追加します。  
以下の4つの技術課題について詳細な調査・選定を行いました：

1. Word 文書パーサー（OOXML `.docx`/`.docm` および OLE2 `.doc`）の実装方式
2. 検索エンジン・データモデル（`SearchMatch` / `MatchType`）との統合
3. テキストプレビュー用 IPC コマンドとフロントエンド描画アーキテクチャ
4. 設定画面およびメイン画面（拡張子トグルバー）の状態管理・UI連動

---

## 2. 技術選定と決定事項

### 課題 1: Word 文書パーサーの選定

- **決定**:  
  既存の依存クレート（`zip` + `quick-xml` + `cfb` + `encoding_rs`）を活用し、`crates/core/src/search/word/` 配下に自前で軽量・高信頼なテキスト抽出エンジンを実装する。
  - **`.docx` / `.docm`**: `zip::ZipArchive` でコンテナを開き、`word/document.xml`（本文）、`word/header*.xml` / `word/footer*.xml`（ヘッダー・フッター）、`word/comments.xml`（コメント）、`word/drawings/`（図形テキスト）から `quick_xml::Reader` で `<w:t>` 要素のテキストをストリーミング抽出する。
  - **`.doc`**: `cfb::CompoundFile` で OLE2 コンテナを開き、`WordDocument` ストリームおよび `Table` ストリームから FIB（File Information Block）のテキスト長・テキストオフセット情報を解析し、CP932 / Shift_JIS / UTF-16LE テキストを `encoding_rs` で文字化けなくデコードする。
- **選定理由**:  
  1. **ゼロ新規依存関係**: `Cargo.toml` にすでに `zip`, `quick-xml`, `cfb`, `encoding_rs` がすべて含まれており、新たな外部クレートの追加が一切不要（バイナリサイズ肥大化やビルド時間増・プラットフォーム不整合のリスクがゼロ）。
  2. **プロジェクト内の既存実績**: `crates/core/src/search/shape/ooxml.rs` および `xls.rs` で同様の ZIP/XML/CFB 解析パターンが確立されており、エラーハンドリング（パニック捕捉・ファイル破損耐性）の品質を均一に保てる。
  3. **要求仕様への最適性**: 本機能で必要なのはレイアウトや組版の完全レンダリングではなく、高速な「テキスト抽出とキーワード位置特定」であるため、最小限のパーサーが最も高速に動作する。
  4. **マクロ有効文書 `.docm` との完全な互換性**: `.docm` は Open Packaging Conventions (OPC) 準拠の ZIP アーカイブであり、XML 構造は `.docx` と同一（`word/document.xml`）。同一パーサーで完全に対応可能。
- **検討した代替案**:
  - *外部クレート `docx-rs` の利用*: 高機能だが非常に重厚で、書き込み用途が中心。`.doc`（バイナリ形式）には非対応であり、不採用。
  - *外部 CLI ツール（antiword や pandoc 等）の同梱*: 外部バイナリの同梱・プロセス起動が必要となり、Tauri デスクトップアプリとしての自己完結性・セキュリティ・配布性を損なうため不採用。

---

### 課題 2: 検索結果データ構造（SearchMatch / MatchType）との統合

- **決定**:  
  既存の `SearchMatch` 構造体および `MatchType` 列挙型を拡張して統合する。
  - `sheet_name` に `"Word"` を設定。
  - `cell_address` に `"-"`（または段落識別子 `"¶{paragraph_index}"`）を設定。
  - `row_index` に 1-based の段落番号（または行番号）を格納。
  - `col_index` に `0` を格納。
  - `match_type` に `MatchType::WordText` を追加（コメント時は `MatchType::Comment`、図形時は `MatchType::Shape` としても識別可能）。
- **選定理由**:  
  - 仮想スクロール結果一覧（`ResultTable`）、CSV / XLSX エクスポートエンジン、検索プログレス管理など、既存の全インフラストラクチャをそのままシームレスに利用可能。
  - 仕様書（FR-008）の要件「シート列に Word、セル列に行/段落番号または - を表示」に直接対応。
- **検討した代替案**:
  - *Word 専用の別構造体 `WordSearchResult` を作成して別ストリームで送信*: フロントエンドの結果テーブルやエクスポートの型分岐が複雑化し、コードベースの肥大化を招くため不採用。

---

### 課題 3: テキストプレビュー用 IPC コマンドと UI コンポーネント

- **決定**:  
  新規 Tauri IPC コマンド `get_word_preview` を追加し、フロントエンドに専用コンポーネント `WordTextPreview.tsx` を新設する。
  - **IPC コマンド**:
    ```rust
    pub async fn get_word_preview(
        file_path: String,
        target_paragraph: u32,
    ) -> Result<WordPreviewPayload, CommandError>
    ```
    - 返却データ `WordPreviewPayload`: 全文段落リスト（`paragraphs: Vec<String>`）、対象段落番号（`target_paragraph: u32`）。
  - **UI プレビュー表示**:
    - `src/components/preview/WordTextPreview.tsx` を新設。
    - 各段落に行/段落番号を表示。
    - 検索キーワードが含まれる箇所を `<mark>` タグでハイライト。
    - 表示時に該当段落へ自動スクロール（`element.scrollIntoView({ block: "center" })`）。
- **選定理由**:  
  - 既存の `SpreadsheetGrid` はセル（行/列）座標を前提としており、テキスト文書のプレビューには適さない。
  - 選択された結果の拡張子が `.doc`, `.docx`, `.docm` の場合に `WordTextPreview` へ自動で切り替えることで、自然な閲覧体験を提供できる。
- **検討した代替案**:
  - *Excel のグリッドに 1行1セルとして無理やり流し込む*: 操作性が著しく悪く、ユーザーの「テキスト形式表示」という要求に反するため不採用。

---

### 課題 4: 設定画面およびメイン画面の連動設計

- **決定**:  
  `DefaultSearchOptions` に `enable_word_search: boolean`（初期値 `false`）を追加。
  1. **設定ダイアログ（SettingsDialog）**:
     - 「検索設定」ページに「Word文書も検索対象に含める (.docx, .docm, .doc)」チェックボックスを新設。
     - 変更内容はローカルストレージ（`localStorage`）および Rust バックエンド設定に永続化。
  2. **メイン検索バー（SearchBar）**:
     - `enable_word_search` が `true` の場合、対象拡張子バーに `.docx`, `.docm`, `.doc` の3つのボタンが動的に追加表示される。
     - 検索ごとに個別にオン/オフが可能。
     - `enable_word_search` が `false` の場合は完全に非表示となり、既存の Excel 検索の表示・操作感をそのまま維持する。
- **選定理由**:  
  - ユーザーの明示的な回答（Q1: デフォルトOFF、Q1: メイン画面連動）に完全準拠。
  - Excel 専用として使いたいユーザーには邪魔にならず、Word 検索を使いたいユーザーには高い柔軟性を提供する。

---

## 3. 未解決事項（NEEDS CLARIFICATION）の確認

- すべての重要論点（初期値、対象範囲、プレビュー動作、UI連動）は仕様策定・明確化フェーズ（`/speckit-specify`, `/speckit-clarify`）で解決済み。
- **残存する未解決事項**: なし（Zero unresolved items）。
