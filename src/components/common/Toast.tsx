/**
 * @fileoverview Toast notification component (src/components/common/Toast.tsx)
 *
 * ## Description
 * Displays a temporary floating notification message in the lower right of the screen,
 * automatically fading out after a configured timeout (referencing constants) or when dismissed.
 * Complies with Constitution Principle II (constant reference) and Principle III (comprehensive documentation).
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
 * ## Description
 * UI component rendering a toast notification popup.
 *
 * ## Arguments
 * @param props - Display message, close callback, and visibility duration
 *
 * ## Returns
 * @returns Rendered toast element, or null if message is absent
 *
 * ## Errors / Exceptions
 * No panic or exceptions occur.
 */
export const Toast: React.FC<ToastProps> = ({
  message,
  onClose,
  // Constant reference: TIMING_CONSTANTS.TOAST_DURATION_MS
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
