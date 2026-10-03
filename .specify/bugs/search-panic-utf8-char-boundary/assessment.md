# Bug Assessment: 検索実行時のスレッドパニックによる進捗停止 (UTF-8文字境界違反 & calamine XLSパニック)

- **Slug**: search-panic-utf8-char-boundary
- **Created**: 2026-09-25T16:24:00+09:00
- **Source**: pasted text
- **Verdict**: valid
- **Severity**: high

## Report (verbatim or summarized)

検索を実行しても進捗が0%のまま停止する。ターミナル/コンソールに以下のパニックログが出力される：

```text
thread '<unnamed>' (22720) panicked at D:\devlibs\rust\cargo\registry\src\index.crates.io-1949cf8c6b5b557f\calamine-0.26.1\src\xls.rs:1428:43:
range start index 4 out of range for slice of length 3

thread '<unnamed>' (36348) panicked at src\search\parser.rs:38:23:
start byte index 19 is not a char boundary; it is inside 'ン' (bytes 18..21 of string)

thread '<unnamed>' (36000) panicked at src\search\parser.rs:38:23:
start byte index 19 is not a char boundary; it is inside 'ス' (bytes 18..21 of string)

thread '<unnamed>' (3260) panicked at src\search\parser.rs:38:23:
start byte index 32 is not a char boundary; it is inside 'を' (bytes 30..33 of string)
```

## Symptom

- 検索ボタン（SEARCH）を押しても、進捗バーやステータスが0%から進まず、検索が停止する。
- バックエンドのワーカースレッドがパニックを起こして異常終了し、ストリーミング結果や進捗イベントが送信されなくなる。

## Reproduction

1. 日本語（マルチバイトUTF-8文字）を含むExcelファイル群を対象に検索を実行する。
2. キーワードが日本語テキストの中間にヒットした際、`make_snippet` がバイト単位で `-25` を引き算し、文字の境界外（char boundary外）をスライスしてパニックを起こす。
3. または、一部の古い/不正なバイナリ形式の `.xls` ファイルを読み込んだ際、`calamine` の `xls.rs` 内部で範囲外スライスによるパニックが発生する。

## Suspected Code Paths

- `src-tauri/src/search/parser.rs:31-46`: `make_snippet` 関数
  - `let start = mat_start.saturating_sub(25);`
  - `&text[start..mat_start]`
  - バイトインデックスで直接スライスしているため、マルチバイト文字（日本語）のバイト境界外を指すとパニックが発生する。
- `src-tauri/src/search/engine.rs:119-163`: `par_iter` ループ内
  - `parse_and_search_file` 呼び出しが `std::panic::catch_unwind` で保護されていないため、パース時やサードパーティライブラリ（`calamine`）内部でパニックが発生するとスレッドがクラッシュし、検索処理全体が復帰不能になる。

## Root Cause Hypothesis

**確信度: high (極めて高い)**

本障害には 2 つの直接的な要因があります：

1. **UTF-8 char boundary 違反パニック (`parser.rs:38`)**:
   Rustの `&str[start..end]` はバイト単位のスライスです。日本語は UTF-8 で 3 バイト消費するため、単純に `mat_start.saturating_sub(25)` を行うと、マルチバイト文字の 2 バイト目や 3 バイト目を指してしまい、Rust ランタイムが `not a char boundary` 例外で直ちにパニックします。
2. **サードパーティライブラリパニック時の非保護 (`engine.rs:130`)**:
   `calamine` の一部の `.xls` パーサー実装においてスライスの境界外アクセスバグが存在します。これに対するパニック回復機構（`catch_unwind`）がないため、並列スレッドが相次いで停止し、走査カウンタが進まなくなっていました。

## Proposed Remediation

**Preferred**:

1. **文字単位（char boundary安全）なスニペット切り出しへの改修 (`parser.rs`)**:
   - バイトインデックスの直接加減算を廃止。
   - `floor_char_boundary` / `ceil_char_boundary`、または文字（`char`）単位のイテレータを用いて、必ず有効な UTF-8 文字境界を特定して切り出す。
   - または、前後の文字数（文字数ベースで20文字など）を文字イテレータから取得する安全な実装に変更する。
2. **ファイル走査時の `std::panic::catch_unwind` による防御的保護 (`engine.rs`)**:
   - `parse_and_search_file` の実行を `std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| ...))` でラップ。
   - `calamine` 内部のバグや未知のファイル破損によるパニックが発生しても、当該ファイルのみをエラーとしてスキップし、他のファイルの走査および進捗通知を安全に継続する。

**Alternatives**:
- `.xls` 拡張子を検索対象からデフォルトで外すことも考えられるが、レガシーファイルのサポート要件があるため、パニック耐性を高める `catch_unwind` の導入が本質的かつ堅牢である。

**Files likely to change**:
- `src-tauri/src/search/parser.rs`
- `src-tauri/src/search/engine.rs`

**Tests to add or update**:
- 日本語文字列（「財務報告書2026年Q3財務データ」等）の中間一致に対する `make_snippet` の単体テスト。
- パニックを発生させる不正ファイルや境界値に対する `catch_unwind` の検証テスト。

## Risks & Considerations

- パニックをキャッチする際は `std::panic::AssertUnwindSafe` を使用する必要がある。
- スニペット生成のパフォーマンスを落とさないよう、文字数カウントは局所的な探索にとどめる。

## Open Questions

- なし（原因・再現・対策ともに明確に特定完了）。
