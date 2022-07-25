# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import os
import sys
import re
from typing import Optional
import shutil
import argparse
import subprocess
from hashlib import sha256
from pathlib import Path


# Fully-qualified path for the executable should be used with subprocess to
# avoid unreliability (especially when running from within a virtualenv)
python = sys.executable
if not python:
    raise SystemExit("Cannot find python command :(")


BUILD_DIR = Path("build").resolve()

TOOLS_VENV_DIR = BUILD_DIR / "tools_venv"
DEFAULT_WHEEL_IT_DIR = BUILD_DIR / "wheel_it"

PYTHON_VERSION = f"{sys.version_info.major}.{sys.version_info.minor}.{sys.version_info.micro}"


def run(cmd, **kwargs):
    if isinstance(cmd, str):
        cmd = cmd.split()
    print(f">>> {' '.join(map(str, cmd))}")
    ret = subprocess.run(cmd, check=True, **kwargs)
    return ret


def main(
    program_source: Path,
    include_parsec_ext: Optional[Path] = None,
    wheel_it_dir: Optional[Path] = None,
):
    BUILD_DIR.mkdir(parents=True, exist_ok=True)

    # Retrieve program version
    global_dict = {}
    if wheel_it_dir is not None:
        candidates = list(wheel_it_dir.glob(f"parsec_cloud-*.whl"))
        if len(candidates) == 0:
            raise SystemExit(f"Parsec wheel not present in {wheel_it_dir}")
        elif len(candidates) > 1:
            raise SystemExit(f"Multiple possible Parsec wheels: {candidates}")
        program_wheel = candidates[0]
        match = re.match(r"parsec_cloud-([0-9][0-9\.+a-z]+)-.*.whl", program_wheel.name)
        if not match:
            raise SystemError(
                f"Found Parsec wheel {program_wheel} but could not determine it version"
            )
        print(f"Parsec wheel is: {program_wheel}")
        program_version = f"v{match.group(1)}"
    else:
        exec((program_source / "parsec/_version.py").read_text(), global_dict)
        program_version = global_dict.get("__version__")
    assert program_version.startswith("v")
    program_version_without_v_prefix = program_version[1:]
    print(f"### Detected Parsec version {program_version} ###")

    # Bootstrap tools virtualenv
    tools_python = TOOLS_VENV_DIR / "bin/python"
    if not TOOLS_VENV_DIR.is_dir():
        print("### Create tool virtualenv ###")
        run(f"{ python } -m venv {TOOLS_VENV_DIR}")
        # Must use poetry>=1.2.0b2 given otherwise `poetry export` produce buggy output
        run(f"{ tools_python } -m pip install pip wheel setuptools poetry>=1.2.0b2 --upgrade")

    # Make poetry commands accessible in sub processes
    os.environ["PATH"] = f"{(TOOLS_VENV_DIR / 'Scripts').absolute()};{os.environ['PATH']}"

    # Generate program wheel and constraints on dependencies
    if wheel_it_dir is not None:
        program_wheel = next(
            wheel_it_dir.glob(f"parsec_cloud-{program_version_without_v_prefix}*.whl"), None
        )
        program_constraints = wheel_it_dir / "constraints.txt"
        if not program_wheel or not program_constraints.exists():
            raise SystemExit(f"Cannot retreive wheel file and/or constraints.txt in {wheel_it_dir}")

    else:
        program_wheel = next(
            DEFAULT_WHEEL_IT_DIR.glob(f"parsec_cloud-{program_version_without_v_prefix}*.whl"), None
        )
        program_constraints = DEFAULT_WHEEL_IT_DIR / "constraints.txt"
        if not program_wheel or not program_constraints.exists():
            print("### Generate program wheel and constraints on dependencies ###")
            run(
                f"{ tools_python } { program_source / 'packaging/wheel/wheel_it.py' } { program_source } --output-dir { DEFAULT_WHEEL_IT_DIR }"
            )
            program_wheel = next(
                DEFAULT_WHEEL_IT_DIR.glob(f"parsec_cloud-{program_version_without_v_prefix}*.whl")
            )

    # Bootstrap PyInstaller virtualenv containing both pyinstaller, parsec & it dependencies
    pyinstaller_venv_dir = BUILD_DIR / "pyinstaller_venv"
    pyinstaller_python = pyinstaller_venv_dir / "bin/python"
    if not pyinstaller_venv_dir.is_dir() or True:
        print("### Installing program & PyInstaller in temporary virtualenv ###")
        run(f"{ python } -m venv {pyinstaller_venv_dir}")
        run(f"{ pyinstaller_python } -m pip install pip wheel --upgrade")
        # First install PyInstaller, note it version & dependencies are pinned in the constraints file
        # TODO: `--use-deprecated=legacy-resolver` is needed due to a bug in pip
        # see: https://github.com/pypa/pip/issues/9243
        run(
            f"{ pyinstaller_python } -m pip install pyinstaller --constraint { program_constraints } --use-deprecated=legacy-resolver"
        )
        # Pip is very picky when installing with hashes, hence it requires to have
        # all the wheels with hash (so including our program wheel we've just generated).
        # So we have to manually compute the hash and create a custom requirements.txt file
        # with it
        pyinstaller_venv_requirements = BUILD_DIR / "pyinstaller_venv_requirements.txt"
        program_wheel_hash = sha256(program_wheel.read_bytes()).hexdigest()
        pyinstaller_venv_requirements_data = (
            f"{ program_wheel.absolute() }[core]; --hash=sha256:{program_wheel_hash}\n"
        )
        if include_parsec_ext is not None:
            pyinstaller_venv_requirements_data += f"{include_parsec_ext}\n"
        pyinstaller_venv_requirements.write_text(pyinstaller_venv_requirements_data)
        # Now do the actual install, note the constraint file that ensures
        # parsec_ext won't mess with parsec dependencies
        # TODO: `--use-deprecated=legacy-resolver` is needed due to a bug in pip
        # see: https://github.com/pypa/pip/issues/9644#issuecomment-813432613
        run(
            f"{ pyinstaller_python } -m pip install --requirement { pyinstaller_venv_requirements } --constraint { program_constraints }  --use-deprecated=legacy-resolver"
        )

    pyinstaller_build = BUILD_DIR / "pyinstaller_build"
    pyinstaller_dist = BUILD_DIR / "pyinstaller_dist"
    if not pyinstaller_dist.is_dir():
        print("### Use Pyinstaller to generate distribution ###")
        spec_file = Path(__file__).joinpath("..", "pyinstaller.spec").resolve()
        env = dict(os.environ)
        if include_parsec_ext is not None:
            env["INCLUDE_PARSEC_EXT"] = "1"
        run(
            f"{ pyinstaller_python } -m PyInstaller {spec_file} --distpath {pyinstaller_dist} --workpath {pyinstaller_build}",
            env=env,
        )

    # Include LICENSE file
    (pyinstaller_dist / "Parsec.app/Contents/LICENSE.txt").write_text(
        (program_source / "licenses/AGPL3.txt").read_text()
    )


def check_python_version():
    if PYTHON_VERSION == "3.7.7":
        raise RuntimeError(
            "CPython 3.7.7 is broken for packaging (see https://bugs.python.org/issue39930)"
        )


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Freeze Parsec")
    parser.add_argument("program_source", type=Path)
    parser.add_argument("--disable-check-python", action="store_true")
    parser.add_argument("--include-parsec-ext", type=Path)
    parser.add_argument("--wheel-it-dir", type=Path)
    parser.add_argument("--install", action="store_true")
    args = parser.parse_args()
    if not args.disable_check_python:
        check_python_version()
    main(
        program_source=args.program_source,
        include_parsec_ext=args.include_parsec_ext,
        wheel_it_dir=args.wheel_it_dir,
    )

    # Helper for testing
    if args.install:
        target = Path("/Applications/parsec.app")
        new_app = BUILD_DIR / "pyinstaller_dist/Parsec.app"
        try:
            print(f"Remove {target}")
            shutil.rmtree(target)
        except FileNotFoundError:
            pass
        print(f"Copy {new_app} -> {target}")
        shutil.copytree(new_app, target)
