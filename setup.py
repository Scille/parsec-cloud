#!/usr/bin/env python
# -*- coding: utf-8 -*-

from setuptools import find_packages, distutils, Command
from setuptools.command.build_py import build_py

try:
    from cx_Freeze import setup, Executable
except ImportError:

    def Executable(x, **kw):
        return x

    from setuptools import setup

import itertools
import glob


# Awesome hack to load `__version__`
__version__ = None
exec(open("parsec/_version.py", encoding="utf-8").read())


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
        files.append(str(tr_dir / "parsec_en.ts"))
        args = [
            "_",
            "extract",
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
            args = [
                "_",
                "update",
                "-i",
                str(tr_dir / "translation.pot"),
                "-o",
                str(tr_dir / f"parsec_{lang}.po"),
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
        self.run_command("generate_pyqt_resources_bundle")
        return super().run()


class build_py_with_pyqt_resource_bundle_generation(build_py):
    def run(self):
        self.run_command("generate_pyqt_resources_bundle")
        return super().run()


def _extract_libs_cffi_backend():
    try:
        import nacl
    except ImportError:
        return []

    import pathlib

    cffi_backend_dir = pathlib.Path(nacl.__file__).parent / "../.libs_cffi_backend"
    return [(lib.as_posix(), lib.name) for lib in cffi_backend_dir.glob("*")]


build_exe_options = {
    "packages": [
        "parsec.core.gui.ui",
        "idna",
        "sentry_sdk.integrations",
        "trio._core",
        "nacl._sodium",
        "html.parser",
        "pkg_resources._vendor",
        "swiftclient",
        "setuptools.msvc",
        "unittest.mock",
    ],
    # nacl store it cffi shared lib in a very strange place...
    "include_files": _extract_libs_cffi_backend(),
}


with open("README.rst") as readme_file:
    readme = readme_file.read()

with open("HISTORY.rst") as history_file:
    history = history_file.read()

requirements = [
    "attrs==18.2.0",
    "click==7.0",
    "msgpack==0.6.0",
    "wsproto==0.12.0",
    # Can use marshmallow or the toasted flavour as you like ;-)
    # "marshmallow==2.14.0",
    "toastedmarshmallow==0.2.6",
    "pendulum==1.3.1",
    "PyNaCl==1.2.1",
    "trio==0.11.0",
    "python-interface==1.4.0",
    "async_generator>=1.9",
    'contextvars==2.1;python_version<"3.7"',
    "sentry-sdk==0.7.7",
    "structlog==18.2.0",
    "importlib_resources==1.0.2",
    "colorama==0.4.0",  # structlog colored output
]


test_requirements = [
    "pytest==4.4.0",
    "pytest-cov==2.6.1",
    "pytest-xdist==1.28.0",
    "pytest-trio==0.5.2",
    "pytest-qt==3.2.2",
    "pluggy==0.9.0",  # see https://github.com/pytest-dev/pytest/issues/3753
    "tox",
    "wheel",
    "Sphinx",
    "flake8==3.7.7",
    "hypothesis==4.5.5",
    "hypothesis-trio==0.4.0",
    "trustme==0.5.2",
    "black==19.3b0",  # Pin black to avoid flaky style check
]


PYQT_DEP = "PyQt5==5.11.2"
BABEL_DEP = ("Babel==2.6.0",)
extra_requirements = {
    "pkcs11": ["python-pkcs11==0.5.0", "pycrypto==2.6.1"],
    "core": [
        PYQT_DEP,
        BABEL_DEP,
        'fusepy==3.0.1;platform_system=="Linux"',
        'winfspy==0.4.2;platform_system=="Windows"',
        "zxcvbn==4.4.27",
    ],
    "backend": [
        # PostgreSQL
        "triopg==0.3.0",
        "trio-asyncio==0.10.0",
        # S3
        "boto3==1.4.4",
        "botocore==1.5.46",
        # Swift
        "python-swiftclient==3.5.0",
        "pbr==4.0.2",
    ],
    "dev": test_requirements,
}
extra_requirements["all"] = sum(extra_requirements.values(), [])
extra_requirements["oeuf-jambon-fromage"] = extra_requirements["all"]

setup(
    name="parsec-cloud",
    version=__version__,
    description="Secure cloud framework",
    long_description=readme + "\n\n" + history,
    author="Scille SAS",
    author_email="contact@scille.fr",
    url="https://github.com/Scille/parsec-cloud",
    packages=find_packages(),
    package_dir={"parsec": "parsec"},
    setup_requires=[PYQT_DEP, BABEL_DEP, "wheel"],  # To generate resources bundle
    install_requires=requirements,
    extras_require=extra_requirements,
    cmdclass={
        "generate_pyqt_resources_bundle": GeneratePyQtResourcesBundle,
        "generate_pyqt_forms": GeneratePyQtForms,
        "extract_translations": ExtractTranslations,
        "compile_translations": CompileTranslations,
        "generate_pyqt": build_py_with_pyqt,
        "build_py": build_py_with_pyqt,
    },
    # As you may know, setuptools is really broken, so we have to roll our
    # globing ourself to include non-python files...
    package_data={
        "parsec.backend.drivers.postgresql": "*.sql",
        "parsec.core.gui": [
            x[len("parsec/core/gui/") :]
            for x in itertools.chain(
                glob.glob("parsec/core/gui/tr/**/*.ts", recursive=True),
                glob.glob("parsec/core/gui/forms/**/*.ui", recursive=True),
                glob.glob("parsec/core/gui/rc/**/*.png", recursive=True),
                glob.glob("parsec/core/gui/rc/**/*.qm", recursive=True),
                glob.glob("parsec/core/gui/rc/**/*.qrc", recursive=True),
                glob.glob("parsec/core/gui/rc/**/*.otf", recursive=True),
            )
        ],
    },
    entry_points={
        "console_scripts": ["parsec = parsec.cli:cli"],
        "babel.extractors": ["extract_qt = misc.babel_qt_extractor.extract_qt"],
    },
    options={"build_exe": build_exe_options},
    executables=[Executable("parsec/cli.py", targetName="parsec")],
    license="AGPLv3",
    zip_safe=False,
    keywords="parsec",
    classifiers=[
        "Development Status :: 4 - Beta",
        "Intended Audience :: Developers",
        "Operating System :: POSIX :: Linux",
        "Operating System :: Microsoft :: Windows",
        "License :: OSI Approved :: GNU Affero General Public License v3 or later (AGPLv3+)",
        "Natural Language :: English",
        "Programming Language :: Python :: 3.6",
        "Programming Language :: Python :: 3.7",
    ],
    test_suite="tests",
    tests_require=test_requirements,
)
