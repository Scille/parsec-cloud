#!/usr/bin/env python
from __future__ import annotations

import argparse
import re
import sys
from itertools import chain, dropwhile
from pathlib import Path
from typing import Iterable, Iterator, Type

PROJECT_DIR = Path(__file__).resolve().parent.parent

DIM = "\x1b[2m"
RESET_DIM = "\x1b[22m"


def extract_shebang_and_header_lines(fd):
    first_line = fd.readline()
    if first_line.startswith("#!"):
        shebang_line = first_line
        header_line = fd.readline()
    else:
        shebang_line = ""
        header_line = first_line
    return shebang_line, header_line


class Licenser:
    SPDX_ID: str

    @classmethod
    def generate_license_line(cls) -> str:
        raise NotImplementedError

    @classmethod
    def generate_license_label(cls) -> str:
        return cls.SPDX_ID

    @classmethod
    def generate_license_text(cls) -> str:
        return f"Parsec Cloud (https://parsec.cloud) Copyright (c) {cls.generate_license_label()} 2016-present Scille SAS"

    @classmethod
    def is_possible_license_line(cls, line: str) -> bool:
        return "Parsec Cloud (https://parsec.cloud) Copyright (c)" in line

    @classmethod
    def check_header(cls, file: Path) -> bool:
        with open(file, "r", encoding="utf-8") as fd:
            shebang_line, header_line = extract_shebang_and_header_lines(fd)
            expected_license_line = cls.generate_license_line()
            if header_line != expected_license_line:
                print(f"{DIM}{file}:{RESET_DIM} Missing {cls.SPDX_ID} header")
                return False

            for line, line_txt in enumerate(fd.readlines(), 3 if shebang_line else 2):
                if cls.is_possible_license_line(line_txt):
                    print(f"{DIM}{file}:{line}:{RESET_DIM} Header wrongly present")
                    return False

        return True

    @classmethod
    def add_header(cls, file: Path) -> bool:
        import difflib

        # Ratio at which we will consider the header line to be almost similar to the expected header line and need only replacement.
        DIFF_MIN_RATIO = 0.75

        with open(file, "r", encoding="utf-8") as fd:
            shebang_line, header_line = extract_shebang_and_header_lines(fd)
            expected_license_line = cls.generate_license_line()
            if header_line != expected_license_line:
                matcher = difflib.SequenceMatcher(None, header_line, expected_license_line)
                if matcher.ratio() >= DIFF_MIN_RATIO:
                    print(f"{DIM}{file}:{RESET_DIM} Replace previous header with `{cls.SPDX_ID}`")
                    updated_data = f"{shebang_line}{expected_license_line}{fd.read()}"
                else:
                    print(f"{DIM}{file}:{RESET_DIM} Add missing {cls.SPDX_ID} header")
                    updated_data = (
                        f"{shebang_line}{expected_license_line}\n{header_line}{fd.read()}"
                    )
                file.write_text(updated_data, encoding="utf-8")
                return True

        return False

    @classmethod
    def remove_header(cls, file: Path) -> bool:
        with open(file, "r", encoding="utf-8") as fd:
            lines = []
            need_rewrite = False
            for line in fd.readlines():
                if cls.is_possible_license_line(line):
                    print(f"{DIM}{file}:{RESET_DIM} Removing license header")
                    need_rewrite = True
                else:
                    lines.append(line)

        if need_rewrite:
            # Also skip the first empty lines
            content = "".join(dropwhile(lambda x: not x.strip(), lines))
            file.write_text(content, encoding="utf-8")
            return True
        else:
            return False


class BuslLicenserMixin(Licenser):
    SPDX_ID = "BUSL-1.1"


class PythonLicenserMixin(Licenser):
    @classmethod
    def generate_license_line(cls) -> str:
        return f"# {cls.generate_license_text()}\n"


class SqlLicenserMixin(Licenser):
    @classmethod
    def generate_license_line(cls) -> str:
        return f"-- {cls.generate_license_text()}\n"


class RustLicenserMixin(Licenser):
    @classmethod
    def generate_license_line(cls) -> str:
        return f"// {cls.generate_license_text()}\n"


class JavascriptLicenserMixin(Licenser):
    @classmethod
    def generate_license_line(cls) -> str:
        return f"// {cls.generate_license_text()}\n"


class HtmlLicenserMixin(Licenser):
    @classmethod
    def generate_license_line(cls) -> str:
        return f"<!-- {cls.generate_license_text()} -->\n"


class RstLicenserMixin(Licenser):
    @classmethod
    def generate_license_line(cls) -> str:
        return f".. {cls.generate_license_text()}\n"


class CppLicenserMixin(Licenser):
    @classmethod
    def generate_license_line(cls) -> str:
        return f"/* {cls.generate_license_text()} */\n"


class PythonBuslLicenser(BuslLicenserMixin, PythonLicenserMixin):
    pass


class SqlBuslLicenser(BuslLicenserMixin, SqlLicenserMixin):
    pass


class RustBuslLicenser(BuslLicenserMixin, RustLicenserMixin):
    pass


class JavascriptBuslLicenser(BuslLicenserMixin, JavascriptLicenserMixin):
    pass


class VueBuslLicenser(BuslLicenserMixin, HtmlLicenserMixin):
    pass


class RstBuslLicenser(BuslLicenserMixin, RstLicenserMixin):
    pass


class HtmlBuslLicenser(BuslLicenserMixin, HtmlLicenserMixin):
    pass


class CppBuslLicenser(BuslLicenserMixin, CppLicenserMixin):
    pass


class SkipLicenser(Licenser):
    @classmethod
    def check_header(cls, file: Path) -> bool:
        return True

    @classmethod
    def add_header(cls, file: Path) -> bool:
        return False

    @classmethod
    def remove_header(cls, file: Path) -> bool:
        return False


LICENSERS_MAP = {
    # First match is used
    re.compile(r"^misc/bench.py$"): SkipLicenser,
    # Special case for ourself given we contain the license headers in the source code !
    re.compile(r"^misc/license_headers.py$"): SkipLicenser,
    re.compile(r"^(.*/)?(target|node_modules|build|dist)/"): SkipLicenser,
    re.compile(r"^(libparsec|bindings)/.*\.(py|pyi)$"): PythonBuslLicenser,
    re.compile(r"^(libparsec|bindings)/.*\.rs$"): RustBuslLicenser,
    re.compile(r"^(libparsec|bindings)/.*\.sql$"): SqlBuslLicenser,
    re.compile(r"^server/parsec/backend/.*\.sql$"): SqlBuslLicenser,
    re.compile(r"^server/parsec/backend/.*\.(py|pyi)"): PythonBuslLicenser,
    re.compile(r"^server/src/.*\.rs"): RustBuslLicenser,
    re.compile(r"^docs/.*\.(py|pyi)$"): PythonBuslLicenser,
    re.compile(r"^docs/.*\.rst$"): RstBuslLicenser,
    re.compile(r"^docs/.*\.md$"): HtmlBuslLicenser,
    # Js project is a minefield full of node_modules/build/dist/assets etc.
    # so we just cut simple and add copyright only to the important stuff
    re.compile(r"^(client|bindings)/.*\.(ts|js)$"): JavascriptBuslLicenser,
    re.compile(r"^bindings/.*\.(ts|js)\.j2$"): JavascriptBuslLicenser,
    re.compile(r"^bindings/.*\.rs\.j2$"): RustBuslLicenser,
    re.compile(r"^client/src/.*\.vue$"): VueBuslLicenser,
    re.compile(r"^.*\.(py|pyi)$"): PythonBuslLicenser,
    re.compile(r"^.*\.sql$"): SqlBuslLicenser,
    re.compile(r"^windows-icon-handler/.*\.(cpp|h|idl)$"): CppBuslLicenser,
}


def get_files(paths: Iterable[Path]) -> Iterator[Path]:
    for path in paths:
        if path.is_dir():
            yield from chain(
                path.glob("**/*.py"),
                path.glob("**/*.pyi"),
                path.glob("**/*.sql"),
                path.glob("**/*.rst"),
                path.glob("**/*.rs"),
                path.glob("**/*.cpp"),
                path.glob("**/*.h"),
                path.glob("**/*.js"),
                path.glob("**/*.ts"),
                path.glob("**/*.vue"),
                path.glob("**/*.cpp"),
                path.glob("**/*.j2"),
                path.glob("**/*.h"),
                path.glob("**/*.md"),
                path.glob("**/*.idl"),
            )
        elif path.is_file():
            yield path
        elif not path.exists():
            raise SystemExit(f"Error: Path `{path}` doesn't exist !")


def get_licenser(path: Path) -> Type[Licenser] | None:
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


def remove_headers(files: Iterable[Path]) -> int:
    for file in get_files(files):
        licenser = get_licenser(file)
        if not licenser:
            continue
        licenser.remove_header(file)
    return 0


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("cmd", choices=["check", "add", "remove"])
    parser.add_argument(
        "files",
        nargs="*",
        type=Path,
        default=[
            Path("server"),
            Path("bindings"),
            Path("libparsec"),
            Path("packaging"),
            Path("misc"),
            Path("docs"),
            Path("windows-icon-handler"),
            Path(".github"),
        ],
    )

    args = parser.parse_args()

    fn = {"check": check_headers, "add": add_headers, "remove": remove_headers}[args.cmd]

    sys.exit(fn(args.files))
