#! /usr/bin/env python
# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from __future__ import annotations

import argparse
import os
import platform
import random
import shutil
import subprocess
import sys
import textwrap
from pathlib import Path
from typing import Iterable, Union

PYTHON_RELEASE_CARGO_FLAGS = "--profile=release --features libparsec/use-sodiumoxide"
PYTHON_DEV_CARGO_FLAGS = "--profile=dev-python --features test-utils"
PYTHON_CI_CARGO_FLAGS = "--profile=ci-python --features test-utils"

ELECTRON_RELEASE_CARGO_FLAGS = "--profile=release"
ELECTRON_RELEASE_SODIUM_CARGO_FLAGS = "--profile=release --features libparsec/use-sodiumoxide"
ELECTRON_DEV_CARGO_FLAGS = "--profile=dev --features test-utils"
ELECTRON_CI_CARGO_FLAGS = "--profile=ci-rust --features test-utils"

WEB_RELEASE_CARGO_FLAGS = "--profile=release"  # Note: on web we use RustCrypto for release
WEB_DEV_CARGO_FLAGS = "--profile=dev --features test-utils"
WEB_CI_CARGO_FLAGS = "--profile=ci-rust --features test-utils"


BASE_DIR = Path(__file__).parent.resolve()
ELECTRON_DIR = BASE_DIR / "oxidation/bindings/electron"
WEB_DIR = BASE_DIR / "oxidation/bindings/web"


CYAN = "\x1b[36m"
GREY = "\x1b[37m"
PINK = "\x1b[35m"
BOLD_YELLOW = "\x1b[1;33m"
NO_COLOR = "\x1b[0;0m"
CUTENESS = [
    "ฅ^◕ﻌ◕^ฅ",
    "(^･㉨･^)∫",
    "^•ﻌ•^ฅ",
    "✺◟(⏓ᴥ⏓▽)◞✺",
    "ฅ(・㉨・˶)ฅ",
    "(ﾐꆤ ﻌ ꆤﾐ)∫",
    "(ﾐㆁ㉨ㆁﾐ)",
    "ฅ(=ච ω ච=)",
]


class Op:
    def display(self, extra_cmd_args: Iterable[str]) -> str:
        raise NotImplementedError

    def run(self, cwd: Path, extra_cmd_args: Iterable[str]) -> None:
        raise NotImplementedError


class Cwd(Op):
    def __init__(self, cwd: Path) -> None:
        self.cwd = cwd

    def display(self, extra_cmd_args: Iterable[str]) -> str:
        return f"cd {GREY}{self.cwd.relative_to(BASE_DIR).as_posix()}{NO_COLOR}"


class Rmdir(Op):
    def __init__(self, target: Path) -> None:
        self.target = target

    def display(self, extra_cmd_args: Iterable[str]) -> str:
        return f"{CYAN}rm -rf {self.target.relative_to(BASE_DIR).as_posix()}{NO_COLOR}"

    def run(self, cwd: Path, extra_cmd_args: Iterable[str]) -> None:
        target = self.target if self.target.is_absolute() else cwd / self.target
        shutil.rmtree(target, ignore_errors=True)


class Echo(Op):
    def __init__(self, msg: str) -> None:
        self.msg = msg

    def display(self, extra_cmd_args: Iterable[str]) -> str:
        return f"{CYAN}echo {self.msg!r}{NO_COLOR}"

    def run(self, cwd: Path, extra_cmd_args: Iterable[str]) -> None:
        print(self.msg, flush=True)


class Cmd(Op):
    def __init__(
        self,
        cmd: str,
        extra_env: dict[str, str] = {},
    ) -> None:
        self.cmd = cmd
        self.extra_env = extra_env

    def display(self, extra_cmd_args: Iterable[str]) -> str:
        display_extra_env = " ".join(
            [f"{GREY}{k}={v}{NO_COLOR}" for k, v in self.extra_env.items()]
        )
        display_cmds = []
        display_extra_cmds = f" {' '.join(extra_cmd_args)}" if extra_cmd_args else ""
        display_cmds.append(f"{CYAN}{self.cmd}{display_extra_cmds}{NO_COLOR}")
        return f"{display_extra_env} {' && '.join(display_cmds) }"

    def run(self, cwd: Path, extra_cmd_args: Iterable[str]) -> None:
        args = self.cmd.split() + list(extra_cmd_args)

        # On Windows, only .exe/.bat can be directly executed,
        # `npm.cmd` is a the .bat version of `npm`
        if args[0] == "npm" and platform.system().lower() == "windows":
            args[0] = "npm.cmd"

        subprocess.check_call(
            args,
            env={**os.environ, **self.extra_env},
            cwd=cwd,
        )


COMMANDS: dict[tuple[str, ...], Union[Op, tuple[Op, ...]]] = {
    #
    # Python bindings
    #
    ("python-dev-install", "i"): (
        # Don't build rust as part of poetry install step.
        # This is because poetry creates a temporary virtualenv to run the
        # build in, hence the python interpreter path changes between
        # `poetry install` and the very next `maturin develop`, causing the
        # latter to wast time on useless rebuild.
        # (note only the first `maturin develop` is impacted, as all maturin
        # develop uses the same default virtualenv)
        Cmd(
            cmd="poetry install -E backend -E core --with=docs",
            extra_env={"POETRY_LIBPARSEC_BUILD_STRATEGY": "no_build"},
        ),
        Cmd(f"maturin develop {PYTHON_DEV_CARGO_FLAGS}"),
    ),
    ("python-dev-rebuild", "r"): Cmd(f"maturin develop {PYTHON_DEV_CARGO_FLAGS}"),
    ("python-ci-install",): Cmd(
        cmd="poetry install -E backend -E core",
        extra_env={"POETRY_LIBPARSEC_BUILD_PROFILE": "ci"},
    ),
    # Flags used in poetry's `build.py` when command is `python-ci-build`
    ("python-ci-libparsec-cargo-flags",): Echo(PYTHON_CI_CARGO_FLAGS),
    # Flags used in poetry's `build.py` when generating the release wheel
    ("python-release-libparsec-cargo-flags",): Echo(PYTHON_RELEASE_CARGO_FLAGS),
    # Flags used in poetry's `build.py` when generating the dev wheel
    ("python-dev-libparsec-cargo-flags",): Echo(PYTHON_DEV_CARGO_FLAGS),
    #
    # Electron bindings
    #
    ("electron-dev-install", "ei"): (
        Cwd(ELECTRON_DIR),
        Cmd(
            cmd="npm install",
        ),
        Cmd(
            cmd=f"npm run build:dev",
        ),
    ),
    ("electron-dev-rebuild", "er"): (
        Cwd(ELECTRON_DIR),
        Cmd(
            cmd=f"npm run build:dev",
        ),
    ),
    ("electron-ci-install",): (
        Cwd(ELECTRON_DIR),
        Cmd(
            cmd="npm install",
        ),
        Cmd(
            cmd=f"npm run build:ci",
        ),
    ),
    # Flags used in `bindings/electron/scripts/build.js`
    ("electron-ci-libparsec-cargo-flags",): Echo(ELECTRON_CI_CARGO_FLAGS),
    # Flags used in `bindings/electron/scripts/build.js`
    ("electron-release-libparsec-cargo-flags",): Echo(ELECTRON_RELEASE_CARGO_FLAGS),
    # Flags used in `bindings/electron/scripts/build.js`
    ("electron-release-sodium-libparsec-cargo-flags",): Echo(ELECTRON_RELEASE_SODIUM_CARGO_FLAGS),
    # Flags used in `bindings/electron/scripts/build.js`
    ("electron-dev-libparsec-cargo-flags",): Echo(ELECTRON_DEV_CARGO_FLAGS),
    #
    # Web bindings
    #
    ("web-dev-install", "wi"): (
        Cwd(WEB_DIR),
        Cmd(
            cmd="npm install",
        ),
        Rmdir(WEB_DIR / "pkg"),
        Cmd(
            cmd="npm run build:dev",
        ),
    ),
    ("web-dev-rebuild", "wr"): (
        Cwd(WEB_DIR),
        Rmdir(WEB_DIR / "pkg"),
        Cmd(
            cmd="npm run build:dev",
        ),
    ),
    ("web-ci-install",): (
        Rmdir(WEB_DIR / "pkg"),
        Cmd(
            cmd="npm install",
        ),
        Cmd(
            cmd=f"npm run build:ci",
        ),
    ),
    # Flags used in `bindings/web/scripts/build.js`
    ("web-ci-libparsec-cargo-flags",): Echo(WEB_CI_CARGO_FLAGS),
    # Flags used in `bindings/web/scripts/build.js`
    ("web-release-libparsec-cargo-flags",): Echo(WEB_RELEASE_CARGO_FLAGS),
    # Flags used in `bindings/web/scripts/build.js`
    ("web-dev-libparsec-cargo-flags",): Echo(WEB_DEV_CARGO_FLAGS),
}


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        formatter_class=argparse.RawDescriptionHelpFormatter,
        description=textwrap.dedent(
            """\
            Tired of remembering multiple silly commands ? Now here is a single silly command to remember !

            Examples:
            python make.py install  # Initial installation
        """
        ),
    )
    parser.add_argument("--quiet", "-q", action="store_true", help="🎵 The sound of silence 🎵")
    parser.add_argument("--dry", action="store_true", help="Don't actually run, just display")
    parser.add_argument("command", help="The command to run", nargs="?")

    # Handle `-- <extra_cmd_args>` in argv
    # (argparse doesn't understand `--`, so we have to implement it by hand)
    has_reached_cmd_extra_args = False
    extra_cmd_args = []
    argv = []
    for arg in sys.argv[1:]:
        if has_reached_cmd_extra_args:
            extra_cmd_args.append(arg)
        elif arg == "--":
            has_reached_cmd_extra_args = True
        else:
            argv.append(arg)

    args = parser.parse_args(argv)
    if not args.command:
        print("Available commands:\n")
        for aliases, cmds in COMMANDS.items():
            print(f"{BOLD_YELLOW}{', '.join(aliases)}{NO_COLOR}")
            cmds = (cmds,) if isinstance(cmds, Op) else cmds
            display_cmds = [cmd.display(extra_cmd_args) for cmd in cmds]
            join = f"{GREY}; and {NO_COLOR}" if "fish" in os.environ.get("SHELL", "") else " && "
            print(f"\t{join.join(display_cmds)}\n")

    else:
        for aliases, cmds in COMMANDS.items():
            if args.command in aliases:
                cmds = (cmds,) if isinstance(cmds, Op) else cmds
                break
        else:
            raise SystemExit(f"Unknown command alias `{args.command}`")

        cwd = BASE_DIR
        for cmd in cmds:
            if not args.quiet:
                # Flush is required to prevent mixing with the output of sub-command
                print(f"{cmd.display(extra_cmd_args)}\n", flush=True)
                if not isinstance(cmd, Cwd):
                    try:
                        print(f"{PINK}{random.choice(CUTENESS)}{NO_COLOR}", flush=True)
                    except UnicodeEncodeError:
                        # Windows crappy term couldn't encode kitty unicode :'(
                        pass

            if args.dry:
                continue

            if isinstance(cmd, Cwd):
                cwd = cmd.cwd
            else:
                try:
                    cmd.run(cwd, extra_cmd_args)
                except subprocess.CalledProcessError as err:
                    raise SystemExit(str(err)) from err
