# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import os
import sys
import subprocess
import tempfile
import zipfile
import pathlib


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


def in_cibuildwheel():
    return bool(int(os.environ.get("CIBUILDWHEEL", "0")))


def force_maturin_release() -> bool:
    return os.environ.get("FORCE_MATURIN_RELEASE", "0") == "1"


def build():
    run(f"{PYTHON_EXECUTABLE_PATH} --version")
    run(f"{PYTHON_EXECUTABLE_PATH} misc/generate_pyqt.py")

    # Maturin provides two commands to compile the Rust code as a Python native module:
    # - `maturin develop` that only compile the native module
    # - `maturin build` that generates an entire wheel of the project
    #
    # Here in theory we'd like to use `maturin develop`, then leave poetry does the
    # wheel generation. However the develop mode expects to be run from a virtualenv
    # with pip available which makes it unreliable to call it from here.
    # So we must ask maturin to build a wheel of the project, only to extract the
    # native module and discard the rest !

    release = "--release" if in_cibuildwheel() or force_maturin_release() else ""
    with tempfile.TemporaryDirectory() as distdir:
        run(f"maturin build {release} --interpreter {PYTHON_EXECUTABLE_PATH} --out {distdir}")

        outputs = list(pathlib.Path(distdir).iterdir())
        if len(outputs) != 1:
            raise RuntimeError(f"Target dir unexpectedly contains multiple files: {outputs}")

        wheel_path = outputs[0]
        if sys.platform == "win32":
            in_wheel_so_path = "parsec/_parsec.cp39-win_amd64.pyd"
        elif sys.platform == "darwin":
            in_wheel_so_path = "parsec/_parsec.cpython-39-darwin.so"
        else:
            in_wheel_so_path = "parsec/_parsec.cpython-39-x86_64-linux-gnu.so"

        display(f"extracting {wheel_path}:{in_wheel_so_path} -> {BASEDIR / in_wheel_so_path}")
        with zipfile.ZipFile(wheel_path) as wheel:
            wheel.extract(in_wheel_so_path, path=BASEDIR)


if __name__ == "__main__":
    display("Launching poetry build.py script")
    build()
