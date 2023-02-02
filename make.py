#! /usr/bin/env python
# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS


import argparse
import os
import random
import subprocess
import sys
import textwrap
from pathlib import Path
from typing import Optional, Union

PYTHON_RELEASE_CARGO_FLAGS = "--profile=release --features libparsec/use-sodiumoxide"
PYTHON_DEV_CARGO_FLAGS = "--profile=dev-python --features test-utils"
PYTHON_CI_CARGO_FLAGS = "--profile=ci-python --features test-utils"
ELECTRON_RELEASE_CARGO_FLAGS = "--profile=release"
ELECTRON_RELEASE_SODIUM_CARGO_FLAGS = "--profile=release --features libparsec/use-sodiumoxide"
ELECTRON_DEV_CARGO_FLAGS = "--profile=dev --features test-utils"
ELECTRON_CI_CARGO_FLAGS = "--profile=ci-rust --features test-utils"


BASE_DIR = Path(__file__).parent.resolve()
ELECTRON_DIR = BASE_DIR / "oxidation/bindings/electron"


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


class Cmd:
    def __init__(
        self,
        cmd: Union[str, list[str]],
        extra_env: dict[str, str] = {},
        cwd: Optional[Path] = None,
        is_script: bool = False,
    ) -> None:
        self.cmds = cmd if isinstance(cmd, list) else [cmd]
        self.extra_env = extra_env
        self.cwd = cwd
        # On Windows only .exe/.bat can be directly executed, so scripts must be
        # launched through a shell instead.
        self.is_script = is_script

    def display(self, extra_cmd_args: list[str] = []) -> str:
        display_extra_env = " ".join(
            [f"{GREY}{k}={v}{NO_COLOR}" for k, v in self.extra_env.items()]
        )
        display_cmds = []
        display_cwd = f"cd {GREY}{self.cwd.relative_to(BASE_DIR)}{NO_COLOR} && " if self.cwd else ""
        display_extra_cmds = f" {' '.join(extra_cmd_args)}" if extra_cmd_args else ""
        for cmd in self.cmds:
            display_cmds.append(f"{CYAN}{cmd}{display_extra_cmds}{NO_COLOR}")
        return f"{display_cwd}{display_extra_env} {' && '.join(display_cmds) }"

    def run(self, extra_cmd_args: list[str] = []) -> None:
        shell = sys.platform == "win32" and self.is_script
        for cmd in self.cmds:
            args = cmd.split() + extra_cmd_args
            # `echo` is not available on Windows
            if args[0] == "echo":
                print(" ".join(args[1:]))
            else:
                subprocess.check_call(
                    args,
                    env={**os.environ, **self.extra_env},
                    cwd=self.cwd or BASE_DIR,
                    shell=shell,
                )


COMMANDS: dict[tuple[str, ...], Union[Cmd, tuple[Cmd, ...]]] = {
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
            cmd="poetry install -E backend -E core",
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
    ("python-ci-libparsec-cargo-flags",): Cmd(
        cmd=f"echo {PYTHON_CI_CARGO_FLAGS}",
    ),
    # Flags used in poetry's `build.py` when generating the release wheel
    ("python-release-libparsec-cargo-flags",): Cmd(
        cmd=f"echo {PYTHON_RELEASE_CARGO_FLAGS}",
    ),
    # Flags used in poetry's `build.py` when generating the dev wheel
    ("python-dev-libparsec-cargo-flags",): Cmd(
        cmd=f"echo {PYTHON_DEV_CARGO_FLAGS}",
    ),
    #
    # Electron bindings
    #
    ("electron-dev-install", "ei"): (
        Cmd(
            cmd="npm install",
            cwd=ELECTRON_DIR,
            is_script=True,
        ),
        Cmd(
            cmd=f"npm run build:dev",
            cwd=ELECTRON_DIR,
            is_script=True,
        ),
    ),
    ("electron-dev-rebuild", "er"): Cmd(
        cmd=f"npm run build:dev",
        cwd=ELECTRON_DIR,
        is_script=True,
    ),
    ("electron-ci-install",): (
        Cmd(
            cmd="npm install",
            cwd=ELECTRON_DIR,
            is_script=True,
        ),
        Cmd(
            cmd=f"npm run build:ci",
            cwd=ELECTRON_DIR,
            is_script=True,
        ),
    ),
    # Flags used in `bindings/electron/scripts/build.js`
    ("electron-ci-libparsec-cargo-flags",): Cmd(
        cmd=f"echo {ELECTRON_CI_CARGO_FLAGS}",
    ),
    # Flags used in `bindings/electron/scripts/build.js`
    ("electron-release-libparsec-cargo-flags",): Cmd(
        cmd=f"echo {ELECTRON_RELEASE_CARGO_FLAGS}",
    ),
    # Flags used in `bindings/electron/scripts/build.js`
    ("electron-release-sodium-libparsec-cargo-flags",): Cmd(
        cmd=f"echo {ELECTRON_RELEASE_SODIUM_CARGO_FLAGS}",
    ),
    # Flags used in `bindings/electron/scripts/build.js`
    ("electron-dev-libparsec-cargo-flags",): Cmd(
        cmd=f"echo {ELECTRON_DEV_CARGO_FLAGS}",
    ),
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
            cmds = (cmds,) if isinstance(cmds, Cmd) else cmds
            display_cmds = [cmd.display() for cmd in cmds]
            join = f"{GREY}; and {NO_COLOR}" if "fish" in os.environ.get("SHELL", "") else " && "
            print(f"\t{join.join(display_cmds)}\n")

    else:
        for aliases, cmds in COMMANDS.items():
            if args.command in aliases:
                cmds = (cmds,) if isinstance(cmds, Cmd) else cmds
                break
        else:
            raise SystemExit(f"Unknown command alias `{args.command}`")

        for cmd in cmds:
            if not args.quiet:
                # Flush is required to prevent mixing with the output of sub-command
                print(
                    f"{cmd.display(extra_cmd_args)}\n",
                    flush=True,
                )
                try:
                    print(f"{PINK}{random.choice(CUTENESS)}{NO_COLOR}", flush=True)
                except UnicodeEncodeError:
                    # Windows crappy term couldn't encode kitty unicode :'(
                    pass
            if not args.dry:
                try:
                    cmd.run(extra_cmd_args)
                except subprocess.CalledProcessError as err:
                    raise SystemExit(str(err)) from err
