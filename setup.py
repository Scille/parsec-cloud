#!/usr/bin/env python
# -*- coding: utf-8 -*-

from setuptools import setup, find_packages, distutils, Command
from setuptools.command.build_py import build_py

try:
    from cx_Freeze import setup, Executable
except ImportError:
    Executable = lambda x, **kw: x


# Awesome hack to Load `__version__`
exec(open("parsec/_version.py", encoding="utf-8").read())


class GeneratePyQtResourcesBundle(Command):
    description = "Generates `parsec.core.gui._resource_rc` bundle module"

    user_options = []

    def initialize_options(self):
        pass

    def finalize_options(self):
        pass

    def run(self):
        try:
            from PyQt5.pyrcc_main import processResourceFile

            self.announce("Generating `parsec.core.gui._resources_rc`", level=distutils.log.INFO)
            processResourceFile(
                [f"parsec/core/gui/rc/resources.qrc"], f"parsec/core/gui/_resources_rc.py", False
            )
        except ImportError:
            print("PyQt5 not installed, skipping `parsec.core.gui._resources_rc` generation.")


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
        "idna",
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
    "attrs==18.1.0",
    "click==6.7",
    "huepy==0.9.6",
    # Can use marshmallow or the toasted flavour as you like ;-)
    # "marshmallow==2.14.0",
    "toastedmarshmallow==0.2.6",
    "pendulum==1.3.1",
    "PyNaCl==1.2.0",
    "simplejson==3.10.0",
    "python-decouple==3.1",
    "trio==0.8.0",
    "python-interface==1.4.0",
    "async_generator>=1.9",
    'contextvars==2.1;python_version<"3.7"',
    "raven==6.8.0",  # Sentry support
    "structlog==18.2.0",
    "colorama==0.4.0",  # structlog colored output
]


test_requirements = [
    # https://github.com/python-trio/pytest-trio/issues/64
    # "pytest>=3.6",
    "pytest==3.8.0",
    "pytest-cov",
    "pytest-trio",
    "tox",
    "wheel",
    "Sphinx",
    "flake8",
    "hypothesis",
    "hypothesis-trio>=0.2.1",
    "black==18.6b1",  # Pin black to avoid flaky style check
]


PYQT_DEP = "PyQt5==5.11.2"
extra_requirements = {
    "core": [PYQT_DEP, "hurry.filesize==0.9", "fusepy==3.0.1"],
    "nitrokey": [
        # Nitrokey POC stuff
        "python-pkcs11==0.5.0",
        "argparse==1.4.0",
        "ecdsa==0.13",
        "progress==1.4",
        "pyasn1==0.4.4",
        "pyasn1-modules==0.2.2",
        "pycrypto==2.6.1",
        "tqdm==4.26.0",
    ],
    "backend": [
        # PostgreSQL
        "triopg==0.3.0",
        "trio-asyncio==0.9.1",
        # S3
        "boto3==1.4.4",
        "botocore==1.5.46",
        # Swift
        "python-swiftclient==3.5.0",
        "pbr==4.0.2",
        "futures==3.1.1",
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
    setup_requires=[PYQT_DEP],  # To generate resources bundle
    install_requires=requirements,
    extras_require=extra_requirements,
    cmdclass={
        "generate_pyqt_resources_bundle": GeneratePyQtResourcesBundle,
        "build_py": build_py_with_pyqt_resource_bundle_generation,
    },
    entry_points={"console_scripts": ["parsec = parsec.cli:cli"]},
    options={"build_exe": build_exe_options},
    executables=[Executable("parsec/cli.py", targetName="parsec")],
    license="AGPLv3",
    zip_safe=False,
    keywords="parsec",
    classifiers=[
        "Development Status :: 2 - Pre-Alpha",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: GNU Affero General Public License v3 or later (AGPLv3+)",
        "Natural Language :: English",
        "Programming Language :: Python :: 3.6",
    ],
    test_suite="tests",
    tests_require=test_requirements,
)
