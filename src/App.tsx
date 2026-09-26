/**
 * @fileoverview アプリケーションルートコンポーネント (src/App.tsx)
 *
 * ## 処理内容
 * Excel Grep アプリケーションの最上位コンポーネント。カスタムフック `useSearch` を通じて
 * 検索状態、結果リスト、プレビューデータ、およびトースト通知状態を一括管理し、
 * タイトルバー、検索バー、結果テーブル、プレビューペイン、ステータスバーを統合配置する。
 * 憲章原則I（自然かつ正確な日本語）、原則III（網羅的なヘッダコメント）、原則IV（モジュール設計）に準拠。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 初版策定。
 * - v1.1.0 (2026-09-26, AI Agent): 憲章原則に準拠し、4要素ヘッダコメントを追加。
 * - v1.2.0 (2026-09-26, AI Agent): Aboutダイアログ表示状態 (isAboutOpen) を追加し、StatusBarにonOpenAboutを連携。
 * - v1.3.0 (2026-09-26, AI Agent): AboutDialogコンポーネントをマウントし、開閉連動を統合。
 * - v1.4.0 (2026-09-26, AI Agent): システムメニュー (macOS) からのAboutダイアログ表示イベント (EVENT_NAMES.OPEN_ABOUT_DIALOG) のリッスン処理を追加。
 */

import React, { useState, useEffect } from "react";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
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
import { AboutDialog } from "./components/about/AboutDialog";
import { SettingsDialog } from "./components/settings/SettingsDialog";
import { useSearch } from "./hooks/useSearch";
import { COMMANDS, EVENT_NAMES, retranslateMessage, UI_MESSAGES } from "./constants";
import { useLocale } from "./hooks/useLocale";
import { DisplayLanguage } from "./locale-core";

/**
 * Excel Grep アプリケーションのメイン画面コンポーネント
 *
 * ## 処理詳細
 * 状態管理フック `useSearch` を初期化し、トースト通知および各サブコンポーネントへの
 * データバインディングとイベントハンドラの受け渡しを行う。Aboutダイアログの表示状態も一元管理する。
 *
 * ## 引数
 * - なし (ルートコンポーネント)
 *
 * ## 戻り値
 * - `React.ReactElement`: アプリケーション全体のUIツリー
 *
 * ## エラー・例外条件
 * - 各種非同期処理のエラーは `showToast` を通じてトースト通知へ集約され、画面全体のクラッシュを防止する。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 初版作成。
 * - v1.1.0 (2026-09-26, AI Agent): 憲章原則に準拠し、4要素コメントを追加。
 * - v1.2.0 (2026-09-26, AI Agent): isAboutOpen 状態管理とStatusBar連携を追加。
 * - v1.3.0 (2026-09-26, AI Agent): システムメニューからのAbout表示要求イベントの購読と状態連動を追加。
 */
export const App: React.FC = () => {
  const [toastNotice, setToastNotice] = useState<{ message: string; language: DisplayLanguage } | null>(null);
  const [isAboutOpen, setIsAboutOpen] = useState(false);
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);
  const { language, preference, selectLanguage } = useLocale();
  const toastMessage = toastNotice
    ? retranslateMessage(toastNotice.message, toastNotice.language, language)
    : null;

  useEffect(() => {
    void invoke(COMMANDS.SET_MENU_LOCALE, { language }).catch(() => {
      console.error("[App] Failed to update application menu language");
      setToastNotice({ message: UI_MESSAGES.MENU_UPDATE_FAILED, language });
    });
  }, [language]);

  // システムメニューからのAboutダイアログ表示要求イベントの購読
  useEffect(() => {
    let unlisten: UnlistenFn | undefined;
    let isCancelled = false;

    const setupListener = async () => {
      try {
        // 定数参照: EVENT_NAMES.OPEN_ABOUT_DIALOG を使用
        const u = await listen(EVENT_NAMES.OPEN_ABOUT_DIALOG, () => {
          if (isCancelled) return;
          setIsAboutOpen(true);
        });

        if (isCancelled) {
          u();
        } else {
          unlisten = u;
        }
      } catch {
        console.error("[App] Failed to register About dialog event listener");
      }
    };

    setupListener();

    return () => {
      isCancelled = true;
      if (unlisten) {
        unlisten();
      }
    };
  }, []);

  const showToast = (msg: string) => {
    setToastNotice({ message: msg, language });
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
        <div className="flex-1 flex flex-col bg-[#161618] min-w-0 min-h-0">
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
            <div className="px-3.5 pb-1 bg-[#141416] flex-shrink-0">
              <SheetTabs
                sheets={previewData.sheets_in_workbook}
                activeSheet={activeSheet}
                onSelectSheet={handleSelectSheet}
              />
            </div>
          )}

          {/* プレビュー下部：セルメタ情報カード */}
          {selectedMatch && (
            <div className="px-3.5 pb-3.5 bg-[#141416] flex-shrink-0">
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
        onOpenAbout={() => setIsAboutOpen(true)}
        onOpenSettings={() => setIsSettingsOpen(true)}
        language={language}
      />

      <SettingsDialog
        isOpen={isSettingsOpen}
        preference={preference}
        language={language}
        onSelect={async (value) => {
          const saved = await selectLanguage(value);
          if (!saved) setToastNotice({ message: UI_MESSAGES.SAVE_FAILED, language });
        }}
        onClose={() => setIsSettingsOpen(false)}
      />

      {/* アプリ情報・ライセンスモーダル */}
      <AboutDialog
        isOpen={isAboutOpen}
        onClose={() => setIsAboutOpen(false)}
        onShowToast={showToast}
      />

      {/* トースト通知 */}
      <Toast
        message={toastMessage}
        onClose={() => setToastNotice(null)}
      />
    </WindowFrame>
  );
};

export default App;
