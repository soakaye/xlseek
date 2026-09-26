# Implementation Plan: UI項目調整 (UI Adjustments)

**Branch**: `soakaye/fixui` | **Date**: 2026-09-26 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/005-ui-adjustments/spec.md`

## Summary

検索キーワード欄における日本語入力(IME)確定時のEnter誤爆発火の防止、フォルダ入力欄におけるEnterキー無効化（フォーカス維持・検索非連動）、対象拡張子のトグルボタンスタイリングと全解除防止、ステータスバー進捗バーの配置変更（メッセージ手前・固定幅）およびフォルダ探索中フェーズの可視化、周辺セルプレビューグリッドの縦横固定見出し（Freeze Panes）と十分なサンプル行（30行以上）拡充による操作性・検証性の改善を実施する。また、憲章原則およびモック同期ルール（Iron Law）に基づき、React実装（`src/`）とデザインモック（`design/mainui/index.html`）を1:1で完全同期させる。

---

## Technical Context

**Language/Version**: TypeScript 5.5 (React 18.3), HTML5 / CSS3 (Tailwind CSS 3.4), Rust 2021 (Tauri v2)

**Primary Dependencies**: React 18, Lucide React / Lucide Icons CDN, Tailwind CSS, `@tauri-apps/api`, `@tauri-apps/plugin-dialog`

**Storage**: ローカルファイルシステム（ファイル直接読み込み、検索結果メモリ保持）

**Testing**: `npm run build` (tsc & vite build), `cargo test`, `python3 html.parser`, ブラウザ手動検証（Chrome / Safari macOS）

**Target Platform**: macOS (WebKit / Safari / WKWebView), Windows (WebView2 / Chromium), Linux

**Project Type**: デスクトップGUIアプリケーション（Tauri v2）およびスタンドアローンプロトタイプ（HTML/JS）

**Performance Goals**:
- キーボードイベント処理遅延 < 16ms (60fps)
- グリッドの縦横スクロール描画 60fps 維持
- フォルダスキャン開始時の探索中表示レイテンシ < 100ms

**Constraints**:
- WebKit (macOS Safari) 特有の `compositionend` 先行発火問題への対処
- 外部CDN依存の維持（スタンドアローンHTMLプロトタイプ）
- 憲章原則II（定数の一元化）の厳格な遵守

**Scale/Scope**:
- プレビュー行数: 最低30行〜40行の同時描画
- 対象拡張子: 4種類（.xlsx, .xlsm, .xlsb, .xls）

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| 憲章原則 | 準拠状況 | 評価詳細 |
|---|---|---|
| **I. 自然かつ正確な日本語出力** | **PASS** | UIメッセージ（「探索中」「フォルダスキャン中」「周辺セルプレビュー」等）はすべて自然な日本語であり、不要な特殊トークンは含まれない。 |
| **II. 定数の外部抽出とハードコードの禁止** | **PASS** | UIテキスト、拡張子リスト、ツールチップ等はすべて `src/constants/index.ts`（`UI_MESSAGES`, `FILE_EXTENSIONS` 等）より参照。 |
| **III. 厳格なヘッダコメントとドキュメンテーション** | **PASS** | 変更対象ファイル（`SearchBar.tsx`, `StatusBar.tsx`, `SpreadsheetGrid.tsx`, `index.html`）に目的・構成・例外・変更履歴を網羅した4要素ヘッダコメントを付与。 |
| **IV. 責務に応じたモジュール分割と標準スタイル準拠** | **PASS** | 検索入力、ステータス表示、プレビューグリッドが各独立コンポーネントとして分割され、Tailwindおよび標準スタイルに準拠。 |
| **V. 堅牢なエラーハンドリングとテスト検証** | **PASS** | イベントハンドラにおける null ガード、IME判定の二重防御、拡張子の全解除防止ガードを導入し、ビルド・テストで検証。 |

---

## Project Structure

### Documentation (this feature)

```text
specs/005-ui-adjustments/
├── spec.md              # 仕様書（/speckit-specify および /speckit-clarify 出力）
├── plan.md              # 実装計画書（本ファイル: /speckit-plan 出力）
├── research.md          # Phase 0: 技術調査と意思決定
├── data-model.md        # Phase 1: データモデルと状態遷移
├── quickstart.md        # Phase 1: 検証シナリオと起動手順
├── contracts/           # Phase 1: UIインターフェース契約
│   ├── search-bar-events.md
│   └── spreadsheet-grid-layout.md
└── checklists/
    └── requirements.md  # 仕様品質検証チェックリスト
```

### Source Code Layout

```text
src/
├── components/
│   ├── common/
│   │   └── StatusBar.tsx            # プログレスバー固定幅配置＆フォルダ探索中表示
│   ├── preview/
│   │   ├── FormulaBar.tsx           # 数式・番地表示
│   │   └── SpreadsheetGrid.tsx      # 縦横スクロール＆Freeze Panes固定見出し
│   └── search/
│       └── SearchBar.tsx            # IME確定Enter誤爆防止、フォルダEnter無効化、拡張子トグル
├── constants/
│   └── index.ts                     # UI文言・レイアウト・拡張子定数定義
└── App.tsx                          # レイアウト統合

design/
└── mainui/
    └── index.html                   # スタンドアローンHTMLプロトタイプ（30行以上プレビューデータ＆全UI同期）

src-tauri/
└── src/
    └── constants.rs                 # バックエンド定数
```

**Structure Decision**: 既存の Tauri + React コンポーネント構造（`src/components/`）およびプロトタイプ（`design/mainui/index.html`）を直接更新し、新機能専用の別ディレクトリへの分割は行わず、既存アーキテクチャに完全に統合する。

---

## Complexity Tracking

*Constitution Check 違反なし（記録不要）*
