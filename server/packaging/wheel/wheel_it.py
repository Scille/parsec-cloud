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
uv = shutil.which("uv")
if not python or not uv:
    raise SystemExit("Cannot find python and/or uv commands :(")
else:
    print(f"Using python: {python}")
    print(f"Using uv: {uv}")


def run(cmd: str, **kwargs: Any) -> subprocess.CompletedProcess[bytes]:
    print(f">>> {cmd}")
    ret = subprocess.run(cmd.split(), check=True, **kwargs)
    return ret


def main(program_source: Path, output_dir: Path, skip_wheel: bool = False) -> None:
    output_dir.mkdir(exist_ok=True)

    requirements = output_dir / "requirements.txt"
    dev_requirements = output_dir / "dev-requirements.txt"
    constraints = output_dir / "constraints.txt"

    run(
        f"{uv} --project {program_source} export --quiet --locked --no-dev --format requirements.txt --output-file {requirements.absolute()}"
    )
    run(
        f"{uv} --project {program_source} export --quiet --locked --dev --format requirements.txt --output-file {dev_requirements.absolute()}"
    )

    # Unlike requirements, constraint file cannot have extras
    # See https://github.com/pypa/pip/issues/8210
    constraints_data = [
        re.sub(r"\[.*\]", "", line)
        for line in dev_requirements.read_text(encoding="utf8").splitlines()
    ]
    constraints.write_text("\n".join(constraints_data), encoding="utf8")

    if not skip_wheel:
        run(
            f"uv --directory {program_source} build --locked --wheel --out-dir {output_dir.absolute()} --force-pep517 --verbose"
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
