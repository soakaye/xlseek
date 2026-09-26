# Interface Contract: AboutダイアログUIコンポーネント仕様

**Feature Branch**: `006-about-dialog-licenses`  
**Date**: 2026-09-26  
**Status**: Completed  

---

## 1. コンポーネント階層とインターフェース定義

### 1.1 `AboutDialog` (最上位モーダルコンポーネント)
`src/components/about/AboutDialog.tsx`

```typescript
export interface AboutDialogProps {
  /** ダイアログの表示・非表示フラグ */
  isOpen: boolean;
  /** ダイアログを閉じるハンドラ */
  onClose: () => void;
  /** コピー成功時のトースト通知発火コールバック */
  onShowToast: (message: string) => void;
  /** カスタムライセンスデータ（省略時は src/constants/licenses.json をデフォルト使用） */
  licenses?: PackageLicenseRecord[];
}
```

### 1.2 `StatusBar` (拡張プロパティ)
`src/components/common/StatusBar.tsx`

```typescript
export interface StatusBarProps {
  progress: ScanProgress | null;
  items: SearchMatch[];
  onShowToast: (msg: string) => void;
  /** Aboutダイアログを開くためのコールバックハンドラ（新規追加） */
  onOpenAbout: () => void;
}
```

### 1.3 `PackageList` (左ペイン一覧コンポーネント)
`src/components/about/PackageList.tsx`

```typescript
export interface PackageListProps {
  /** フィルタリング対象の全パッケージ一覧 */
  items: PackageLicenseRecord[];
  /** 現在選択されているパッケージの一意識別子 */
  selectedId: string | null;
  /** パッケージ選択ハンドラ */
  onSelect: (pkg: PackageLicenseRecord) => void;
  /** 検索キーワード入力値 */
  searchKeyword: string;
  /** 検索キーワード変更ハンドラ */
  onChangeSearchKeyword: (keyword: string) => void;
}
```

### 1.4 `PackageDetail` (右ペイン詳細ビューコンポーネント)
`src/components/about/PackageDetail.tsx`

```typescript
export interface PackageDetailProps {
  /** 現在詳細表示対象のパッケージ（未選択時は null） */
  selectedPackage: PackageLicenseRecord | null;
  /** ライセンス本文コピーハンドラ */
  onCopyLicense: (text: string) => void;
  /** コピー完了一時フィードバック表示中フラグ */
  isCopied: boolean;
}
```

---

## 2. 定数定義インターフェース (`src/constants/index.ts`)

憲章原則II（定数の一元化）に基づき、以下の定数セットを `src/constants/index.ts` に追加する。

```typescript
export const ABOUT_DIALOG_CONSTANTS = {
  TITLE: "Excel Grep について",
  APP_NAME: "Excel Grep",
  APP_VERSION: "0.1.0",
  APP_DESCRIPTION: "高速・セキュアなExcel専用ファイル内検索デスクトップアプリケーション",
  COPYRIGHT: "Copyright © 2026 Excel Grep Contributors",
  APP_LICENSE_LABEL: "配布ライセンス: MIT License",
  
  TAB_ABOUT: "アプリ情報",
  TAB_LICENSES: "オープンソースライセンス",
  
  SEARCH_PLACEHOLDER: "パッケージ名・ライセンスで検索...",
  SEARCH_CLEAR_TOOLTIP: "検索キーワードをクリア",
  NO_PACKAGES_FOUND: "一致するパッケージが見つかりません",
  
  BUTTON_COPY_LICENSE: "ライセンス本文をコピー",
  BUTTON_COPIED: "コピー完了",
  BUTTON_CLOSE: "閉じる",
  BUTTON_ABOUT_TOOLTIP: "Excel Grep について",
  
  AUTHOR_LABEL: "著作者 / 著作権表記:",
  LICENSE_TYPE_LABEL: "ライセンス種別:",
  REPOSITORY_LABEL: "リポジトリ / 公式サイト:",
  SOURCE_RUST_LABEL: "Rust (バックエンド)",
  SOURCE_NPM_LABEL: "npm (フロントエンド)",
  
  TOAST_COPIED: "ライセンス本文をクリップボードにコピーしました",
  TOAST_COPY_FAILED: "クリップボードへのコピーに失敗しました",
} as const;
```

---

## 3. デザインプロトタイプ（`design/mainui/index.html`）同期仕様

- **ID体系**:
  - `#about-modal`: ダイアログのルート要素（`hidden` クラスで開閉トグル）
  - `#btn-open-about`: ステータスバー右端のAbout起動ボタン
  - `#btn-close-about`: モーダル右上の閉じるボタン
  - `#tab-about` / `#tab-licenses`: タブ切り替えボタン
  - `#about-tab-content` / `#licenses-tab-content`: 各タブのコンテナ
  - `#license-search-input`: 検索バー
  - `#license-list-container`: 左ペインパッケージ一覧
  - `#license-detail-container`: 右ペインライセンス詳細
  - `#btn-copy-license`: ライセンスコピーボタン
