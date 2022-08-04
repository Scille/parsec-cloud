# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import os
import sys
import logging
import subprocess

YELLOW_FG = "\x1b[33m"
DEFAULT_FG = "\x1b[39m"

log = logging.getLogger("build")
logging.basicConfig(
    level=logging.DEBUG, format=f"{YELLOW_FG}[%(levelname)s] %(message)s{DEFAULT_FG}"
)

PYTHON_PATH = sys.executable
log.debug(f"PYTHON_PATH={PYTHON_PATH}")


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
    # Build with cibuildwheel detected, set `VIRTUAL_ENV`
    if in_cibuildwheel():
        os.environ["VIRTUAL_ENV"] = os.path.dirname(os.path.dirname(PYTHON_PATH))
        log.debug(
            f"Build with cibuildwheel detected, set `VIRTUAL_ENV` as: {os.environ['VIRTUAL_ENV']}"
        )
    if not os.environ.get("CONDA_PREFIX") and not os.environ.get("VIRTUAL_ENV"):
        raise RuntimeError("A virtual env is required to run `maturin develop`")


def build():
    # Debug
    log.debug("Environment:")
    for key, value in os.environ.items():
        log.debug(f"- {key} = {value}")
    run(f'{PYTHON_PATH} -c "import sys; print(sys.executable)"')
    run(f'python -c "import sys; print(sys.executable)"')

    import pathlib

    for x in pathlib.Path(os.environ["PYTHONPATH"]).iterdir():
        log.debug(x)

    run(f'{PYTHON_PATH} -c "import maturin; print(maturin)"')
    run(f"{PYTHON_PATH} -m ensurepip -vvv")
    run(f'{PYTHON_PATH} -c "import pip; print(pip)"')

    run(f"{PYTHON_PATH} --version")
    run(f"{PYTHON_PATH} -m pip -V")
    run(f"{PYTHON_PATH} -m pip freeze")
    run(f"{PYTHON_PATH} misc/generate_pyqt.py")
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
