# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import os
import sys
import logging
import subprocess

log = logging.getLogger("build")

YELLOW_FG = "\x1b[33m"
DEFAULT_FG = "\x1b[39m"

logging.basicConfig(
    level=logging.DEBUG, format=f"{YELLOW_FG}[%(levelname)s] %(message)s{DEFAULT_FG}"
)


def run(cmd, **kwargs):
    log.debug(f">>> {cmd}")
    ret = subprocess.run(cmd, shell=True, check=True, **kwargs)
    return ret


def in_cibuildwheel():
    return bool(int(os.environ.get("CIBUILDWHEEL", "0")))


def check_venv():
    # Both conda and venv are configured, pick conda
    if os.environ.get("CONDA_PREFIX") and os.environ.get("VIRTUAL_ENV"):
        del os.environ["VIRTUAL_ENV"]
    # Linux build with cibuildwheel detected, set `VIRTUAL_ENV`
    if sys.platform == "linux" and in_cibuildwheel():
        proc = subprocess.run(
            "which python", shell=True, check=True, capture_output=True, text=True
        )
        os.environ["VIRTUAL_ENV"] = os.path.dirname(os.path.dirname(proc.stdout.strip()))
        log.debug(
            f"Linux build with cibuildwheel detected, set `VIRATUAL_ENV` as: {os.environ['VIRTUAL_ENV']}"
        )
    if not os.environ.get("CONDA_PREFIX") and not os.environ.get("VIRTUAL_ENV"):
        raise RuntimeError("A virtual env is required to run `maturin develop`")


def build():
    run("python --version")
    run("pip freeze")
    run("python misc/generate_pyqt.py")
    check_venv()
    release = "--release" if in_cibuildwheel() else ""
    run(f"maturin develop {release}")
    # An alternative to `maturin develop` is:
    # run("maturin build -r")
    # run("unzip -o target/wheels/parsec-*.whl parsec/_parsec.abi3.so")
    # Note that it does not require a virtualenv, as opposed to `develop`


if __name__ == "__main__":
    log.debug("Launching poetry build.py script")
    build()
