# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import argparse
from pathlib import Path
import subprocess
import shutil
import re


# Fully-qualified path for the executable should be used with subprocess to
# avoid unreliability (especially when running from within a virtualenv)
python = shutil.which("python")
poetry = shutil.which("poetry")
if not python or not poetry:
    raise SystemExit("Cannot find python and/or poetry commands :(")
else:
    print(f"Using python: {python}")
    print(f"Using poetry: {poetry}")


def run(cmd, **kwargs):
    print(f">>> {cmd}")
    ret = subprocess.run(cmd.split(), check=True, **kwargs)
    return ret


def main(program_source: Path, output_dir: Path):
    output_dir.mkdir(exist_ok=True)

    core_requirements = output_dir / "core-requirements.txt"
    backend_requirements = output_dir / "backend-requirements.txt"
    all_requirements = output_dir / "all-requirements.txt"
    constraints = output_dir / "constraints.txt"

    # LICENSE is needed by `freeze_program.py`
    shutil.copyfile(src=program_source / "licenses/AGPL3.txt", dst=output_dir / "LICENSE.txt")

    # poetry export has a --output option, but it alway consider the file relative to
    # the project directory !
    # On top of that we cannot use stdout because poetry may print random `Creating virtualenv`
    # if we are not already within a virtualenv (please poetry, add a --no-venv option !!!)
    run(
        f"{poetry} export --no-interaction --extras core --format requirements.txt --output wheel_it-core-requirements.txt",
        cwd=program_source,
    )
    shutil.move(program_source / "wheel_it-core-requirements.txt", core_requirements)
    run(
        f"{poetry} export --no-interaction --extras backend --format requirements.txt --output wheel_it-backend-requirements.txt",
        cwd=program_source,
    )
    shutil.move(program_source / "wheel_it-backend-requirements.txt", backend_requirements)
    run(
        f"{poetry} export --no-interaction --with dev --extras core --extras backend --format requirements.txt --output wheel_it-dev-requirements.txt",
        cwd=program_source,
    )
    shutil.move(program_source / "wheel_it-dev-requirements.txt", all_requirements)

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
    run(f"{python} {program_source / 'misc/generate_pyqt.py'}")

    # Finally generate the wheel, note we don't use Poetry for the job given:
    # - It is not possible to choose the output directory
    # - And more importantly, Poetry is not PEP517 compliant and build wheel
    #   without building binary resources (it basically only zip the source code)
    run(f"{python} -m pip wheel {program_source} --wheel-dir {output_dir} --use-pep517 --no-deps")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Build .whl file for parsec and export requirements & constraints"
    )
    parser.add_argument("program_source", type=Path)
    parser.add_argument("--output-dir", type=Path, required=True)

    args = parser.parse_args()
    main(program_source=args.program_source, output_dir=args.output_dir)
