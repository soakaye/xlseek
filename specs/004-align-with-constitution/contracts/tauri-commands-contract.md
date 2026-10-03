# Interface Contract: Tauri IPC Commands (Tauriコマンド通信規約)

**Feature Branch**: `004-align-with-constitution`  
**Date**: 2026-09-26  
**Status**: Completed  

## 1. 概要
本規約は、フロントエンドとバックエンド間の通信インターフェース（Tauri Commands）におけるエラー伝搬形式、コマンド名定数、および日本語エラーメッセージの保証規約を定義する。

## 2. コマンド一覧とシグネチャ

| コマンド名 | 引数 | 戻り値 | エラー返却形式 |
| :--- | :--- | :--- | :--- |
| `start_search` | `query: SearchQuery` | `Result<Vec<SearchMatch>, String>` | 日本語エラー文字列 |
| `cancel_search` | なし | `Result<(), String>` | 日本語エラー文字列 |
| `get_cell_preview` | `filePath: String, sheetName: String, row: u32, col: u32` | `Result<CellPreviewData, String>` | 日本語エラー文字列 |
| `open_in_excel` | `path: String` | `Result<(), String>` | 日本語エラー文字列 |
| `open_in_folder` | `path: String` | `Result<(), String>` | 日本語エラー文字列 |
| `export_results` | `request: ExportRequest` | `Result<(), String>` | 日本語エラー文字列 |
| `resolve_dropped_path` | `rawPath: String` | `Result<String, String>` | 日本語エラー文字列 |
| `get_supported_apps` | なし | `Result<Vec<SupportedApp>, String>` | 日本語エラー文字列 |
| `launch_associated_app` | `filePath: String, appId: String` | `Result<(), String>` | 日本語エラー文字列 |
| `show_open_with_dialog` | `filePath: String` | `Result<(), String>` | 日本語エラー文字列 |

## 3. エラーメッセージの日本語保証
- `Err(String)` で返却される文字列は、必ず自然かつ正確な日本語で構成する。
- 英語の生スタックトレースやライブラリメッセージは直接返却せず、`constants::ErrorMessages` を介してフォーマットする。
- 文字化けおよび `<PAD>`、`<pad>` 等の不要トークンの混入は許容しない。
