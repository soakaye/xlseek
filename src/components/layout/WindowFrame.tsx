/**
 * @fileoverview Window frame layout component (src/components/layout/WindowFrame.tsx)
 *
 * ## Description
 * Provides the root outer frame layout for the entire application, applying dark theme styling,
 * fixed full-screen height, user-select restrictions, and main content overflow management.
 * Complies with Constitution Principle III (comprehensive documentation).
 */

import React from "react";

interface WindowFrameProps {
  children: React.ReactNode;
}

/**
 * ## Description
 * Top-level container layout component for the application.
 *
 * ## Arguments
 * @param props - Component properties containing child elements
 *
 * ## Returns
 * @returns Rendered element
 *
 * ## Errors / Exceptions
 * No panic or exceptions occur.
 */
export const WindowFrame: React.FC<WindowFrameProps> = ({ children }) => {
  return (
    <div className="bg-[#121214] text-zinc-100 min-h-screen flex flex-col font-sans select-none overflow-hidden h-screen">
      {/* Main content container */}
      <div className="flex-1 flex flex-col overflow-hidden">
        {children}
      </div>
    </div>
  );
};
