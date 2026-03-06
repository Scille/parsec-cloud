#!/usr/bin/env python
# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import argparse
import base64
import hashlib
import re
import sys
from pathlib import Path

PROJECT_DIR = Path(__file__).resolve().parent.parent
STATIC_DIR = PROJECT_DIR / "server/parsec/static"
TEMPLATES_DIR = PROJECT_DIR / "server/parsec/templates"

# Static asset filenames follow the pattern: <base_name>-<hash>.<extension>
ASSET_NAME_RE = re.compile(r"^(.+)-([A-Za-z0-9_\-]{8})(\.[^.]+)$")


def compute_hash(path: Path) -> str:
    data = path.read_bytes()
    digest = hashlib.sha256(data).digest()
    # Take first 6 bytes, encode as base64url (gives 8 chars, no padding)
    return base64.urlsafe_b64encode(digest[:6]).decode("ascii")


def get_static_assets() -> list[Path]:
    return [
        p
        for p in STATIC_DIR.iterdir()
        if p.is_file() and p.name != "__init__.py" and ASSET_NAME_RE.match(p.name)
    ]


def get_template_files() -> list[Path]:
    return [p for p in TEMPLATES_DIR.rglob("*") if p.is_file() and not p.suffix == ".pyc"]


def expected_name(asset: Path) -> str:
    m = ASSET_NAME_RE.match(asset.name)
    assert m, f"Unexpected asset name format: {asset.name}"
    base, _, ext = m.group(1), m.group(2), m.group(3)
    new_hash = compute_hash(asset)
    return f"{base}-{new_hash}{ext}"


def run_check() -> int:
    assets = get_static_assets()
    template_files = get_template_files()

    errors: list[str] = []

    for asset in sorted(assets):
        expected = expected_name(asset)

        if asset.name != expected:
            errors.append(f"  {asset.name!r} -> should be {expected!r}")
            continue

        # Check that the asset name is referenced at least once in templates
        found = False
        for tmpl in template_files:
            if asset.name in tmpl.read_text(encoding="utf-8", errors="replace"):
                found = True
                break
        if not found:
            errors.append(f"  {asset.name!r} is not referenced in any template file")

    if errors:
        print("Static asset hash check failed:", file=sys.stderr)
        for e in errors:
            print(e, file=sys.stderr)
        return 1

    print(f"All {len(assets)} static asset(s) have correct names and are referenced in templates.")
    return 0


def run_update() -> int:
    assets = get_static_assets()
    template_files = get_template_files()

    # Cache template contents to avoid repeated disk reads
    template_texts: dict[Path, str] = {t: t.read_text(encoding="utf-8") for t in template_files}

    changed = 0

    for asset in sorted(assets):
        expected = expected_name(asset)

        if asset.name == expected:
            continue

        old_name = asset.name
        new_path = asset.parent / expected

        # Rename the file
        asset.rename(new_path)
        print(f"Renamed: {old_name} -> {expected}")

        # Update references in all template files
        for tmpl, text in template_texts.items():
            if old_name in text:
                new_text = text.replace(old_name, expected)
                tmpl.write_text(new_text, encoding="utf-8")
                template_texts[tmpl] = new_text
                print(f"  Updated reference in {tmpl.relative_to(PROJECT_DIR)}")

        changed += 1

    if changed == 0:
        print("All static asset names are already up to date.")
    else:
        print(f"\nUpdated {changed} static asset(s).")

    return 0


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Update or check cache-busting hash suffixes for server static assets."
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="Only check that file names are correct and referenced in templates (no changes).",
    )
    args = parser.parse_args()

    if args.check:
        return run_check()
    else:
        return run_update()


if __name__ == "__main__":
    sys.exit(main())
