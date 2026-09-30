/**
 * @fileoverview Frontend application entry point (src/main.tsx)
 *
 * ## Description
 * Initializes the React 18 root renderer and mounts the `App` component
 * into the DOM tree root element (`#root`) under StrictMode. Applies the global stylesheet `index.css`.
 * Complies with Constitution Principle I (English documentation), Principle II (external constants), and Principle III (comprehensive documentation).
 */

import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./index.css";
import { APP_CONSTANTS } from "./constants";

// Constant reference: APP_CONSTANTS.ROOT_ELEMENT_ID ("root")
const rootElement = document.getElementById(APP_CONSTANTS.ROOT_ELEMENT_ID);

if (rootElement) {
  ReactDOM.createRoot(rootElement).render(
    <React.StrictMode>
      <App />
    </React.StrictMode>
  );
}
