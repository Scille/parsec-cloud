# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import argparse
import pathlib
import json
import subprocess


def process_translation_file(translation_source: pathlib.Path):
    content = json.loads(translation_source.read_text())

    def _flatten_dict(d: dict, path: str):
        res = []

        for key, value in d.items():
            new_path = f"{path}.{key}" if path else key
            if isinstance(value, dict):
                res.extend(_flatten_dict(value, new_path))
            else:
                res.append(new_path)
        return res

    return _flatten_dict(content, "")


def get_translation_keys(folder: pathlib.Path):
    locales = {}

    locales["en"] = process_translation_file(folder / "en-US.json")
    locales["fr"] = process_translation_file(folder / "fr-FR.json")
    return locales


def is_present_in_sources(translation_key: str, src):
    res = subprocess.run(["git", "grep", "-q", translation_key, src])
    return res.returncode == 0


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("-t", help="Translation files dir")
    parser.add_argument("src")

    args = parser.parse_args()

    translation_keys = get_translation_keys(pathlib.Path(args.t))

    print("Checking if everything in `en` is in `fr`...")
    for key in translation_keys["en"]:
        if not key in translation_keys["fr"]:
            print(f"`{key}` not in fr")

    print("Checking if everything in `fr` is in `en`...")
    for key in translation_keys["fr"]:
        if not key in translation_keys["en"]:
            print(f"`{key}` not in en")

    print("Looking for unused translations...")
    missing = 0
    for key in translation_keys["en"]:
        if not is_present_in_sources(key, args.src):
            print(f"`{key}` not found in sources")
            missing += 1
    if missing > 0:
        print(
            f"Missing {missing} / {len(translation_keys['en'])} (~{int(missing / len(translation_keys['en']) * 100)}%)"
        )
    else:
        print("Nothing to report, congratulations!")
