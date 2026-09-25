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


def process_typescript(filepath: Path):
    found_keys = []
    # Check if file defines a 'translationPrefix' constant.
    # This is not bulletproof, but it seems to be an established convention :shrug:
    res = subprocess.run(
        ["grep", "const translationPrefix", filepath], capture_output=True, text=True
    )
    if res.returncode == 0:
        try:
            # Extract base key from the line where prefix is defined
            # e.g. "const translationPrefix = 'my.base.key';" --> my.base.key
            base_key = res.stdout.split("'")[1]

            # Look for lines using the translationPrefix variable and replace it with the base key
            # e.g. "... `${translationPrefix}.title`;" --> my.base.key.title
            with open(filepath) as vue_file:
                TRANSLATION_PREFIX_TAG = "${translationPrefix}"
                found_keys.extend(
                    line.split("`")[1].replace(TRANSLATION_PREFIX_TAG, base_key)
                    for line in vue_file
                    if TRANSLATION_PREFIX_TAG in line
                )

            print(f"-> Found prefixed keys in '{filepath}' ({len(found_keys)} keys)")

        except Exception as e:
            # The parsing above is the *antithesis* of bulletproof,
            # so just skip the file in case of error
            print(f"Error parsing '{filepath}': {e}", sys.stderr)

    return found_keys


def is_present_in_sources(translation_key: str, src: str):
    res = subprocess.run(["git", "grep", "-q", translation_key, src])
    return res.returncode == 0


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("src", help="Path to the client/src directory")
    parser.add_argument(
        "--locales",
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
        sys.stdout = open(os.devnull, "a")
        sys.stderr = open(os.devnull, "a")

    locales = Path(args.locales) if args.locales else Path(args.src) / "locales"

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
        # Process Vue and TS files to check for translations keys used with a prefix
        # e.g. `${translationPrefix}.title`
        used_with_prefix = []
        for subdir, _, files in os.walk(args.src):
            for file in files:
                if file.endswith((".vue", ".ts")):
                    used_with_prefix.extend(process_typescript(os.path.join(subdir, file)))

        for key in sorted(translation_keys[ref_lang]):
            if not is_present_in_sources(key, args.src) and key not in used_with_prefix:
                print(f"{HIGHLIGHT}{key}{RESET} was not found in sources", file=sys.stderr)
                unused_keys += 1

        if unused_keys:
            print(
                f"Missing {unused_keys}/{len(translation_keys[ref_lang])} (~{int(unused_keys / len(translation_keys[ref_lang]) * 100)}%)"
            )

    if missing_keys or unused_keys:
        sys.exit(1)
