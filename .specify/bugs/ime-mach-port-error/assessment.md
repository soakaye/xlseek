# Bug Assessment: IME 起動時の Mach port ログ

- **Slug**: ime-mach-port-error
- **Created**: 2026-09-27
- **Source**: pasted text（ユーザーのログと追加回答）、https://github.com/electron/electron/issues/45002
- **Verdict**: invalid
- **Severity**: low

## Report (verbatim or summarized)

> IMEを起動するとエラーが発生する。
> エラー: 2026-09-27 13:25:50.816 exlgrep[7844:25772236] error messaging the mach port for IMKCFRunLoopWakeUpReliable

追加回答: macOS 標準の日本語入力を使用。ログは出るが、日本語入力はできた。

追加資料: [Electron issue #45002](https://github.com/electron/electron/issues/45002) に同じログ文字列が報告されている。URL のホストは `github.com`、取得ポリシーは `allowlisted`。issue はクローズ済みだが、表示されているラベルは `blocked/need-repro` であり、「無害と確認した」という結論や修正内容は記載されていない。

## Symptom

macOS 標準の日本語入力を有効にした際、アプリのプロセス名を含む `IMKCFRunLoopWakeUpReliable` の診断ログが出る。報告された範囲では文字入力の失敗、アプリ停止、データ損失は発生していない。期待される動作は日本語入力が正常に使えることで、その点は満たされている。

## Reproduction

1. macOS 上で Excel Grep を起動する。
2. macOS 標準の日本語入力を有効にする。
3. ユーザー報告では上記のログが出るが、日本語入力はできる。

[NEEDS CLARIFICATION: ログの再現頻度、発生時にフォーカスしていた入力欄、他の macOS アプリでも出るかは未確認。評価者による実機再現は未実施。]

## Suspected Code Paths

- `src/components/search/SearchBar.tsx:264-270` — キーワード欄は通常の HTML `input` で、IME の composition イベントを受ける。
- `src/components/search/SearchBar.tsx:313-317` — フォルダ欄も HTML `input` で、キー押下時に IME 変換中の Enter を無視する。
- `src-tauri/src/lib.rs:173-213` — Tauri のアプリ起動と macOS 上の WebView を含むイベントループへの入口。

コード検索では `IMKCFRunLoopWakeUpReliable` を生成する箇所は見つからなかった。該当する入力欄または行がログを直接出した証拠はない。

## Root Cause Hypothesis

**確信度: medium。** ログ名と表示形式から、macOS の入力方式とアプリのテキスト入力クライアント間の処理で発生した診断ログと推測する。[Apple の InputMethodKit 説明](https://developer.apple.com/documentation/inputmethodkit)は入力方式とクライアントアプリ間の通信を扱うことを示している。ただし `IMKCFRunLoopWakeUpReliable` 自体は公開仕様で確認できず、この 1 行だけで失敗箇所や原因を確定できない。同じログは[Electron の issue](https://github.com/electron/electron/issues/45002)、[別のアプリの入力方式に関する報告](https://github.com/LadybirdBrowser/ladybird/issues/9712)、[ネイティブ AppKit まで切り分けた調査](https://gist.github.com/ph0ryn/eb7572f1eabf469289ce5f6da56d53ad)にも現れる。最後の調査は azooKeyMac の条件であり、今回の macOS 標準日本語入力にその原因を当てはめることはできない。

## Proposed Remediation

**Preferred**: 現時点ではアプリのコードを変更しない。日本語入力が正常に使えるため、ログだけを根拠に IME イベント処理を変更したり、標準エラー出力を抑制したりしない。入力不能、変換候補の異常、アプリ停止などの実害が生じた場合に、発生する入力欄と操作を記録して新たに評価する。

**Files likely to change**:

- なし。実害が確認された場合に限り、該当する `src/components/search/SearchBar.tsx` などを再調査する。

**Tests to add or update**:

- 現時点ではなし。実害が確認された場合は、macOS 標準日本語入力での変換開始・候補確定・入力欄への反映・検索実行を対象に再現テストを設計する。

## Risks & Considerations

- プロセス名 `exlgrep` が付くログでも、アプリのコードが直接出力したことは意味しない。
- 将来、入力障害と同時に出るなら関連性を再評価する必要がある。ログの非表示は診断材料を失う。
- サードパーティ IME を使った外部調査結果を、今回の macOS 標準日本語入力の原因断定に利用しない。

## Open Questions

- [NEEDS CLARIFICATION: 同じ操作で毎回ログが出るか。]
- [NEEDS CLARIFICATION: TextEdit など別の macOS アプリでも標準日本語入力の有効化時に出るか。]
- [NEEDS CLARIFICATION: 今後、入力不能や変換異常が発生するか。]
