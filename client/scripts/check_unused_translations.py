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


def remove_subkeys(json_data, subkeys_to_remove):
    count = 0
    for subkey in subkeys_to_remove:
        keys = subkey.split(".")
        current_level = json_data
        for key in keys[:-1]:
            if key in current_level:
                current_level = current_level[key]
            else:
                break
        else:
            if keys[-1] in current_level:
                count += 1
                del current_level[keys[-1]]
    return count


def clean_data(obj):
    if isinstance(obj, dict):
        return {k: clean_data(v) for k, v in obj.items() if v != {}}
    return obj


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
    parser.add_argument(
        "--fix",
        help="Remove unused translation keys",
        action="store_true",
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

    # Process translation files to extract subkeys (as a flatten dict)
    translation_subkeys = {}
    for lang in languages:
        translation_subkeys[lang] = set(process_translation_file(locales / lang))

    # Compute unused subkeys only if needed
    unused_subkeys = []
    if args.fix or not args.skip_unused:
        print("Computing unused translation keys. This may take a while...")
        unused_subkeys = [
            subkey
            for subkey in sorted(translation_subkeys[ref_lang])
            if not is_present_in_sources(subkey, args.src)
        ]

    # fix
    if args.fix:
        for lang in languages:
            # 1. read
            with open(locales / lang, encoding="utf-8") as f:
                json_data = json.load(f)

            # 2. remove unused subkeys
            count = remove_subkeys(json_data, unused_subkeys)

            # 3. clean empty objects that may remain after subkey removal
            json_data = clean_data(json_data)

            # 4. write
            with open(locales / lang, mode="w", encoding="utf-8") as f:
                json.dump(json_data, f, indent=4, ensure_ascii=False)

            print(f"Removed {count} unused translations keys from {BOLD}{lang}{RESET}")

    # check
    else:
        if not args.skip_unused:
            for subkey in unused_subkeys:
                print(f"{HIGHLIGHT}{subkey}{RESET} was not found in sources", file=sys.stderr)

            if unused_subkeys:
                unused, total = len(unused_subkeys), len(translation_subkeys[ref_lang])
                print(
                    f"Unused {unused}/{total} (~{unused / total * 100:.2f}%). Re-run with --fix to remove them."
                )

        missing_subkeys = False
        if not args.skip_missing:
            for from_lang, to_lang in permutations(languages):
                for missing_subkey in sorted(
                    translation_subkeys[from_lang] - translation_subkeys[to_lang]
                ):
                    missing_subkeys = True
                    print(
                        f"{HIGHLIGHT}{missing_subkey}{RESET} is present in {BOLD}{from_lang}{RESET} but missing in {BOLD}{to_lang}{RESET}",
                        file=sys.stderr,
                    )
            if missing_subkeys:
                print(f"{HIGHLIGHT}Missing translation keys cannot be fixed automatically{RESET}")

        if missing_subkeys or unused_subkeys:
            sys.exit(1)
