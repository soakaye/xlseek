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
 * - v1.5.0 (2026-09-27, Codex): 検索履歴と保存件数設定を検索・設定画面へ接続。
 * - v1.6.0 (2026-09-28, AI Agent): デフォルト検索オプションの永続化および即時画面同期を統合。
 */

import React, { Suspense, lazy, useState, useEffect } from "react";
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
import { SettingsDialog } from "./components/settings/SettingsDialog";
import { useSearch } from "./hooks/useSearch";
import { useSearchHistory } from "./hooks/useSearchHistory";
import { APP_LOGS, COMMANDS, EVENT_NAMES } from "./constants";
import { useLocale } from "./hooks/useLocale";
import { LocaleProvider, t, TranslationKey, TranslationValues } from "./i18n";
import { DefaultSearchOptions } from "./types/defaultOptions";
import { loadDefaultSearchOptions, saveDefaultSearchOptions } from "./default-options-core";

const AboutDialog = lazy(() => import("./components/about/AboutDialog").then((module) => ({ default: module.AboutDialog })));

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
  const [toastNotice, setToastNotice] = useState<{ key: TranslationKey; values?: TranslationValues } | null>(null);
  const [isAboutOpen, setIsAboutOpen] = useState(false);
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);
  const { language, preference, selectLanguage, ready } = useLocale();
  const toastMessage = toastNotice
    ? t(language, toastNotice.key, toastNotice.values)
    : null;

  useEffect(() => {
    if (!ready) return;
    void invoke(COMMANDS.SET_MENU_LOCALE, { language }).catch(() => {
      console.error(APP_LOGS.MENU_UPDATE_FAILED);
      setToastNotice({ key: "ui.MENU_UPDATE_FAILED" });
    });
  }, [language, ready]);

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
        console.error(APP_LOGS.ABOUT_LISTENER_FAILED);
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

  const showToast = (key: TranslationKey, values?: TranslationValues) => {
    setToastNotice({ key, values });
  };

  const [defaultOptions, setDefaultOptions] = useState<DefaultSearchOptions>(() => loadDefaultSearchOptions());

  const { maxEntries, keywords, directories, addSearch, setMaxEntries } = useSearchHistory(() => showToast("ui.SAVE_FAILED"));

  const {
    query,
    updateQuery,
    applyDefaultOptions,
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
  } = useSearch({ onShowToast: showToast, onSearchAccepted: (acceptedQuery) => addSearch(acceptedQuery.keyword, acceptedQuery.target_dir) });

  const handleSaveDefaultOptions = (newOptions: DefaultSearchOptions) => {
    const saved = saveDefaultSearchOptions(newOptions);
    if (!saved) {
      showToast("ui.SAVE_FAILED");
      return;
    }
    setDefaultOptions(newOptions);
    applyDefaultOptions(newOptions);
    showToast("ui.DEFAULT_OPTIONS_SAVED");
  };

  if (!ready) return null;

  return (
    <LocaleProvider value={language}>
    <WindowFrame>
      {/* 検索入力 & 設定コントロールパネル */}
      <SearchBar
        query={query}
        onChangeQuery={updateQuery}
        onSearch={startSearch}
        onCancel={cancelSearch}
        isScanning={isScanning}
        history={{ keywords, directories }}
        onSelectHistory={(field, value) => updateQuery(field === "keyword" ? { keyword: value } : { target_dir: value })}
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

          {selectedMatch?.match_type !== "Shape" && (
            <>
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
            </>
          )}

          {/* Excel風シートタブバー */}
          {previewData && selectedMatch?.match_type !== "Shape" && (
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
        maxEntries={maxEntries}
        onSetMaxEntries={(value) => { if (!setMaxEntries(value)) showToast("ui.SAVE_FAILED"); }}
        defaultOptions={defaultOptions}
        onSaveDefaultOptions={handleSaveDefaultOptions}
        onSelect={async (value) => {
          const saved = await selectLanguage(value);
          if (!saved) setToastNotice({ key: "ui.SAVE_FAILED" });
        }}
        onClose={() => setIsSettingsOpen(false)}
      />

      {/* アプリ情報・ライセンスモーダル */}
      <Suspense fallback={null}>
        {isAboutOpen && <AboutDialog
          isOpen={isAboutOpen}
          onClose={() => setIsAboutOpen(false)}
          onShowToast={showToast}
        />}
      </Suspense>

      {/* トースト通知 */}
      <Toast
        message={toastMessage}
        onClose={() => setToastNotice(null)}
      />
    </WindowFrame>
    </LocaleProvider>
  );
};

export default App;
