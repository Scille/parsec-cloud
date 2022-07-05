# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import os
import sys
import shutil
import argparse
import platform
import subprocess
from hashlib import sha256
from urllib.request import urlopen
from pathlib import Path


BUILD_DIR = Path("build").resolve()

WINFSP_URL = "https://github.com/billziss-gh/winfsp/releases/download/v1.8/winfsp-1.8.20304.msi"
WINFSP_HASH = "8d6f2c519f3f064881b576452fbbd35fe7ad96445aa15d9adcea1e76878b4f00"
TOOLS_VENV_DIR = BUILD_DIR / "tools_venv"
WHEELS_DIR = BUILD_DIR / "wheels"

PYTHON_VERSION = f"{sys.version_info.major}.{sys.version_info.minor}.{sys.version_info.micro}"


def get_archslug():
    bits, _ = platform.architecture()
    return "win32" if bits == "32bit" else "win64"


def run(cmd, **kwargs):
    print(f">>> {cmd}")
    ret = subprocess.run(cmd, shell=True, **kwargs)
    assert ret.returncode == 0, ret.returncode
    return ret


def main(program_source, include_parsec_ext=None):
    BUILD_DIR.mkdir(parents=True, exist_ok=True)

    # Retrieve program version
    global_dict = {}
    exec((program_source / "parsec/_version.py").read_text(), global_dict)
    program_version = global_dict.get("__version__")
    print(f"### Detected Parsec version {program_version} ###")

    winfsp_installer = BUILD_DIR / WINFSP_URL.rsplit("/", 1)[1]
    if not winfsp_installer.is_file():
        print("### Fetching WinFSP installer (will be needed by NSIS packager later) ###")
        req = urlopen(WINFSP_URL)
        data = req.read()
        assert sha256(data).hexdigest() == WINFSP_HASH
        winfsp_installer.write_bytes(data)

    # Bootstrap tools virtualenv
    if not TOOLS_VENV_DIR.is_dir():
        print("### Create tool virtualenv ###")
        run(f"python -m venv {TOOLS_VENV_DIR}")
        run(f"{ TOOLS_VENV_DIR / 'Scripts/python' } -m pip install pip wheel setuptools --upgrade")

    if not WHEELS_DIR.is_dir():
        print("### Generate wheels from Parsec, Parsec and dependencies ###")
        # Generate wheels for parsec (with it core extra) and it dependencies
        # Also generate wheels for PyInstaller in the same command so that
        # dependency resolution is done together with parsec.
        packages = [f"{program_source.absolute()}[core]", "pyinstaller"]
        if include_parsec_ext is not None:
            packages.append(f"{include_parsec_ext.absolute()}")
        run(
            f"{ TOOLS_VENV_DIR / 'Scripts/python' } -m pip wheel --wheel-dir {WHEELS_DIR} {' '.join(packages)}"
        )

    # Bootstrap PyInstaller virtualenv
    pyinstaller_venv_dir = BUILD_DIR / "pyinstaller_venv"
    if not pyinstaller_venv_dir.is_dir():
        print("### Installing wheels & PyInstaller in temporary virtualenv ###")
        run(f"python -m venv {pyinstaller_venv_dir}")
        run(f"{ pyinstaller_venv_dir / 'Scripts/python' } -m pip install pip --upgrade")
        packages = ["parsec-cloud[core]", "pyinstaller"]
        if include_parsec_ext is not None:
            packages.append("parsec-ext")
        run(
            f"{ pyinstaller_venv_dir / 'Scripts/python' } -m pip install --no-index --find-links {WHEELS_DIR} {' '.join(packages)}"
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
            f"{ pyinstaller_venv_dir / 'Scripts/python' } -m PyInstaller {spec_file} --distpath {pyinstaller_dist} --workpath {pyinstaller_build}",
            env=env,
        )

    target_dir = BUILD_DIR / f"parsec-{program_version}-{get_archslug()}"
    if target_dir.exists():
        raise SystemExit(f"{target_dir} already exists, exiting...")
    shutil.move(pyinstaller_dist / "parsec", target_dir)

    # Include LICENSE file
    (target_dir / "LICENSE.txt").write_text((program_source / "licenses/AGPL3.txt").read_text())

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
    parser.add_argument("program_source")
    parser.add_argument("--disable-check-python", action="store_true")
    parser.add_argument("--include-parsec-ext", type=Path)
    args = parser.parse_args()
    if not args.disable_check_python:
        check_python_version()
    main(Path(args.program_source), include_parsec_ext=args.include_parsec_ext)
