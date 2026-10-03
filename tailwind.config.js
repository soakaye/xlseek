/**
 * Copyright (c) 2026 soakaye
 *
 * @fileoverview Tailwind CSS configuration (tailwind.config.js)
 *
 * ## Description
 * Configures Tailwind CSS content paths, dark mode, theme extensions, and plugins.
 *
 * ## Arguments & Returns
 * None. Exports Tailwind CSS Config object.
 *
 * ## Errors / Exceptions
 * None.
 */

/** @type {import('tailwindcss').Config} */
export default {
  darkMode: 'class',
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        excel: {
          DEFAULT: '#107C41',
          hover: '#0E6837',
          light: '#16A34A',
          subtle: '#064E3B'
        },
        surface: {
          DEFAULT: '#18181B', // zinc-900
          card: '#27272A',    // zinc-800
          input: '#1F1F23',
          hover: '#3F3F46',   // zinc-700
          border: '#3F3F46'
        }
      },
      fontFamily: {
        sans: ['"Segoe UI"', 'Meiryo', 'Inter', 'sans-serif'],
        mono: ['"Cascadia Code"', 'Consolas', 'monospace']
      }
    },
  },
  plugins: [],
}
