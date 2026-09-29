<!-- 処理内容: 実装時に実行した品質ゲートと受け入れ確認の結果、未確認範囲を記録する。入力・出力: 実コマンドと実行結果を入力し、再現可能な検証記録を出力する。エラー: 未実行・中断・未対応を成功として扱わず明記する。変更履歴: v1.0.0 2026-09-29 Codex 初回実装確認を記録。 -->
# 検証記録

実行環境: macOS、2026-09-29。Windows実機確認は未実施。

## 実行結果

| コマンド | 結果 |
|---|---|
| `cd src-tauri && cargo test` | 成功。42 unit tests、10 integration testsが成功。並行実行時に衝突していたShape/CLIテストの一時パスを一意化。 |
| `cd src-tauri && cargo clippy --all-targets -- -D warnings` | 成功。 |
| `cd src-tauri && cargo fmt --check` | 成功（修正後に再実行）。 |
| `npm run build` | 成功。TypeScriptとVite build完了。 |
| `npm run lint` | 成功。 |
| `npm test` | 未完了。Vitestが一部のReactテストでCPUを使い続けて結果を返さず、Ctrl-Cで中断。`--pool=threads` とworker上限1または2、`--testTimeout=10000` でも完了を確認できず。単独実行した `locale.test.ts`（6件）と `settings-default-options.test.tsx`（3件）は成功。 |
| `cargo build -p exlgrep --release --bin exlgrep-cli` | 成功。 |
| release CLI `--help` / `--help --language en` | 成功。日本語と英語のヘルプを表示し、GUIを起動しない。 |
| CLI統合テスト | CSV/XLSX出力、空検索、部分失敗、全ブック失敗、不正regex、既存出力保護、overwrite、入力同一性保護、A12の座標と従来メモを確認。 |

## 未完了・未確認

- `.xls` / `.xlsb` のメモ、OOXMLスレッドコメントは未実装。CLIが共通parserで検索できる形式と、コメント抽出対応形式は同じではない。
- ブックを開いた後の抽出段階別の部分失敗をSearchIssueとして返す詳細parser、および下位フォルダー競合など全契約ケースは未完了。
- 検索中の出力作成競合、出力symlink/hardlink、Excel上限境界など、全T029/T030ケースは未確認。
- Windowsビルド・終了コード・コンソール出力・公開動作は未確認。
- Vitest全件の完了を確認できていないため、全品質ゲート合格とはしない。
