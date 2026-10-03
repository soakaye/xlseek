# Phase 0 調査結果 (Research Findings): デフォルト動作設定

## 調査の目的
1. **デフォルト検索オプションの永続化方法**:
   - アプリケーション起動時に遅延なく即時ロードされ、ブラウザ/Tauriのサンドボックス環境で安全に動作する保存手段を選定する。
2. **既存の設定ダイアログ (`SettingsDialog.tsx`) との統合設計**:
   - 言語設定・検索履歴件数設定が既存で提供されているダイアログ内に、UIの一貫性とアクセシビリティを保ちながら「デフォルト検索オプション」を配置する。
3. **メイン画面（`useSearch`）との即時同期アーキテクチャ**:
   - 設定保存時に、現在開いているメイン画面の検索オプション（トグル群・拡張子トグル）へ即座に反映させるメカニズム。
4. **CSV/Excelエクスポート機能の現状と改善**:
   - 既存のステータスバーにおけるCSV/Excel保存ダイアログ呼び出しと、仕様上の要件との差分・確認。

---

## 調査結果と決定事項

### 1. デフォルト検索オプションの永続化方式
- **決定**: `localStorage` を使用したJSON永続化（専用フック `useDefaultOptions` または `default-options-core.ts` による一元管理）。
- **根拠**:
  - 表示言語（`exlgrep.language`）および検索履歴（`exlgrep.search_history`）と同様に、端末内 `localStorage` で同期的に読み取れるため、初回レンダリング時（コンポーネントマウント時）に非同期のIPC待ち時間（チラつき）なしで即時初期値を反映可能。
  - バックエンドへの不要なディスクI/OやIPCオーバーヘッドを抑え、クライアント完結で高い応答性を実現できる。
  - スキーマバリデーションおよび破損・不正値検出時にフォールバック値（公式規定値）を自動復元する関数を設計する。
- **代替案の検討**:
  - *Tauri Storeプラグイン / ファイル保存*: IPC非同期呼び出しが必要となり、起動時にUIトグルのチラつき（FOUC）が発生するリスクがあるため却下。

### 2. 公式規定値（フォールバック初期値）の定数管理
- **決定**: 憲章原則IIに準拠し、すべてのデフォルト値・ストレージキーを `src/constants/index.ts` に定義。
  - `DEFAULT_SEARCH_OPTIONS`:
    - `match_case`: `false`
    - `use_regex`: `false`
    - `include_formula`: `true`
    - `include_shape`: `false`
    - `include_comment`: `true`
    - `include_hidden`: `false`
    - `extensions`: `[".xlsx", ".xlsm", ".xlsb", ".xls"]`
  - `DEFAULT_OPTIONS_STORAGE_KEY`: `"exlgrep.default_search_options"`
- **根拠**:
  - ハードコードの禁止を徹底し、定数参照コメントを記載。

### 3. 設定ダイアログとメイン画面の即時同期
- **決定**:
  - `App.tsx` またはカスタムフック層でデフォルトオプション状態を管理、あるいは設定保存時にコールバック `onSaveDefaultOptions(options)` を介して `useSearch` のクエリ状態（`query.match_case`, `query.use_regex`, `query.include_formula`, `query.include_shape`, `query.include_comment`, `query.include_hidden`, `query.extensions`）へ直接反映する。
  - 設定ダイアログ内に「デフォルトに戻す」ボタンを配置し、クリックで公式規定値に戻せるようにする。
- **根拠**:
  - Clarificationで合意された「設定保存時に現在の検索バーのオプション状態も即座に新しいデフォルト値で更新する」要件を満たす。

### 4. CSV/Excel出力機能の現状確認と統合
- **決定**:
  - ステータスバー（`StatusBar.tsx`）に既に実装されている `handleExport("csv")` および `handleExport("xlsx")` （Tauriの `@tauri-apps/plugin-dialog` の `save` ダイアログ呼び出しと `COMMANDS.EXPORT_RESULTS` 発行）の動作・受入シナリオ・エッジケース（0件時トースト通知、保存キャンセル時の非破壊処理、保存完了時のトースト通知）を仕様およびクイックスタート検証に組み込む。
- **根拠**:
  - 既存のバックエンド出力ロジック（`export_results`）およびフロントエンド保存ダイアログフローは強固に動作しており、今回の機能仕様に適合している。
