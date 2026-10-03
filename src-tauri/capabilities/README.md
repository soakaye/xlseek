# Tauri capability configuration

## Description

`default.json` consolidates Tauri plugin capabilities and permissions required by the web frontend. `i18n:default` allows reading embedded translation catalogs and referencing/updating the display language.

## Arguments & Returns

Tauri loads `default.json` at startup and exposes only explicitly declared plugin permissions to the WebView.

## Errors / Exceptions

Invalid JSON or undeclared permissions cause Tauri startup and build validation to fail.

