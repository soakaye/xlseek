# Implementation Plan: i18n 基盤を Tauri プラグインへ移行する

**Branch**: `009-migrate-tauri-i18n` | **Date**: 2026-09-26 | **Spec**: [spec.md](spec.md)

**Input**: `specs/009-migrate-tauri-i18n/spec.md`

## Summary

既存の言語設定と利用者向け動作を保ち、翻訳文言の正本を `src-tauri/locales/` に移す。Tauri と React は `tauri-plugin-i18n` を通じて同じ翻訳資源を利用する。プラグインが英語へ自動代替しないため、文言単位の英語フォールバックを薄いアダプタで補う。保存済み設定、端末言語の判定、英語ログを維持する。

## Technical Context

**Language/Version**: TypeScript 5.5、React 18、Rust 2021 edition

**Primary Dependencies**: Tauri v2、`tauri-plugin-i18n` 2.0.2、`@razein97/tauri-plugin-i18n` 2.0.1、既存の `@tauri-apps/plugin-os`、YAML 読込用 `yaml` 2.x

**Storage**: 既存の WebView `localStorage` キー `exlgrep.language`、値 `default|ja|en`。翻訳正本は `src-tauri/locales/en.yml` と `ja.yml`。

**Testing**: Vitest の既存単体・UI テスト、Rust 単体テスト、`npm test`、`npm run lint`、`npm run build`、`cargo test`、`cargo clippy --all-targets -- -D warnings`、`cargo fmt --check`、Tauri 実機確認

**Target Platform**: Windows、macOS、Linux 向けの既存 Tauri デスクトップアプリ

**Project Type**: React と Rust による Tauri デスクトップアプリ

**Performance Goals**: 利用者による切替操作は30秒以内。検索性能に変更なし。

**Constraints**: 完全オフライン、保存値の後方互換、検索状態と利用者データの保持、文言単位の英語フォールバック、英語ログ、最小権限、4要素ヘッダ・定数の憲章準拠。

**Scale/Scope**: 表示は日本語と英語の2言語、設定はデフォルトを含む3選択肢。画面、通知、エラー、メニュー、CSV・Excel のアプリ作成文言が対象。

## Constitution Check

*GATE: Phase 0 着手前と Phase 1 完了後に確認。*

| 原則・制約 | Phase 0 前 | Phase 1 後の設計確認 |
|---|---|---|
| I. 指定言語の優先 | 合格。日英表示と英語ログを維持 | 合格。表示文言はプラグイン資源、ログは英語固定 |
| II. 定数の外部抽出 | 合格。翻訳は言語ファイルへ、その他の固定値は既存定数へ | 合格。独自辞書を削除し、翻訳キーとイベント名は定数で参照 |
| III. 4要素ヘッダ | 合格。変更するコードと翻訳ファイルに適用する | 合格。コードと翻訳ファイルの説明・入出力・失敗・履歴を記録 |
| IV. 分割・標準スタイル | 合格。翻訳アダプタ、言語状態、Rust メニュー・出力を責務分離 | 合格。型検査、ESLint、Clippy、rustfmt を品質ゲートに設定 |
| V. エラー処理・テスト | 合格。プラグインと端末言語取得・保存失敗を扱う | 合格。失敗時の英語表示、保存互換、画面状態、出力をテスト対象に設定 |
| オフライン・プライバシー | 合格。翻訳をアプリに同梱 | 合格。利用者データを翻訳・外部送信せず、権限を限定 |

**ゲート判定**: 未解決の技術事項なし。翻訳ファイルは4要素コメントを記述できる YAML を使用する。

## Project Structure

### Documentation (this feature)

```text
specs/009-migrate-tauri-i18n/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
└── contracts/
    └── localization-contract.md
```

### Source Code (repository root)

```text
Cargo.toml                       # プラグインの翻訳同梱に必要なワークスペース
Cargo.lock                       # ワークスペース化後のロックファイル
src/
├── constants/index.ts           # 非翻訳定数と翻訳キー
├── i18n.ts                       # プラグイン利用と英語フォールバック
├── locale-core.ts                # 保存値と端末言語の検証
├── hooks/useLocale.tsx           # 初期化と設定切替
├── App.tsx                       # 通知とメニューの言語連動
└── components/                  # 既存の描画をプラグイン翻訳へ接続
src-tauri/
├── Cargo.toml
├── locales/{en,ja}.yml          # 4要素コメント付き翻訳正本
├── capabilities/default.json    # i18n 権限
└── src/
    ├── lib.rs                   # プラグイン登録とメニュー更新
    ├── constants.rs             # 非翻訳定数と翻訳キー
    ├── i18n.rs                  # Rust 側の翻訳フォールバック
    └── export/                  # 要求言語による見出し・一致種別
tests/                            # 保存互換、翻訳欠落、画面切替
design/mainui/index.html         # UI 変更があれば試作を同期
```

**Structure Decision**: 008 の画面構造と保存キーを維持する。既存コンポーネントへの変更を絞るため翻訳参照の薄い互換アダプタを置くが、日英辞書を二重に保持しない。翻訳キー・差込値を通知の状態として保持し、文字列の逆引き置換をなくす。Rust の出力は要求言語から翻訳文言を選び、処理中の言語変更から切り離す。フロントエンドの読込失敗時は `yaml` で同じ英語正本を解析し、最終フォールバックには英語正本で必須とする `common.translationUnavailable` を使う。

## Migration Sequence

1. Cargo ワークスペースと翻訳ファイルを追加し、プラグインが日英資源を同梱することを確認する。現行の Rust lockfile はワークスペースの位置へ移す。
2. フロントエンドの翻訳アダプタと起動時の初期化・フォールバックを実装し、既存保存値の互換テストを通す。
3. 画面文言・通知を翻訳キー参照へ移し、独自日英辞書と文字列逆引き処理を削除する。
4. Rust のメニュー、CSV、Excel を同じ翻訳資源へ移し、言語切替・出力言語・英語ログを確認する。
5. 自動品質ゲートと実機シナリオを実行し、翻訳資源同梱と再起動後の設定を確認する。
