# Tauri capability configuration

## 処理内容

`default.json` はアプリ画面が利用する Tauri プラグイン権限をまとめる。`i18n:default` は同梱翻訳カタログの読込と表示言語の参照・変更に使う。

## 入力・出力

Tauri は `default.json` を起動時に読み込み、宣言されたプラグイン権限だけを WebView に公開する。

## エラー

不正な JSON や未宣言の権限は Tauri の起動・ビルド検証を失敗させる。

## 変更履歴

- v1.0.0 (2026-09-26, Codex): i18n 権限追加の説明を記録。
