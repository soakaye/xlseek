# Implementation Plan: アプリ利用パッケージの著作権・ライセンス表示 (About Dialog & Package Licenses)

**Branch**: `006-about-dialog-licenses` | **Date**: 2026-09-26 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/006-about-dialog-licenses/spec.md`

---

## Summary

アプリケーションで利用しているすべての本番ランタイムサードパーティ製パッケージ（バックエンドのRust実行時クレートおよびフロントエンドのnpmプロダクション依存ライブラリ）の著作権・ライセンス情報を収集するスクリプト `scripts/generate-licenses.py` を策定し、静的リソース `src/constants/licenses.json` としてアプリケーションに同梱する。

メイン画面最下部のステータスバー最右端（CSV/Excel出力ボタンの右隣）に配置される情報アイコン（`Info`）ボタンから呼び出される左右2ペイン（マスター／ディテール）構成のAboutダイアログ（`src/components/about/AboutDialog.tsx`）を新設する。ダイアログ内では、アプリケーションの基本情報・著作権表示の確認、全パッケージ一覧のリアルタイムキーワード検索、選択パッケージの著作権者表記および正規ライセンス全文の独立スクロール閲覧、ならびにワンクリックでのクリップボードコピー（トースト通知付き）を提供する。

また、憲章原則およびモック同期ルール（Iron Law）に基づき、スタンドアローンHTMLプロトタイプ（`design/mainui/index.html`）にも同等のAboutダイアログとライセンス閲覧機能を1:1で完全同期実装する。

---

## Technical Context

**Language/Version**: TypeScript 5.5 (React 18.3), Rust 2021 (Tauri v2), Python 3.10+ (ライセンス収集スクリプト用)

**Primary Dependencies**: React 18, Lucide React (`lucide-react`), Tailwind CSS 3.4, `@tauri-apps/api`, `@tauri-apps/plugin-dialog`

**Storage**: 静的JSONデータバンドル（`src/constants/licenses.json`。Viteビルド時にインラインバンドルされ、完全オフラインでメモリ展開）

**Testing**:
- フロントエンド型検証 & ビルド: `npm run build` (`tsc --noEmit` & `vite build`)
- バックエンド静的解析 & フォーマット: `cargo clippy --all-targets -- -D warnings`, `cargo fmt --check`
- ライセンスJSONスキーマ検証: `python3 scripts/generate-licenses.py`
- ブラウザ手動検証（Chrome / Safari macOS, スタンドアローンHTMLモック `design/mainui/index.html`）

**Target Platform**: macOS (WebKit / WKWebView), Windows (WebView2 / Chromium), Linux

**Project Type**: デスクトップGUIアプリケーション（Tauri v2）およびスタンドアローンプロトタイプ（HTML/JS）

**Performance Goals**:
- ダイアログ起動応答時間: < 100ms (SC-001)
- 検索キーワード入力による絞り込み応答時間: < 200ms (SC-004)
- リストおよびライセンス本文のスクロール描画: 60fps 維持

**Constraints**:
- 完全オフライン動作保証（外部通信一切不可、SC-005）
- 憲章原則I（自然かつ正確な日本語UI表示。原著作者のライセンス本文・著作権表示は法的一貫性のため原文保持）
- 憲章原則II（定数の一元管理。UIラベル、ダイアログタイトル、定型文はすべて `src/constants/index.ts` に集約）
- 憲章原則III（網羅的な4要素ヘッダコメントの付与）
- 憲章原則IV（責務に応じたモジュール分割）
- スタンドアローンHTMLモック（`design/mainui/index.html`）との完全同期（Iron Law）

**Scale/Scope**:
- 対象パッケージ数: 約80〜150件（Rust実行時クレート推移的依存関係 + npmプロダクション依存パッケージ）
- 開発専用依存関係（devDependencies / build-dependencies）は除外

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| 憲章原則 | 準拠状況 | 評価詳細 |
|---|---|---|
| **I. 自然かつ正確な日本語出力** | **PASS** | ダイアログの全UIフレーム、見出し（「Excel Grep について」「オープンソースライセンス」等）、操作ボタン、検索プレースホルダー、案内文、トースト通知はすべて自然な日本語で統一し、`<PAD>` 等の不要トークンを排除。ライセンス本文および著作権表記は法的な正確性を保つため原著作者の原文（英語）のまま表示。 |
| **II. 定数の外部抽出とハードコードの禁止** | **PASS** | ダイアログ見出し、アプリ概要、著作権文字列、検索プレースホルダー、ボタンラベル、トーストメッセージ等はすべて `src/constants/index.ts`（`ABOUT_DIALOG_CONSTANTS`）へ抽出し、コンポーネント側で定数参照コメントを明記。 |
| **III. 厳格なヘッダコメントとドキュメンテーション** | **PASS** | 新設・変更する全ファイル（`AboutDialog.tsx`, `PackageList.tsx`, `PackageDetail.tsx`, `StatusBar.tsx`, `generate-licenses.py` 等）に処理内容、引数/戻り値型、エラー/例外、変更履歴を含む4要素ヘッダコメントを付与。 |
| **IV. 責務に応じたモジュール分割と標準スタイル準拠** | **PASS** | `AboutDialog` をダイアログシェル、パッケージ一覧（`PackageList`）、詳細ビュー（`PackageDetail`）に適切に分割し、単一責任の原則（SRP）およびTailwind標準スタイルに準拠。 |
| **V. 堅牢なエラーハンドリングとテスト検証** | **PASS** | クリップボードAPI失敗時の例外捕捉とトーストエラー通知、ライセンスデータ欠損時のフォールバック、検索0件時のプレースホルダー表示を実装し、ビルド・型チェック・手動検証で正当性を確認。 |

---

## Project Structure

### Documentation (this feature)

```text
specs/006-about-dialog-licenses/
├── spec.md              # 仕様書（/speckit-specify および /speckit-clarify 出力）
├── plan.md              # 実装計画書（本ファイル: /speckit-plan 出力）
├── research.md          # Phase 0: 技術調査と意思決定
├── data-model.md        # Phase 1: データモデルと状態遷移
├── quickstart.md        # Phase 1: 検証シナリオと起動手順
├── contracts/           # Phase 1: インターフェース契約
│   ├── license-schema.json          # バンドルJSONスキーマ仕様
│   └── about-dialog-interface.md    # UIコンポーネントプロパティ仕様
└── checklists/
    └── requirements.md  # 仕様品質検証チェックリスト
```

### Source Code Layout

```text
scripts/
└── generate-licenses.py             # Rustクレートおよびnpm依存ライセンス自動収集スクリプト

src/
├── components/
│   ├── about/
│   │   ├── AboutDialog.tsx          # Aboutダイアログ最上位モーダル（タブ切り替え・シェル）
│   │   ├── PackageList.tsx          # 左ペイン: 検索入力バー＋パッケージ一覧リスト
│   │   └── PackageDetail.tsx        # 右ペイン: 選択パッケージ情報＋ライセンス本文＋コピー
│   └── common/
│       └── StatusBar.tsx            # 最右端にAboutダイアログ起動ボタン（Infoアイコン）を追加
├── constants/
│   ├── index.ts                     # ABOUT_DIALOG_CONSTANTS 定数定義の追加
│   └── licenses.json                # 静的バンドルライセンスデータ（generate-licenses.py 出力）
├── types/
│   └── license.ts                   # PackageLicenseRecord, AboutDialogState 型定義
└── App.tsx                          # AboutDialog のマウントと表示状態管理

design/
└── mainui/
    └── index.html                   # スタンドアローンHTMLプロトタイプ（Aboutダイアログ・ライセンス閲覧完全同期）
```

**Structure Decision**: 既存のコンポーネントディレクトリ構造に `src/components/about/` を新設してAboutダイアログ関連コンポーネントを集約し、データは `src/constants/licenses.json` に静的バンドルする。これにより既存の検索・プレビューアーキテクチャに一切の負荷をかけず、クリーンに独立して機能を提供する。

---

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|---|---|---|
| なし | 憲章原則およびプロジェクト規約に完全準拠し、特例やアーキテクチャ違反は存在しない。 | N/A |
