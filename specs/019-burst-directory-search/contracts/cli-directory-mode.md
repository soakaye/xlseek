<!--
Purpose: Defines the command-line option, defaults, diagnostics, and compatibility rules for directory search modes.
Inputs: The feature specification, current CLI syntax, and data model (Markdown); output: an implementable user-facing CLI contract.
Errors: This document has no runtime behavior; invalid options and search failures are specified with existing exit conventions.
-->
# Contract: CLI Directory Search Mode

## Syntax

```text
xlseek-cli [OPTIONS] [QUERY] [PATH]...
--directory-mode sequential|burst
--directory-mode=sequential|burst
--burst-workers N
--burst-workers=N
```

`--directory-mode` takes one of two case-sensitive, language-neutral values. `--burst-workers` takes a whole number from 2 through 32. Neither option has a short alias. They may accompany the existing named or positional query and path forms. The selected mode and count apply to every directory root in an invocation. For a single-file input, the options are accepted but have no directory work to schedule.

If both options are omitted, the CLI uses `sequential` and does not inspect desktop settings. `--directory-mode burst` without `--burst-workers` uses Automatic, resolved to a bounded count from 2 through 8 based on available parallelism. `--burst-workers N` without a mode selects Burst with the specified count. A count combined with explicit `--directory-mode sequential` is a usage error. The CLI does not persist either choice. The mode and count control directory traversal and file discovery; existing concurrent workbook parsing, extension filtering, output formats, and output destination rules remain in effect.

## Validation and diagnostics

- A missing, duplicate, or unknown mode value is a usage error and exits with status 2, using the existing localized CLI error convention.
- A missing, duplicate, non-integer, or out-of-range worker count, or a worker count combined with explicit Sequential mode, is a usage error and exits with status 2. Values are rejected rather than silently clamped.
- `--help` continues to accept only the existing language selector alongside it. Both English and Japanese help list the mode and count options, their defaults, and the valid custom range.
- An invalid or unreadable root retains the current fatal error behavior. An inaccessible descendant folder is reported on stderr with its path while accessible folders continue. Preserve current partial-issue exit status 1 when readable workbooks exist, and status 2 for fatal input/output errors or when all discovered workbooks are unreadable.
- Search with zero matches and no issues remains successful with status 0.

## Result compatibility

CSV stdout, CSV file, and XLSX file formats remain unchanged. Search result row order and generated IDs are not stable across the two modes. Compare the multiset of semantic match fields, including path, sheet, cell address, match type, and content. Overlapping CLI roots keep the existing same-file deduplication policy.

## Acceptance examples

```text
xlseek-cli --query example --path <directory> --directory-mode sequential
xlseek-cli --query example --path <directory> --directory-mode=burst
xlseek-cli --query example --path <directory> --directory-mode burst --burst-workers 6
xlseek-cli --query example --path <directory> --burst-workers=6
xlseek-cli --query example --path <directory>
```

The first and fifth commands use sequential discovery. The second uses Burst with Automatic. The third and fourth use Burst with a maximum of six concurrent directory visitors. Given the same inputs, all five produce the same semantic matches.
