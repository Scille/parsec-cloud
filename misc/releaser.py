#!/usr/bin/env python3
# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS


"""
This script is used to ease the parsec release process.

The produced tag is going to be signed so make sure you have a GPG key available.
If not, it can be generated using the following command:

    $ gpg --gen-key

A typical release process looks as follow:

    # Switch to master and make sure all the latest commits are pulled
    $ git checkout master
    $ git pull master

    # Run the releaser script with the expected version
    $ ./misc/releaser.py build v1.2.3

    # Push the produced tag and commits
    $ git push --follow-tags

    # Alternally, you can revert the commits and delete the tag in case you made
    # an error (of course don't do that if you have already push the changes !)
    $ ./misc/releaser.py rollback

"""

from __future__ import annotations

import argparse
import math
import re
import subprocess
import sys
import textwrap
from collections import defaultdict
from copy import copy
from datetime import date, datetime
from pathlib import Path
from typing import Any

import version_updater

PYTHON_EXECUTABLE_PATH = sys.executable
LICENSE_CONVERSION_DELAY = 4 * 365 * 24 * 3600  # 4 years
PROJECT_DIR = Path(__file__).resolve().parent.parent
HISTORY_FILE = PROJECT_DIR / "HISTORY.rst"
BUSL_LICENSE_FILE = PROJECT_DIR / "licenses/BUSL-Scille.txt"
VERSION_UPDATER = PROJECT_DIR / "misc/version_updater.py"
VERSION_FILE = PROJECT_DIR / "parsec/_version.py"
PYPROJECT_FILE = PROJECT_DIR / "pyproject.toml"
FILES_TO_COMMIT_ON_VERSION_CHANGE = [
    BUSL_LICENSE_FILE.absolute(),
    VERSION_FILE.absolute(),
    PYPROJECT_FILE.absolute(),
    VERSION_UPDATER.absolute(),
]

FRAGMENTS_DIR = PROJECT_DIR / "newsfragments"
FRAGMENT_TYPES = {
    "feature": "Features",
    "bugfix": "Bugfixes",
    "doc": "Improved Documentation",
    "removal": "Deprecations and Removals",
    "api": "Client/Backend API evolutions",
    "misc": "Miscellaneous internal changes",
    "empty": "Miscellaneous internal changes that shouldn't even be collected",
}
COLOR_END = "\033[0m"
COLOR_RED = "\033[91m"
COLOR_GREEN = "\033[92m"
COLOR_YELLOW = "\033[93m"
DESCRIPTION = (
    f"""TL,DR:
Create release commit&tag: {COLOR_GREEN}./misc/releaser.py build v1.2.3{COLOR_END}
Oops I've made a mistake: {COLOR_GREEN}./misc/releaser.py rollback{COLOR_END}
    """
    + __doc__
)

PRERELEASE_EXPR = r"(?P<pre_l>(a|b|rc))(?P<pre_n>[0-9]+)"

# Inspired by https://peps.python.org/pep-0440/#appendix-b-parsing-version-strings-with-regular-expressions
RELEASE_REGEX = re.compile(
    r"^"
    r"v?"
    r"(?P<release>(?P<major>[0-9]+)\.(?P<minor>[0-9]+)\.(?P<patch>[0-9]+))"
    rf"(?:-(?P<pre>{PRERELEASE_EXPR}))?"
    r"(?:-dev\.(?P<dev>[0-9]+))?"
    r"(?:\+(?P<local>[a-z0-9]+(?:[-_\.][a-z0-9]+)*))?"
    r"$"
)


class ReleaseError(Exception):
    pass


class Version:
    def __init__(
        self,
        major: int,
        minor: int,
        patch: int,
        prerelease: str | None = None,
        dev: int | None = None,
        local: str | None = None,
    ) -> None:
        self.major = major
        self.minor = minor
        self.patch = patch
        self.pre: None | tuple[str, int] = None
        self.dev = dev
        self.local: str | None = local

        if prerelease is not None:
            match = re.match(PRERELEASE_EXPR, prerelease)
            assert match is not None
            self.pre = (match.group("pre_l"), int(match.group("pre_n")))

    @classmethod
    def parse(cls, raw: str, git: bool = False) -> Version:
        raw = raw.strip()
        if raw.startswith("v"):
            raw = raw[1:]
        if git:
            # Git describe show our position relative to an existing release.
            # `git describe` could return for example `1.2.3-15-g3b5f5762`, here:
            #
            # - `1.2.3` is the `major.minor.patch`.
            # - `-15` indicate that we have `15` commit since the tag `1.2.3`.
            # - `-g3b5f5762` indicate that we are currently on the commit `3b5f5762`.
            #
            # A semver compatible version could be `1.2.3-dev.15+git.3b5f5762`
            #
            # Convert `1.2.3-N-gSHA` to `1.2.3-dev.N+git.SHA`
            raw, _ = re.subn(
                pattern=r"-(\d+)-g([0-9A-Za-z]+)", repl=r"-dev.\1+git.\2", string=raw, count=1
            )
        match = RELEASE_REGEX.match(raw)
        if not match:
            raise ValueError(
                f"Invalid version format {raw!r}, should be `[v]<major>.<minor>.<patch>[-<post>][+dev|-<X>-g<commit>]` (e.g. `v1.0.0`, `1.2.3+dev`, `1.6.7-rc1`)"
            )

        major = int(match.group("major"))
        minor = int(match.group("minor"))
        patch = int(match.group("patch"))
        prerelease = match.group("pre")
        dev = int(match.group("dev")) if match.group("dev") is not None else None
        local = match.group("local")

        return Version(major, minor, patch, prerelease=prerelease, dev=dev, local=local)

    def evolve(self, **kwargs: Any) -> Version:
        new = copy(self)
        new.__dict__.update(kwargs)
        return new

    def with_v_prefix(self) -> str:
        return "v" + str(self)

    def __repr__(self) -> str:
        return f"Version(major={self.major}, minor={self.minor}, patch={self.patch}, prerelease={self.prerelease}, dev={self.dev}, local={self.local})"

    def __str__(self) -> str:
        base = f"{self.major}.{self.minor}.{self.patch}"
        if self.pre is not None:
            type, index = self.pre
            base += f"-{type}{index}"
        if self.dev is not None:
            base += f"-dev.{self.dev}"
        if self.local is not None:
            base += "+" + self.local
        return base

    @property
    def is_preversion(self) -> bool:
        return self.pre is not None

    @property
    def prerelease(self) -> str | None:
        if self.pre is not None:
            return f"{self.pre[0]}{self.pre[1]}"
        else:
            return None

    @property
    def is_dev(self) -> bool:
        return self.dev is not None

    def __eq__(self, other: object) -> bool:
        if isinstance(other, Version):
            string_other = str(other)
        elif isinstance(other, tuple):
            string_other = (
                "v"
                + ".".join([str(i) for i in other[:3]])
                + (f"-{str().join(other[3:])}" if len(other) > 3 else "")
            )
        else:
            raise TypeError(f"Unsupported type `{type(other).__name__}`")

        return str(self) == string_other

    def __lt__(self, other: Version) -> bool:
        k = (self.major, self.minor, self.patch)
        other_k = (other.major, other.minor, other.patch)

        def _pre_type_to_val(pre_type: str) -> int:
            return {"a": 1, "b": 2, "rc": 3}[pre_type]

        if k == other_k:
            # Must take into account dev and pre info
            if self.pre is not None:
                type, index = self.pre
                pre_type: int | float = _pre_type_to_val(type)
                pre_index: int | float = index
            else:
                pre_type = pre_index = math.inf

            if other.pre is not None:
                type, index = other.pre
                other_pre_type: int | float = _pre_type_to_val(type)
                other_pre_index: int | float = index
            else:
                other_pre_type = other_pre_index = math.inf

            dev = self.dev or 0
            other_dev = other.dev or 0

            local = self.local is not None
            other_local = other.local is not None

            return (pre_type, pre_index, dev, local) < (
                other_pre_type,
                other_pre_index,
                other_dev,
                other_local,
            )

        else:
            return k < other_k

    def __le__(self, other: Version) -> bool:
        if self == other:
            return True
        else:
            return self < other


# Version is non-trivial code, so let's run some pseudo unit tests...
def _test_parse_version_string(raw: str, expected: Version, git: bool = False) -> None:
    v = Version.parse(raw, git)
    assert v == expected, f"From raw version `{raw}`, we parsed `{v}` but we expected `{expected}`"


def _test_format_version(version: Version, expected: str) -> None:
    s = str(version)
    assert (
        s == expected
    ), f"Version {version!r} is not well formatted: got `{s}` but expected `{expected}`"


_test_parse_version_string("1.2.3", Version(1, 2, 3))
_test_parse_version_string("1.2.3+dev", Version(1, 2, 3, local="dev"))
_test_parse_version_string(
    "1.2.3-10-g3b5f5762", Version(1, 2, 3, dev=10, local="git.3b5f5762"), git=True
)
_test_parse_version_string(
    "v2.12.1-2160-g1c38d13f8", Version(2, 12, 1, dev=2160, local="git.1c38d13f8"), git=True
)
_test_parse_version_string("1.2.3-b42", Version(1, 2, 3, prerelease="b42"))
_test_parse_version_string("1.2.3-rc1+dev", Version(1, 2, 3, prerelease="rc1", local="dev"))
_test_parse_version_string("v2.15.0+dev.2950f88", Version(2, 15, 0, local="dev.2950f88"))
assert Version.parse("1.2.3") < Version.parse("1.2.4")
assert Version.parse("1.2.3-a1") < Version.parse("1.2.3-a2")
assert Version.parse("1.2.3-a10") < Version.parse("1.2.3-b1")
assert Version.parse("1.2.3-b10") < Version.parse("1.2.3-rc1")
assert Version.parse("1.2.3-b1") < Version.parse("1.2.3-b1+dev")
assert Version.parse("1.2.3-rc1") < Version.parse("1.2.3")
assert Version.parse("1.2.3") < Version.parse("1.2.3+dev")
assert Version.parse("1.2.3+dev") < Version.parse("1.2.4-rc1")
assert Version.parse("1.2.4-rc1") < Version.parse("1.2.4-rc1+dev")
assert Version.parse("1.2.4-rc1+dev") < Version.parse("1.2.4")
assert Version.parse("1.2.3") < Version.parse("1.2.3-10-g3b5f5762", git=True)
assert Version.parse("1.2.3-10-g3b5f5762", git=True) < Version.parse("1.2.4-b42")
assert Version.parse("1.2.4-b42") < Version.parse("1.2.4-b42-10-g3b5f5762", git=True)
assert Version.parse("1.2.4-b42-10-g3b5f5762", git=True) < Version.parse("1.2.4")
assert Version.parse("1.2.3-rc10+dev") < Version.parse("1.2.3")
assert Version.parse("1.2.3-b10+dev") < Version.parse("1.2.3-rc1+dev")
assert Version.parse("1.2.3-b10+dev") < Version.parse("1.2.3+dev")
_test_format_version(Version(1, 2, 3), "1.2.3")
_test_format_version(Version(1, 2, 3, prerelease="a1"), "1.2.3-a1")
_test_format_version(Version(1, 2, 3, prerelease="b2", dev=4), "1.2.3-b2-dev.4")
_test_format_version(Version(1, 2, 3, dev=0), "1.2.3-dev.0")
_test_format_version(Version(1, 2, 3, dev=5), "1.2.3-dev.5")
_test_format_version(Version(1, 2, 3, prerelease="a1", local="foo.bar"), "1.2.3-a1+foo.bar")
_test_format_version(
    Version(1, 2, 3, prerelease="b2", dev=4, local="foo.bar"), "1.2.3-b2-dev.4+foo.bar"
)
_test_format_version(Version(1, 2, 3, dev=4, local="foo.bar"), "1.2.3-dev.4+foo.bar")


def run_git(*cmd: Any, verbose: bool = False) -> str:
    cmd = ["git", *cmd]
    proc = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if proc.returncode != 0:
        raise RuntimeError(
            f"Error while running `{cmd}`: returned {proc.returncode}\n"
            f"stdout:\n{proc.stdout.decode()}\n"
            f"stderr:\n{proc.stderr.decode()}\n"
        )
    stderr = proc.stderr.decode()
    if verbose and stderr:
        print(
            f"[Stderr stream from {COLOR_RED}{cmd}{COLOR_END}]\n{stderr}[End stderr stream]",
            file=sys.stderr,
        )
    return proc.stdout.decode()


def get_version_from_code() -> Version:
    return Version.parse(version_updater.TOOLS_VERSION[version_updater.Tool.Parsec])


def update_version_files(new_version: Version) -> None:
    version_updater.check_tool(version_updater.Tool.Parsec, str(new_version), update=True)


def update_license_file(new_version: Version, new_release_date: date) -> None:
    license_txt = BUSL_LICENSE_FILE.read_text(encoding="utf8")
    half_updated_license_txt = re.sub(
        r"Change Date:.*", f"Change Date:  {new_release_date.strftime('%b %d, %Y')}", license_txt
    )
    updated_version_txt = re.sub(
        r"Licensed Work:.*",
        f"Licensed Work:  Parsec {new_version.with_v_prefix()}",
        half_updated_license_txt,
    )
    assert (
        updated_version_txt != half_updated_license_txt
    ), f"The `Licensed Work` field should have changed, but hasn't (likely because the new version `{new_version}` correspond to the version written on the license file)"
    BUSL_LICENSE_FILE.write_bytes(
        updated_version_txt.encode("utf8")
    )  # Use write_bytes to keep \n on Windows


def collect_newsfragments() -> list[Path]:
    fragments = []
    fragment_regex = re.compile(r"^[0-9]+\.(" + "|".join(FRAGMENT_TYPES.keys()) + r")\.rst$")
    for entry in FRAGMENTS_DIR.iterdir():
        if entry.name in (".gitkeep", "README.rst"):
            continue
        # Sanity check
        if not fragment_regex.match(entry.name) or not entry.is_file():
            raise ReleaseError(f"Invalid entry detected in newsfragments dir: `{entry.name}`")
        fragments.append(entry)

    return fragments


def build_release(version: Version, yes: bool) -> None:
    if version.is_dev:
        raise ReleaseError(f"Don't release a dev version: {version}")

    print(f"Building release {COLOR_GREEN}{version}{COLOR_END} ...")
    old_version = get_version_from_code()
    if version <= old_version:
        raise ReleaseError(
            f"Previous version is greater that the new version ({COLOR_YELLOW}{old_version}{COLOR_END} >= {COLOR_YELLOW}{version}{COLOR_END})"
        )

    now = datetime.utcnow()
    release_date = now.date()
    # Cannot just add years to date given it wouldn't handle february 29th
    license_conversion_date = datetime.fromtimestamp(
        now.timestamp() + LICENSE_CONVERSION_DELAY
    ).date()
    assert release_date.toordinal() < license_conversion_date.toordinal()

    # Check repo is clean
    stdout = run_git("status", "--porcelain", "--untracked-files=no")
    if stdout.strip():
        raise ReleaseError("Repository is not clean, aborting")

    # Update BUSL license date marker & version info
    update_license_file(version, license_conversion_date)
    # Update parsec version
    update_version_files(version)

    # Update HISTORY.rst
    history_header, history_body = split_history_file()

    newsfragments = collect_newsfragments()
    issues_per_type: defaultdict[str, list[str]] = convert_newsfragments_to_rst(newsfragments)

    new_entry = gen_rst_release_entry(version, release_date, issues_per_type)

    updated_history_txt = f"{history_header}{new_entry}{history_body}".strip() + "\n"
    HISTORY_FILE.write_bytes(
        updated_history_txt.encode("utf8")
    )  # Use write_bytes to keep \n on Windows

    print("New entry in `history.rst`:\n\n```")
    print(new_entry)
    print("```")

    # Make git commit

    if not yes:
        input(
            f"Pausing so you can check {COLOR_YELLOW}HISTORY.rst{COLOR_END} is okay, press enter when ready"
        )

    commit_msg = f"Bump version {old_version} -> {version}"
    print(f"Create commit {COLOR_GREEN}{commit_msg}{COLOR_END}")
    run_git("add", HISTORY_FILE.absolute(), *FILES_TO_COMMIT_ON_VERSION_CHANGE)
    if newsfragments:
        fragments_pathes = [str(x.absolute()) for x in newsfragments]
        run_git("rm", *fragments_pathes)
    # FIXME: the `releaser` steps in pre-commit is disable is `no-verify` still required ?
    # Disable pre-commit hooks given this commit wouldn't pass `releaser check`
    run_git("commit", "-m", commit_msg, "--no-verify")

    print(f"Create tag {COLOR_GREEN}{version}{COLOR_END}")
    run_git("tag", version.with_v_prefix(), "-m", f"Release version {version}", "-a", "-s")

    # Update __version__ with dev suffix
    dev_version = version.evolve(local="dev")
    commit_msg = f"Bump version {version} -> {dev_version}"
    print(f"Create commit {COLOR_GREEN}{commit_msg}{COLOR_END}")
    update_license_file(dev_version, license_conversion_date)
    update_version_files(dev_version)
    run_git("add", *FILES_TO_COMMIT_ON_VERSION_CHANGE)
    # Disable pre-commit hooks given this commit wouldn't pass `releaser check`
    run_git("commit", "-m", commit_msg, "--no-verify")


def gen_rst_release_entry(
    version: Version, release_date: date, issues_per_type: defaultdict[str, list[str]]
) -> str:
    new_entry_title = f"Parsec {version.with_v_prefix()} ({release_date.isoformat()})"
    new_entry = f"\n\n{new_entry_title}\n{len(new_entry_title) * '-'}\n"

    if not issues_per_type:
        new_entry += "\nNo significant changes.\n"
    else:
        for fragment_type, fragment_title in FRAGMENT_TYPES.items():
            if fragment_type not in issues_per_type:
                continue
            new_entry += f"\n{fragment_title}\n{len(fragment_title) * '~'}\n\n"
            new_entry += "\n".join(issues_per_type[fragment_type])
            new_entry += "\n"
    return new_entry


def convert_newsfragments_to_rst(newsfragments: list[Path]) -> defaultdict[str, list[str]]:
    issues_per_type = defaultdict(list)
    for fragment in newsfragments:
        issue_id, type, _ = fragment.name.split(".")
        # Don't add empty fragments. Still needed to be collected as they will be deleted later
        if type == "empty":
            continue
        issue_txt = f"{fragment.read_text(encoding='utf8')} (`#{issue_id} <https://github.com/Scille/parsec-cloud/issues/{issue_id}>`__)\n"
        wrapped_issue_txt = textwrap.fill(
            issue_txt, width=80, break_long_words=False, initial_indent="* ", subsequent_indent="  "
        )
        issues_per_type[type].append(wrapped_issue_txt)
    return issues_per_type


def split_history_file() -> tuple[str, str]:
    history_txt = HISTORY_FILE.read_text(encoding="utf8")
    header_split = ".. towncrier release notes start\n"
    if header_split in history_txt:
        header, history_body = history_txt.split(header_split, 1)
        history_header = header + header_split
    else:
        raise ValueError(f"Cannot the header split tag in `{HISTORY_FILE!s}`")
    return history_header, history_body


def check_release(version: Version) -> None:
    print(f"Checking release {COLOR_GREEN}{version}{COLOR_END}")

    def success() -> None:
        print(f" [{COLOR_GREEN}OK{COLOR_END}]")

    def failed() -> None:
        print(f" [{COLOR_RED}FAILED{COLOR_END}]")

    # Check __version__
    print(f"Validating version {COLOR_GREEN}{version}{COLOR_END} across our repo ...", end="")
    code_version = get_version_from_code()
    if code_version != version:
        raise ReleaseError(
            f"Invalid __version__ in parsec/_version.py: expected `{COLOR_YELLOW}{version}{COLOR_END}`, got `{COLOR_YELLOW}{code_version}{COLOR_END}`"
        )
    version_updater.check_tool(version_updater.Tool.Parsec, str(version), update=False)
    success()

    # Check newsfragments
    print(f"Checking we dont have any left over fragment ...", end="")
    fragments = collect_newsfragments()
    if fragments:
        fragments_names = [fragment.name for fragment in fragments]
        failed()
        raise ReleaseError(
            f"newsfragments still contains fragments files ({', '.join(fragments_names)})"
        )
    success()

    # Check tag exist and is an annotated&signed one
    show_info = run_git("show", "--quiet", version.with_v_prefix())
    tag_type = show_info.split(" ", 1)[0]

    print(f"Checking we have an annotated tag for {COLOR_GREEN}{version}{COLOR_END} ...", end="")
    if tag_type != "tag":
        failed()
        raise ReleaseError(f"{version} is not an annotated tag (type: {tag_type})")
    success()

    print("Checking we have signed our release commit ...", end="")
    if "BEGIN PGP SIGNATURE" not in show_info:
        failed()
        raise ReleaseError(f"{version} is not signed")
    success()


def build_main(args: argparse.Namespace) -> None:
    if not args.version:
        raise SystemExit("version is required for build command")
    # TODO: rethink the non-release checks
    # current_version = get_version_from_repo_describe_tag(args.verbose)
    # check_non_release(current_version)
    build_release(args.version, yes=args.yes)


def check_main(args: argparse.Namespace) -> None:
    version: Version = args.version or get_version_from_code()

    if version.is_dev:
        # TODO: rethink the non-release checks
        # check_non_release(version)
        print(f"Detected dev version {COLOR_GREEN}{version}{COLOR_END}, nothing to check...")
        pass

    else:
        check_release(version)


def rollback_main(args: argparse.Namespace) -> None:
    """
    Revert the change made by `build_release`.

    Note: for rollback to work, we expect that you don't have made and commit after `Bump version vX.Y.B -> vX.Y.B+dev` (the last commit created by `build_release`).
    """
    if run_git("diff-index", "HEAD", "--").strip():
        raise ReleaseError("Local changes are present, aborting...")

    current_version = get_version_from_code()
    if not current_version.is_dev:
        raise ReleaseError(
            f"Invalid __version__ in parsec/_version.py: expected {COLOR_YELLOW}vX.Y.Z+dev{COLOR_END}, got {COLOR_YELLOW}{current_version}{COLOR_END}"
        )

    version = current_version.evolve(local=None)
    print(
        f"__version__ in parsec/_version.py contains version {COLOR_GREEN}{current_version}{COLOR_END}, hence we should rollback version {COLOR_GREEN}{version}{COLOR_END}"
    )

    # Retrieve `Bump version vX.Y.A+dev -> vX.Y.B` and `Bump version vX.Y.B -> vX.Y.B+dev` commits
    head, head_minus_1, head_minus_2 = run_git("rev-list", "-n", "3", "HEAD").strip().splitlines()

    tag_commit = run_git("rev-list", "-n", "1", version.with_v_prefix()).strip()
    if tag_commit != head_minus_1:
        raise ReleaseError(
            f"Cannot rollback as tag {COLOR_YELLOW}{version}{COLOR_END} doesn't point on {COLOR_YELLOW}HEAD^{COLOR_END}"
        )

    print(f"Removing tag {COLOR_GREEN}{version}{COLOR_END}")
    run_git("tag", "--delete", version.with_v_prefix())
    print(f"Reset mastor to {COLOR_GREEN}{head_minus_2}{COLOR_END} (i.e. HEAD~2)")
    run_git("reset", "--hard", head_minus_2)


def parse_version_main(args: argparse.Namespace) -> None:
    raw_version = args.version

    assert isinstance(raw_version, str)
    version = Version.parse(raw_version)

    print(
        "\n".join(
            [
                f"full={version}",
                f"major={version.major}",
                f"minor={version.minor}",
                f"patch={version.patch}",
                f"prerelease={version.prerelease or ''}",
                f"dev={version.dev or ''}",
                f"local={version.local or ''}",
            ]
        )
    )


def cli(description: str) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=description,
        # We use `RawDescriptionHelpFormatter` to not have `argparse` mess-up our formatted description.
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument("-v", "--verbose", action="store_true")

    subparsers = parser.add_subparsers(
        title="commands",
        required=True,
        dest="command",
    )

    build = subparsers.add_parser(
        "build",
        help="Prepare for a new release",
        description=build_main.__doc__,
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    build.add_argument("version", type=Version.parse, help="The new release version")
    build.add_argument("-y", "--yes", help="Reply `yes` to asked question", action="store_true")

    check = subparsers.add_parser(
        "check",
        help="Verify the we have correctly set the parsec's version across the repository",
        description=check_main.__doc__,
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    check.add_argument(
        "version",
        type=Version.parse,
        nargs="?",
        help="The version to use (default to `git describe`)",
    )

    subparsers.add_parser(
        "rollback",
        help="Rollback the last release",
        description=rollback_main.__doc__,
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )

    parse_version = subparsers.add_parser(
        "parse-version",
        help="Parse a provided version",
        description=parse_version_main.__doc__,
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parse_version.add_argument("version", help="The version to parse")

    return parser.parse_args()


if __name__ == "__main__":
    COMMAND = {
        "build": build_main,
        "check": check_main,
        "rollback": rollback_main,
        "parse-version": parse_version_main,
    }

    args = cli(DESCRIPTION)

    if args.command not in COMMAND.keys():
        raise ValueError(f"Unknown command `{args.command}`")

    COMMAND[args.command](args)
