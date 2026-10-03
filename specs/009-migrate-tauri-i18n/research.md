# 調査結果: tauri-plugin-i18n への移行

## 利用するプラグイン

- **決定**: Rust の `tauri-plugin-i18n` 2.0.2 と JavaScript の `@razein97/tauri-plugin-i18n` 2.0.1 を対象とし、実装時に固定した版の API とビルドを確認する。
- **理由**: 利用者が指定した名称に一致する元の Tauri v2 用プラグインで、両側の API を持つ。[上流 README](https://github.com/razein97/tauri-plugin-i18n)、[Rust 公開版](https://docs.rs/crate/tauri-plugin-i18n/2.0.2)、[JavaScript 公開版](https://registry.npmjs.org/%40razein97%2Ftauri-plugin-i18n/latest)。
- **検討した代替**: `tauri-plugin-i18n-vsk` は別名の派生品であり、利用者の指定と一致しない。

## 翻訳ファイルとビルド配置

- **決定**: 唯一の翻訳正本をヘッダコメント付きの `src-tauri/locales/en.yml` と `ja.yml` に置き、フロントエンドは `yaml` パーサーで同じ資源を読み込む。リポジトリ直下に Cargo ワークスペース定義を追加し、`src-tauri` をメンバーにする。実装時には Cargo の lockfile と生成物の移動を確認する。
- **理由**: プラグインは YAML を扱い、ビルド処理は `OUT_DIR` から親方向へ `[workspace]` を持つ `Cargo.toml` を探して、その配下の `src-tauri/locales` をバイナリに組み込む。現状は `src-tauri/Cargo.toml` しかなく、そのままでは翻訳が空になる。[上流 build.rs](https://github.com/razein97/tauri-plugin-i18n/blob/main/build.rs)。
- **検討した代替**: `src-tauri/Cargo.toml` だけに `[workspace]` を追加しても、プラグインが探す `src-tauri` はその下に存在しない。実行時ファイルへの依存は配布後のパス管理を増やす。

## フロントエンドの翻訳と欠落時の代替

- **決定**: プラグインの JavaScript API で翻訳データを読み込み、言語を切り替える。React の再描画は既存の表示言語状態で行い、プラグインの DOM 自動書換えは React 管理要素に使わない。日本語キーが欠けた時とプラグイン読込失敗時は、同じ正本の `en.yml` をフロントエンドにも取り込み、英語文言を返す。
- **理由**: `I18n.getInstance().load()`、`translate(key)`、`I18n.setLocale(locale)` が公開 API。未翻訳キーは英語ではなくキー名を返し、`load()` は `data-i18n` 要素を直接書き換える。英語フォールバックと React の描画整合には薄いアダプタが必要。[上流 JavaScript 実装](https://github.com/razein97/tauri-plugin-i18n/blob/main/guest-js/index.ts)。
- **検討した代替**: 現在の独自日英辞書を残す案は翻訳正本が二重になる。React 要素への `data-i18n` 指定は DOM をプラグインと React の双方が更新する。

## Rust のメニューと出力

- **決定**: メニュー文言と出力見出し・一致種別・シート名も同じ翻訳正本から取得する。プラグインの `PluginI18nExt` と翻訳データ取得 API を使用し、出力は要求に含まれる表示言語で文言を選ぶ。欠けたキーは英語データへ代替する。ログと内部エラー理由は英語固定にする。
- **理由**: プラグインは `app.i18n().translate(key)`、`get_translations_data()`、`set_locale(locale)` を公開する。現在のメニュー・CSV・Excel 文言は Rust 定数に日英で重複している。出力中の言語切替で見出しが混ざらないよう、出力要求の言語を固定して参照する。[上流 Rust API](https://github.com/razein97/tauri-plugin-i18n/blob/main/src/models.rs)。
- **検討した代替**: 出力時にプラグインのグローバル言語を一時変更する案は、同時の UI 切替や複数出力と競合する。

## 既存設定と起動順

- **決定**: 保存キー `exlgrep.language` と値 `default|ja|en` を保持する。起動時に保存値と端末言語を解決し、プラグインを読み込んで有効言語を設定した後、画面を表示する。プラグイン失敗時は同じ英語翻訳ファイルをフロントエンドで解析して表示を継続する。
- **理由**: 現在の設定を移行処理なしで引き継ぎ、初期表示に日本語と英語が一瞬混在するのを避ける。デフォルトの端末言語取得には既存の Tauri OS プラグインを利用する。
- **検討した代替**: 新しい保存キーに移す案は不要な移行処理を増やす。初期描画後の非同期切替は誤言語の一時表示を生む。

## 検証方針

- **決定**: 翻訳キーの日英網羅、欠落キーの英語代替、保存値互換、プラグイン読込失敗、切替中の検索状態維持、Rust のメニュー・出力文言を自動確認する。実機で設定、再起動、CSV・Excel 出力、メニューを確認し、既存のビルド・Lint・Rust 品質ゲートを実行する。
- **理由**: この移行の主な失敗要因は翻訳データがビルドへ入らないことと、独自辞書の残存・言語状態の不一致である。
- **検討した代替**: 単体テストのみでは配布バイナリへの翻訳同梱とネイティブメニューを確認できない。
