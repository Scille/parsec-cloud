#!/usr/bin/env python
# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-2021 Scille SAS


import sys
import re
import argparse
from pathlib import Path
from itertools import chain
from typing import Iterable, Iterator


PROJECT_DIR = Path(__file__).resolve().parent.parent


def extract_shabang_and_header_lines(fd):
    first_line = fd.readline()
    if first_line.startswith("#!"):
        shabang_line = first_line
        header_line = fd.readline()
    else:
        shabang_line = ""
        header_line = first_line
    return shabang_line, header_line


class Licenser:
    NAME: str
    HEADER: str
    HEADER_RE: re.Pattern

    @classmethod
    def check_header(cls, file: Path) -> bool:
        with open(file, "r", encoding="utf-8") as fd:
            shabang_line, header_line = extract_shabang_and_header_lines(fd)
            match = cls.HEADER_RE.match(header_line.strip())
            if not match:
                print(f"Missing {cls.NAME} header", file)
                return False

            for line, line_txt in enumerate(fd.read().split("\n"), 3 if shabang_line else 2):
                if cls.HEADER_RE.match(line_txt.strip()):
                    print("Header wrongly present at line", line, file)
                    return False

        return True

    @classmethod
    def add_header(cls, file: Path) -> bool:
        with open(file, "r", encoding="utf-8") as fd:
            shabang_line, header_line = extract_shabang_and_header_lines(fd)
            match = cls.HEADER_RE.match(header_line.strip())
            if not match:
                print(f"Add missing {cls.NAME} header", file)
                updated_data = f"{shabang_line}{cls.HEADER}\n\n{header_line}{fd.read()}"
                file.write_text(updated_data)
                return True

        return False


class PythonAgplLicenser(Licenser):
    NAME = "AGPL-3.0"
    HEADER = (
        f"# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS"
    )
    HEADER_RE = re.compile(
        r"^# Parsec Cloud \(https://parsec\.cloud\) Copyright \(c\) AGPL-3.0 2016-present Scille SAS$"
    )


class PythonBuslLicenser(Licenser):
    NAME = "BUSL-1.1"
    HEADER = f"# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS"
    HEADER_RE = re.compile(
        r"^# Parsec Cloud \(https://parsec\.cloud\) Copyright \(c\) BUSL-1.1 \(eventually AGPL-3.0\) 2016-present Scille SAS$"
    )


class SqlBuslLicenser(Licenser):
    NAME = "BUSL-1.1"
    HEADER = f"-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS"
    HEADER_RE = re.compile(
        r"^-- Parsec Cloud \(https://parsec\.cloud\) Copyright \(c\) BUSL-1.1 \(eventually AGPL-3.0\) 2016-present Scille SAS$"
    )


class SqlAgplLicenser(Licenser):
    NAME = "AGPL-3.0"
    HEADER = (
        f"-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS"
    )
    HEADER_RE = re.compile(
        r"^-- Parsec Cloud \(https://parsec\.cloud\) Copyright \(c\) AGPL-3.0 2016-present Scille SAS$"
    )


class RustBuslLicenser(Licenser):
    NAME = "BUSL-1.1"
    HEADER = f"// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS"
    HEADER_RE = re.compile(
        r"^// Parsec Cloud \(https://parsec\.cloud\) Copyright \(c\) BUSL-1.1 \(eventually AGPL-3.0\) 2016-present Scille SAS$"
    )


class JavascriptBuslLicenser(Licenser):
    NAME = "BUSL-1.1"
    HEADER = f"// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS"
    HEADER_RE = re.compile(
        r"^// Parsec Cloud \(https://parsec\.cloud\) Copyright \(c\) BUSL-1.1 \(eventually AGPL-3.0\) 2016-present Scille SAS$"
    )


class VueBuslLicenser(Licenser):
    NAME = "BUSL-1.1"
    HEADER = f"<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->"
    HEADER_RE = re.compile(
        r"^<!-- Parsec Cloud \(https://parsec\.cloud\) Copyright \(c\) BUSL-1.1 \(eventually AGPL-3.0\) 2016-present Scille SAS -->$"
    )


class RstBuslLicenser(Licenser):
    NAME = "BUSL-1.1"
    HEADER = f".. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS"
    HEADER_RE = re.compile(
        r"^\.\. Parsec Cloud \(https://parsec\.cloud\) Copyright \(c\) BUSL-1.1 \(eventually AGPL-3.0\) 2016-present Scille SAS$"
    )


class SkipLicenser(Licenser):
    @classmethod
    def check_header(cls, file: Path) -> bool:
        return True

    @classmethod
    def add_header(cls, file: Path) -> bool:
        return False


LICENSERS_MAP = {
    # First match is used
    re.compile(r"^parsec/backend/.*\.sql$"): SqlBuslLicenser,
    re.compile(r"^parsec/backend/.*\.py"): PythonBuslLicenser,
    re.compile(r"^parsec/core/gui/_resources_rc.py$"): SkipLicenser,
    re.compile(r"^parsec/core/gui/ui/"): SkipLicenser,
    re.compile(r"^oxidation/(.*/)?(target|node_modules|build|dist)/"): SkipLicenser,
    re.compile(r"^oxidation/.*\.rs$"): RustBuslLicenser,
    re.compile(r"^oxidation/.*\.py$"): PythonBuslLicenser,
    re.compile(r"^oxidation/.*\.sql$"): SqlBuslLicenser,
    re.compile(r"^docs/.*\.py$"): PythonBuslLicenser,
    re.compile(r"^docs/.*\.rst$"): RstBuslLicenser,
    # Js project is a minefield full of node_modules/build/dist/assets etc.
    # so we just cut simple and add copyright only to the important stuff
    re.compile(r"^oxidation/client/src/.*\.(ts|js)$"): JavascriptBuslLicenser,
    re.compile(r"^oxidation/client/src/.*\.vue$"): VueBuslLicenser,
    re.compile(r"^.*\.py$"): PythonAgplLicenser,
    re.compile(r"^.*\.sql$"): SqlAgplLicenser,
}


def get_files(pathes: Iterable[Path]) -> Iterator[Path]:
    for path in pathes:
        if path.is_dir():
            for f in chain(
                path.glob("**/*.py"),
                path.glob("**/*.sql"),
                path.glob("**/*.rst"),
                path.glob("**/*.rs"),
                path.glob("**/*.js"),
                path.glob("**/*.ts"),
                path.glob("**/*.vue"),
            ):
                yield f
        elif path.is_file():
            yield path
        elif not path.exists():
            raise SystemExit(f"Error: Path `{path}` doesn't exist !")


def get_licenser(path: Path) -> Licenser:
    for regex, licenser in LICENSERS_MAP.items():
        if regex.match(path.absolute().relative_to(PROJECT_DIR).as_posix()):
            return licenser
    else:
        return None


def check_headers(files: Iterable[Path]) -> int:
    ret = 0
    for file in get_files(files):
        licenser = get_licenser(file)
        if not licenser:
            continue
        if not licenser.check_header(file):
            ret = 1
    return ret


def add_headers(files: Iterable[Path]) -> int:
    for file in get_files(files):
        licenser = get_licenser(file)
        if not licenser:
            continue
        licenser.add_header(file)
    return 0


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("cmd", choices=["check", "add"])
    parser.add_argument(
        "files", nargs="*", type=Path, default=[Path("parsec"), Path("tests"), Path("oxidation")]
    )

    args = parser.parse_args()
    if args.cmd == "check":
        sys.exit(check_headers(args.files))
    else:
        sys.exit(add_headers(args.files))
