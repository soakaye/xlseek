import React from "react";

interface WindowFrameProps {
  children: React.ReactNode;
}

export const WindowFrame: React.FC<WindowFrameProps> = ({ children }) => {
  return (
    <div className="bg-[#121214] text-zinc-100 min-h-screen flex flex-col font-sans select-none overflow-hidden h-screen">
      {/* メインコンテンツ */}
      <div className="flex-1 flex flex-col overflow-hidden">
        {children}
      </div>
    </div>
  );
};

