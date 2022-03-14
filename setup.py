#!/usr/bin/env python
# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import os
from pathlib import Path

import pkg_resources
from setuptools import setup, find_packages, distutils, Command
from setuptools.command.build_py import build_py

# Awesome hack to load `__version__`
__version__ = None
exec(open("parsec/_version.py", encoding="utf-8").read())
# Provide normalized version format (i.e. without leading "v") to avoid warning from setuptools
assert __version__.startswith("v")
__version__ = __version__[1:]


def fix_pyqt_import():
    # PyQt5-sip is a distinct pip package that provides PyQt5.sip
    # However it setuptools handles `setup_requires` by downloading the
    # dependencies in the `./.eggs` directory without really installing
    # them. This causes `import PyQt5.sip` to fail given the `PyQt5` folder
    # doesn't contains `sip.so` (or `sip.pyd` on windows)...
    import sys
    import glob
    import importlib

    for module_name, path_glob in (
        ("PyQt5", ".eggs/*PyQt5*/PyQt5/__init__.py"),
        ("PyQt5.sip", ".eggs/*PyQt5_sip*/PyQt5/sip.*"),
    ):
        # If the module has already been installed in the environment
        # setuptools won't populate the `.eggs` directory and we have
        # nothing to do
        try:
            importlib.import_module(module_name)
        except ImportError:
            pass
        else:
            continue

        for path in glob.glob(path_glob):

            spec = importlib.util.spec_from_file_location(module_name, path)
            if spec:
                module = importlib.util.module_from_spec(spec)
                sys.modules[module_name] = module
                break

        else:
            raise RuntimeError("Cannot found module `%s` in .eggs" % module_name)


class GeneratePyQtResourcesBundle(Command):
    description = "Generates `parsec.core.gui._resource_rc` bundle module"

    user_options = []

    def initialize_options(self):
        pass

    def finalize_options(self):
        pass

    def run(self):
        fix_pyqt_import()
        try:
            from PyQt5.pyrcc_main import processResourceFile

            self.announce("Generating `parsec.core.gui._resources_rc`", level=distutils.log.INFO)
            processResourceFile(
                ["parsec/core/gui/rc/resources.qrc"], "parsec/core/gui/_resources_rc.py", False
            )
        except ImportError:
            print("PyQt5 not installed, skipping `parsec.core.gui._resources_rc` generation.")


class GenerateChangelog(Command):
    description = "Convert HISTORY.rst to HTML"

    user_options = []

    def initialize_options(self):
        pass

    def finalize_options(self):
        pass

    def run(self):
        import docutils.core

        destination_folder = "parsec/core/gui/rc/generated_misc"
        self.announce(
            f"Converting HISTORY.rst to {destination_folder}/history.html", level=distutils.log.INFO
        )
        os.makedirs(destination_folder, exist_ok=True)
        docutils.core.publish_file(
            source_path="HISTORY.rst",
            destination_path=f"{destination_folder}/history.html",
            writer_name="html",
        )


class GeneratePyQtForms(Command):
    description = "Generates `parsec.core.ui.*` forms module"

    user_options = []

    def initialize_options(self):
        pass

    def finalize_options(self):
        pass

    def run(self):
        import os
        import pathlib
        from collections import namedtuple

        fix_pyqt_import()
        try:
            from PyQt5.uic.driver import Driver
        except ImportError:
            print("PyQt5 not installed, skipping `parsec.core.gui.ui` generation.")
            return

        self.announce("Generating `parsec.core.gui.ui`", level=distutils.log.INFO)
        Options = namedtuple(
            "Options",
            ["output", "import_from", "debug", "preview", "execute", "indent", "resource_suffix"],
        )
        ui_dir = pathlib.Path("parsec/core/gui/forms")
        ui_path = "parsec/core/gui/ui"
        os.makedirs(ui_path, exist_ok=True)
        for f in ui_dir.iterdir():
            o = Options(
                output=os.path.join(ui_path, "{}.py".format(f.stem)),
                import_from="parsec.core.gui",
                debug=False,
                preview=False,
                execute=False,
                indent=4,
                resource_suffix="_rc",
            )
            d = Driver(o, str(f))
            d.invoke()


class ExtractTranslations(Command):
    description = "Extract translation strings"

    user_options = []

    def initialize_options(self):
        pass

    def finalize_options(self):
        pass

    def run(self):
        import os
        import pathlib
        from unittest.mock import patch
        from babel.messages.frontend import CommandLineInterface

        fix_pyqt_import()
        try:
            from PyQt5.pylupdate_main import main as pylupdate_main
        except ImportError:
            print("PyQt5 not installed, skipping `parsec.core.gui.ui` generation.")
            return

        self.announce("Generating ui translation files", level=distutils.log.INFO)
        ui_dir = pathlib.Path("parsec/core/gui")
        tr_dir = ui_dir / "tr"
        os.makedirs(tr_dir, exist_ok=True)

        new_args = ["pylupdate", str(ui_dir / "parsec-gui.pro")]
        with patch("sys.argv", new_args):
            pylupdate_main()

        files = [str(f) for f in ui_dir.iterdir() if f.is_file() and f.suffix == ".py"]
        files.sort()
        files.append(str(tr_dir / "parsec_en.ts"))
        args = [
            "_",
            "extract",
            "-k",
            "translate",
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


class CompileTranslations(Command):
    description = "Compile translations"

    user_options = []

    def initialize_options(self):
        pass

    def finalize_options(self):
        pass

    def run(self):
        import os
        import pathlib
        from babel.messages.frontend import CommandLineInterface

        self.announce("Compiling ui translation files", level=distutils.log.INFO)
        ui_dir = pathlib.Path("parsec/core/gui")
        tr_dir = ui_dir / "tr"
        rc_dir = ui_dir / "rc" / "translations"
        os.makedirs(rc_dir, exist_ok=True)
        languages = ["fr", "en"]
        for lang in languages:
            args = [
                "_",
                "compile",
                "-i",
                str(tr_dir / f"parsec_{lang}.po"),
                "-o",
                str(rc_dir / f"parsec_{lang}.mo"),
            ]
            CommandLineInterface().run(args)


class build_py_with_pyqt(build_py):
    def run(self):
        self.run_command("generate_pyqt_forms")
        self.run_command("compile_translations")
        self.run_command("generate_changelog")
        self.run_command("generate_pyqt_resources_bundle")
        return super().run()


class build_py_with_pyqt_resource_bundle_generation(build_py):
    def run(self):
        self.run_command("generate_pyqt_resources_bundle")
        return super().run()


with open("README.rst", encoding="utf-8") as readme_file:
    readme = readme_file.read()

with open("HISTORY.rst", encoding="utf-8") as history_file:
    history = history_file.read()


def get_requirement_from_file(file):
    with open(file, encoding="utf-8") as requirements_txt:
        return [str(r) for r in pkg_resources.parse_requirements(requirements_txt)]


requirements = get_requirement_from_file("requirement/install_requirement.txt")
test_requirements = get_requirement_from_file("requirement/test_requirement.txt")
core_requirements = get_requirement_from_file("requirement/core_requirement.txt")
backend_requirements = get_requirement_from_file("requirement/backend_requirement.txt")

extra_requirements = {
    "core": core_requirements,
    "backend": backend_requirements,
    "dev": test_requirements,
    # Oxidation is a special case: for the moment it is experimental (i.e. not
    # shipped in production) and only contains rewriting of Python parts so
    # it can be safely ignored for any purpose.
    "oxidation": [
        f"libparsec @ file://localhost{Path(os.path.dirname(os.path.abspath(__file__))) / 'oxidation/libparsec_python'}"
    ],
}


# Cannot have direct dependency when upload on Pypi
if os.environ.get("IGNORE_OXIDATION"):
    extra_requirements.pop("oxidation")


extra_requirements["all"] = sum([v for k, v in extra_requirements.items() if k != "oxidation"], [])
extra_requirements["oeuf-jambon-fromage"] = extra_requirements["all"]

setup(
    name="parsec-cloud",
    version=__version__,
    description="Secure cloud framework",
    long_description=readme + "\n\n" + history,
    author="Scille SAS",
    author_email="contact@scille.fr",
    url="https://github.com/Scille/parsec-cloud",
    python_requires="~=3.7",
    packages=find_packages(include=["parsec", "parsec.*"]),
    package_dir={"parsec": "parsec"},
    install_requires=requirements,
    extras_require=extra_requirements,
    cmdclass={
        "generate_pyqt_resources_bundle": GeneratePyQtResourcesBundle,
        "generate_changelog": GenerateChangelog,
        "generate_pyqt_forms": GeneratePyQtForms,
        "extract_translations": ExtractTranslations,
        "compile_translations": CompileTranslations,
        "generate_pyqt": build_py_with_pyqt,
        "build_py": build_py_with_pyqt,
    },
    # Omitting GUI resources given they end up packaged in `parsec/core/gui/_resources_rc.py`
    package_data={
        "parsec.backend.postgresql.migrations": ["*.sql"],
        "parsec.backend.templates": ["*"],
        "parsec.backend.static": ["*"],
        "parsec.core.resources": ["*.ico", "*.icns", "*.ignore"],
    },
    entry_points={
        "console_scripts": ["parsec = parsec.cli:cli"],
        "babel.extractors": ["extract_qt = misc.babel_qt_extractor.extract_qt"],
    },
    license="AGPLv3",
    zip_safe=False,
    keywords="parsec",
    classifiers=[
        "Development Status :: 5 - Production/Stable",
        "Intended Audience :: Developers",
        "Operating System :: POSIX :: Linux",
        "Operating System :: Microsoft :: Windows",
        "Operating System :: MacOS",
        "License :: OSI Approved :: GNU Affero General Public License v3 or later (AGPLv3+)",
        "Natural Language :: English",
        "Programming Language :: Python :: 3.7",
        "Programming Language :: Python :: 3.8",
    ],
    test_suite="tests",
    tests_require=test_requirements,
    long_description_content_type="text/x-rst",
)
