#!/usr/bin/env python3
"""Regenerate every platform icon from `ferrous-lineup.svg`.

    python3 packaging/icon/generate.py

The SVG is the only source. This script writes the files that are
committed, so they can be checked in and CI needs no image tools:

    packaging/macos/AppIcon.icns
    packaging/windows/ferrous-lineup.ico
    crates/lineup-app/assets/icon/icon-256.rgba

It needs `rsvg-convert` (librsvg) and Pillow. The macOS `.icns` is
assembled here rather than with `iconutil`, so the script also runs on
Linux and Windows.
"""

from __future__ import annotations

import shutil
import struct
import subprocess
import tempfile
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
SVG = ROOT / "packaging" / "icon" / "ferrous-lineup.svg"
MACOS_ICNS = ROOT / "packaging" / "macos" / "AppIcon.icns"
WINDOWS_ICO = ROOT / "packaging" / "windows" / "ferrous-lineup.ico"
APP_ICON = ROOT / "crates" / "lineup-app" / "assets" / "icon" / "icon-256.rgba"

# ICNS types and the pixel size they hold. The second block is the same
# picture at "retina" scale, which is what macOS actually asks for.
ICNS_SIZES = [
    ("icp4", 16),
    ("icp5", 32),
    ("icp6", 64),
    ("ic07", 128),
    ("ic08", 256),
    ("ic09", 512),
    ("ic10", 1024),
    ("ic11", 32),
    ("ic12", 64),
    ("ic13", 256),
    ("ic14", 512),
]
ICO_SIZES = [16, 24, 32, 48, 64, 128, 256]


def render(png: Path, size: int) -> None:
    subprocess.run(
        ["rsvg-convert", "-w", str(size), "-h", str(size), str(SVG), "-o", str(png)],
        check=True,
    )


def write_icns(pngs: dict[int, Path], out: Path) -> None:
    chunks = b""
    for kind, size in ICNS_SIZES:
        data = pngs[size].read_bytes()
        chunks += kind.encode("ascii") + struct.pack(">I", len(data) + 8) + data
    out.write_bytes(b"icns" + struct.pack(">I", len(chunks) + 8) + chunks)


def main() -> None:
    if shutil.which("rsvg-convert") is None:
        raise SystemExit("rsvg-convert is needed (install librsvg2-bin)")

    for parent in (MACOS_ICNS.parent, WINDOWS_ICO.parent, APP_ICON.parent):
        parent.mkdir(parents=True, exist_ok=True)

    with tempfile.TemporaryDirectory() as tmp:
        pngs = {}
        for size in sorted(set(ICO_SIZES) | {size for _, size in ICNS_SIZES}):
            path = Path(tmp) / f"{size}.png"
            render(path, size)
            pngs[size] = path

        # Windows: a multi-resolution .ico. Pillow resamples the 256 px
        # rendering into the smaller sizes.
        Image.open(pngs[256]).convert("RGBA").save(
            WINDOWS_ICO, format="ICO", sizes=[(s, s) for s in ICO_SIZES]
        )

        write_icns(pngs, MACOS_ICNS)

        # egui wants raw, unpremultiplied RGBA pixels for the window icon.
        APP_ICON.write_bytes(
            Image.open(pngs[256]).convert("RGBA").tobytes()
        )

    print(f"wrote {MACOS_ICNS.relative_to(ROOT)}")
    print(f"wrote {WINDOWS_ICO.relative_to(ROOT)}")
    print(f"wrote {APP_ICON.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
