# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import os
import sys
import re
from typing import Optional
import shutil
import argparse
import platform
import subprocess
from hashlib import sha256
from urllib.request import urlopen
from pathlib import Path


# Fully-qualified path for the executable should be used with subprocess to
# avoid unreliability (especially when running from within a virtualenv)
python = sys.executable
if not python:
    raise SystemExit("Cannot find python command :(")


BUILD_DIR = Path("build").resolve()

WINFSP_URL = "https://github.com/billziss-gh/winfsp/releases/download/v1.8/winfsp-1.8.20304.msi"
WINFSP_HASH = "8d6f2c519f3f064881b576452fbbd35fe7ad96445aa15d9adcea1e76878b4f00"
TOOLS_VENV_DIR = BUILD_DIR / "tools_venv"
DEFAULT_WHEEL_IT_DIR = BUILD_DIR / "wheel_it"

PYTHON_VERSION = f"{sys.version_info.major}.{sys.version_info.minor}.{sys.version_info.micro}"


def display(line: str):
    YELLOW_FG = "\x1b[33m"
    DEFAULT_FG = "\x1b[39m"

    # Flush is required to prevent mixing with the output of sub-command with the output of the script
    print(f"{YELLOW_FG}{line}{DEFAULT_FG}", flush=True)


def get_archslug():
    bits, _ = platform.architecture()
    return "win32" if bits == "32bit" else "win64"


def run(cmd, **kwargs):
    if isinstance(cmd, str):
        cmd = cmd.split()
    display(f">>> {' '.join(map(str, cmd))}")
    ret = subprocess.run(cmd, check=True, **kwargs)
    return ret


def main(
    src_dir: Path, include_parsec_ext: Optional[Path] = None, wheel_it_dir: Optional[Path] = None
):
    BUILD_DIR.mkdir(parents=True, exist_ok=True)

    display(f"Building in {BUILD_DIR}")
    display(f"Using sources at {src_dir}")

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
        display(f"Parsec wheel is: {program_wheel}")
        program_version = f"v{match.group(1)}"
    else:
        exec((src_dir / "parsec/_version.py").read_text(), global_dict)
        program_version = global_dict.get("__version__")
    assert program_version.startswith("v")
    program_version_without_v_prefix = program_version[1:]
    display(f"### Detected Parsec version {program_version} ###")

    winfsp_installer = BUILD_DIR / WINFSP_URL.rsplit("/", 1)[1]
    if not winfsp_installer.is_file():
        display("### Fetching WinFSP installer (will be needed by NSIS packager later) ###")
        req = urlopen(WINFSP_URL)
        data = req.read()
        assert sha256(data).hexdigest() == WINFSP_HASH
        winfsp_installer.write_bytes(data)

    # Bootstrap tools virtualenv
    tools_python = TOOLS_VENV_DIR / "Scripts/python.exe"
    if not TOOLS_VENV_DIR.is_dir():
        display("### Create tool virtualenv ###")
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
            display("### Generate program wheel and constraints on dependencies ###")
            run(
                f"{ tools_python } { src_dir / 'packaging/wheel/wheel_it.py' } { src_dir } --output-dir { DEFAULT_WHEEL_IT_DIR }"
            )
            program_wheel = next(
                DEFAULT_WHEEL_IT_DIR.glob(f"parsec_cloud-{program_version_without_v_prefix}*.whl")
            )

    # Bootstrap PyInstaller virtualenv containing both pyinstaller, parsec & it dependencies
    pyinstaller_venv_dir = BUILD_DIR / "pyinstaller_venv"
    pyinstaller_python = pyinstaller_venv_dir / "Scripts/python.exe"
    if not pyinstaller_venv_dir.is_dir() or True:
        display("### Installing program & PyInstaller in temporary virtualenv ###")
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
        display("### Use Pyinstaller to generate distribution ###")
        spec_file = Path(__file__).joinpath("..", "pyinstaller.spec").resolve()
        env = dict(os.environ)
        if include_parsec_ext is not None:
            env["INCLUDE_PARSEC_EXT"] = "1"
        run(
            f"{ pyinstaller_python } -m PyInstaller {spec_file} --distpath {pyinstaller_dist} --workpath {pyinstaller_build}",
            env=env,
        )

    target_dir = BUILD_DIR / f"parsec-{program_version}-{get_archslug()}"
    if target_dir.exists():
        raise SystemExit(f"{target_dir} already exists, exiting...")
    shutil.move(pyinstaller_dist / "parsec", target_dir)

    # Include LICENSE file
    (target_dir / "LICENSE.txt").write_text((src_dir / "licenses/AGPL3.txt").read_text())

    # Create build info file for NSIS installer
    (BUILD_DIR / "BUILD.tmp").write_text(
        f'target = "{target_dir}"\n'
        f'program_version = "{program_version}"\n'
        f'python_version = "{PYTHON_VERSION}"\n'
        f'platform = "{get_archslug()}"\n'
        f'winfsp_installer_name = "{winfsp_installer.name}"\n'
        f'winfsp_installer_path = "{winfsp_installer}"\n'
    )

    # Create the install and uninstall file list for NSIS installer
    # Crawling order is important in [install|uninstall]_files.nsh, we cannot
    # create a file if it parent folder doesn't exist and we cannot remove a
    # folder if it is not empty.
    # On top of that, we have to jump into a directory before installing it
    # files (see the `SetOutPath` command).
    # We don't use `Path.rglob` given it would require further tweaking
    # due to it crawling order, so it's just simpler to roll our own crawler.
    target_files = []

    def _recursive_collect_target_files(curr_dir):
        subdirs = []
        for entry in curr_dir.iterdir():
            if entry.is_dir():
                subdirs.append(entry)
            else:
                target_files.append((False, entry.relative_to(target_dir)))
        for subdir in subdirs:
            target_files.append((True, subdir.relative_to(target_dir)))
            _recursive_collect_target_files(subdir)

    _recursive_collect_target_files(target_dir)

    install_files_txt = '; Files to install\nSetOutPath "$INSTDIR\\"\n'
    installed_check = {Path(".")}
    for target_is_dir, target_file in target_files:
        if target_is_dir:
            # Jump into the folder (create it if needed)
            install_files_txt += f'SetOutPath "$INSTDIR\\{target_file}"\n'
        else:
            # Copy the file in the current folder
            install_files_txt += f'File "${{PROGRAM_FREEZE_BUILD_DIR}}\\{target_file}"\n'
        # Installation simulation sanity check
        assert target_file not in installed_check
        assert target_file.parent in installed_check
        installed_check.add(target_file)
    (BUILD_DIR / "install_files.nsh").write_text(install_files_txt)

    uninstall_files_txt = "; Files to uninstall\n"
    for target_is_dir, target_file in reversed(target_files):
        if target_is_dir:
            uninstall_files_txt += f'RMDir "$INSTDIR\\{target_file}"\n'
        else:
            uninstall_files_txt += f'Delete "$INSTDIR\\{target_file}"\n'
        # Uninstallation simulation sanity check
        assert target_file.parent in installed_check
        installed_check.remove(target_file)
    (BUILD_DIR / "uninstall_files.nsh").write_text(uninstall_files_txt)


def check_python_version():
    if PYTHON_VERSION == "3.7.7":
        raise RuntimeError(
            "CPython 3.7.7 is broken for packaging (see https://bugs.python.org/issue39930)"
        )


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Freeze Parsec")
    parser.add_argument("src_dir", type=Path)
    parser.add_argument("--disable-check-python", action="store_true")
    parser.add_argument("--include-parsec-ext", type=Path)
    parser.add_argument("--wheel-it-dir", type=Path)
    args = parser.parse_args()
    if not args.disable_check_python:
        check_python_version()
    main(
        src_dir=args.src_dir,
        include_parsec_ext=args.include_parsec_ext,
        wheel_it_dir=args.wheel_it_dir,
    )
