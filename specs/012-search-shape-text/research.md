<!-- 処理内容: 4形式の Shape 文字列抽出と既存機能への統合方針を記録する。引数・戻り値: 入力は仕様・コード・形式仕様、出力は決定事項。エラー: 未対応形式や破損入力を実装リスクとして明示する。変更履歴: v1.0.0 2026-09-27 Codex 初版作成。 -->
# Research: Excel Shape 内テキストの検索

## 1. 形式別の抽出

**Decision**: セル値・数式は既存の `calamine` を維持し、Shape は独立した読取処理で取得する。`.xlsx` と `.xlsm` はブック内のシート名とシートパートの対応を確認し、シートから描画パートへの参照を辿って DrawingML の `xdr:sp` にある `txBody` を読む。`xdr:grpSp` は再帰的に辿り、子図形ごとに結果を作る。`.xlsb` はバイナリブック内のシート名とシートパートの対応を確認し、シートの `BrtDrawing` とリレーションから同じ DrawingML 抽出器へ渡す。`.xls` は CFB 内の BIFF `MsoDrawing`、`TxO`、`Continue` と OfficeArt のグループ構造を別処理で解析する。

**Rationale**: `calamine 0.36.1` の公開 `Reader` はセル・数式等を返すが Shape 文字列 API がない。新しい処理を既存検索に合流させれば、検索条件と結果配送を再利用できる。`.xlsb` の描画参照は [Microsoft の BrtDrawing 仕様](https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-xlsb/36daad1e-9b3e-4d6a-8fff-167914be517a) に従う。`.xls` のテキスト対応は [MS-XLS の描画関連レコード](https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-xls/1b0b0ea3-5e1d-41ce-8e79-e27f1041331e) と [TxO 仕様](https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-xls/638c08e6-2942-4783-b71b-144ccf758fc7) に従う。

**Alternatives considered**: `calamine` のみでは要件を満たせない。Office アプリの自動操作や外部変換は OS 依存と追加インストールを招くため採用しない。描画 XML 全文への文字列検索は画像・グラフ等の文字を混入させるため採用しない。

**Dependency choice**: ZIP、XML、CFB の読取には Rust の既存ロックファイルに含まれる `zip`、`quick-xml`、`cfb` を直接依存として明示して使う。実装前に各 API とライセンスを確認し、必要なものだけ追加する。`.xls` の BIFF/OfficeArt レコード解析は専用の小さなモジュールに閉じ込める。

**Risk**: `.xls` は OOXML と共有できず、文字コードと `Continue` の分割、OfficeArt の階層と名前の対応が難しい。4形式の実物 fixture を先に揃え、形式別抽出を最初に検証する。fixture で証明できるまで4形式対応を完了と判定しない。

## 2. Shape の識別と文字列

**Decision**: 1 Shape につき1結果とし、グループ内の子図形はそれぞれ別結果にする。Shape 名にはグループから子までの表示可能な経路を含め、同名の子図形も区別する。文字列は図形内の文字の順序で結合し、段落境界を保つ。空文字列、画像、グラフの文字は除外する。Shape ID はファイル内の対応付けに使い、画面上の結果 ID は既存の一意 ID を使う。

**Rationale**: [DrawingML のグループ構造](https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.drawing.spreadsheet.groupshape?view=openxml-3.0.1) と [OfficeArt のグループ構造](https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-odraw/6ab2b867-0d8d-4d8c-bb5f-c1ab8e521eaa) では、グループと子図形は別の要素である。グループ単位で文字を合算すると、利用者が選択した図形を識別しづらい。

**Alternatives considered**: グループ全体を1件とする方式は明確化で却下された。図形名だけを識別子とする方式は重複名で曖昧になる。

## 3. シート、位置、プレビュー

**Decision**: 非表示シートは `calamine::Reader::sheets_metadata()` の可視状態で判定する。既存の「シート名に hidden を含む」判定は置き換える。Shape のアンカーが分かる時だけセル位置を持たせ、分からない時はセル番地を空にする。Shape 選択時はセルプレビュー要求を行わず、Shape 名・シート・全文を結果側から表示する。

**Rationale**: シート名は可視状態を示さない。現在のプレビューは行・列を必須としており、位置不明の Shape に仮のセル位置を与えると誤表示になる。`calamine` は公開 [SheetVisible と sheets_metadata](https://docs.rs/calamine/0.36.0/calamine/trait.Reader.html#method.sheets_metadata) を提供する。

**Alternatives considered**: 位置不明の Shape を A1 に置く方式は仕様に反する。Shape のためにブック全体を再読込するプレビューは検索性能に不利。

## 4. 安全性と検索の継続

**Decision**: Shape オプションがオフなら描画パートを開かない。オンなら ZIP エントリ・展開量・XML 深さ、BIFF レコード長を検証し、図形単位で読取失敗を隔離する。キャンセルを形式別走査の適切な間隔で確認する。既存の一致判定とスニペット生成を再利用し、未信頼文字列がスニペット HTML に入る箇所では共通経路でエスケープまたは安全な描画に改める。

**Rationale**: 大量・破損ファイルでも UI を止めず、検索結果を失わないため。検索結果のスニペットは現状 HTML として描画されるため、Shape 文字列も信頼境界として扱う必要がある。

**Alternatives considered**: 全ファイルをメモリに展開する方式、読取エラーで検索全体を停止する方式は性能・堅牢性要件に反する。

## 5. 結果出力と検証

**Decision**: 結果型に Shape 種別と Shape 名を追加する。CSV / Excel には Shape 名を独立列として設け、既存セル番地列には実際の位置だけを入れる。形式ごとに通常図形、テキストボックス、グループ内の子図形、空文字、非表示シート、破損データの fixture を用意する。出力と日英表示も同じ fixture で検証する。

**Rationale**: セル番地と Shape 名を混在させず、画面と出力で同じ所在を示せる。4形式必須の受け入れ条件は形式別 fixture なしでは検証できない。

**Alternatives considered**: Shape 名をセル番地列に埋める方式は列の意味を壊す。単一形式の fixture で他形式を推定する方式は `.xls` と `.xlsb` の異なる格納方式を検証できない。
