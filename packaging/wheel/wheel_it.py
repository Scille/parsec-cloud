# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import argparse
from pathlib import Path
import subprocess
import shutil
import re
# import sys


# Fully-qualified path for the executable should be used with subprocess to
# avoid unreliability (especially when running from within a virtualenv)
python = shutil.which("python")
# python = sys.executable
poetry = shutil.which("poetry")


def display(line: str):
    YELLOW_FG = "\x1b[33m"
    DEFAULT_FG = "\x1b[39m"

    # Flush is required to prevent mixing with the output of sub-command with the output of the script
    print(f"{YELLOW_FG}{line}{DEFAULT_FG}", flush=True)


def run(cmd, **kwargs):
    display(f">>> {cmd}")
    ret = subprocess.run(cmd.split(), check=True, **kwargs)
    return ret


if not python or not poetry:
    raise SystemExit("Cannot find python and/or poetry commands :(")
else:
    display(f"Using python: {python}")
    display(f"Using poetry: {poetry}")
    display(f"Poetry using python version:")
    run(f"{poetry} run python --version")


def main(src_dir: Path, output_dir: Path):
    output_dir.mkdir(exist_ok=True)

    core_requirements = output_dir / "core-requirements.txt"
    backend_requirements = output_dir / "backend-requirements.txt"
    all_requirements = output_dir / "all-requirements.txt"
    constraints = output_dir / "constraints.txt"

    # poetry export has a --output option, but it alway consider the file relative to
    # the project directory !
    # On top of that we cannot use stdout because poetry may print random `Creating virtualenv`
    # if we are not already within a virtualenv (please poetry, add a --no-venv option !!!)
    run(
        f"{poetry} export --no-interaction --extras core --format requirements.txt --output wheel_it-core-requirements.txt",
        cwd=src_dir,
    )
    shutil.move(src_dir / "wheel_it-core-requirements.txt", core_requirements)
    run(
        f"{poetry} export --no-interaction --extras backend --format requirements.txt --output wheel_it-backend-requirements.txt",
        cwd=src_dir,
    )
    shutil.move(src_dir / "wheel_it-backend-requirements.txt", backend_requirements)
    run(
        f"{poetry} export --no-interaction --with dev --extras core --extras backend --format requirements.txt --output wheel_it-dev-requirements.txt",
        cwd=src_dir,
    )
    shutil.move(src_dir / "wheel_it-dev-requirements.txt", all_requirements)

    # Unlike requirements, constraint file cannot have extras
    # See https://github.com/pypa/pip/issues/8210
    constraints_data = []
    for line in all_requirements.read_text(encoding="utf8").splitlines():
        constraints_data.append(re.sub(r"\[.*\]", "", line))
    constraints.write_text("\n".join(constraints_data), encoding="utf8")

    # Make sure the dependencies needed to run generate_pyqt.py are in place
    # TODO: `--use-deprecated=legacy-resolver` is needed due to a bug in pip
    # see: https://github.com/pypa/pip/issues/9644#issuecomment-813432613
    run(
        f"{python} -m pip install pyqt5 babel docutils --constraint {constraints} --use-deprecated=legacy-resolver"
    )

    # Make sure PyQT resources are generated otherwise we will end up with
    # a .whl with missing parts !
    run(f"{poetry} run python {src_dir / 'misc/generate_pyqt.py'}")

    # Make sure we build the rust lib to be included in parsec
    run(f"{poetry} run maturin develop --release")
    display(f"{list(src_dir.glob('parsec/_parsec*'))}")
    # Finally generate the wheel, note we don't use Poetry for the job given:
    # - It is not possible to choose the output directory
    # - And more importantly, Poetry is not PEP517 compliant and build wheel
    #   without building binary resources (it basically only zip the source code)
    run(f"{python} -m pip wheel {src_dir} --wheel-dir {output_dir} --use-pep517 --no-deps")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Build .whl file for parsec and export requirements & constraints"
    )
    parser.add_argument("src_dir", type=Path)
    parser.add_argument("--output-dir", type=Path, required=True)

    args = parser.parse_args()
    main(src_dir=args.src_dir, output_dir=args.output_dir)
