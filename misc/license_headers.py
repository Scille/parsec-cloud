#!/usr/bin/env python
# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS


import sys
import re
import argparse
from pathlib import Path
from itertools import chain
from typing import Iterable, Iterator

THIS_YEAR = "2021"
BASE_HEADER = (
    f"Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-{THIS_YEAR} Scille SAS"
)
HEADERS = {".py": f"# {BASE_HEADER}", ".sql": f"-- {BASE_HEADER}"}
HEADER_RE = re.compile(
    r"^(#|--) Parsec Cloud \(https://parsec\.cloud\) Copyright \(c\) AGPLv3 2016-(?P<year>[0-9]{4}) Scille SAS$"
)
assert all(HEADER_RE.match(x) for x in HEADERS.values())  # Sanity check

SKIP_PATHES = (Path("parsec/core/gui/_resources_rc.py"), Path("parsec/core/gui/ui/"))


def need_skip(path: Path) -> bool:
    for skip_path in SKIP_PATHES:
        try:
            path.relative_to(skip_path)
            return True
        except ValueError:
            pass
    return False


def get_files(pathes: Iterable[Path]) -> Iterator[Path]:
    for path in pathes:
        if path.is_dir():
            for f in chain(Path(path).glob("**/*.py"), Path(path).glob("**/*.sql")):
                if need_skip(f):
                    continue
                yield f
        elif path.is_file():
            yield path
        elif not path.exists():
            raise SystemExit(f"Error: Path `{path}` doesn't exist !")


def extract_shabang_and_header_lines(fd):
    first_line = fd.readline()
    if first_line.startswith("#!"):
        shabang_line = first_line
        header_line = fd.readline()
    else:
        shabang_line = ""
        header_line = first_line
    return shabang_line, header_line


def check_headers(files: Iterable[Path]) -> int:
    ret = 0
    for file in get_files(files):
        with open(file, "r", encoding="utf-8") as fd:
            shabang_line, header_line = extract_shabang_and_header_lines(fd)
            match = HEADER_RE.match(header_line.strip())
            if not match:
                print("Missing header", file)
                ret = 1
            elif match["year"] != THIS_YEAR:
                print(
                    f"Wrong year in the header (got {match['year']}, should be {THIS_YEAR})", file
                )
                ret = 1
            for line, line_txt in enumerate(fd.read().split("\n"), 3 if shabang_line else 2):
                if HEADER_RE.match(line_txt.strip()):
                    print("Header wrongly present at line", line, file)
                    ret = 1
    return ret


def add_headers(files: Iterable[Path]) -> int:
    for file in get_files(files):
        with open(file, "r", encoding="utf-8") as fd:
            shabang_line, header_line = extract_shabang_and_header_lines(fd)
            match = HEADER_RE.match(header_line.strip())
            if not match:
                print("Add missing header", file)
                updated_data = f"{shabang_line}{HEADERS[file.suffix]}\n\n{header_line}{fd.read()}"
                file.write_text(updated_data)
            elif match["year"] != THIS_YEAR:
                print("Correct copyright year", file)
                updated_data = f"{shabang_line}{HEADERS[file.suffix]}\n{fd.read()}"
                file.write_text(updated_data)
    return 0


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("cmd", choices=["check", "add"])
    parser.add_argument("files", nargs="*", type=Path, default=[Path("parsec"), Path("tests")])

    args = parser.parse_args()
    if args.cmd == "check":
        sys.exit(check_headers(args.files))
    else:
        sys.exit(add_headers(args.files))
