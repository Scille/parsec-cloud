# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import re
import argparse
import itertools
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
    args = parser.parse_args()
    build_dir: Path = args.build_dir

    assert which("makensis"), "makensis command not in PATH !"

    if args.sign_mode == "none":
        print("### Building installer ###")
        run(f"makensis { BASE_DIR / 'installer.nsi' }")
        print(
            f"{YELLOW_FG}/!\\{DEFAULT_FG} Installer generated with no signature {YELLOW_FG}/!\\{DEFAULT_FG}"
        )
        (installer,) = build_dir.glob("parsec-*-setup.exe")
        print(f"{installer} is ready")

    else:
        assert which("signtool"), "signtool command not in PATH !"

        build_manifest = (build_dir / "manifest.ini").read_text()
        match = re.match(r"^target = \"(.*)\"$", build_manifest, re.MULTILINE)
        if not match:
            raise SystemExit("Build manifest not found, aborting")
        freeze_program = Path(match.group(1))
        # Retrieve frozen program and sign all .dll and .exe
        print("### Signing application executable ###")
        sign(freeze_program / APPLICATION_EXE)
        # Make sure everything is signed
        if args.sign_mode == "all":
            print("### Checking all shipped exe/dll are signed ###")
            not_signed = []
            for file in itertools.chain(
                freeze_program.rglob("*.exe"), freeze_program.rglob("*.dll")
            ):
                if not is_signed(file):
                    not_signed.append(file)
                    print("Unsigned file detected:", file)
            if not_signed:
                raise SystemExit("Some file are not signed, aborting")
        # Generate installer
        print("### Building installer ###")
        run(f"makensis { BASE_DIR / 'installer.nsi' }")
        # Sign installer
        print("### Signing installer ###")
        (installer,) = build_dir.glob("parsec-*-setup.exe")
        sign(installer)
        print(f"{installer} is ready")
