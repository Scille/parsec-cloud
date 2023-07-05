# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

"""
Helper that help changing the version of a tool across the repository.
"""

import enum
import re
import sys
from argparse import ArgumentParser
from dataclasses import dataclass
from fileinput import FileInput
from pathlib import Path
from typing import Callable, Dict, List, Pattern, Tuple, Union

ReplacementPattern = Union[str, Callable[[str], str]]

RawRegexes = List["ReplaceRegex"]

ROOT_DIR = (Path(__file__) / "../..").resolve()


@dataclass
class ReplaceRegex:
    regex: str
    replaced: ReplacementPattern

    def compile(self, version: str) -> Tuple[Pattern[str], str]:
        regex = re.compile(self.regex)
        replace = (
            self.replaced.format(version=version)
            if isinstance(self.replaced, str)
            else self.replaced(version)
        )

        return (regex, replace)


def hide_patch_version(template: str, separator: str = ".") -> Callable[[str], str]:
    def _hide_patch_version(version: str) -> str:
        return template.format(version=separator.join(version.split(".")[:-1]))

    return _hide_patch_version


PYTHON_GA_VERSION = ReplaceRegex(
    r"python-version: [0-9.]+", hide_patch_version("python-version: {version}")
)
POETRY_GA_VERSION = ReplaceRegex(r"poetry-version: [0-9.]+", "poetry-version: {version}")
NODE_GA_VERSION = ReplaceRegex(r"node-version: [0-9.]+", "node-version: {version}")
WASM_PACK_GA_VERSION = ReplaceRegex(r"wasm-pack-version: [0-9.]+", "wasm-pack-version: {version}")
PYTHON_DOCKER_VERSION = ReplaceRegex(r"python:\d.\d+", hide_patch_version("python:{version}"))
PYTHON_SMALL_VERSION = ReplaceRegex(r"python\d.\d+", hide_patch_version("python{version}"))


@enum.unique
class Tool(enum.Enum):
    Rust = "rust"
    Python = "python"
    Poetry = "poetry"
    Node = "node"
    WasmPack = "wasm-pack"
    Parsec = "parsec"


TOOLS_VERSION: Dict[Tool, str] = {
    Tool.Rust: "1.68.0",
    Tool.Python: "3.9.10",
    Tool.Poetry: "1.3.2",
    Tool.Node: "18.12.0",
    Tool.WasmPack: "0.11.0",
    Tool.Parsec: "2.16.0-rc.1+dev",
}


FILES_WITH_VERSION_INFO: Dict[Path, Dict[Tool, RawRegexes]] = {
    ROOT_DIR
    / ".github/workflows/ci-python.yml": {
        Tool.Python: [PYTHON_GA_VERSION],
        Tool.Poetry: [POETRY_GA_VERSION],
    },
    ROOT_DIR
    / ".github/workflows/ci-web.yml": {
        Tool.Node: [NODE_GA_VERSION],
    },
    ROOT_DIR
    / ".github/workflows/codeql.yml": {
        Tool.Python: [PYTHON_GA_VERSION],
        Tool.Poetry: [POETRY_GA_VERSION],
    },
    ROOT_DIR
    / ".github/workflows/package-python.yml": {
        Tool.Python: [PYTHON_GA_VERSION],
        Tool.Poetry: [POETRY_GA_VERSION],
        Tool.Node: [NODE_GA_VERSION],
    },
    ROOT_DIR
    / ".github/workflows/package-webapp.yml": {
        Tool.Node: [NODE_GA_VERSION],
        Tool.WasmPack: [WASM_PACK_GA_VERSION],
    },
    ROOT_DIR
    / "docs/development/quickstart.md": {
        Tool.Rust: [
            ReplaceRegex(r"Rust v[0-9.]+", "Rust v{version}"),
            ReplaceRegex(
                r"--default-toolchain none # You can replace `none` with `[0-9.]+`",
                "--default-toolchain none # You can replace `none` with `{version}`",
            ),
        ],
        Tool.Python: [
            ReplaceRegex(
                r"python v[0-9.]+",
                hide_patch_version("python v{version}"),
            ),
            ReplaceRegex(r"pyenv install [0-9.]+", "pyenv install {version}"),
            ReplaceRegex(r"pyenv prefix [0-9.]+", "pyenv prefix {version}"),
        ],
        Tool.Poetry: [
            ReplaceRegex(r"poetry >=[0-9.]+", "poetry >={version}"),
            ReplaceRegex(
                r"https://install.python-poetry.org/ \| python - --version=.*",
                "https://install.python-poetry.org/ | python - --version={version}",
            ),
        ],
        Tool.Node: [ReplaceRegex(r"Node [0-9.]+", "Node {version}")],
        Tool.WasmPack: [ReplaceRegex(r"wasm-pack@[0-9.]+", "wasm-pack@{version}")],
    },
    ROOT_DIR
    / "licenses/BUSL-Scille.txt": {
        Tool.Parsec: [
            ReplaceRegex(r"^Licensed Work:  Parsec v.*$", "Licensed Work:  Parsec v{version}")
        ]
    },
    ROOT_DIR
    / "misc/version_updater.py": {
        Tool.Rust: [ReplaceRegex(r'Tool.Rust: "[0-9.]+"', 'Tool.Rust: "{version}"')],
        Tool.Python: [ReplaceRegex(r'Tool.Python: "[0-9.]+"', 'Tool.Python: "{version}"')],
        Tool.Poetry: [ReplaceRegex(r'Tool.Poetry: "[0-9.]+"', 'Tool.Poetry: "{version}"')],
        Tool.Node: [ReplaceRegex(r'Tool.Node: "[0-9.]+"', 'Tool.Node: "{version}"')],
        Tool.WasmPack: [ReplaceRegex(r'Tool.WasmPack: "[0-9.]+"', 'Tool.WasmPack: "{version}"')],
        Tool.Parsec: [ReplaceRegex(r'Tool.Parsec: "[0-9.]+.*",', 'Tool.Parsec: "{version}",')],
    },
    ROOT_DIR / "packaging/server/res/build-backend.sh": {Tool.Python: [PYTHON_SMALL_VERSION]},
    ROOT_DIR / "packaging/server/server.dockerfile": {Tool.Python: [PYTHON_DOCKER_VERSION]},
    ROOT_DIR / "packaging/snap/bin/parsec-setup.sh": {Tool.Python: [PYTHON_SMALL_VERSION]},
    ROOT_DIR
    / "packaging/snap/snap/snapcraft.yaml": {
        Tool.Parsec: [ReplaceRegex(r"^version: (.*)$", "version: {version}")],
        Tool.Python: [
            PYTHON_SMALL_VERSION,
            ReplaceRegex(
                r"python3 --version \| grep 'Python [0-9.]+'",
                "python3 --version | grep 'Python {version}'",
            ),
        ],
    },
    ROOT_DIR / "packaging/testbed-server/build-testbed.sh": {Tool.Python: [PYTHON_SMALL_VERSION]},
    ROOT_DIR
    / "packaging/testbed-server/testbed-server.dockerfile": {Tool.Python: [PYTHON_DOCKER_VERSION]},
    ROOT_DIR
    / "parsec/_version.py": {
        Tool.Parsec: [ReplaceRegex(r'^__version__ = ".*"$', '__version__ = "v{version}"')]
    },
    ROOT_DIR
    / "mypy.ini": {
        Tool.Python: [
            ReplaceRegex(
                r"python_version = \d.\d+", hide_patch_version("python_version = {version}")
            )
        ]
    },
    ROOT_DIR
    / ".pre-commit-config.yaml": {
        Tool.Rust: [ReplaceRegex(r"rust: [0-9.]+", "rust: {version}")],
        Tool.Node: [ReplaceRegex(r"node: [0-9.]+", "node: {version}")],
    },
    ROOT_DIR
    / "pyproject.toml": {
        Tool.Python: [
            ReplaceRegex(
                r'"Programming Language :: Python :: .*"',
                hide_patch_version('"Programming Language :: Python :: {version}"'),
            ),
            ReplaceRegex(r'python = "~.*"', 'python = "~{version}"'),
            ReplaceRegex(
                r'build = "cp\d+-{manylinux,macos,win}\*"',
                hide_patch_version('build = "cp{version}-{{manylinux,macos,win}}*"', separator=""),
            ),
            ReplaceRegex(r"py\d+", hide_patch_version("py{version}", separator="")),
        ],
        Tool.Parsec: [ReplaceRegex(r'^version = ".*"$', 'version = "v{version}"')],
    },
    ROOT_DIR
    / "rust-toolchain.toml": {
        Tool.Rust: [ReplaceRegex(r'channel = ".*"', 'channel = "{version}"')]
    },
}


def check_tool_version(filename: Path, raw_regexes: RawRegexes, version: str) -> List[str]:
    try:
        regexes = compile_regexes(raw_regexes, version)
    except re.error:
        raise ValueError(f"Failed to compile regexes for file `{filename}`")
    matched = {regex[0].pattern: False for regex in regexes}
    errors = []

    for line_nu, line in enumerate(filename.read_text().splitlines()):
        for regex, expected_line in regexes:
            res = regex.search(line)
            if res is not None:
                matched[regex.pattern] = True
                matched_part = res.group(0)
                if matched_part != expected_line:
                    effective_line = line_nu + 1
                    effective_col_start = res.start(0)
                    errors.append(
                        f"{filename}:{effective_line}:{effective_col_start}: Wrong version detected, got `{matched_part}` but expected `{expected_line}`"
                    )

    match_errors = does_every_regex_where_used(filename, matched)
    errors.extend(match_errors)
    return errors


def update_tool_version(filename: Path, raw_regexes: RawRegexes, version: str) -> List[str]:
    try:
        regexes = compile_regexes(raw_regexes, version)
    except re.error:
        raise ValueError(f"Failed to compile regexes for file `{filename}`")
    matched = {regex[0].pattern: False for regex in regexes}

    with FileInput(filename, inplace=True) as f:
        for line in f:
            for regex, replaced_line in regexes:
                line, substitution_count = regex.subn(replaced_line, line, count=1)
                if substitution_count >= 1:
                    matched[regex.pattern] = True
            print(line, end="")

    return does_every_regex_where_used(filename, matched)


def compile_regexes(regexes: RawRegexes, version: str) -> List[Tuple[Pattern[str], str]]:
    return [regex.compile(version) for regex in regexes]


def does_every_regex_where_used(filename: Path, have_matched: Dict[str, bool]) -> List[str]:
    errors = []
    for pattern, matched in have_matched.items():
        if not matched:
            errors.append(f"{filename}: Regex `{pattern}` found no matches")

    return errors


def check_tool(tool: Tool, version: str, update: bool) -> Dict[Path, List[str]]:
    errors = {}

    check_or_udpate: Callable[[Path, RawRegexes, str], List[str]] = update_tool_version
    if not update:
        check_or_udpate = check_tool_version

    for filename, tools_in_file in FILES_WITH_VERSION_INFO.items():
        regexes = tools_in_file.get(tool, None)
        if regexes is None:
            continue

        files_errors = check_or_udpate(filename, regexes, version)

        if files_errors:
            errors[filename] = files_errors

    return errors


def handle_errors(errors: Dict[Tool, Dict[Path, List[str]]]) -> None:
    for tool, tool_errors in errors.items():
        for filename, file_errors in tool_errors.items():
            print("\n".join(file_errors), file=sys.stderr)


if __name__ == "__main__":
    parser = ArgumentParser("version_updater")
    parser.add_argument(
        "--tool",
        choices=[e.value for e in Tool],
        help="Check or Update for the specific tool, if no provided it will check for all tools",
    )
    parser.add_argument(
        "--version",
        help="Overwrite the configured version of the tool (will be ignored if no tool is provided)",
        type=str,
        default=None,
    )
    parser.add_argument(
        "--check", action="store_true", help="Check only for the version, no change will be done"
    )
    args = parser.parse_args()
    failed = False

    errors: Dict[Tool, Dict[Path, List[str]]] = {}

    if args.tool is not None:
        tool = Tool(args.tool)
        tool_version: str = args.version or TOOLS_VERSION[tool]

        tool_errors = check_tool(tool, tool_version, update=not args.check)
        for file, file_errors in tool_errors.items():
            errors.setdefault(tool, {}).setdefault(file, []).extend(file_errors)
    else:
        for tool in Tool:
            tool_errors = check_tool(tool, TOOLS_VERSION[tool], update=not args.check)

            for file, file_errors in tool_errors.items():
                errors.setdefault(tool, {}).setdefault(file, []).extend(file_errors)

    if errors:
        handle_errors(errors)
        exit(1)
