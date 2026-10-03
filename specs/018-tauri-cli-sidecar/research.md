# Research Findings: Bundle xlseek-cli as a Tauri Sidecar

## Decision 1: Use Tauri `bundle.externalBin`

- **Decision**: Declare the Sidecar base path in the bundle configuration in `src-tauri/tauri.conf.json` and provide target-specific CLI executables in `src-tauri/binaries/`.
- **Rationale**: The user requirement is to bundle the CLI as a Tauri Sidecar. Tauri's external-binary feature matches this boundary. Treating the CLI as a regular resource would not clearly enforce target-specific executable matching and would not use the Sidecar declaration.
- **Alternatives considered**: `bundle.resources` can include extra files, but `externalBin` better expresses the requirement for a target-specific executable. Making users download the CLI separately would not meet the bundling requirement.
- **Evidence**: Tauri v2 documents `externalBin` as embedding external binaries in the application and treats the configured path as relative to `tauri.conf.json`. [Tauri: Embedding External Binaries](https://v2.tauri.app/develop/sidecar/), [Tauri: Bundle Configuration](https://v2.tauri.app/reference/config/#bundleconfig)

## Decision 2: Build the CLI for Tauri's target triple

- **Decision**: Use `TAURI_ENV_TARGET_TRIPLE`, provided to Tauri's `beforeBuildCommand`, to build the CLI for that target. For a standalone `npm run build:cli`, use the Rust host triple. After a successful build, stage the executable in Tauri's binaries directory as `xlseek-cli-<triple>` (with `.exe` on Windows).
- **Rationale**: Always using the host triple can produce a GUI and CLI for different targets during cross-compilation. Tauri provides the actual build target to its before-build hook, so both artifacts can use the same input.
- **Alternatives considered**: Letting Cargo use its host default without specifying a target makes cross-target mismatch harder to detect. Checking precompiled executables into the repository risks stale binaries or binaries from another release.
- **Evidence**: Tauri requires each `externalBin` entry to have a target-triple suffix; its Windows example includes `.exe`. `TAURI_ENV_TARGET_TRIPLE` is set for `beforeBuildCommand`. [Tauri: Embedding External Binaries](https://v2.tauri.app/develop/sidecar/), [Tauri: Environment Variables](https://v2.tauri.app/ja/reference/environment-variables/), [Tauri: Configuration / beforeBuildCommand](https://v2.tauri.app/reference/config/)

## Decision 3: Limit this feature to the OS/CPU targets in current project documentation

- **Decision**: Build the CLI for native Windows x64 (`x86_64-pc-windows-msvc`), macOS Intel (`x86_64-apple-darwin`), macOS Apple Silicon (`aarch64-apple-darwin`), and Linux x64 GNU (`x86_64-unknown-linux-gnu`) targets. Do not add a macOS universal binary.
- **Rationale**: The READMEs list package formats for Windows, macOS, and Linux, and the macOS support plan includes Intel and Apple Silicon. The repository has no Linux arm64 or macOS universal distribution instructions, so this feature does not add CPU targets. Tauri `targets: "all"` selects package formats for one build target; it does not build every OS/CPU combination automatically.
- **Alternatives considered**: Supporting only the three OS families and making macOS universal was rejected because existing packaging and signing procedures were not established and this would introduce another target and packaging validation. Adding undocumented architectures would expand the existing support scope.
- **Evidence**: The READMEs list Windows, macOS, and Linux package types. `specs/003-macos-support/plan.md` describes Windows 10/11 and macOS 12+ for Intel and Apple Silicon. Tauri's target-specific binary examples require a matching file for each triple. [Tauri: Embedding External Binaries](https://v2.tauri.app/ja/develop/sidecar/), [Tauri: Bundle Configuration](https://v2.tauri.app/reference/config/#bundleconfig)

## Decision 4: Keep the bundled CLI standalone; do not launch it from the Tauri app

- **Decision**: Do not add Shell plugin Sidecar invocation, GUI controls, or new wrapper IPC for the CLI. Inspect and document the CLI's location in each OS distribution and verify that users can launch the executable directly.
- **Rationale**: Clarification answer A limits this feature to bundling and excludes launching the CLI from the app. This avoids adding application permissions and preserves the existing terminal CLI experience and exit statuses.
- **Alternatives considered**: Adding `shell:allow-execute` / `shell:allow-spawn` and launching from Tauri is out of scope. Registering the Sidecar on `PATH` would change installer and OS configuration beyond bundling.
- **Evidence**: Tauri's Sidecar examples describe the Shell plugin API and capability configuration, while declaring `externalBin` configures bundling. [Tauri: Embedding External Binaries](https://v2.tauri.app/develop/sidecar/), [Tauri: Shell Plugin](https://v2.tauri.app/reference/javascript/shell/)

## Risks and Validation Notes

- Tauri documentation defines Sidecar embedding and target-triple filenames, but does not guarantee a user-facing post-install path or `PATH` registration. Inspect actual Windows, macOS, and Linux distributions and run `--help` from the discovered location. If installer layouts differ, document locations per OS.
- Validate macOS Intel/Apple Silicon, Windows x64, and Linux x64 on their matching build hosts or with a properly configured cross-build environment.
- The CLI build hook must fail with the target triple and expected path if a target binary is missing; Tauri bundling must not succeed without it.
- The repository currently has no release workflow under `.github`. Do not add CI or signing secrets as part of this change. Ensure user documentation matches observed build artifacts.
