#!/usr/bin/env python
# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS


import argparse
import re
import sys
from itertools import chain, dropwhile
from pathlib import Path
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
            shabang_line, header_line = extract_shabang_and_header_lines(fd)
            expected_license_line = cls.generate_license_line()
            if header_line != expected_license_line:
                print(f"Missing {cls.SPDX_ID} header", file)
                return False

            for line, line_txt in enumerate(fd.readlines(), 3 if shabang_line else 2):
                if cls.is_possible_license_line(line_txt):
                    print(f"Header wrongly present at {file}:{line}")
                    return False

        return True

    @classmethod
    def add_header(cls, file: Path) -> bool:
        with open(file, "r", encoding="utf-8") as fd:
            shabang_line, header_line = extract_shabang_and_header_lines(fd)
            expected_license_line = cls.generate_license_line()
            if header_line != expected_license_line:
                print(f"Add missing {cls.SPDX_ID} header: {file}")
                updated_data = f"{shabang_line}{expected_license_line}\n{header_line}{fd.read()}"
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
                    print(f"Removing license header from {file}")
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


class AgplLicenserMixin(Licenser):
    SPDX_ID = "AGPL-3.0"


class BuslLicenserMixin(Licenser):
    SPDX_ID = "BUSL-1.1"

    @classmethod
    def generate_license_label(cls) -> str:
        return f"{cls.SPDX_ID} (eventually AGPL-3.0)"


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


class VueLicenserMixin(Licenser):
    @classmethod
    def generate_license_line(cls) -> str:
        return f"<!-- {cls.generate_license_text()} -->\n"


class RstLicenserMixin(Licenser):
    @classmethod
    def generate_license_line(cls) -> str:
        return f".. {cls.generate_license_text()}\n"


class PythonAgplLicenser(AgplLicenserMixin, PythonLicenserMixin):
    pass


class PythonBuslLicenser(BuslLicenserMixin, PythonLicenserMixin):
    pass


class SqlAgplLicenser(AgplLicenserMixin, SqlLicenserMixin):
    pass


class SqlBuslLicenser(BuslLicenserMixin, SqlLicenserMixin):
    pass


class RustAgplLicenser(AgplLicenserMixin, RustLicenserMixin):
    pass


class RustBuslLicenser(BuslLicenserMixin, RustLicenserMixin):
    pass


class JavascriptAgplLicenser(AgplLicenserMixin, JavascriptLicenserMixin):
    pass


class JavascriptBuslLicenser(BuslLicenserMixin, JavascriptLicenserMixin):
    pass


class VueAgplLicenser(AgplLicenserMixin, VueLicenserMixin):
    pass


class VueBuslLicenser(BuslLicenserMixin, VueLicenserMixin):
    pass


class RstAgplLicenser(AgplLicenserMixin, RstLicenserMixin):
    pass


class RstBuslLicenser(BuslLicenserMixin, RstLicenserMixin):
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
    re.compile(r"^parsec/backend/.*\.sql$"): SqlBuslLicenser,
    re.compile(r"^parsec/backend/.*\.(py|pyi)"): PythonBuslLicenser,
    re.compile(r"^parsec/core/gui/_resources_rc.py$"): SkipLicenser,
    re.compile(r"^parsec/core/gui/ui/"): SkipLicenser,
    re.compile(r"^oxidation/(.*/)?(target|node_modules|build|dist)/"): SkipLicenser,
    re.compile(r"^oxidation/.*\.rs$"): RustBuslLicenser,
    re.compile(r"^oxidation/.*\.(py|pyi)$"): PythonBuslLicenser,
    re.compile(r"^oxidation/.*\.sql$"): SqlBuslLicenser,
    re.compile(r"^docs/.*\.(py|pyi)$"): PythonBuslLicenser,
    re.compile(r"^docs/.*\.rst$"): RstBuslLicenser,
    # Js project is a minefield full of node_modules/build/dist/assets etc.
    # so we just cut simple and add copyright only to the important stuff
    re.compile(r"^oxidation/client/src/.*\.(ts|js)$"): JavascriptBuslLicenser,
    re.compile(r"^oxidation/client/src/.*\.vue$"): VueBuslLicenser,
    # Special case for ourself given we contain the license headers in the source code !
    re.compile(r"^misc/license_headers.py$"): SkipLicenser,
    re.compile(r"^.*\.(py|pyi)$"): PythonAgplLicenser,
    re.compile(r"^.*\.sql$"): SqlAgplLicenser,
}


def get_files(pathes: Iterable[Path]) -> Iterator[Path]:
    for path in pathes:
        if path.is_dir():
            yield from chain(
                path.glob("**/*.py"),
                path.glob("**/*.pyi"),
                path.glob("**/*.sql"),
                path.glob("**/*.rst"),
                path.glob("**/*.rs"),
                path.glob("**/*.js"),
                path.glob("**/*.ts"),
                path.glob("**/*.vue"),
            )
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
            Path("parsec"),
            Path("tests"),
            Path("oxidation"),
            Path("packaging"),
            Path("misc"),
            Path("docs"),
            Path(".github"),
        ],
    )

    args = parser.parse_args()

    fn = {"check": check_headers, "add": add_headers, "remove": remove_headers}[args.cmd]

    sys.exit(fn(args.files))
