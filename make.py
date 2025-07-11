#! /usr/bin/env python
# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

import argparse
import itertools
import os
import platform
import random
import shutil
import subprocess
import sys
import textwrap
from collections.abc import Iterable
from pathlib import Path

CYAN = "\x1b[36m"
GREY = "\x1b[37m"
PINK = "\x1b[35m"
BOLD_RED = "\x1b[1;31m"
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


# Depending on packer's needs, we may or may not want to vendor OpenSSL
# (e.g. Docker image already ships openssl so no need to vendor, wheel
# distributed on pypi requires vendoring to comply with manylinux)
# This is only needed for Linux as other OSs don't provide OpenSSL (and
# hence it obviously must be vendored anytime we need it).
maybe_force_vendored_dbus_cargo_flags = maybe_force_vendored_openssl_cargo_flags = ""
if sys.platform == "linux":
    LIBPARSEC_FORCE_VENDORED_LIBS = os.environ.get("LIBPARSEC_FORCE_VENDORED_LIBS", "false").lower()
    if (
        os.environ.get("LIBPARSEC_FORCE_VENDORED_OPENSSL", LIBPARSEC_FORCE_VENDORED_LIBS).lower()
        == "true"
    ):
        maybe_force_vendored_openssl_cargo_flags = "--features vendored-openssl"
    if (
        os.environ.get("LIBPARSEC_FORCE_VENDORED_KEYRING", LIBPARSEC_FORCE_VENDORED_LIBS).lower()
        == "true"
    ):
        maybe_force_vendored_dbus_cargo_flags = "--features vendored-dbus"

PYTHON_RELEASE_CARGO_FLAGS = (
    f"--profile=release --features use-sodiumoxide {maybe_force_vendored_openssl_cargo_flags}"
)
PYTHON_DEV_CARGO_FLAGS = "--profile=dev-python --features test-utils"
PYTHON_CI_CARGO_FLAGS = "--profile=ci-python --features test-utils"

ELECTRON_RELEASE_CARGO_FLAGS = f"--profile=release --features libparsec/use-sodiumoxide {maybe_force_vendored_openssl_cargo_flags} {maybe_force_vendored_dbus_cargo_flags}"
ELECTRON_DEV_CARGO_FLAGS = "--profile=dev --features test-utils"
ELECTRON_CI_CARGO_FLAGS = "--profile=ci-rust --features test-utils"

WEB_RELEASE_CARGO_FLAGS = "--release"  # Note: on web we use RustCrypto for release
# Small hack here: we always pass the `--dev` flag to wasm-pack given it
# considers there is no need to pass extra args to cargo (i.e. cargo build
# for dev by default). This way we can pass our own `--profile=foo` extra args
# without cargo complaining it is clashing with `--release` (provided by wasm-pack, and
# equivalent to `--profile=release`).
# This should be safe given if anything change, cargo won't allow multiple profiles
# to be passed (hence this script will simply fail).
# cf. https://github.com/rustwasm/wasm-pack/blob/b4e619c8a13a8441b804895348afbfd4fb1a68a3/src/build/mod.rs#L91-L106
# and https://github.com/rustwasm/wasm-pack/blob/b4e619c8a13a8441b804895348afbfd4fb1a68a3/src/command/build.rs#L220
WEB_DEV_CARGO_FLAGS = "--dev -- --features test-utils"
WEB_CI_CARGO_FLAGS = f"{WEB_DEV_CARGO_FLAGS} --profile=ci-rust"

CLI_RELEASE_CARGO_FLAGS = f"--profile=release {maybe_force_vendored_dbus_cargo_flags} {maybe_force_vendored_openssl_cargo_flags}"

# TL;DR: ONLY USE THE REAL ZSTD IN PRODUCTION !!!
#
# `libparsec_zstd` is just a shim over `zstd` crate to provide a simpler-to-build
# pure Rust implementation when compiling for WASM on Windows/MacOS (see
# https://github.com/gyscos/zstd-rs/issues/93).
#
# However `Cargo.toml` doesn't support specifying dependencies based on the host platform,
# so we have to do the detection here and pass a cfg parameter accordingly.

USE_PURE_RUST_BUT_DIRTY_ZSTD_EXTRA_ENV = {"RUSTFLAGS": "--cfg use_pure_rust_but_dirty_zstd"}

_web_non_release_build_uses_pure_rust_but_dirty_zstd = None


def get_web_non_release_build_uses_pure_rust_but_dirty_zstd() -> bool:
    # Use a global variable so that the warning is only displayed once
    global _web_non_release_build_uses_pure_rust_but_dirty_zstd

    if _web_non_release_build_uses_pure_rust_but_dirty_zstd is not None:
        return _web_non_release_build_uses_pure_rust_but_dirty_zstd

    _web_non_release_build_uses_pure_rust_but_dirty_zstd = False
    if platform.system().lower() in ("windows", "darwin"):
        _web_non_release_build_uses_pure_rust_but_dirty_zstd = True
    libparsec_force_web_zstd_env = os.environ.get("LIBPARSEC_FORCE_WEB_ZSTD", "").lower()
    if libparsec_force_web_zstd_env == "real":
        _web_non_release_build_uses_pure_rust_but_dirty_zstd = False
    elif libparsec_force_web_zstd_env == "dirty":
        _web_non_release_build_uses_pure_rust_but_dirty_zstd = True

    if _web_non_release_build_uses_pure_rust_but_dirty_zstd:
        print(
            f"{BOLD_RED}WARNING: Enabling `libparsec_zstd`'s pure Rust but dirty implementation cfg for WASM !{NO_COLOR}",
            file=sys.stderr,
        )
        print(
            f"{BOLD_RED}WARNING: This is done to simplify WASM compilation on Windows&MacOS and should be compatible with the real Zstd implementation, but DON'T USE THAT FOR PRODUCTION !{NO_COLOR}",
            file=sys.stderr,
        )
        print(
            f"{BOLD_RED}WARNING: You can set `LIBPARSEC_FORCE_WEB_ZSTD=real` to switch to disable this (you may need to install LLVM){NO_COLOR}",
            file=sys.stderr,
        )

    return _web_non_release_build_uses_pure_rust_but_dirty_zstd


BASE_DIR = Path(__file__).parent.resolve()
SERVER_DIR = BASE_DIR / "server"
BINDINGS_ELECTRON_DIR = BASE_DIR / "bindings/electron"
BINDINGS_WEB_DIR = BASE_DIR / "bindings/web"


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
            [f"{GREY}{k}='{v}'{NO_COLOR}" for k, v in self.extra_env.items()]
        )
        display_cmds = []
        display_extra_cmds = f" {' '.join(extra_cmd_args)}" if extra_cmd_args else ""
        display_cmds.append(f"{CYAN}{self.cmd}{display_extra_cmds}{NO_COLOR}")
        return f"{display_extra_env} {' && '.join(display_cmds)}"

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


class CmdWithConfigurableWebZstd(Cmd):
    def display(self, extra_cmd_args: Iterable[str]) -> str:
        self.prepare()
        return super().display(extra_cmd_args)

    def run(self, cwd: Path, extra_cmd_args: Iterable[str]) -> None:
        self.prepare()
        return super().run(cwd, extra_cmd_args)

    def prepare(self) -> None:
        if get_web_non_release_build_uses_pure_rust_but_dirty_zstd():
            self.extra_env.update(USE_PURE_RUST_BUT_DIRTY_ZSTD_EXTRA_ENV)


COMMANDS: dict[tuple[str, ...], Op | tuple[Op, ...]] = {
    #
    # Python bindings for the server
    #
    ("python-dev-install", "i"): (
        Cwd(SERVER_DIR),
        # Don't build rust as part of poetry install step.
        # This is because poetry creates a temporary virtualenv to run the
        # build in, hence the python interpreter path changes between
        # `poetry install` and the very next `maturin develop`, causing the
        # latter to wast time on useless rebuild.
        # (note only the first `maturin develop` is impacted, as all maturin
        # develop uses the same default virtualenv)
        Cmd(
            cmd="poetry install --with=testbed-server",
            extra_env={"POETRY_LIBPARSEC_BUILD_STRATEGY": "no_build"},
        ),
        Cmd(f"poetry run maturin develop --locked {PYTHON_DEV_CARGO_FLAGS}"),
    ),
    ("python-dev-rebuild", "r"): (
        Cwd(SERVER_DIR),
        Cmd(f"poetry run maturin develop --locked {PYTHON_DEV_CARGO_FLAGS}"),
    ),
    ("python-ci-install",): (
        Cwd(SERVER_DIR),
        Cmd(
            cmd="poetry install",
            extra_env={"POETRY_LIBPARSEC_BUILD_PROFILE": "ci"},
        ),
    ),
    # Flags used in poetry's `server/build.py` when command is `python-ci-build`
    ("python-ci-libparsec-cargo-flags",): Echo(PYTHON_CI_CARGO_FLAGS),
    # Flags used in poetry's `server/build.py` when generating the release wheel
    ("python-release-libparsec-cargo-flags",): Echo(PYTHON_RELEASE_CARGO_FLAGS),
    # Flags used in poetry's `server/build.py` when generating the dev wheel
    ("python-dev-libparsec-cargo-flags",): Echo(PYTHON_DEV_CARGO_FLAGS),
    #
    # Parsec CLI
    #
    ("cli-release-cargo-flags",): Echo(CLI_RELEASE_CARGO_FLAGS),
    #
    # Electron bindings
    #
    ("electron-dev-install", "ei"): (
        Cwd(BINDINGS_ELECTRON_DIR),
        Cmd(
            cmd="npm install",
        ),
        Cmd(
            cmd="npm run build:dev",
        ),
    ),
    ("electron-dev-rebuild", "er"): (
        Cwd(BINDINGS_ELECTRON_DIR),
        Cmd(
            cmd="npm run build:dev",
        ),
    ),
    ("electron-ci-install",): (
        Cwd(BINDINGS_ELECTRON_DIR),
        Cmd(
            cmd="npm install",
        ),
        Cmd(
            cmd="npm run build:ci",
        ),
    ),
    ("electron-release-install",): (
        Cwd(BINDINGS_ELECTRON_DIR),
        Cmd(
            cmd="npm install",
        ),
        Cmd(
            cmd="npm run build:release",
        ),
    ),
    # Flags used in `bindings/electron/scripts/build.js`
    ("electron-ci-libparsec-cargo-flags",): Echo(ELECTRON_CI_CARGO_FLAGS),
    # Flags used in `bindings/electron/scripts/build.js`
    ("electron-release-libparsec-cargo-flags",): Echo(ELECTRON_RELEASE_CARGO_FLAGS),
    # Flags used in `bindings/electron/scripts/build.js`
    ("electron-dev-libparsec-cargo-flags",): Echo(ELECTRON_DEV_CARGO_FLAGS),
    #
    # Web bindings
    #
    ("web-dev-install", "wi"): (
        Cwd(BINDINGS_WEB_DIR),
        Cmd(
            cmd="npm install",
        ),
        Rmdir(BINDINGS_WEB_DIR / "pkg"),
        CmdWithConfigurableWebZstd(
            cmd="npm run build:dev",
        ),
    ),
    ("web-dev-rebuild", "wr"): (
        Cwd(BINDINGS_WEB_DIR),
        Rmdir(BINDINGS_WEB_DIR / "pkg"),
        CmdWithConfigurableWebZstd(
            cmd="npm run build:dev",
        ),
    ),
    ("web-ci-install",): (
        Rmdir(BINDINGS_WEB_DIR / "pkg"),
        Cmd(
            cmd="npm install",
        ),
        CmdWithConfigurableWebZstd(
            cmd="npm run build:ci",
        ),
    ),
    ("web-release-install",): (
        Cwd(BINDINGS_WEB_DIR),
        Cmd(
            cmd="npm install",
        ),
        # Don't use `CmdWithConfigurableWebZstd` here: Release should ALWAYS uses the real Zstd implementation !
        Cmd(
            cmd="npm run build:release",
        ),
    ),
    ("run-testbed-server", "rts"): (
        Cwd(SERVER_DIR),
        Cmd(
            cmd="poetry run python -m parsec testbed",
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
    parser.add_argument(
        "command",
        help="The command to run",
        nargs="?",
        choices=list(itertools.chain.from_iterable(COMMANDS.keys())),
        metavar="command",
    )

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
