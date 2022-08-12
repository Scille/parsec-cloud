# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import os
import sys
import subprocess


def display(line: str):
    YELLOW_FG = "\x1b[33m"
    DEFAULT_FG = "\x1b[39m"

    # Flush is required to prevent mixing with the output of sub-command with the output of the script
    print(f"{YELLOW_FG}{line}{DEFAULT_FG}", flush=True)


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
    release = "--release" if in_cibuildwheel() or force_maturin_release() else ""
    run(f"maturin build {release}")
    if sys.platform == "win32":
        run("unzip -o target/wheels/parsec-*.whl parsec/_parsec.pyd")
    else:
        run("unzip -o target/wheels/parsec-*.whl parsec/_parsec.abi3.so")


if __name__ == "__main__":
    display("Launching poetry build.py script")
    build()
