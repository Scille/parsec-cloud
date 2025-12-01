# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
"""
Helper that help changing the version of a tool across the repository.
"""

from __future__ import annotations

import enum
import glob
import re
import tomllib
from argparse import ArgumentParser
from collections.abc import Callable
from dataclasses import dataclass
from fileinput import FileInput
from pathlib import Path
from re import Pattern

ReplacementPattern = str | Callable[[str], str]

RawRegexes = list["ReplaceRegex"]

ROOT_DIR = Path(__file__).parent.parent.resolve()
VERSIONS_FILE = (Path(__file__).parent / "versions.toml").resolve()
TOOLS_VERSION: dict[Tool, str] | None = None


@dataclass
class ReplaceRegex:
    regex: str
    replaced: ReplacementPattern

    def compile(self, version: str) -> tuple[Pattern[str], str]:
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
PYTHON_GA_VERSION = ReplaceRegex(
    r"python-version: [0-9.]+", hide_patch_version("python-version: {version}")
)
NODE_GA_VERSION = ReplaceRegex(r"node-version: [0-9.]+", "node-version: {version}")
WASM_PACK_GA_VERSION = ReplaceRegex(r"wasm-pack-version: [0-9.]+", "wasm-pack-version: {version}")
PYTHON_DOCKER_VERSION = ReplaceRegex(r"python:\d.\d+", hide_patch_version("python:{version}"))
PYTHON_SMALL_VERSION = ReplaceRegex(r"python\d.\d+", hide_patch_version("python{version}"))
TOML_VERSION_FIELD = ReplaceRegex(r'version = ".*"', 'version = "{version}"')
JSON_LICENSE_FIELD = ReplaceRegex(r'"license": ".*"', '"license": "{version}"')
JSON_VERSION_FIELD = ReplaceRegex(r'"version": ".*"', '"version": "{version}"')
CI_WINFSP_VERSION = ReplaceRegex(r"WINFSP_VERSION: .*", "WINFSP_VERSION: {version}")
WINFSP_RELEASE_DOWNLOAD = ReplaceRegex(
    r"winfsp/releases/download/v\d+.\d+/",
    hide_patch_version("winfsp/releases/download/v{version}/"),
)
TESTBED_VERSION = ReplaceRegex(
    r"ghcr.io/scille/parsec-cloud/parsec-testbed-server:[^\s]+",
    "ghcr.io/scille/parsec-cloud/parsec-testbed-server:{version}",
)
RUSTUP_INSTALL = ReplaceRegex(
    r"curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh -s -- -y --default-toolchain [0-9.]+",
    "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain {version}",
)
PARSEC_REPO_SRC_BASE_URL = ReplaceRegex(
    r"github.com/Scille/parsec-cloud/tree/v.*?/",
    "github.com/Scille/parsec-cloud/tree/v{version}/",
)


@enum.unique
class Tool(enum.Enum):
    CargoDeny = "cargo-deny"
    CargoUdeps = "cargo-udeps"
    Cross = "cross"
    License = "license"
    Nextest = "nextest"
    Node = "node"
    Parsec = "parsec"
    Poetry = "poetry"
    PostgreSQL = "postgres"
    PreCommit = "pre-commit"
    Python = "python"
    Rust = "rust"
    Testbed = "testbed"
    WasmPack = "wasm-pack"
    WinFSP = "winfsp"

    def post_update_hook(self, updated_files: set[Path]) -> set[Path]:
        updated: set[Path] = set()
        match self:
            case Tool.Parsec:
                updated |= refresh_cargo_lock(updated_files)
                updated |= refresh_npm_package_lock(updated_files)
            case Tool.License:
                updated |= refresh_npm_package_lock(updated_files)
            case _:
                pass
        return updated


def refresh_cargo_lock(updated_files: set[Path]) -> set[Path]:
    a_cargo_file_has_been_updated = any(file.name == "Cargo.toml" for file in updated_files)
    if not a_cargo_file_has_been_updated:
        return set()

    print("Refreshing Cargo.lock file ...")
    cargo_lock = ROOT_DIR / "Cargo.lock"

    lines: list[str] = []
    previous_line = None
    regex, replace = TOML_VERSION_FIELD.compile(get_tool_version(Tool.Parsec))
    for line in cargo_lock.read_text().splitlines():
        if previous_line and (
            previous_line.startswith('name = "libparsec')
            or previous_line.startswith('name = "parsec')
        ):
            match = regex.search(line)
            assert match, f"Expected to find a version field after a name field in {line!r}"
            prefix = line[: match.start()]
            suffix = line[match.end() :]
            lines.append(f"{prefix}{replace}{suffix}")
        else:
            lines.append(line)
        previous_line = line

    content = ("\n".join(lines) + "\n").encode("utf8")
    cargo_lock.write_bytes(content)  # Use write_bytes to keep \n on Windows
    return {cargo_lock}


def refresh_npm_package_lock(update_files: set[Path]) -> set[Path]:
    updated: set[Path] = set()

    for file in update_files:
        if file.name != "package.json":
            continue

        lock_file = file.parent / "package-lock.json"

        print(f"Refreshing npm lock file {lock_file} ...")
        input_lines = lock_file.read_text().splitlines()

        # The project's license field is the first "license" field in the file
        regex, replace = JSON_LICENSE_FIELD.compile(get_tool_version(Tool.License))
        step1_lines: list[str] = []
        found_license_count = 0
        for line in input_lines:
            match = regex.search(line)
            if match and found_license_count < 1:
                found_license_count += 1
                prefix = line[: match.start()]
                suffix = line[match.end() :]
                step1_lines.append(f"{prefix}{replace}{suffix}")
            else:
                step1_lines.append(line)
        assert found_license_count == 1, "No license field found in package.json"

        # The project's version field is present twice at the beginning of the file
        found_version_count = 0
        step2_lines: list[str] = []
        regex, replace = JSON_VERSION_FIELD.compile(get_tool_version(Tool.Parsec))
        for line in step1_lines:
            match = regex.search(line)
            if match and found_version_count < 2:
                found_version_count += 1
                prefix = line[: match.start()]
                suffix = line[match.end() :]
                step2_lines.append(f"{prefix}{replace}{suffix}")
            else:
                step2_lines.append(line)
        assert found_version_count == 2, "Expected two version fields to modify in package.json"

        content = ("\n".join(step2_lines) + "\n").encode("utf8")
        lock_file.write_bytes(content)  # Use write_bytes to keep \n on Windows
        updated.add(lock_file)

    return updated


def get_tools_version() -> dict[Tool, str]:
    global TOOLS_VERSION
    if TOOLS_VERSION is None:
        with open(VERSIONS_FILE, "br") as f:
            data = tomllib.load(f)
        TOOLS_VERSION = {Tool(tool): version for tool, version in data.items()}
    return TOOLS_VERSION


def get_tool_version(tool: Tool) -> str:
    return get_tools_version()[tool]


def set_tool_version(tool: Tool, version: str) -> None:
    tools_version = get_tools_version()
    tools_version[tool] = version


FILES_WITH_VERSION_INFO: dict[Path, dict[Tool, RawRegexes]] = {
    ROOT_DIR / ".github/actions/use-pre-commit/action.yml": {
        Tool.PreCommit: [
            ReplaceRegex(r"default: .* # __VERSION__", "default: {version} # __VERSION__")
        ]
    },
    ROOT_DIR / ".github/workflows/_releaser_nightly_build.yml": {Tool.Python: [PYTHON_GA_VERSION]},
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
        Tool.CargoDeny: [ReplaceRegex(r"cargo-deny@[0-9.]+", "cargo-deny@{version}")],
        Tool.CargoUdeps: [ReplaceRegex(r"cargo-udeps@[0-9.]+", "cargo-udeps@{version}")],
        Tool.Nextest: [ReplaceRegex(r"nextest@[0-9.]+", "nextest@{version}")],
        Tool.Testbed: [TESTBED_VERSION],
        Tool.WasmPack: [ReplaceRegex(r"wasm-pack@[0-9.]+", "wasm-pack@{version}")],
        Tool.WinFSP: [CI_WINFSP_VERSION, WINFSP_RELEASE_DOWNLOAD],
    },
    ROOT_DIR / ".github/workflows/ci-web.yml": {
        Tool.Node: [NODE_GA_VERSION],
        Tool.Testbed: [TESTBED_VERSION],
    },
    ROOT_DIR / ".github/workflows/ci.yml": {
        Tool.Python: [PYTHON_GA_VERSION],
    },
    ROOT_DIR / ".github/workflows/_parse_version.yml": {
        Tool.Python: [PYTHON_GA_VERSION],
    },
    ROOT_DIR / ".github/workflows/codeql.yml": {
        Tool.Poetry: [POETRY_GA_VERSION],
    },
    ROOT_DIR / ".github/workflows/docker-server.yml": {
        Tool.Python: [PYTHON_GA_VERSION],
    },
    ROOT_DIR / ".github/workflows/docker-testbed.yml": {
        Tool.Python: [PYTHON_GA_VERSION],
    },
    ROOT_DIR / ".github/workflows/package-server.yml": {
        Tool.Poetry: [POETRY_GA_VERSION],
        Tool.Node: [NODE_GA_VERSION],
    },
    ROOT_DIR / ".github/workflows/package-cli.yml": {
        Tool.Cross: [ReplaceRegex(r"cross-version: .*", "cross-version: {version}")],
        Tool.WinFSP: [CI_WINFSP_VERSION, WINFSP_RELEASE_DOWNLOAD],
    },
    ROOT_DIR / ".github/workflows/package-desktop.yml": {
        Tool.Node: [NODE_GA_VERSION],
        Tool.WinFSP: [CI_WINFSP_VERSION, WINFSP_RELEASE_DOWNLOAD],
    },
    ROOT_DIR / ".github/workflows/package-webapp.yml": {
        Tool.Node: [NODE_GA_VERSION],
        Tool.WasmPack: [WASM_PACK_GA_VERSION],
    },
    ROOT_DIR / "bindings/electron/package.json": {
        Tool.License: [JSON_LICENSE_FIELD],
        Tool.Parsec: [JSON_VERSION_FIELD],
    },
    ROOT_DIR / "bindings/web/package.json": {
        Tool.License: [JSON_LICENSE_FIELD],
        Tool.Parsec: [JSON_VERSION_FIELD],
    },
    ROOT_DIR / "cli/tests/integration/version.rs": {
        Tool.Parsec: [ReplaceRegex(r'"parsec-cli .*", ', '"parsec-cli {version}", ')]
    },
    ROOT_DIR / "client/electron/assets/installer.nsh": {
        Tool.WinFSP: [ReplaceRegex(r'WINFSP_VERSION ".*"', 'WINFSP_VERSION "{version}"')],
    },
    ROOT_DIR / "client/electron/scripts/before-pack.js": {
        Tool.WinFSP: [
            ReplaceRegex(r"WINFSP_VERSION = '.*'", "WINFSP_VERSION = '{version}'"),
            ReplaceRegex(
                r"WINFSP_RELEASE_BRANCH = 'v.*'",
                hide_patch_version("WINFSP_RELEASE_BRANCH = 'v{version}'"),
            ),
        ]
    },
    ROOT_DIR / "client/electron/snap/snapcraft.yaml": {
        Tool.Parsec: [ReplaceRegex(r"version: .*", "version: {version}")],
        Tool.Rust: [RUSTUP_INSTALL],
    },
    ROOT_DIR / "client/electron/assets/electron-publisher-custom.js": {
        Tool.Parsec: [ReplaceRegex(r"VERSION = '.*';", "VERSION = '{version}';")]
    },
    ROOT_DIR / "client/electron/package.js": {
        Tool.Parsec: [ReplaceRegex(r"buildVersion: '.*',", "buildVersion: '{version}',")],
    },
    ROOT_DIR / "client/electron/package.json": {
        Tool.License: [JSON_LICENSE_FIELD],
        Tool.Parsec: [JSON_VERSION_FIELD],
    },
    ROOT_DIR / "client/electron/scripts/sign-windows-package.ps1": {
        Tool.Node: [
            ReplaceRegex(
                r'expected_node_version = "v.*"',
                'expected_node_version = "v{version}"',
            )
        ],
    },
    ROOT_DIR / "client/package.json": {
        Tool.License: [JSON_LICENSE_FIELD],
        Tool.Parsec: [JSON_VERSION_FIELD],
    },
    ROOT_DIR / "docs/development/README.md": {
        Tool.Rust: [
            ReplaceRegex(r"Rust v[0-9.]+", "Rust v{version}"),
            RUSTUP_INSTALL,
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
    ROOT_DIR / "docs/hosting/deployment/index.rst": {
        Tool.PostgreSQL: [ReplaceRegex(r"postgres-\d+", only_major_version("postgres-{version}"))],
        Tool.Python: [
            ReplaceRegex(r"- Python v[0-9.]+", hide_patch_version("- Python v{version}")),
        ],
        Tool.Parsec: [
            ReplaceRegex(
                r"parsec-cloud==.+'",
                "parsec-cloud=={version}'",
            ),
        ],
    },
    ROOT_DIR / "docs/hosting/deployment/parsec-server.docker.yaml": {
        Tool.Parsec: [
            ReplaceRegex(
                r"ghcr.io/scille/parsec-cloud/parsec-server:.+$",
                "ghcr.io/scille/parsec-cloud/parsec-server:{version}",
            )
        ],
        Tool.PostgreSQL: [
            ReplaceRegex(
                r"image: postgres:\d+.\d+-alpine",
                hide_patch_version("image: postgres:{version}-alpine"),
            )
        ],
    },
    ROOT_DIR / "docs/hosting/custom_branding.rst": {Tool.Parsec: [PARSEC_REPO_SRC_BASE_URL]},
    ROOT_DIR / "docs/hosting/install_cli.rst": {
        Tool.Parsec: [
            ReplaceRegex(
                r".. _Parsec CLI v.*: https://github.com/Scille/parsec-cloud/releases/download/v.*/parsec-cli_.*_linux-x86_64-musl",
                ".. _Parsec CLI v{version}: https://github.com/Scille/parsec-cloud/releases/download/v{version}/parsec-cli_{version}_linux-x86_64-musl",
            ),
            ReplaceRegex(
                r"`Parsec CLI v.*`_",
                "`Parsec CLI v{version}`_",
            ),
            ReplaceRegex(
                r"parsec-cli_.*_linux-x86_64-musl",
                "parsec-cli_{version}_linux-x86_64-musl",
            ),
            ReplaceRegex(r"parsec-cli .*", "parsec-cli {version}"),
        ]
    },
    ROOT_DIR / "docs/locale/fr/LC_MESSAGES/hosting/install_cli.po": {
        Tool.Parsec: [
            ReplaceRegex(
                r"`Parsec CLI v.*`_",
                "`Parsec CLI v{version}`_",
            ),
        ]
    },
    ROOT_DIR / "docs/conf.py": {
        Tool.Parsec: [ReplaceRegex(r'version = ".*"', 'version = "{version}"')]
    },
    ROOT_DIR / "docs/pyproject.toml": {
        Tool.Python: [
            ReplaceRegex(r'^requires-python = "~=.*"', 'requires-python = "~={version}"')
        ],
        Tool.License: [ReplaceRegex(r'^license = *".*"', 'license = "{version}"')],
    },
    ROOT_DIR / "libparsec/version": {Tool.Parsec: [ReplaceRegex(r"^.*$", "{version}")]},
    ROOT_DIR / "LICENSE": {
        Tool.Parsec: [
            ReplaceRegex(r"^Licensed Work:  Parsec v.*$", "Licensed Work:  Parsec v{version}")
        ]
    },
    ROOT_DIR / "misc/versions.toml": {
        tool: [ReplaceRegex(rf'^{tool.value} = ".*"', f'{tool.value} = "{{version}}"')]
        for tool in Tool
    },
    ROOT_DIR / "misc/setup-rust.sh": {
        Tool.Rust: [RUSTUP_INSTALL],
    },
    ROOT_DIR / "server/packaging/server/in-docker-build.sh": {
        Tool.Rust: [RUSTUP_INSTALL],
        Tool.Poetry: [
            ReplaceRegex(
                r"curl -sSL https://install.python-poetry.org \| python - --version=[0-9.]+",
                "curl -sSL https://install.python-poetry.org | python - --version={version}",
            )
        ],
    },
    ROOT_DIR / "server/packaging/server/server.dockerfile": {Tool.Python: [PYTHON_DOCKER_VERSION]},
    ROOT_DIR / "server/packaging/testbed-server/in-docker-build.sh": {
        Tool.Rust: [RUSTUP_INSTALL],
        Tool.Python: [PYTHON_SMALL_VERSION],
        Tool.Poetry: [
            ReplaceRegex(
                r"curl -sSL https://install.python-poetry.org \| python - --version=[0-9.]+",
                "curl -sSL https://install.python-poetry.org | python - --version={version}",
            )
        ],
    },
    ROOT_DIR / "server/packaging/testbed-server/README.md": {
        Tool.Testbed: [TESTBED_VERSION],
    },
    ROOT_DIR / "server/packaging/testbed-server/testbed-server.dockerfile": {
        Tool.Python: [PYTHON_DOCKER_VERSION]
    },
    ROOT_DIR / "server/parsec/_version.py": {
        Tool.Parsec: [ReplaceRegex(r'^__version__ = ".*"$', '__version__ = "{version}"')]
    },
    ROOT_DIR / "server/pyproject.toml": {
        Tool.Python: [
            ReplaceRegex(
                r'"Programming Language :: Python :: .*"',
                hide_patch_version('"Programming Language :: Python :: {version}"'),
            ),
            ReplaceRegex(r'requires-python = "~=.*"', 'requires-python = "~={version}"'),
            ReplaceRegex(
                r'build = "cp\d+-{manylinux,macos,win}\*"',
                hide_patch_version('build = "cp{version}-{{manylinux,macos,win}}*"', separator=""),
            ),
        ],
        Tool.Parsec: [ReplaceRegex(r'^version = ".*"$', 'version = "{version}"')],
        Tool.License: [ReplaceRegex(r'^license = *".*"', 'license = "{version}"')],
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
        Tool.License: [ReplaceRegex(r'license = ".*"', 'license = "{version}"')],
        Tool.Parsec: [
            ReplaceRegex(
                r'^version = ".*" # __PARSEC_VERSION__$',
                'version = "{version}" # __PARSEC_VERSION__',
            )
        ],
    },
}


@dataclass
class VersionUpdateResult:
    errors: list[str]
    updated: set[Path]


def check_tool_version(
    filename: Path, raw_regexes: RawRegexes, version: str
) -> VersionUpdateResult:
    try:
        regexes = compile_regexes(raw_regexes, version)
    except re.error:
        raise ValueError(f"Failed to compile regexes for file `{filename}`")
    matched = {regex[0].pattern: False for regex in regexes}
    errors: list[str] = []

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
    return VersionUpdateResult(
        errors,
        set(),  # No file are updated in check mode
    )


def update_tool_version(
    filename: Path, raw_regexes: RawRegexes, version: str
) -> VersionUpdateResult:
    try:
        regexes = compile_regexes(raw_regexes, version)
    except re.error:
        raise ValueError(f"Failed to compile regexes for file `{filename}`")
    matched = {regex[0].pattern: False for regex in regexes}
    updated: set[Path] = set()

    with FileInput(filename, inplace=True) as f:
        for old_line in f:
            line = old_line
            for regex, replaced_line in regexes:
                line, substitution_count = regex.subn(replaced_line, line, count=1)
                if substitution_count >= 1:
                    matched[regex.pattern] = True
                    if line != old_line:
                        updated.add(filename)
            print(line, end="")

    return VersionUpdateResult(does_every_regex_where_used(filename, matched), updated)


def compile_regexes(regexes: RawRegexes, version: str) -> list[tuple[Pattern[str], str]]:
    return [regex.compile(version) for regex in regexes]


def does_every_regex_where_used(filename: Path, have_matched: dict[str, bool]) -> list[str]:
    errors: list[str] = []
    for pattern, matched in have_matched.items():
        if not matched:
            errors.append(f"{filename}: Regex `{pattern}` found no matches")

    return errors


def check_tool(tool: Tool, update: bool) -> VersionUpdateResult:
    version = get_tool_version(tool)
    update_res = VersionUpdateResult([], set())

    for glob_path, tools_in_file in FILES_WITH_VERSION_INFO.items():
        regexes = tools_in_file.get(tool, None)
        if regexes is None:
            continue

        matched_files = glob.glob(str(glob_path), recursive=True)
        if not matched_files:
            update_res.errors.append(f"No match with regex `{glob_path}`")
            continue

        for file in map(Path, matched_files):
            if update:
                res = update_tool_version(file, regexes, version)
                update_res.errors += res.errors
                update_res.updated.update(res.updated)
            else:
                update_res.errors += check_tool_version(file, regexes, version).errors

    if not update_res.errors and update_res.updated:
        update_res.updated |= tool.post_update_hook(update_res.updated)

    return update_res


if __name__ == "__main__":
    parser = ArgumentParser("version_updater")
    parser.add_argument(
        "--tool",
        choices=[e.value for e in Tool],
        help="Check or Update for the specific tool, if no provided it will check for all tools",
    )
    parser.add_argument(
        "--version",
        help="Overwrite the configured version of the tool (must be used with --tool)",
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

    errors: list[str] = []

    if args.tool is not None:
        tool = Tool(args.tool)
        if args.version is not None:
            set_tool_version(tool, args.version)

        errors += check_tool(tool, update=not args.check).errors
    else:
        if args.version is not None:
            raise SystemExit("Parameter --version requires --tool to be set")
        for tool in Tool:
            errors += check_tool(tool, update=not args.check).errors

    if errors:
        raise SystemExit("\n".join(errors))
