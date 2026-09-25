/**
 * @fileoverview トースト通知コンポーネント (src/components/common/Toast.tsx)
 *
 * ## 処理内容
 * 画面右下に一時的な通知メッセージをフロート表示し、一定時間経過（定数定義準拠）
 * または閉じるボタンの押下によって自動的にフェードアウト・非表示とする。
 * 憲章原則II（定数参照）および原則III（ヘッダコメント）に準拠。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化およびJSDocドキュメントの付与。
 */

import React, { useEffect } from "react";
import { CheckCircle2, X } from "lucide-react";
import { TIMING_CONSTANTS } from "../../constants";

interface ToastProps {
  message: string | null;
  onClose: () => void;
  duration?: number;
}

/**
 * ## 処理内容
 * トースト通知ポップアップを表示するUIコンポーネント。
 *
 * ## 引数
 * @param props - 表示メッセージ、クローズコールバック、表示持続時間
 *
 * ## 戻り値
 * @returns レンダリング要素、またはメッセージ不在時はnull
 *
 * ## エラー / 例外発生条件
 * panicや例外は発生しない。
 *
 * ## 変更履歴
 * - v1.0.0 (2026-09-26, AI Agent): 初版策定。定数参照化（TIMING_CONSTANTS.TOAST_DURATION_MS）。
 */
export const Toast: React.FC<ToastProps> = ({
  message,
  onClose,
  // 定数参照: TIMING_CONSTANTS.TOAST_DURATION_MS を使用
  duration = TIMING_CONSTANTS.TOAST_DURATION_MS,
}) => {
  useEffect(() => {
    if (!message) return;
    const timer = setTimeout(() => {
      onClose();
    }, duration);
    return () => clearTimeout(timer);
  }, [message, duration, onClose]);

  if (!message) return null;

  return (
    <div className="fixed bottom-12 right-6 z-50 flex items-center gap-2 bg-zinc-800 text-zinc-100 border border-emerald-600/60 shadow-xl px-3.5 py-2.5 rounded-lg text-xs animate-in fade-in slide-in-from-bottom-2 duration-200">
      <CheckCircle2 className="w-4 h-4 text-emerald-400 flex-shrink-0" />
      <span className="font-medium">{message}</span>
      <button
        onClick={onClose}
        className="ml-2 text-zinc-400 hover:text-zinc-200 p-0.5 rounded transition"
      >
        <X className="w-3.5 h-3.5" />
      </button>
    </div>
  );
};
