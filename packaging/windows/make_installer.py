# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import json
import argparse
import itertools
import shutil
import subprocess
from shutil import which
from pathlib import Path


SIGNATURE_AUTHOR = "Scille"
SIGNATURE_DESCRIPTION = f"Parsec by {SIGNATURE_AUTHOR}"
APPLICATION_EXE = "parsec.exe"

BASE_DIR = Path(__name__).resolve().parent
DEFAULT_BUILD_DIR = BASE_DIR / "build"

YELLOW_FG = "\x1b[33m"
DEFAULT_FG = "\x1b[39m"


def run(cmd, **kwargs):
    print(f"{YELLOW_FG}>>> {cmd}{DEFAULT_FG}", flush=True)
    ret = subprocess.run(cmd, shell=True, **kwargs)
    ret.check_returncode()
    return ret


def is_signed(target):
    ret = subprocess.run(["signtool", "verify", "/pa", str(target)], capture_output=True)
    return ret.returncode == 0


def sign(target):
    run(
        f'signtool sign /n "{SIGNATURE_AUTHOR}" /t http://time.certum.pl /fd sha256 /d "{SIGNATURE_DESCRIPTION}" /v {target}'
    )


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Build&sign Parsec installer")
    parser.add_argument(
        "--sign-mode", choices=("all", "exe", "none"), default="none", type=lambda x: x.lower()
    )
    parser.add_argument("--build-dir", type=Path, default=DEFAULT_BUILD_DIR)
    parser.add_argument("--installer-inputs", type=Path, default=None)
    args = parser.parse_args()
    build_dir: Path = args.build_dir
    if args.installer_inputs:
        installer_inputs: Path = args.installer_inputs
    else:
        installer_inputs: Path = build_dir / "installer_inputs"

    assert which("makensis"), "makensis command not in PATH !"

    # Configure the NSIS script
    nsis_config_in_path = BASE_DIR / "installer.in.nsi"
    nsis_config_in = nsis_config_in_path.read_text(encoding="utf8")
    build_manifest = json.loads((installer_inputs / "manifest.json").read_text(encoding="utf8"))
    build_manifest["output_dir_path"] = build_dir
    build_manifest["installer_inputs_path"] = installer_inputs
    nsis_config_path = build_dir / "installer.nsi"
    nsis_config = nsis_config_in.format(**build_manifest)
    nsis_config_path.write_text(nsis_config, encoding="utf8")
    assert nsis_config_path.read_bytes() != bytes(nsis_config_in, 'utf-8')  # Sanity check

    # Copy NSIS resources
    for res in ("icon.ico", "installer-side.bmp", "installer-top.bmp"):
        shutil.copyfile(src=BASE_DIR / res, dst=build_dir / res)

    if args.sign_mode == "none":
        print("### Building installer ###")
        run(f"makensis { BASE_DIR / 'installer.nsi' }")
        print(
            f"{YELLOW_FG}/!\\{DEFAULT_FG} Installer generated with no signature {YELLOW_FG}/!\\{DEFAULT_FG}"
        )
        (installer,) = installer_inputs.glob("parsec-*-setup.exe")
        print(f"{installer} is ready")

    else:
        assert which("signtool"), "signtool command not in PATH !"

        installer_content = installer_inputs / "installer_content"
        # Retrieve frozen program and sign all .dll and .exe
        print("### Signing application executable ###")
        sign(installer_content / APPLICATION_EXE)
        # Make sure everything is signed
        if args.sign_mode == "all":
            print("### Checking all shipped exe/dll are signed ###")
            not_signed = []
            for file in itertools.chain(
                installer_content.rglob("*.exe"), installer_content.rglob("*.dll")
            ):
                if not is_signed(file):
                    not_signed.append(file)
                    print("Unsigned file detected:", file)
            if not_signed:
                raise SystemExit("Some file are not signed, aborting")
        # Generate installer
        print("### Building installer ###")
        run(f"makensis { nsis_config_path }")
        # Sign installer
        print("### Signing installer ###")
        (installer,) = build_dir.glob("parsec-*-setup.exe")
        sign(installer)
        print(f"{installer} is ready")
