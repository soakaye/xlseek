<!-- 処理内容: 検索履歴とパス補完の実装計画。入力: spec.md と調査結果。出力: 設計・検証方針。エラー: 未解決事項または憲章違反があれば計画を停止。変更履歴: v1.0.0 2026-09-27 Codex 初版。v1.1.0 2026-09-27 Codex 保存上限と Windows パス検証を更新。v1.2.0 2026-09-27 Codex 検索受付前検証と非同期パス処理を明確化。 -->
# Implementation Plan: 検索履歴とディレクトリ補完

**Branch**: `010-search-history-path-completion` | **Date**: 2026-09-27 | **Spec**: [spec.md](spec.md)

**Input**: `specs/010-search-history-path-completion/spec.md`

## Summary

検索テキストとディレクトリの履歴を別々に端末内へ保存し、検索欄の専用ボタンから選べるようにする。設定画面には両履歴共通の保存件数を加える。ディレクトリの補完は、入力中の OS に応じた絶対パスまたはホーム省略表記の親直下をバックエンドが調べて提示する。Windows ではドライブ付き・UNC 絶対パスと `~\` に対応する。検索開始コマンドは処理を受け付ける前にディレクトリと正規表現を検証し、その成功後にのみ元の入力表記を履歴へ記録する。

## Technical Context

**Language/Version**: TypeScript 5.5、React 18、Rust 2021 edition

**Primary Dependencies**: 既存の Tauri v2、React、Vitest、標準ファイルシステム機能。追加依存なし。

**Storage**: 既存の表示言語設定と同じ WebView `localStorage`。履歴と上限を単一の保存値として扱い、外部送信はしない。

**Testing**: `npm test`、`npm run lint`、`npm run build`、`cargo test`、`cargo clippy --all-targets -- -D warnings`、`cargo fmt --check`、デスクトップ画面での操作確認。

**Target Platform**: 既存の Windows、macOS、Linux デスクトップアプリ。

**Project Type**: React と Rust による Tauri デスクトップアプリ。

**Performance Goals**: 読み取り可能な一般的なローカルの親ディレクトリでは入力から 1 秒以内に候補を提示する。ネットワーク共有を含め、列挙は画面を停止させない。

**Constraints**: 完全オフライン、最小権限、IME 変換を阻害しない、履歴は各 0～50 件、Windows のドライブ付き・UNC 絶対パス、日英表示、憲章の定数・4要素ヘッダ・テスト要件。

**Scale/Scope**: 入力欄 2 つ、設定項目 1 つ、親直下のディレクトリ候補のみ。検索オプションや結果の保存、再帰的な候補探索は対象外。

## Constitution Check

*GATE: Phase 0 着手前と Phase 1 完了後に確認。*

| 原則・制約 | Phase 0 前 | Phase 1 後の設計確認 |
|---|---|---|
| I. 指定言語 | 合格。仕様は日本語、画面は既存の日英表示に従う | 合格。追加文言は既存の日英カタログに置き、ログは既存方針の英語とする |
| II. 定数の外部抽出 | 合格。保存キー・上限・遅延・コマンド名・表示キーは定数化する | 合格。TypeScript は `src/constants/index.ts`、Rust は `src-tauri/src/constants.rs` を参照し、利用箇所にコメントを置く |
| III. 4要素ヘッダ | 合格。変更するファイルと関数に適用する | 合格。処理、入出力、エラー、変更履歴を追加・更新する |
| IV. 責務分割・標準スタイル | 合格。履歴管理、検索画面、パス処理の責務を分ける | 合格。既存モジュールへ薄く接続し、型検査・ESLint・Clippy・rustfmt を通す |
| V. エラー処理・テスト | 合格。保存失敗、無効なパス、候補取得失敗を扱う | 合格。履歴の件数・永続化、パス解決・候補列挙、IME・キーボード操作をテストする |
| オフライン・最小権限 | 合格。端末内保存と既存 IPC を使う | 合格。新たなプラグイン権限や外部サービスは不要 |

**ゲート判定**: 憲章違反と未解決の技術的明確化事項はない。性能は画面の非同期性と代表的なディレクトリで検証し、極端に大きなディレクトリの処理時間は実測で確認する。

## Project Structure

### Documentation (this feature)

```text
specs/010-search-history-path-completion/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
└── contracts/
    └── search-history-path-contract.md
```

### Source Code (repository root)

```text
src/
├── App.tsx                              # 検索・履歴・設定の接続
├── constants/index.ts                   # 保存キー、件数、補完遅延、IPC 名
├── hooks/
│   ├── useSearch.ts                     # 検索開始成功時に履歴を通知
│   └── useSearchHistory.ts              # 履歴読込・更新・保存と件数変更
└── components/
    ├── search/SearchBar.tsx             # 履歴・補完リストと IME 操作
    └── settings/SettingsDialog.tsx      # 保存件数入力
src-tauri/
├── locales/{ja,en}.yml                  # 追加する日英文言
└── src/
    ├── constants.rs                     # パス補完用の固定値
    ├── lib.rs                           # IPC 登録
    ├── commands/search_cmd.rs           # 検索事前検証と補完 IPC
    └── search/
        ├── mod.rs                       # パス処理モジュールの公開
        └── path.rs                      # ホーム省略表記と Windows 絶対パスの解決、検証、親直下の候補取得
tests/                                   # 履歴・画面操作の Vitest テスト
design/mainui/index.html                 # SearchBar・設定画面の試作を同期
```

**Structure Decision**: 履歴は既存の表示言語設定と同じ保存手段を使い、追加パッケージを導入しない。`start_search` の成功応答を「検索開始できた」の境界にするため、ディレクトリの読取確認をブロッキング用スレッドで完了させ、正規表現も検索処理起動前に検証する。検索エンジン内の検証は、検証後にファイルシステムが変わる場合の防御として残す。

## Design Sequence

1. パス解決・事前検証・補完 IPC と Rust 単体テストを用意する。パスの読取確認と候補列挙はブロッキング用スレッドで行い、候補は整列してから件数制限する。既存検索コマンドの成功応答を履歴追加の根拠にする。
2. 履歴の保存値検証・重複排除・上限変更を実装して単体テストを通す。
3. 検索画面と設定画面を接続し、日英文言、IME、キーボード操作、失敗通知を検証する。
4. `design/mainui/index.html` を同期し、全品質ゲートとデスクトップでの手順を確認する。
