# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import os
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


def is_in_venv():
    if os.environ.get("CONDA_PREFIX") or os.environ.get("VIRTUAL_ENV"):
        del os.environ["VIRTUAL_ENV"]
    return os.environ.get("CONDA_PREFIX") or os.environ.get("VIRTUAL_ENV")


def build(setup_kargs):
    log.debug(f"setup_kargs: {str(setup_kargs)}")

    run("python --version")
    run("pip freeze")
    run("python misc/generate_pyqt.py")
    if is_in_venv():
        run("maturin develop")
    else:
        run("maturin build -r")
        run("unzip -o target/wheels/parsec-*.whl parsec/_parsec.abi3.so")


if __name__ == "__main__":
    log.debug("launching build script in manual")
    build({})
