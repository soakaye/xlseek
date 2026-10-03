# UI Interface Contract: Spreadsheet Grid Layout & Freeze Panes

## 概要
本ドキュメントは、周辺セルプレビューグリッドにおけるスクロール動作、固定見出し（Freeze Panes）、およびセル選択のUI契約を規定する。

---

## 1. コンテナおよびテーブル階層

```html
<!-- スクロールコンテナ -->
<div id="spreadsheetGridContainer" class="flex-1 min-h-0 min-w-0 overflow-auto ...">
  <table class="excel-grid w-max min-w-full text-xs border-collapse font-sans">
    <thead class="sticky top-0 z-20">
      <tr id="gridHeaderRow">
        <!-- 角セル (#): sticky left-0 top-0 z-30 -->
        <th class="sticky left-0 top-0 z-30 ...">#</th>
        <!-- 列見出し (A, B, C...): sticky top-0 z-20 -->
        <th class="sticky top-0 z-20 ...">A</th>
        ...
      </tr>
    </thead>
    <tbody id="gridBody">
      <tr>
        <!-- 行番号 (#): sticky left-0 z-10 -->
        <td class="sticky left-0 z-10 ...">8</td>
        <!-- データセル -->
        <td class="grid-cell ...">...</td>
      </tr>
    </tbody>
  </table>
</div>
```

---

## 2. スクロールおよび固定見出しの契約

| 要素 | スタイル契約 | スクロール時の挙動 |
|---|---|---|
| 左上角 `#` セル | `sticky left-0 top-0 z-30` | 縦スクロール・横スクロールのいずれにおいても常に左上端に静止 |
| 列見出しセル（A, B, C...） | `sticky top-0 z-20` | 縦スクロール時に上端に固定され、横スクロール時には水平同期して移動 |
| 行番号セル（数値） | `sticky left-0 z-10` | 横スクロール時に左端に固定され、縦スクロール時には垂直同期して移動 |
| データセル | - | 縦横両方向のスクロールに従って移動 |

---

## 3. インタラクション契約

| イベント | 対象 | 契約 |
|---|---|---|
| クリック | `.grid-cell` | 選択状態（アクティブ枠線）を付与し、名前ボックス（`#nameBox`）に番地（例: `B12`）、数式バー（`#formulaBarContent`）にそのセルの数式または値を即座に反映する |
| スクロール | コンテナ | 慣性スクロール（タッチパッド/ホイール）およびスクロールバードラッグの両方で60fpsを維持し、描画の破綻やちらつきが発生しないこと |
