# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import os
import pathlib
import subprocess
import sys
import tempfile
import zipfile

# The default rust build profile to use when compiling the rust extension.
DEFAULT_CARGO_PROFILE = "release"


def display(line: str):
    YELLOW_FG = "\x1b[33m"
    DEFAULT_FG = "\x1b[39m"

    # Flush is required to prevent mixing with the output of sub-command with the output of the script
    print(f"{YELLOW_FG}{line}{DEFAULT_FG}", flush=True)


BASEDIR = pathlib.Path(__file__).parent
PYTHON_EXECUTABLE_PATH = sys.executable
display(f"PYTHON_EXECUTABLE_PATH={PYTHON_EXECUTABLE_PATH}")


def run(cmd, **kwargs):
    display(f">>> {cmd}")
    ret = subprocess.run(cmd, shell=True, check=True, **kwargs)
    return ret


def build():
    run(f"{PYTHON_EXECUTABLE_PATH} --version")
    run(f"{PYTHON_EXECUTABLE_PATH} misc/generate_pyqt.py")

    if sys.platform == "win32":
        libparsec_path = "parsec/_parsec.cp39-win_amd64.pyd"
    elif sys.platform == "darwin":
        libparsec_path = "parsec/_parsec.cpython-39-darwin.so"
    else:
        libparsec_path = "parsec/_parsec.cpython-39-x86_64-linux-gnu.so"

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

    # Maturin provides two commands to compile the Rust code as a Python native module:
    # - `maturin develop` that only compile the native module
    # - `maturin build` that generates an entire wheel of the project
    #
    # Here in theory we'd like to use `maturin develop`, then leave poetry does the
    # wheel generation. However the develop mode expects to be run from a virtualenv
    # with pip available which makes it unreliable to call it from here.
    # So we must ask maturin to build a wheel of the project, only to extract the
    # native module and discard the rest !

    maturin_build_profile = "--profile=" + os.environ.get("CARGO_PROFILE", DEFAULT_CARGO_PROFILE)
    features = (
        "--no-default-features --features use-sodiumoxide"
        if maturin_build_profile == "--profile=release"
        else ""
    )
    with tempfile.TemporaryDirectory() as distdir:
        run(
            f"maturin build {maturin_build_profile} --interpreter {PYTHON_EXECUTABLE_PATH} --out {distdir} {features}"
        )

        outputs = list(pathlib.Path(distdir).iterdir())
        if len(outputs) != 1:
            raise RuntimeError(f"Target dir unexpectedly contains multiple files: {outputs}")

        wheel_path = outputs[0]
        display(f"extracting {wheel_path}:{libparsec_path} -> {BASEDIR / libparsec_path}")
        with zipfile.ZipFile(wheel_path) as wheel:
            wheel.extract(libparsec_path, path=BASEDIR)


if __name__ == "__main__":
    display("Launching poetry build.py script")
    build()
