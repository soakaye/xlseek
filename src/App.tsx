import React, { useState } from "react";
import { WindowFrame } from "./components/layout/WindowFrame";
import { SearchBar } from "./components/search/SearchBar";
import { ResultTable } from "./components/results/ResultTable";
import { PreviewHeader } from "./components/preview/PreviewHeader";
import { FormulaBar } from "./components/preview/FormulaBar";
import { SpreadsheetGrid } from "./components/preview/SpreadsheetGrid";
import { SheetTabs } from "./components/preview/SheetTabs";
import { MetaInfoCard } from "./components/preview/MetaInfoCard";
import { StatusBar } from "./components/common/StatusBar";
import { Toast } from "./components/common/Toast";
import { useSearch } from "./hooks/useSearch";

export const App: React.FC = () => {
  const [toastMessage, setToastMessage] = useState<string | null>(null);

  const showToast = (msg: string) => {
    setToastMessage(msg);
  };

  const {
    query,
    updateQuery,
    results,
    progress,
    isScanning,
    selectedMatch,
    previewData,
    loadingPreview,
    activeSheet,
    selectedCell,
    formulaOrValue,
    handleSelectItem,
    handleSelectSheet,
    handleSelectCell,
    startSearch,
    cancelSearch,
  } = useSearch({ onShowToast: showToast });

  return (
    <WindowFrame>
      {/* 検索入力 & 設定コントロールパネル */}
      <SearchBar
        query={query}
        onChangeQuery={updateQuery}
        onSearch={startSearch}
        onCancel={cancelSearch}
        isScanning={isScanning}
      />

      {/* メイン作業エリア (2ペイン分割: 左58%, 右42%) */}
      <div className="flex-1 flex overflow-hidden">
        {/* 左ペイン: 検索結果テーブル */}
        <ResultTable
          items={results}
          selectedId={selectedMatch?.id ?? null}
          onSelectItem={handleSelectItem}
        />

        {/* 右ペイン: セル & スプレッドシート プレビュー */}
        <div className="flex-1 flex flex-col bg-[#161618] min-w-0">
          {/* プレビュー上部ヘッダー & アクションバー */}
          <PreviewHeader
            selectedMatch={selectedMatch}
            onShowToast={showToast}
          />

          {/* Excel 数式バー */}
          <FormulaBar
            cellAddress={selectedCell?.address || selectedMatch?.cell_address || ""}
            formulaOrValue={formulaOrValue}
          />

          {/* スプレッドシート領域 */}
          <SpreadsheetGrid
            previewData={previewData}
            isLoading={loadingPreview}
            selectedCell={selectedCell}
            onSelectCell={handleSelectCell}
          />

          {/* Excel風シートタブバー */}
          {previewData && (
            <div className="px-3.5 pb-1 bg-[#141416]">
              <SheetTabs
                sheets={previewData.sheets_in_workbook}
                activeSheet={activeSheet}
                onSelectSheet={handleSelectSheet}
              />
            </div>
          )}

          {/* プレビュー下部：セルメタ情報カード */}
          {selectedMatch && (
            <div className="px-3.5 pb-3.5 bg-[#141416]">
              <MetaInfoCard match={selectedMatch} />
            </div>
          )}
        </div>
      </div>

      {/* フッター & ステータスバー */}
      <StatusBar
        progress={progress}
        items={results}
        onShowToast={showToast}
      />

      {/* トースト通知 */}
      <Toast
        message={toastMessage}
        onClose={() => setToastMessage(null)}
      />
    </WindowFrame>
  );
};

export default App;
