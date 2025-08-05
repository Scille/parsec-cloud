#!/usr/bin/env python3
# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from setuptools import setup, find_packages
from setuptools.command.build_py import build_py
import pathlib
import sys

# Import the build function from build.py
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from build import build as build_rust


class BuildPyWithRust(build_py):
    """Custom build command that builds Rust extensions before Python packages."""

    def run(self):
        # Build Rust extensions first
        build_rust()
        # Then run the standard build_py
        super().run()


setup(
    packages=find_packages(),
    package_data={
        "parsec": [
            "_parsec*.so",  # Rust lib for Linux & MacOS
            "_parsec*.pyd",  # Rust lib for Windows
            "libs/*",  # Bundled shared libraries if any
        ]
    },
    cmdclass={
        "build_py": BuildPyWithRust,
    },
)
