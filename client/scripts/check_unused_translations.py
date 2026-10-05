# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import argparse
import json
import os
import subprocess
import sys
from itertools import permutations
from pathlib import Path
from typing import Any

# ANSI colors for output (disable with --no-color)
BOLD = "\033[1m"
HIGHLIGHT = "\033[1;93m"
RESET = "\033[0m"

# Supported language files (add new ones here)
languages = (
    "en-US.json",  # reference language
    "fr-FR.json",
)
ref_lang = languages[0]


def process_translation_file(translation_source: Path) -> list[str]:
    content: dict[str, Any] = json.loads(translation_source.read_text())

    def _flatten_dict(d: dict[str, Any], path: str) -> list[str]:
        res: list[str] = []
        for key, value in d.items():
            new_path = f"{path}.{key}" if path else key
            if isinstance(value, dict):
                res.extend(_flatten_dict(value, new_path))
            else:
                res.append(new_path)
        return res

    return _flatten_dict(content, "")


def is_present_in_sources(translation_key: str, src: str):
    res = subprocess.run(["git", "grep", "-q", translation_key, src])
    return res.returncode == 0


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("src", help="Path to the client/src directory")
    parser.add_argument(
        "--locales-dir",
        help="Path to the directory containing the locale files (defaults to <src>/locales)",
    )
    parser.add_argument(
        "--skip-missing", help="Skip check for missing translation keys", action="store_true"
    )
    parser.add_argument(
        "--skip-unused", help="Skip check for unused translation keys", action="store_true"
    )
    parser.add_argument("--no-color", help="Disable colored output", action="store_true")
    parser.add_argument(
        "--quiet", help="Do not write anything to standard output", action="store_true"
    )

    args = parser.parse_args()

    if args.no_color:
        BOLD = HIGHLIGHT = RESET = ""

    if args.quiet:
        sys.stdout = sys.stderr = open(os.devnull, "a")

    locales = Path(args.locales_dir) if args.locales_dir else Path(args.src) / "locales"

    # Process translation files
    translation_keys = {}
    for lang in languages:
        translation_keys[lang] = set(process_translation_file(locales / lang))

    missing_keys = False
    if not args.skip_missing:
        print("Checking for missing translations...")
        for from_lang, to_lang in permutations(languages):
            print(f"-> Checking translation keys from {BOLD}{from_lang}{RESET}...")
            for missing_key in sorted(translation_keys[from_lang] - translation_keys[to_lang]):
                missing_keys = True
                print(
                    f"{HIGHLIGHT}{missing_key}{RESET} is missing in {BOLD}{to_lang}{RESET}",
                    file=sys.stderr,
                )

    unused_keys = 0
    if not args.skip_unused:
        print("Checking for unused translations...")
        for key in sorted(translation_keys[ref_lang]):
            if not is_present_in_sources(key, args.src):
                print(f"{HIGHLIGHT}{key}{RESET} was not found in sources", file=sys.stderr)
                unused_keys += 1

        if unused_keys:
            print(
                f"Missing {unused_keys}/{len(translation_keys[ref_lang])} (~{int(unused_keys / len(translation_keys[ref_lang]) * 100)}%)"
            )

    if missing_keys or unused_keys:
        sys.exit(1)
