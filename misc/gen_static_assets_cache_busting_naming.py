#!/usr/bin/env python
# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

import argparse
import base64
import hashlib
import re
import sys
from fnmatch import fnmatch
from pathlib import Path

# Static asset filenames follow the pattern: <base_name>-<hash>.<extension>
ASSET_NAME_RE = re.compile(r"^(.+)-([A-Za-z0-9_\-]{8})(\.[^.]+)$")
PROJECT_DIR = Path(__file__).parent.parent.resolve()


def compute_hash(path: Path) -> str:
    data = path.read_bytes()
    digest = hashlib.sha256(data).digest()
    # Take first 6 bytes, encode as base64url (gives 8 chars, no padding)
    return base64.urlsafe_b64encode(digest[:6]).decode("ascii")


def expected_name(asset: Path) -> str:
    m = ASSET_NAME_RE.match(asset.name)
    assert m, f"Unexpected asset name format: {asset.name}"
    base, _, ext = m.group(1), m.group(2), m.group(3)
    new_hash = compute_hash(asset)
    return f"{base}-{new_hash}{ext}"


def compute_asset_name_re(asset: Path) -> re.Pattern[str]:
    m = ASSET_NAME_RE.match(asset.name)
    assert m, f"Unexpected asset name format: {asset.name}"
    base, _, ext = m.group(1), m.group(2), m.group(3)
    return re.compile(re.escape(base) + r"-([A-Za-z0-9_\-]{8})" + re.escape(ext))


def run_check(assets: list[Path], references: list[Path]) -> int:
    errors: list[str] = []

    for asset in sorted(assets):
        expected = expected_name(asset)

        if asset.name != expected:
            errors.append(f"  `{asset.relative_to(PROJECT_DIR)}` -> should be named `{expected}`")
            continue

        # Check that the asset name is referenced at least once among reference files
        # and ensure it is not referenced somewhere else with an outdated hash.
        asset_name_re = compute_asset_name_re(asset)
        found = False
        for reference in references:
            try:
                text = reference.read_text("utf8")
            except UnicodeDecodeError:
                # Ignore binary files (they are typically present if the assets
                # directory also contains some references).
                continue

            for match in asset_name_re.finditer(text):
                found = True
                if match.group(0) != expected:
                    errors.append(
                        f"  `{reference.relative_to(PROJECT_DIR)}` references `{expected}` with the wrong hash: `{match.group(0)}`"
                    )

        if not found:
            errors.append(f"  `{asset.name}` is not referenced in any reference file")

    if errors:
        print("Static asset hash check failed:", file=sys.stderr)
        for e in errors:
            print(e, file=sys.stderr)
        return 1

    print(f"All {len(assets)} static asset(s) have correct names and are referenced.")
    return 0


def run_update(assets: list[Path], references: list[Path]) -> int:

    # Cache template contents to avoid repeated disk reads
    reference_texts: dict[Path, str] = {}
    for reference in references:
        try:
            reference_texts[reference] = reference.read_text("utf8")
        except UnicodeDecodeError:
            # Ignore binary files (they are typically present if the assets
            # directory also contains some references).
            continue

    # After a rename we must update everything referencing the old path to avoid `FileNotFound`
    def _update_lists_after_rename(old: Path, new: Path) -> None:
        for i, asset in enumerate(assets):
            if asset != old:
                continue
            assets[i] = new

        for i, asset in enumerate(references):
            if asset != old:
                continue
            references[i] = new

        if old in reference_texts:
            reference_texts[new] = reference_texts.pop(old)

    changes = 0

    for asset in sorted(assets):
        expected = expected_name(asset)

        if asset.name != expected:
            # Rename the file
            new_path = asset.parent / expected
            asset.rename(new_path)
            _update_lists_after_rename(asset, new_path)
            changes += 1
            print(f"Renamed: `{asset.relative_to(PROJECT_DIR)}` -> `{expected}`")

        # Update references in all template files
        asset_name_re = compute_asset_name_re(asset)
        for reference, text in reference_texts.items():
            new_text = asset_name_re.sub(expected, text)
            if new_text == text:
                continue
            reference.write_text(new_text, encoding="utf-8")
            reference_texts[reference] = new_text
            changes += 1
            print(f"  Updated reference in `{reference.relative_to(PROJECT_DIR)}`")

    return changes


TARGETS: dict[str, tuple[Path, list[Path], list[str]]] = {
    "server": (
        PROJECT_DIR / "server/parsec/static/",
        [PROJECT_DIR / "server/parsec/templates/"],
        ["**/README.md", "**/__init__.py", "*.pyc"],
    ),
    "client": (
        PROJECT_DIR / "client/public/",
        [PROJECT_DIR / "client/public/", PROJECT_DIR / "client/src/"],
        ["**/README.md"],
    ),
}


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Update or check cache-busting hash suffixes for static assets."
    )
    parser.add_argument("target", choices=TARGETS, help="The static assets to process.")
    parser.add_argument(
        "--check",
        action="store_true",
        help="Only check that file names are correct and referenced (no changes).",
    )
    args = parser.parse_args()

    assets_dir, references_dirs, ignores = TARGETS[args.target]

    def _should_ignore(x: str) -> bool:
        return any(fnmatch(x, ignore) for ignore in ignores)

    assets = [
        asset_file
        for asset_file in assets_dir.rglob("*")
        if asset_file.is_file() and not _should_ignore(str(asset_file))
    ]
    references = [
        reference_file
        for references_dir in references_dirs
        for reference_file in references_dir.rglob("*")
        if reference_file.is_file() and not _should_ignore(str(reference_file))
    ]

    if args.check:
        return run_check(assets, references)
    else:
        changes = run_update(assets, references)
        # Some assets contain references, in this case we have to re-run the
        # update until it stabilizes... or until we detect a cyclic dependency!
        for _ in range(3):
            if run_update(assets, references) == 0:
                break
        else:
            raise SystemExit("Cyclic dependency between assets and references")

        if changes == 0:
            print("All static asset names are already up to date.")
            return 0
        else:
            print(f"\nUpdated {changes} static asset(s) & reference(s).")
            return 1


if __name__ == "__main__":
    sys.exit(main())
