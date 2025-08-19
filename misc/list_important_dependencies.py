#!/usr/bin/env python3
# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

"""
List versions of what we consider important dependencies of our client & server application.
"""

from __future__ import annotations

import argparse
import csv
import tomllib
from abc import ABC, abstractmethod
from collections.abc import Callable, Generator, Sequence
from enum import Enum, auto
from pathlib import Path
from typing import BinaryIO, NamedTuple

from pydantic import BaseModel, ConfigDict, Field, RootModel
from pydantic.dataclasses import dataclass

ROOT_DIR = Path(__file__).parent.parent.resolve()
NPM_LOCK_FILES = (
    ROOT_DIR / "client/package-lock.json",
    ROOT_DIR / "client/electron/package-lock.json",
)
CARGO_LOCK_FILE = ROOT_DIR / "Cargo.lock"
# We do not include the lock file for the documentations as it's not important for the application.
PYTHON_LOCK_FILE = ROOT_DIR / "server/poetry.lock"
RUST_TOOLCHAIN_FILE = ROOT_DIR / "rust-toolchain.toml"
VERSION_FILE = ROOT_DIR / "misc/versions.toml"


@dataclass(frozen=True, slots=True)
class Dependency:
    name: str
    version: str
    type: DependencyType


class DependencyType(Enum):
    Rust = auto()
    Python = auto()
    Javascript = auto()

    def __str__(self) -> str:
        return self.name


def main():
    args = parse_args()

    with open(VERSION_FILE, "rb") as versions_file:
        versions: dict[str, str] = tomllib.load(versions_file)

    print(f"Will write versions into {args.output} CSV file")
    with open(args.output, "w") as csvfile:
        csv_writer = csv.DictWriter(
            csvfile,
            fieldnames=Dependency.__pydantic_fields__.keys(),  # pyright: ignore [reportAttributeAccessIssue]
            strict=True,
            dialect="unix",
        )
        csv_writer.writeheader()

        if not args.skip_javascript:
            deps = list_javascript_deps(versions)
            write_deps(csv_writer, deps)

        if not args.skip_rust:
            deps = list_rust_deps(versions)
            write_deps(csv_writer, deps)

        if not args.skip_python:
            deps = list_python_deps(versions)
            write_deps(csv_writer, deps)


DependencyModel = RootModel[Dependency]


def write_deps(csv: csv.DictWriter[Dependency], deps: set[Dependency]):
    deps_list = list(deps)
    deps_list.sort(key=lambda dep: dep.name)
    csv.writerows(DependencyModel(dep).model_dump() for dep in deps_list)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()

    parser.add_argument("--output", default=Path("versions.csv"), type=Path)
    parser.add_argument("--skip-javascript", action="store_true")
    parser.add_argument("--skip-rust", action="store_true")
    parser.add_argument("--skip-python", action="store_true")

    return parser.parse_args()


class Dependencies(ABC, BaseModel):
    @abstractmethod
    def dependencies(self) -> Generator[Dependency]:
        raise NotImplementedError()


class ListingResult(NamedTuple):
    dependencies: set[Dependency]
    not_found: set[str]


def list_dependencies_from_file(
    filepath: Path,
    loader: Callable[[BinaryIO], Dependencies],
    dependencies_to_include: Sequence[str],
) -> ListingResult:
    deps = set()
    with open(filepath, "rb") as file:
        print(f"Loading {filepath.relative_to(ROOT_DIR)} content")
        lock = loader(file)

        for dep in lock.dependencies():
            if dep.name in dependencies_to_include:
                deps.add(dep)

    not_found = set(dependencies_to_include).difference(dep.name for dep in deps)

    return ListingResult(deps, not_found)


def list_dependencies_from_toml_file[T: Dependencies](
    filepath: Path,
    ty: type[T],
    dependencies_to_include: Sequence[str],
) -> ListingResult:
    def _toml_loader(file: BinaryIO) -> Dependencies:
        data = tomllib.load(file)
        return ty.model_validate(data)

    return list_dependencies_from_file(filepath, _toml_loader, dependencies_to_include)


def list_javascript_deps(versions: dict[str, str]) -> set[Dependency]:
    IMPORTANT_JAVASCRIPT_DEPS = (
        "@capacitor-community/electron",
        "@ionic/core",
        "@ionic/storage",
        "@ionic/vue",
        "@ionic/vue-router",
        "@zip.js/zip.js",
        "docx-preview",
        "luxon",
        "monaco-editor",
        "pdfjs-dist",
        "typescript",
        "uuid",
        "vue",
        "vue-i18n",
        "vue-router",
        "xlsx",
        "@electron/asar",
        "@electron/notarize",
        "@electron/osx-sign",
        "@electron/rebuild",
        "@electron/universal",
        "electron",
        "electron-builder",
        "electron-builder-squirrel-windows",
        "electron-is-dev",
        "electron-updater",
        "electron-window-state",
        "regedit",
        "typescript",
    )

    deps = {Dependency("nodejs", versions["node"], DependencyType.Javascript)}

    deps_not_found: set[str] = set(IMPORTANT_JAVASCRIPT_DEPS)

    for lock_filepath in NPM_LOCK_FILES:
        deps_from_file = list_dependencies_from_file(
            lock_filepath,
            lambda file: NpmPackageLock.model_validate_json(file.read()),
            IMPORTANT_JAVASCRIPT_DEPS,
        )
        deps_not_found.difference_update(dep.name for dep in deps_from_file.dependencies)
        deps.update(deps_from_file.dependencies)

    assert not deps_not_found, (
        f"Some expected dependencies where not found in lock file {', '.join(deps_not_found)}"
    )

    return deps


class NpmPackageLock(Dependencies):
    packages: NpmPackages

    def dependencies(self) -> Generator[Dependency]:
        return self.packages.dependencies()


class NpmPackage(BaseModel):
    version: str


class NpmPackages(Dependencies):
    __pydantic_extra__: dict[str, NpmPackage] = Field(init=False)  # pyright: ignore [reportIncompatibleVariableOverride]
    main: NpmMainPackage = Field(alias="")

    model_config = ConfigDict(extra="allow")

    def dependencies(self) -> Generator[Dependency, None, None]:
        yield Dependency(self.main.name, self.main.version, DependencyType.Javascript)
        if self.__pydantic_extra__:
            for k, v in self.__pydantic_extra__.items():
                yield Dependency(
                    k.removeprefix("node_modules/"),
                    v.version,
                    DependencyType.Javascript,
                )


class NpmMainPackage(NpmPackage):
    name: str


def list_rust_deps(_versions: dict[str, str]) -> set[Dependency]:
    IMPORTANT_RUST_DEPS = (
        # Crypto
        "argon2",
        "blake2",
        "crypto_box",
        "crypto_secretbox",
        "digest",
        "ed25519-dalek",
        "x25519-dalek",
        "getrandom",
        "libsodium-sys-stable",
        "libsodium-rs",
        "openssl",
        "sha2",
        "uuid",
        "rsa",
        "blahaj",
        "keyring",
        # HTTP Client
        "reqwest",
        # bindings
        "pyo3",
        "wasm-bindgen",
        "js-sys",
        "web-sys",
        "windows-sys",
        "fuser",
        "winfsp_wrs",
        # Encoding
        "data-encoding",
        "rmp-serde",
        "ruzstd",
        "zstd",
        # Others
        "libc",
        "libsqlite3-sys",
        "sqlx",
        "tokio",
        "unicode-normalization",
        "email-address-parser",
        "chrono",
        "flume",
        "futures",
        "regex",
        "rpassword",
    )
    with open(RUST_TOOLCHAIN_FILE, "rb") as toolchain:
        print(f"Loading {RUST_TOOLCHAIN_FILE.relative_to(ROOT_DIR)} content")
        data = tomllib.load(toolchain)
        version = str(data["toolchain"]["channel"])
        deps = {Dependency("rust", version, DependencyType.Rust)}

    deps_from_lock = list_dependencies_from_toml_file(
        CARGO_LOCK_FILE, CargoLock, IMPORTANT_RUST_DEPS
    )
    deps.update(deps_from_lock.dependencies)

    assert not deps_from_lock.not_found, (
        f"Some expected dependencies where not found in lock file {', '.join(deps_from_lock.not_found)}"
    )

    return deps


class CargoLock(Dependencies):
    version: int
    package: list[CargoPackage]

    def dependencies(self) -> Generator[Dependency, None, None]:
        for pkg in self.package:
            yield Dependency(pkg.name, pkg.version, DependencyType.Rust)


class CargoPackage(BaseModel):
    name: str
    version: str


def list_python_deps(versions: dict[str, str]) -> set[Dependency]:
    IMPORTANT_PYTHON_DEPS = (
        # HTTP
        "fastapi",
        "starlette",
        "httpx",
        "uvicorn",
        # Databases & Block storage
        "asyncpg",
        "boto3",
        "botocore",
        "python-swiftclient",
        # Others
        "pydantic",
        "jinja2",
        # Build
        "cibuildwheel",
        "maturin",
        "setuptools",
        "patchelf",
    )
    deps = {
        Dependency("python", versions["python"], DependencyType.Python),
        Dependency("poetry", versions["poetry"], DependencyType.Python),
    }

    deps_from_lock = list_dependencies_from_toml_file(
        PYTHON_LOCK_FILE, PoetryLock, IMPORTANT_PYTHON_DEPS
    )
    deps.update(deps_from_lock.dependencies)

    assert not deps_from_lock.not_found, (
        f"Some expected dependencies where not found in lock file {', '.join(deps_from_lock.not_found)}"
    )

    return deps


class PoetryLock(Dependencies):
    metadata: PoetryMetadata
    package: list[PoetryPackage]

    def dependencies(self) -> Generator[Dependency, None, None]:
        for pkg in self.package:
            yield Dependency(pkg.name, pkg.version, DependencyType.Python)


class PoetryMetadata(BaseModel):
    lock_version: str = Field(alias="lock-version")
    python_versions: str = Field(alias="python-versions")


class PoetryPackage(BaseModel):
    name: str
    version: str


if __name__ == "__main__":
    main()
