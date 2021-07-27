# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import re
import shutil
import argparse
from pathlib import Path


def main(program_source: Path, output: Path, force: bool):
    if output.exists():
        if force:
            shutil.rmtree(output)
        else:
            raise SystemExit(f"{output} already exists")
    output.mkdir(parents=True)

    # Retrieve program version
    global_dict = {}
    exec((program_source / "parsec/_version.py").read_text(), global_dict)
    program_version = global_dict.get("__version__")
    print(f"### Detected Parsec version {program_version} ###")

    # Copy sources in an isolated dir (snap doesn't support using a snapcraft.yml parent folder as source)
    src_dir = output / "src"
    print(f"### Copy Parsec sources to {src_dir} ###")

    src_dir.mkdir(parents=True, exist_ok=True)
    shutil.copytree(program_source / "parsec", src_dir / "parsec")
    shutil.copy(program_source / "README.rst", src_dir / "README.rst")
    shutil.copy(program_source / "HISTORY.rst", src_dir / "HISTORY.rst")

    # Ugly hack given snapcraft doesn't support extra requirements...
    setup_py_src = (program_source / "setup.py").read_text()
    patched_setup_py_src = setup_py_src.replace(
        "install_requires=requirements",
        "install_requires=requirements + extra_requirements['core']",
    )
    assert patched_setup_py_src != setup_py_src
    # We want to use the Qt5 provided by snap, so don't install PyQt which comes
    # with it own copy of Qt5. Instead the snap should depend on python3-pyqt5 package.
    patched2_setup_py_src = re.sub(r"PYQT_DEPS\W*=.*", r"PYQT_DEPS = []", patched_setup_py_src)
    assert patched2_setup_py_src != patched_setup_py_src
    patched3_setup_py_src = re.sub(
        r"def fix_pyqt_import\(\):", "def fix_pyqt_import():\n    return", patched2_setup_py_src
    )
    assert patched3_setup_py_src != patched2_setup_py_src
    (src_dir / "setup.py").write_text(patched3_setup_py_src)

    shutil.copytree(program_source / "packaging/snap/bin", output / "bin")

    # Copy snapcraft.yaml and set version info
    snapcraft_yaml_src = (program_source / "packaging/snap/snapcraft.yaml").read_text()
    patched_snapcraft_yaml_src = snapcraft_yaml_src.format(
        program_version=program_version, program_src=src_dir
    )
    # Fun facts about "snapcraft.yaml":
    # - it cannot be named "snapcraft.yml"
    # - it must be stored in a "snap" folder
    # - there is no "--config" in snapcraft to avoid having to build a specific
    #   folder structure and use cd before running a command...
    snapcraft_yaml = output / "snap/snapcraft.yaml"
    snapcraft_yaml.parent.mkdir(parents=True)
    snapcraft_yaml.write_text(patched_snapcraft_yaml_src)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Generate build dir for Snap with parsec sources, ready to run snapcraft on it"
    )
    parser.add_argument(
        "--program-source", type=Path, default=Path(__file__).joinpath("../../..").resolve()
    )
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--force", action="store_true")
    args = parser.parse_args()
    main(args.program_source.absolute(), args.output.absolute(), args.force)
