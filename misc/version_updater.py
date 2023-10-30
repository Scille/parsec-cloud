# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

"""
Helper that help changing the version of a tool across the repository.
"""

import enum
import glob
import re
import subprocess
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
        return template.format(version=separator.join(version.split(".")[:2]))

    return _hide_patch_version


def only_major_version(template: str, separator: str = ".") -> Callable[[str], str]:
    def _only_major_version(version: str) -> str:
        return template.format(version=version.split(separator)[0])

    return _only_major_version


POETRY_GA_VERSION = ReplaceRegex(r"poetry-version: [0-9.]+", "poetry-version: {version}")
NODE_GA_VERSION = ReplaceRegex(r"node-version: [0-9.]+", "node-version: {version}")
WASM_PACK_GA_VERSION = ReplaceRegex(r"wasm-pack-version: [0-9.]+", "wasm-pack-version: {version}")
PYTHON_DOCKER_VERSION = ReplaceRegex(r"python:\d.\d+", hide_patch_version("python:{version}"))
PYTHON_SMALL_VERSION = ReplaceRegex(r"python\d.\d+", hide_patch_version("python{version}"))
TOML_LICENSE_FIELD = ReplaceRegex(r'license = ".*"', 'license = "{version}"')
JSON_LICENSE_FIELD = ReplaceRegex(r'"license": ".*",', '"license": "{version}",')


@enum.unique
class Tool(enum.Enum):
    Rust = "rust"
    Python = "python"
    Poetry = "poetry"
    Node = "node"
    WasmPack = "wasm-pack"
    Nextest = "nextest"
    Parsec = "parsec"
    License = "license"
    PostgreSQL = "postgres"

    def post_update_hook(self) -> None:
        match self:
            case Tool.Parsec:
                refresh_cargo_lock()
            case _:
                pass


def refresh_cargo_lock() -> None:
    print("Listing installed rust toolchains ...")
    rust_installed_toolchain = subprocess.check_output(["rustup", "toolchain", "list"]).decode()

    if "nightly" not in rust_installed_toolchain:
        print("Missing nightly toolchain, installing it ...")
        subprocess.check_call(["rustup", "toolchain", "install", "nightly"])

    print("Refreshing Cargo.lock ...")
    subprocess.check_call(
        [
            "cargo",
            "+nightly",
            "generate-lockfile",
            "-Z",
            "direct-minimal-versions",
        ]
    )


TOOLS_VERSION: Dict[Tool, str] = {
    Tool.Rust: "1.75.0",
    Tool.Python: "3.12.0",
    Tool.Poetry: "1.5.1",
    Tool.Node: "18.12.0",
    Tool.WasmPack: "0.11.0",
    Tool.Parsec: "2.16.0-a.0+dev",
    Tool.Nextest: "0.9.54",
    Tool.License: "BUSL-1.1",
    Tool.PostgreSQL: "14.10",
}


FILES_WITH_VERSION_INFO: Dict[Path, Dict[Tool, RawRegexes]] = {
    ROOT_DIR / ".github/workflows/ci-docs.yml": {
        Tool.Poetry: [POETRY_GA_VERSION],
    },
    ROOT_DIR / ".github/workflows/ci-python.yml": {
        Tool.Poetry: [POETRY_GA_VERSION],
        Tool.PostgreSQL: [
            ReplaceRegex(
                r"postgresql-version: \d+",
                only_major_version("postgresql-version: {version}"),
            )
        ],
    },
    ROOT_DIR / ".github/workflows/ci-rust.yml": {
        Tool.WasmPack: [ReplaceRegex(r"wasm-pack@[0-9.]+", "wasm-pack@{version}")],
        Tool.Nextest: [ReplaceRegex(r"nextest@[0-9.]+", "nextest@{version}")],
    },
    ROOT_DIR / ".github/workflows/ci-web.yml": {
        Tool.Node: [NODE_GA_VERSION],
    },
    ROOT_DIR / ".github/workflows/ci.yml": {
        Tool.Python: [
            ReplaceRegex(
                r"python-version: [0-9.]+", hide_patch_version("python-version: {version}")
            )
        ],
    },
    ROOT_DIR / ".github/workflows/codeql.yml": {
        Tool.Poetry: [POETRY_GA_VERSION],
    },
    ROOT_DIR / ".github/workflows/package-server.yml": {
        Tool.Poetry: [POETRY_GA_VERSION],
        Tool.Node: [NODE_GA_VERSION],
    },
    ROOT_DIR / ".github/workflows/package-ionic-app.yml": {
        Tool.Node: [NODE_GA_VERSION],
        Tool.WasmPack: [WASM_PACK_GA_VERSION],
    },
    ROOT_DIR / "client/package.json": {Tool.License: [JSON_LICENSE_FIELD]},
    ROOT_DIR / "client/electron/package.json": {Tool.License: [JSON_LICENSE_FIELD]},
    ROOT_DIR / "bindings/electron/package.json": {Tool.License: [JSON_LICENSE_FIELD]},
    ROOT_DIR / "bindings/web/package.json": {Tool.License: [JSON_LICENSE_FIELD]},
    ROOT_DIR / "docs/development/quickstart.md": {
        Tool.Rust: [
            ReplaceRegex(r"Rust v[0-9.]+", "Rust v{version}"),
            ReplaceRegex(
                r"--default-toolchain none # You can replace `none` with `[0-9.]+`",
                "--default-toolchain none # You can replace `none` with `{version}`",
            ),
        ],
        Tool.Python: [
            ReplaceRegex(r"python v[0-9.]+", hide_patch_version("python v{version}")),
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
        Tool.Node: [
            ReplaceRegex(r"Node [0-9.]+", "Node {version}"),
            ReplaceRegex(r"nvm install [0-9.]+", "nvm install {version}"),
        ],
        Tool.WasmPack: [ReplaceRegex(r"wasm-pack@[0-9.]+", "wasm-pack@{version}")],
    },
    ROOT_DIR / "docs/adminguide/hosting/index.rst": {
        Tool.PostgreSQL: [ReplaceRegex(r"postgres-\d+", only_major_version("postgres-{version}"))],
        Tool.Python: [
            ReplaceRegex(r"- Python v[0-9.]+", hide_patch_version("- Python v{version}")),
        ],
        Tool.Parsec: [
            ReplaceRegex(
                r"parsec-cloud\[backend\]==.+'",
                "parsec-cloud[backend]=={version}'",
            ),
        ],
    },
    ROOT_DIR / "docs/adminguide/hosting/parsec-server.docker.yaml": {
        Tool.PostgreSQL: [
            ReplaceRegex(
                r"image: postgres:\d+.\d+-alpine",
                hide_patch_version("image: postgres:{version}-alpine"),
            )
        ]
    },
    ROOT_DIR / "docs/conf.py": {
        Tool.Parsec: [ReplaceRegex(r'version = ".*"', 'version = "{version}"')]
    },
    ROOT_DIR / "docs/pyproject.toml": {
        Tool.Python: [ReplaceRegex(r'^python = "\^[0-9.]+"$', 'python = "^{version}"')]
    },
    ROOT_DIR / "libparsec/version": {Tool.Parsec: [ReplaceRegex(r"^.*$", "{version}")]},
    ROOT_DIR / "licenses/BUSL-Scille.txt": {
        Tool.Parsec: [
            ReplaceRegex(r"^Licensed Work:  Parsec v.*$", "Licensed Work:  Parsec v{version}")
        ]
    },
    ROOT_DIR / "misc/version_updater.py": {
        Tool.Rust: [ReplaceRegex(r'Tool.Rust: "[0-9.]+"', 'Tool.Rust: "{version}"')],
        Tool.Python: [ReplaceRegex(r'Tool.Python: "[0-9.]+"', 'Tool.Python: "{version}"')],
        Tool.Poetry: [ReplaceRegex(r'Tool.Poetry: "[0-9.]+"', 'Tool.Poetry: "{version}"')],
        Tool.Node: [ReplaceRegex(r'Tool.Node: "[0-9.]+"', 'Tool.Node: "{version}"')],
        Tool.WasmPack: [ReplaceRegex(r'Tool.WasmPack: "[0-9.]+"', 'Tool.WasmPack: "{version}"')],
        Tool.Nextest: [ReplaceRegex(r'Tool.Nextest: "[0-9.]+"', 'Tool.Nextest: "{version}"')],
        Tool.Parsec: [ReplaceRegex(r'Tool.Parsec: "[0-9.]+.*",', 'Tool.Parsec: "{version}",')],
        Tool.License: [
            ReplaceRegex(r'^    Tool.License: "[^\"]*",', '    Tool.License: "{version}",')
        ],
    },
    ROOT_DIR / "server/packaging/server/in-docker-build.sh": {
        Tool.Poetry: [
            ReplaceRegex(
                r"curl -sSL https://install.python-poetry.org \| python - --version=[0-9.]+",
                "curl -sSL https://install.python-poetry.org | python - --version={version}",
            )
        ],
    },
    ROOT_DIR / "server/packaging/server/server.dockerfile": {Tool.Python: [PYTHON_DOCKER_VERSION]},
    ROOT_DIR / "server/packaging/testbed-server/in-docker-build.sh": {
        Tool.Python: [PYTHON_SMALL_VERSION],
        Tool.Poetry: [
            ReplaceRegex(
                r"curl -sSL https://install.python-poetry.org \| python - --version=[0-9.]+",
                "curl -sSL https://install.python-poetry.org | python - --version={version}",
            )
        ],
    },
    ROOT_DIR / "server/packaging/testbed-server/testbed-server.dockerfile": {
        Tool.Python: [PYTHON_DOCKER_VERSION]
    },
    ROOT_DIR / "server/parsec/_version.py": {
        Tool.Parsec: [ReplaceRegex(r'^__version__ = ".*"$', '__version__ = "v{version}"')]
    },
    ROOT_DIR / "server/pyproject.toml": {
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
            ReplaceRegex(
                r"python_version = \"\d.\d+\"",
                hide_patch_version('python_version = "{version}"'),
            ),
        ],
        Tool.Parsec: [ReplaceRegex(r'^version = ".*"$', 'version = "v{version}"')],
        Tool.License: [TOML_LICENSE_FIELD],
    },
    ROOT_DIR / ".pre-commit-config.yaml": {
        Tool.Rust: [ReplaceRegex(r"rust: [0-9.]+", "rust: {version}")],
        Tool.Node: [ReplaceRegex(r"node: [0-9.]+", "node: {version}")],
    },
    ROOT_DIR / "rust-toolchain.toml": {
        Tool.Rust: [ReplaceRegex(r'channel = ".*"', 'channel = "{version}"')]
    },
    # Cargo workspace members should use the license value defined in the root cargo manifest
    ROOT_DIR / "*/**/Cargo.toml": {
        Tool.License: [ReplaceRegex(r"license.workspace = true", "license.workspace = true")]
    },
    ROOT_DIR / "Cargo.toml": {
        Tool.License: [TOML_LICENSE_FIELD],
        Tool.Parsec: [ReplaceRegex(r'^version = ".*"$', 'version = "{version}"')],
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
                    effective_col_start = res.start(0) + 1
                    errors.append(
                        f"{filename}:{effective_line}:{effective_col_start}: Wrong version detected, got `{matched_part}` but expected `{expected_line}`"
                    )

    errors += does_every_regex_where_used(filename, matched)
    return errors


def update_tool_version(filename: Path, raw_regexes: RawRegexes, version: str) -> List[str]:
    root_cargo = filename.name == "Cargo.toml"
    try:
        regexes = compile_regexes(raw_regexes, version)
    except re.error:
        raise ValueError(f"Failed to compile regexes for file `{filename}`")
    matched = {regex[0].pattern: False for regex in regexes}

    if root_cargo:
        print(f"Update {filename}")
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


def check_tool(tool: Tool, version: str, update: bool) -> List[str]:
    errors = []
    for filename, tools_in_file in FILES_WITH_VERSION_INFO.items():
        regexes = tools_in_file.get(tool, None)
        if regexes is None:
            continue

        for glob_file in glob.glob(str(filename), recursive=True):
            file = Path(glob_file)
            if update:
                errors += update_tool_version(file, regexes, version)
            else:
                errors += check_tool_version(file, regexes, version)

    if not errors and update:
        tool.post_update_hook()
    return errors


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
        "--check",
        action="store_true",
        help="Check only for the version, no change will be done",
    )
    args = parser.parse_args()
    failed = False

    errors: List[str] = []

    if args.tool is not None:
        tool = Tool(args.tool)
        tool_version: str = args.version or TOOLS_VERSION[tool]

        errors += check_tool(tool, tool_version, update=not args.check)
    else:
        for tool in Tool:
            errors += check_tool(tool, TOOLS_VERSION[tool], update=not args.check)

    if errors:
        raise SystemExit("\n".join(errors))
