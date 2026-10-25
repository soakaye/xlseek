# Data Model: Standalone Multi-Platform xlseek-mcp Binary Release

**Feature**: `036-release-mcp-binaries`
**Date**: 2026-10-25

## Entities and Schemas

### 1. Release Asset

Represents an individual distributable archive uploaded to a GitHub Release.

| Field | Type | Description | Example (Synthetic Dummy Data) |
|---|---|---|---|
| `name` | String | Filename of the release archive | `xlseek-mcp-v1.0.0-aarch64-apple-darwin.tar.gz` |
| `archive_format` | Enum | Archive compression format (`tar.gz` or `zip`) | `tar.gz` |
| `target_triple` | String | Rust target compilation triple | `aarch64-apple-darwin` |
| `executable_name` | String | Bundled executable inside archive | `xlseek-mcp` |
| `version_tag` | String | Git version tag triggering the release | `v1.0.0` |
| `content_type` | String | MIME type for upload | `application/gzip` |

#### Synthetic Instances:

1. **macOS Apple Silicon Asset**:
   - `name`: `xlseek-mcp-v1.0.0-aarch64-apple-darwin.tar.gz`
   - `target_triple`: `aarch64-apple-darwin`
   - `executable_name`: `xlseek-mcp`
   - `archive_format`: `tar.gz`

2. **Linux x86_64 Asset**:
   - `name`: `xlseek-mcp-v1.0.0-x86_64-unknown-linux-gnu.tar.gz`
   - `target_triple`: `x86_64-unknown-linux-gnu`
   - `executable_name`: `xlseek-mcp`
   - `archive_format`: `tar.gz`

3. **Windows x86_64 Asset**:
   - `name`: `xlseek-mcp-v1.0.0-x86_64-pc-windows-msvc.zip`
   - `target_triple`: `x86_64-pc-windows-msvc`
   - `executable_name`: `xlseek-mcp.exe`
   - `archive_format`: `zip`

---

### 2. Matrix Target Configuration

Defines a build runner and packaging profile within the GitHub Actions workflow matrix.

| Field | Type | Description | Synthetic Example |
|---|---|---|---|
| `platform` | String | GitHub Actions runner label | `ubuntu-24.04` |
| `target` | String | Rust target triple | `x86_64-unknown-linux-gnu` |
| `binary_name` | String | Output binary name | `xlseek-mcp` |
| `archive_ext` | String | Archive extension | `tar.gz` |

#### Matrix Mapping:

```yaml
matrix:
  include:
    - platform: macos-latest
      target: aarch64-apple-darwin
      binary_name: xlseek-mcp
      archive_ext: tar.gz
    - platform: ubuntu-24.04
      target: x86_64-unknown-linux-gnu
      binary_name: xlseek-mcp
      archive_ext: tar.gz
    - platform: windows-latest
      target: x86_64-pc-windows-msvc
      binary_name: xlseek-mcp.exe
      archive_ext: zip
```

---

### 3. GitHub Release Context

Represents the release event context received from the repository trigger.

| Field | Type | Description | Synthetic Example |
|---|---|---|---|
| `ref_name` | String | Git reference name (tag) | `v1.0.0` |
| `release_title` | String | Title of the GitHub Release | `Excel Seek v1.0.0` |
| `draft` | Boolean | Whether release is initially drafted | `true` |
| `prerelease` | Boolean | Whether release is pre-release | `false` |
