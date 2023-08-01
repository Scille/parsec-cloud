# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import argparse
import re
import shutil
import subprocess
from pathlib import Path
from typing import Any

# Fully-qualified path for the executable should be used with subprocess to
# avoid unreliability (especially when running from within a virtualenv)
python = shutil.which("python")
poetry = shutil.which("poetry")
if not python or not poetry:
    raise SystemExit("Cannot find python and/or poetry commands :(")
else:
    print(f"Using python: {python}")
    print(f"Using poetry: {poetry}")


def run(cmd: str, **kwargs: Any) -> subprocess.CompletedProcess[bytes]:
    print(f">>> {cmd}")
    ret = subprocess.run(cmd.split(), check=True, **kwargs)
    return ret


def main(program_source: Path, output_dir: Path, skip_wheel: bool = False) -> None:
    output_dir.mkdir(exist_ok=True)

    requirements = output_dir / "requirements.txt"
    dev_requirements = output_dir / "dev-requirements.txt"
    constraints = output_dir / "constraints.txt"

    # poetry export has a --output option, but it always consider the file relative to
    # the project directory !
    # On top of that we cannot use stdout because poetry may print random `Creating virtualenv`
    # if we are not already within a virtualenv (please poetry, add a --no-venv option !!!)
    run(
        f"{poetry} export --no-interaction --format requirements.txt --output wheel_it-requirements.txt",
        cwd=program_source,
    )
    shutil.move(program_source / "wheel_it-requirements.txt", requirements)
    run(
        f"{poetry} export --no-interaction --with dev --format requirements.txt --output wheel_it-dev-requirements.txt",
        cwd=program_source,
    )
    shutil.move(program_source / "wheel_it-dev-requirements.txt", dev_requirements)

    # Unlike requirements, constraint file cannot have extras
    # See https://github.com/pypa/pip/issues/8210
    constraints_data = []
    for line in dev_requirements.read_text(encoding="utf8").splitlines():
        constraints_data.append(re.sub(r"\[.*\]", "", line))
    constraints.write_text("\n".join(constraints_data), encoding="utf8")

    if not skip_wheel:
        # Finally generate the wheel, note we don't use Poetry for the job because
        # It is not possible to choose the output directory
        run(
            f"{python} -m pip wheel {program_source} --wheel-dir {output_dir} --use-pep517 --no-deps"
        )


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Build .whl file for parsec and export requirements & constraints"
    )
    parser.add_argument("program_source", type=Path)
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument("--skip-wheel", action="store_true", help="Skip build wheel")

    args = parser.parse_args()
    main(program_source=args.program_source, output_dir=args.output_dir, skip_wheel=args.skip_wheel)
