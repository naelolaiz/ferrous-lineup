#!/usr/bin/env python3
"""Generate the Flatpak's cargo sources from `Cargo.lock`.

    python3 packaging/flathub/generate-sources.py

The output is the same format as
https://github.com/flatpak/flatpak-builder-tools/tree/master/cargo
without needing its extra Python packages: every crate is downloaded
from static.crates.io by its checksum in Cargo.lock, and Cargo is told
to read them from `cargo/vendor` (see `CARGO_HOME` in the manifest).

Run this after changing Cargo.lock and commit the result.
"""

from __future__ import annotations

import json
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
LOCKFILE = ROOT / "Cargo.lock"
OUTFILE = Path(__file__).with_name("cargo-sources.json")

CRATES_IO = "https://static.crates.io/crates"

CONFIG = """\
[source.vendored-sources]
directory = "cargo/vendor"

[source.crates-io]
replace-with = "vendored-sources"
"""


def main() -> None:
    lock = tomllib.loads(LOCKFILE.read_text(encoding="utf-8"))
    sources: list[dict[str, str]] = []

    for package in lock["package"]:
        source = package.get("source")
        if source is None:
            # A workspace member (lineup-core, lineup-app, ...), built
            # from the git checkout, not from crates.io.
            continue
        if not source.startswith("registry+"):
            raise SystemExit(
                f"{package['name']} comes from {source}, which this script "
                "does not know how to vendor. Use the official "
                "flatpak-cargo-generator.py for git dependencies."
            )

        name = package["name"]
        version = package["version"]
        checksum = package.get("checksum")
        if not checksum:
            raise SystemExit(f"{name} {version} has no checksum in Cargo.lock")

        dest = f"cargo/vendor/{name}-{version}"
        sources.append(
            {
                "type": "archive",
                "archive-type": "tar-gzip",
                "url": f"{CRATES_IO}/{name}/{name}-{version}.crate",
                "sha256": checksum,
                "dest": dest,
            }
        )
        sources.append(
            {
                "type": "inline",
                "contents": json.dumps({"package": checksum, "files": {}}),
                "dest": dest,
                "dest-filename": ".cargo-checksum.json",
            }
        )

    sources.append(
        {
            "type": "inline",
            "contents": CONFIG,
            "dest": "cargo",
            "dest-filename": "config",
        }
    )

    OUTFILE.write_text(json.dumps(sources, indent=4) + "\n", encoding="utf-8")
    print(f"wrote {OUTFILE.relative_to(ROOT)} ({len(sources) // 2} crates)")


if __name__ == "__main__":
    main()
