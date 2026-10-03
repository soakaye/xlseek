# Bug Fix: Debianパッケージ名に空白が含まれる問題

- **Slug**: deb-package-name-spaces
- **Fixed**: 2026-10-02T17:23:45+09:00
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

Linux 向け Debian パッケージ（`.deb`）のビルド時、パッケージ出力名やファイル名に空白（`Excel Seek_0.1.0_amd64.deb`）が含まれる問題を修正しました。
Tauri v2 のプラットフォーム固有設定ファイル `src-tauri/tauri.linux.conf.json` を新規追加し、Linux ビルド環境での `productName` を空白のない `xlseek` に指定することで、生成されるパッケージ名が `xlseek_0.1.0_amd64.deb` となり、Linux の命名規則およびプロジェクト統一識別名に合致するようにしました。

## Changes

| File | Change | Notes |
|------|--------|-------|
| `src-tauri/tauri.linux.conf.json` | added | Linux 専用のオーバーライド設定として `"productName": "xlseek"` を定義 |
| `src-tauri/src/constants.rs` | modified | `tauri.linux.conf.json` が存在し `productName` が `xlseek` であることを検証する単体テストを追加 |

## Diff Highlights

```json
// src-tauri/tauri.linux.conf.json
{
  "$schema": "https://raw.githubusercontent.com/tauri-apps/tauri/dev/crates/tauri-cli/schema.json",
  "productName": "xlseek"
}
```

```rust
// src-tauri/src/constants.rs
#[test]
fn test_tauri_linux_config_product_name() {
    let manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| "src-tauri".to_string());
    let linux_conf_path = std::path::Path::new(&manifest_dir).join("tauri.linux.conf.json");
    let content =
        std::fs::read_to_string(&linux_conf_path).expect("tauri.linux.conf.json should exist");
    let parsed: serde_json::Value =
        serde_json::from_str(&content).expect("tauri.linux.conf.json should be valid JSON");
    assert_eq!(
        parsed["productName"], "xlseek",
        "Linux package productName must be xlseek without spaces"
    );
}
```

## Tests Added or Updated

- `src-tauri/src/constants.rs::tests::test_tauri_linux_config_product_name`: `tauri.linux.conf.json` が配置されており、`productName` が空白なしの `"xlseek"` に設定されていることを検証。

## Local Verification

- `cargo test -p xlseek --lib constants::tests`: PASS (7 passed)
- `cargo fmt --check`: PASS (Clean format)
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS (0 warnings)
- `npm run build`: PASS (0 TypeScript errors, bundle successful)
- `npx vitest run --pool=threads`: PASS (19 test files, 125 passed)

## Deviations from Assessment

None. Preferred remediation 1 (`src-tauri/tauri.linux.conf.json`) was applied as planned.

## Follow-ups

- 次の検証ステップ: `/speckit-bug-test slug=deb-package-name-spaces`
