# UI Interface Contract: SearchBar Events & Controls

## 概要
本ドキュメントは、検索バーコンポーネントにおけるキーボード入力、フォーカス、およびクリック操作に関するUI契約（イベント仕様と振る舞い基準）を規定する。

---

## 1. 検索キーワード入力欄（`#searchInput`）

| イベント | 条件 | アクション / 期待される挙動 |
|---|---|---|
| `compositionstart` | 日本語等のIME入力開始 | `isComposing` フラグを `true` に設定する |
| `compositionend` | IME入力確定または破棄 | `isComposing` フラグを `false` に設定し、`compositionEndTime` に現在時刻を記録する |
| `keydown` | `key === "Enter"` かつ IME変換中（`isComposing \|\| keyCode === 229`） | 何もしない（検索を実行しない、確定操作を通過させる） |
| `keydown` | `key === "Enter"` かつ 確定直後（`Date.now() - compositionEndTime < 50ms`） | 何もしない（WebKitの先行 `compositionend` による誤爆を遮断） |
| `keydown` | `key === "Enter"` かつ 非IME変換中 かつ `!isScanning` | `onSearch()` を呼び出し、検索実行を開始する |
| クリック (`#clearBtn`) | キーワード文字列が存在 | `keyword` を空文字にし、入力欄へフォーカスを戻す |

---

## 2. フォルダパス入力欄（`#folderInput`）

| イベント | 条件 | アクション / 期待される挙動 |
|---|---|---|
| `keydown` | `key === "Enter"` かつ IME変換中（`isComposing \|\| keyCode === 229`） | 何もしない（通常のIME確定処理を通過させる） |
| `keydown` | `key === "Enter"` かつ 非IME変換中 | `e.preventDefault()` を実行してEnter入力を無効化（検索は実行せず、入力フォーカスを維持） |
| クリック (`#browseBtn`) | - | OSネイティブのフォルダ選択ダイアログを表示する |
| ドロップ (`#folderDropZone`) | フォルダがドラッグ＆ドロップされた | ドロップされたパスを解決して `target_dir` にセットする |

---

## 3. 対象拡張子トグルボタン（`.ext-btn`）

| イベント | 条件 | アクション / 期待される挙動 |
|---|---|---|
| クリック | 当該拡張子が選択中 かつ 選択総数が2以上 | 当該拡張子を選択から除外し、非選択スタイル（ダーク系）へ遷移する |
| クリック | 当該拡張子が選択中 かつ 選択総数が1 | 何もしない（最低1つの選択を維持、トースト通知等で保護） |
| クリック | 当該拡張子が未選択 | 当該拡張子を選択に追加し、選択スタイル（エメラルド系）へ遷移する |
