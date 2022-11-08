#!/usr/bin/env python
# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS


import argparse
from pathlib import Path
from collections import namedtuple
from unittest.mock import patch

try:
    from PyQt5.pylupdate_main import main as pylupdate_main
    from PyQt5.pyrcc_main import processResourceFile
    from PyQt5.uic.driver import Driver
except ImportError:
    raise SystemExit("PyQt5 not installed :'(")

try:
    from babel.messages.frontend import CommandLineInterface
except ImportError:
    raise SystemExit("Babel not installed :'(")

try:
    import docutils.core
except ImportError:
    raise SystemExit("DocUtils not installed :'(")


PROJECT_DIR = Path(__file__).resolve().parent.parent


def generate_pyqt_resources_bundle():
    print(">>> Generating `parsec.core.gui._resource_rc` bundle module")

    processResourceFile(
        filenamesIn=[str(PROJECT_DIR / "parsec/core/gui/rc/resources.qrc")],
        filenameOut=str(PROJECT_DIR / "parsec/core/gui/_resources_rc.py"),
        listFiles=False,
    )


def generate_changelog():
    destination_folder = PROJECT_DIR / "parsec/core/gui/rc/generated_misc"
    target = destination_folder / "history.html"

    print(f">>> Converting HISTORY.rst to {target}")

    destination_folder.mkdir(parents=True, exist_ok=True)
    docutils.core.publish_file(
        source_path=str(PROJECT_DIR / "HISTORY.rst"),
        destination_path=str(target),
        writer_name="html",
    )


def generate_pyqt_forms():
    print(">>> Generating `parsec.core.ui.*` forms module")

    Options = namedtuple(
        "Options",
        ["output", "import_from", "debug", "preview", "execute", "indent", "resource_suffix"],
    )
    in_dir = PROJECT_DIR / "parsec/core/gui/forms"
    out_dir = PROJECT_DIR / "parsec/core/gui/ui"
    out_dir.mkdir(parents=True, exist_ok=True)
    for f in in_dir.iterdir():
        o = Options(
            output=str(out_dir / f"{f.stem}.py"),
            import_from="parsec.core.gui",
            debug=False,
            preview=False,
            execute=False,
            indent=4,
            resource_suffix="_rc",
        )
        d = Driver(o, str(f))
        d.invoke()


def extract_translations():
    print(">>> Generating ui translation files")

    gui_dir = PROJECT_DIR / "parsec/core/gui"
    tr_dir = gui_dir / "tr"
    tr_dir.mkdir(parents=True, exist_ok=True)

    new_args = ["pylupdate", str(gui_dir / "parsec-gui.pro")]
    with patch("sys.argv", new_args):
        pylupdate_main()

    files = [str(f) for f in gui_dir.iterdir() if f.is_file() and f.suffix == ".py"]
    files.sort()
    files.append(str(tr_dir / "parsec_en.ts"))
    args = [
        "_",
        "extract",
        "-k",
        "translate T",
        "-s",
        "--no-location",
        "-F",
        ".babel.cfg",
        "--omit-header",
        "-o",
        str(tr_dir / "translation.pot"),
        *files,
    ]
    CommandLineInterface().run(args)
    languages = ["fr", "en"]
    for lang in languages:
        po_file = tr_dir / f"parsec_{lang}.po"
        if not po_file.is_file():
            po_file.touch()
        args = [
            "_",
            "update",
            "-i",
            str(tr_dir / "translation.pot"),
            "-o",
            str(po_file),
            "-l",
            lang,
        ]
        CommandLineInterface().run(args)


def compile_translations():
    print(">>> Compiling ui translation files")

    in_dir = PROJECT_DIR / "parsec/core/gui/tr"
    out_dir = PROJECT_DIR / "parsec/core/gui/rc/translations"
    out_dir.mkdir(parents=True, exist_ok=True)
    languages = ["fr", "en"]
    for lang in languages:
        args = [
            "_",
            "compile",
            "-i",
            str(in_dir / f"parsec_{lang}.po"),
            "-o",
            str(out_dir / f"parsec_{lang}.mo"),
        ]
        CommandLineInterface().run(args)


def build_py_with_pyqt(build_py):
    generate_pyqt_forms()
    compile_translations()
    generate_changelog()
    generate_pyqt_resources_bundle()


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "cmd", choices=["build", "extract_translations"], default="build", nargs="?"
    )

    args = parser.parse_args()
    if args.cmd == "build":
        generate_pyqt_forms()
        compile_translations()
        generate_changelog()
        # This must be last given it packages generated resources (except .py files)
        generate_pyqt_resources_bundle()
    else:
        assert args.cmd == "extract_translations"
        extract_translations()
