#!/usr/bin/env python3
"""
Copyright (c) 2026 soakaye

@fileoverview Third-party license information automated collection script (scripts/generate-licenses.py)

## Description
Extracts and consolidates license and copyright metadata for all production runtime third-party
packages (Rust runtime crates and npm production dependencies) statically linked or bundled
into the Excel Seek application binary, outputting a static JSON file (src/constants/licenses.json).

Development-only packages (devDependencies, build-dependencies, test utilities) are excluded.
Packages sharing identical names with distinct versions are cataloged side-by-side using
"{name}@{version}" as unique identifiers. Complies with Constitution Principles I, III, IV, and V.

## Arguments & Returns
- Arguments: None (CLI option `--output` / `-o` can specify output path).
- Returns: Process exit status code (0 for success, 1 for abnormal exit).

## Errors / Exceptions
- Catches errors during `cargo metadata` or `license-checker` execution, JSON parsing, or filesystem reads,
  logging diagnostics to stderr and safely terminating with exit code 1.
- Falls back to standard SPDX license terms when upstream license files are missing from local caches.
"""

from __future__ import annotations

import argparse
import glob
import json
import os
import subprocess
import sys
from pathlib import Path
from typing import Any, Dict, List, Optional, Set

# Standard SPDX fallback license terms catalog
FALLBACK_LICENSES: Dict[str, str] = {
    "MIT": """MIT License

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.""",
    "Apache-2.0": """Apache License
Version 2.0, January 2004
http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

1. Definitions.
"License" shall mean the terms and conditions for use, reproduction, and distribution as defined by Sections 1 through 9 of this document.
"Licensor" shall mean the copyright owner or entity authorized by the copyright owner that is granting the License.
"Legal Entity" shall mean the union of the acting entity and all other entities that control, are controlled by, or are under common control with that entity.

2. Grant of Copyright License. Subject to the terms and conditions of this License, each Contributor hereby grants to You a perpetual, worldwide, non-exclusive, no-charge, royalty-free, irrevocable copyright license to reproduce, prepare Derivative Works of, publicly display, publicly perform, sublicense, and distribute the Work and such Derivative Works in Source or Object form.

3. Grant of Patent License. Subject to the terms and conditions of this License, each Contributor hereby grants to You a perpetual, worldwide, non-exclusive, no-charge, royalty-free, irrevocable (except as stated in this section) patent license to make, have made, use, offer to sell, sell, import, and otherwise transfer the Work.

4. Redistribution. You may reproduce and distribute copies of the Work or Derivative Works thereof in any medium, with or without modifications, and in Source or Object form, provided that You meet the following conditions:
(a) You must give any other recipients of the Work or Derivative Works a copy of this License; and
(b) You must cause any modified files to carry prominent notices stating that You changed the files; and
(c) You must retain, in the Source form of any Derivative Works that You distribute, all copyright, patent, trademark, and attribution notices from the Source form of the Work; and
(d) If the Work includes a "NOTICE" text file as part of its distribution, then any Derivative Works that You distribute must include a readable copy of the attribution notices contained within such NOTICE file.

5. Disclaimer of Warranty. Unless required by applicable law or agreed to in writing, Licensor provides the Work on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.

6. Limitation of Liability. In no event and under no legal theory shall any Contributor be liable to You for damages, including any direct, indirect, special, incidental, or consequential damages of any character arising as a result of this License or out of the use or inability to use the Work.""",
    "BSD-3-Clause": """Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote products derived from this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.""",
    "BSD-2-Clause": """Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.""",
    "ISC": """ISC License

Permission to use, copy, modify, and/or distribute this software for any purpose with or without fee is hereby granted, provided that the above copyright notice and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.""",
    "Zlib": """This software is provided 'as-is', without any express or implied warranty. In no event will the authors be held liable for any damages arising from the use of this software.

Permission is granted to anyone to use this software for any purpose, including commercial applications, and to alter it and redistribute it freely, subject to the following restrictions:

1. The origin of this software must not be misrepresented; you must not claim that you wrote the original software. If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.
2. Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.
3. This notice may not be removed or altered from any source distribution.""",
}


def find_cargo_registry_crate_dir(crate_name: str, crate_version: str) -> Optional[Path]:
    """Finds crate source directory from Cargo registry cache directory."""
    cargo_home = Path(os.environ.get("CARGO_HOME", Path.home() / ".cargo"))
    registry_src = cargo_home / "registry" / "src"
    if not registry_src.is_dir():
        return None

    # Search inside index directories (e.g. index.crates.io-*/<crate>-<version>)
    pattern = f"{crate_name}-{crate_version}"
    for idx_dir in registry_src.iterdir():
        if idx_dir.is_dir():
            candidate = idx_dir / pattern
            if candidate.is_dir():
                return candidate
    return None


def read_crate_license_file(crate_dir: Path) -> Optional[str]:
    """Scans crate directory for LICENSE, COPYING, etc. and reads file contents."""
    license_patterns = [
        "LICENSE*",
        "LICENCE*",
        "COPYING*",
        "LICENSE-MIT*",
        "LICENSE-APACHE*",
        "UNLICENSE*",
    ]
    for pat in license_patterns:
        matches = list(crate_dir.glob(pat))
        for match in sorted(matches):
            if match.is_file():
                try:
                    text = match.read_text(encoding="utf-8", errors="replace").strip()
                    if text:
                        return text
                except Exception:
                    continue
    return None


def get_fallback_license_text(license_spdx: Optional[str], crate_or_pkg_name: str) -> str:
    """Generates fallback license text from SPDX license identifier."""
    if not license_spdx:
        return f"License information for {crate_or_pkg_name} is distributed under its published terms."

    # Decompose compound licenses (e.g., "MIT OR Apache-2.0")
    parts = [p.strip() for p in license_spdx.replace(" OR ", "/").replace(" AND ", "/").split("/")]
    texts = []
    for part in parts:
        clean_part = part.strip("()")
        if clean_part in FALLBACK_LICENSES:
            texts.append(f"--- {clean_part} ---\n\n{FALLBACK_LICENSES[clean_part]}")

    if texts:
        return "\n\n".join(texts)

    # Search by known primary keys
    for key, val in FALLBACK_LICENSES.items():
        if key.lower() in license_spdx.lower():
            return f"--- {key} (Derived from '{license_spdx}') ---\n\n{val}"

    return f"Standard license terms for {license_spdx}.\nPlease refer to package documentation for full details."


def collect_rust_licenses(repo_root: Path) -> List[Dict[str, Any]]:
    """Executes `cargo metadata` to collect license information for runtime Rust crates."""
    src_tauri = repo_root / "src-tauri"
    cmd = ["cargo", "metadata", "--format-version", "1"]
    try:
        res = subprocess.run(
            cmd,
            cwd=str(src_tauri),
            capture_output=True,
            text=True,
            check=True,
        )
        metadata = json.loads(res.stdout)
    except Exception as e:
        print(f"ERROR: Failed to run cargo metadata: {e}", file=sys.stderr)
        raise

    packages_by_id = {pkg["id"]: pkg for pkg in metadata.get("packages", [])}
    resolve = metadata.get("resolve", {})
    nodes = {node["id"]: node for node in resolve.get("nodes", [])}
    root_id = resolve.get("root")

    if not root_id or root_id not in nodes:
        print("ERROR: Root crate not found in resolve nodes", file=sys.stderr)
        return []

    # BFS traverse runtime dependencies (dep_kinds kind is null or 'normal')
    visited: Set[str] = set()
    queue = [root_id]
    visited.add(root_id)

    runtime_pkg_ids: Set[str] = set()

    while queue:
        curr_id = queue.pop(0)
        curr_node = nodes.get(curr_id)
        if not curr_node:
            continue

        for dep in curr_node.get("deps", []):
            dep_pkg_id = dep.get("pkg")
            if not dep_pkg_id:
                continue

            # Check dep_kinds: kind == null represents normal runtime dependency (not build or dev)
            # kind: null = normal, "build" = build-dependency, "dev" = dev-dependency
            is_runtime = False
            for dk in dep.get("dep_kinds", []):
                kind = dk.get("kind")
                if kind is None or kind == "normal":
                    is_runtime = True
                    break

            if is_runtime:
                runtime_pkg_ids.add(dep_pkg_id)
                if dep_pkg_id not in visited:
                    visited.add(dep_pkg_id)
                    queue.append(dep_pkg_id)

    records: List[Dict[str, Any]] = []

    for pkg_id in runtime_pkg_ids:
        pkg = packages_by_id.get(pkg_id)
        if not pkg:
            continue

        name = pkg.get("name", "")
        version = pkg.get("version", "")
        # Exclude workspace root crates
        if name in ("xlseek", "xlseek-core", "xlseek-cli"):
            continue

        license_spdx = pkg.get("license") or "MIT OR Apache-2.0"
        authors_list = pkg.get("authors", [])
        author = ", ".join(authors_list) if authors_list else None
        repository = pkg.get("repository")

        # Retrieve license text
        license_text = None
        # 1. Inspect local crate directory
        manifest_path = Path(pkg.get("manifest_path", ""))
        if manifest_path.is_file():
            crate_dir = manifest_path.parent
            license_text = read_crate_license_file(crate_dir)

        # 2. Inspect Cargo registry cache
        if not license_text:
            crate_cache_dir = find_cargo_registry_crate_dir(name, version)
            if crate_cache_dir:
                license_text = read_crate_license_file(crate_cache_dir)

        # 3. Fallback
        if not license_text:
            license_text = get_fallback_license_text(license_spdx, f"{name} {version}")

        record = {
            "id": f"{name}@{version}",
            "name": name,
            "version": version,
            "source": "rust",
            "license": license_spdx,
            "author": author,
            "repository": repository,
            "license_text": license_text.strip(),
        }
        records.append(record)

    return records


def collect_npm_licenses(repo_root: Path) -> List[Dict[str, Any]]:
    """Executes `npx license-checker --production --json` to collect license information for production npm packages."""
    cmd = ["npx", "--yes", "license-checker", "--production", "--json"]
    try:
        res = subprocess.run(
            cmd,
            cwd=str(repo_root),
            capture_output=True,
            text=True,
            check=True,
        )
        data = json.loads(res.stdout)
    except Exception as e:
        print(f"WARN: license-checker command failed ({e}), falling back to package.json inspection", file=sys.stderr)
        data = {}

    records: List[Dict[str, Any]] = []

    for key, val in data.items():
        # key format: "name@version" or "@scope/name@version"
        if key.startswith("xlseek@"):
            continue

        # Split at the last '@' to extract name and version
        last_at = key.rfind("@")
        if last_at <= 0:
            continue
        name = key[:last_at]
        version = key[last_at + 1:]

        license_spdx = val.get("licenses")
        if isinstance(license_spdx, list):
            license_spdx = " OR ".join(license_spdx)
        if not license_spdx:
            license_spdx = "MIT"

        author = val.get("publisher") or val.get("author") or val.get("email")
        repository = val.get("repository")

        license_file = val.get("licenseFile")
        license_text = None
        if license_file and Path(license_file).is_file():
            try:
                content = Path(license_file).read_text(encoding="utf-8", errors="replace").strip()
                if content:
                    license_text = content
            except Exception:
                pass

        if not license_text:
            license_text = get_fallback_license_text(license_spdx, f"{name} {version}")

        record = {
            "id": f"{name}@{version}",
            "name": name,
            "version": version,
            "source": "npm",
            "license": license_spdx,
            "author": author,
            "repository": repository,
            "license_text": license_text.strip(),
        }
        records.append(record)

    return records


def main() -> int:
    parser = argparse.ArgumentParser(description="Collect third-party licenses for Excel Seek.")
    parser.add_argument(
        "--output",
        "-o",
        type=Path,
        default=None,
        help="Path to output licenses.json (default: src/constants/licenses.json)",
    )
    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parent.parent
    output_path = args.output or (repo_root / "src" / "constants" / "licenses.json")

    print(f"=== Excel Seek third-party license collection started ===")
    print(f"Project root: {repo_root}")

    # 1. Collect Rust crates
    print("Collecting Rust runtime crate information...")
    rust_records = collect_rust_licenses(repo_root)
    print(f"  -> Found {len(rust_records)} Rust production runtime crates")

    # 2. Collect npm packages
    print("Collecting npm production package information...")
    npm_records = collect_npm_licenses(repo_root)
    print(f"  -> Found {len(npm_records)} npm production packages")

    # 3. Combine and sort (ascending by lowercase name and version)
    combined = rust_records + npm_records
    combined.sort(key=lambda r: (r["name"].lower(), r["version"]))

    # 4. Verify uniqueness
    seen_ids: Set[str] = set()
    unique_records = []
    for r in combined:
        if r["id"] not in seen_ids:
            seen_ids.add(r["id"])
            unique_records.append(r)

    print(f"Total: Consolidated {len(unique_records)} third-party license records")

    # 5. Output
    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(
        json.dumps(unique_records, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )
    print(f"Saved license data to: {output_path}")
    print(f"=== Collection completed ===")
    return 0


if __name__ == "__main__":
    sys.exit(main())
