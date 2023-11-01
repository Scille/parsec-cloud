# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import os
import pathlib
import subprocess
import sys
import tempfile
import zipfile
from typing import Any

# The profile we pass to `make.py` to get the flags (cargo profile & features) for cargo build
DEFAULT_BUILD_PROFILE = "release"


def display(line: str) -> None:
    YELLOW_FG = "\x1b[33m"
    DEFAULT_FG = "\x1b[39m"

    # Flush is required to prevent mixing with the output of sub-command with the output of the script
    print(f"{YELLOW_FG}{line}{DEFAULT_FG}", flush=True)


BASEDIR = pathlib.Path(__file__).parent
PYTHON_EXECUTABLE_PATH = sys.executable
display(f"PYTHON_EXECUTABLE_PATH={PYTHON_EXECUTABLE_PATH}")


def run(cmd: str, **kwargs: Any) -> subprocess.CompletedProcess[bytes]:
    display(f">>> {cmd}")
    ret = subprocess.run(cmd, shell=True, check=True, **kwargs)
    return ret


def build() -> None:
    run(f"{PYTHON_EXECUTABLE_PATH} --version")
    run(f"maturin --version")

    if sys.platform == "linux":
        run("patchelf --version")

    if sys.platform == "win32":
        libparsec_path = "parsec/_parsec.cp312-win_amd64.pyd"
    elif sys.platform == "darwin":
        libparsec_path = "parsec/_parsec.cpython-312-darwin.so"
    else:
        libparsec_path = "parsec/_parsec.cpython-312-x86_64-linux-gnu.so"

    build_strategy = (
        os.environ.get("POETRY_LIBPARSEC_BUILD_STRATEGY", "always_build").strip().lower()
    )
    if build_strategy == "no_build":
        display(f"Skipping maturin build: POETRY_LIBPARSEC_BUILD_STRATEGY set to `no_build`")
        return
    elif build_strategy == "build_if_missing" and (BASEDIR / libparsec_path).exists():
        display(
            f"Skipping maturin build: POETRY_LIBPARSEC_BUILD_STRATEGY set to `build_if_missing` and {libparsec_path} already exists"
        )
        return

    # Retrieve the options to use for Cargo Compilation.
    # The idea here is to have the per-profile options centralized at the same place
    # (i.e. within `make.py`) to have a readable single source of truth (including
    # when compiling bindings unrelated to Python) on this sensible piece of configuration.
    build_profile = os.environ.get("POETRY_LIBPARSEC_BUILD_PROFILE", DEFAULT_BUILD_PROFILE)
    ret = run(
        f"{PYTHON_EXECUTABLE_PATH} {BASEDIR.parent}/make.py --quiet python-{build_profile}-libparsec-cargo-flags",
        stdout=subprocess.PIPE,
    )
    cargo_flags = ret.stdout.decode("ascii").strip()

    # When building with manylinux compatibility, maturin bundles the external shared
    # libraries and use patchelf to modify `_parsec.so` so that it only those ones
    # get loaded (and not e.g. the version potentially provided by the OS).
    #
    # So far in Parsec, we only have OpenSSL (so libssl.so + libcrypto.so) that falls
    # in this case.
    #
    # However we want to disable this feature for some packagings (e.g. Docker, Snap)
    # given they provide a image that already ships with the correct openssl (in this
    # case bundling would only mean ending up with two copies of the same lib).
    # We also want to disable this on CI given we build and run on the same machine
    # (so bundling increases needlessly the cache size).
    bundle_extra_so = (
        os.environ.get("POETRY_LIBPARSEC_BUNDLE_EXTRA_SHARED_LIBRARIES", "true").lower() == "true"
    )

    # Maturin provides two commands to compile the Rust code as a Python native module:
    # - `maturin develop` that only compile the native module
    # - `maturin build` that generates an entire wheel of the project
    #
    # Here in theory we'd like to use `maturin develop`, then leave poetry does the
    # wheel generation. However the develop mode expects to be run from a virtualenv
    # with pip available which makes it unreliable to call it from here.
    # So we must ask maturin to build a wheel of the project, only to extract the
    # native module and discard the rest !

    with tempfile.TemporaryDirectory() as distdir:
        cmd = f"maturin build --locked {cargo_flags} --manifest-path {BASEDIR / 'Cargo.toml'} --interpreter {PYTHON_EXECUTABLE_PATH} --out {distdir}"

        if not bundle_extra_so:
            cmd += " --compatibility linux"

        run(cmd)

        outputs = list(pathlib.Path(distdir).iterdir())
        if len(outputs) != 1:
            raise RuntimeError(f"Target dir unexpectedly contains multiple files: {outputs}")

        wheel_path = outputs[0]
        display(f"extracting {wheel_path}:{libparsec_path} -> {BASEDIR / libparsec_path}")
        with zipfile.ZipFile(wheel_path) as wheel:
            wheel.extract(libparsec_path, path=BASEDIR)
            # Also include bundled extra shared libraries if any
            for item in wheel.namelist():
                if item.startswith("parsec.libs/"):
                    display(f"extracting {wheel_path}:{item} -> {BASEDIR / item}")
                    wheel.extract(item, path=BASEDIR)


if __name__ == "__main__":
    display("Launching poetry build.py script")
    build()
