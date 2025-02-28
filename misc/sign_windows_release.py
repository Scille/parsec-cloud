#!/usr/bin/env python3
# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

"""
Automate the signing of the latest GUI & CLI release artifacts on Windows.

This script downloads the latest pre-built artifact from the GitHub repository,
unzips it, checks the version, and create a signed windows installer using the
embedded signing script.


Requirements:
- Python 3.9+
- gh (GitHub CLI) installed and authenticated
"""

from __future__ import annotations

import argparse
import json
import re
import shutil
import subprocess
from dataclasses import dataclass
from functools import lru_cache
from hashlib import sha256
from http.client import HTTPResponse
from pathlib import Path
from urllib.request import HTTPRedirectHandler, Request, urlopen
from zipfile import ZipFile

ROOTDIR = Path(__file__).parent.parent.resolve()
REPOSITORY = "scille/parsec-cloud"
BASE_URL = f"https://api.github.com/repos/{REPOSITORY}"
CLI_ARTIFACT_NAME = "Windows-x86_64-pc-windows-msvc-cli-pre-build"
CLI_EXECUTABLE_PATTERN = r"^parsec-cli_.*_windows-x86_64-msvc.exe$"
GUI_ARTIFACT_NAME = "windows-exe-X64-electron-pre-built"
WINDOWS_KITS_BIN_DIR = Path("C:/Program Files (x86)/Windows Kits/10/bin")
# Note the signing strategy differs between the CLI and the GUI:
# - CLI: We just need to call signtool.exe on the executable and call it a day.
# - GUI: We need to sign the main executable, build an installer, and sign it.
#
# For this reason, this script takes care of the whole CLI signing process, while
# relying on another sub-script embedded in the artifact to do the GUI signing.
CLI_SIGN_CERTIFICATE_SUBJECT_NAME = "Scille"
CLI_SIGN_CERTIFICATE_SHA1 = "4505A81975EF724601813DF296AB74A07ECFA991"
CLI_SIGN_TIMESTAMP_SERVER = "http://time.certum.pl"
GUI_SIGN_SCRIPT_NAME = "sign-windows-package.cmd"


def get_github_token() -> str:
    process = subprocess.run(
        "gh auth token", capture_output=True, shell=True, check=True, text=True
    )
    return process.stdout.strip()


class HTTPRedirectHandlerDropAuthorization(HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        req.headers.pop("Authorization")
        return super().redirect_request(req, fp, code, msg, headers, newurl)


def get(url: str, token: str) -> HTTPResponse:
    request = Request(url, method="GET")
    request.add_header("Accept", "application/vnd.github+json")
    request.add_header("X-GitHub-Api-Version", "2022-11-28")
    request.add_unredirected_header("Authorization", f"Bearer {token}")
    return urlopen(request)


@dataclass(frozen=True, slots=True)
class Artifact:
    name: str
    size: int
    url: str
    timestamp: str
    version: str


@dataclass(frozen=True, slots=True)
class Artifacts:
    name: str
    items: tuple[Artifact]
    # Number of artifacts matching the `name`, but which have been omitted
    # when querying Github API given they are too old.
    omitted: int


@lru_cache
def get_artifacts(base_url: str, token: str, name: str) -> Artifacts:
    per_page = 100  # Max allowed by Github API
    url = f"{base_url}/actions/artifacts?name={name}&per_page={per_page}"
    response = get(url, token)
    artifacts = []
    body = json.loads(response.read())
    for raw in body["artifacts"]:
        head = raw["workflow_run"]["head_branch"]
        if not head.startswith("v"):
            continue
        artifacts.append(
            Artifact(
                name=raw["name"],
                size=raw["size_in_bytes"],
                url=raw["archive_download_url"],
                timestamp=raw["updated_at"],
                version=head,
            )
        )
    return Artifacts(
        name=name,
        items=tuple(artifacts),
        omitted=body["total_count"] - per_page,
    )


def get_artifact(
    artifact_name: str, base_url: str, token: str, version: None | str = None
) -> Artifact:
    artifacts = get_artifacts(base_url, token, artifact_name)
    for artifact in artifacts.items:
        if version is not None and artifact.version != version:
            continue
        return artifact

    # Since we filter the artifact by name and are only interested into signing recent
    # releases, it is unlikely that what we are looking for is among the omitted items
    # (more likely we are looking for something that doesn't exist).
    if artifacts.omitted:
        print(
            f"WARNING: {artifacts.omitted} older items correspond to `{artifact_name}` artifact have been omitted"
        )

    if version is None:
        raise RuntimeError(f"Artifact {artifact_name} not found")
    else:
        raise RuntimeError(f"Artifact {artifact_name} not found for version {version}")


def download_artifact(url: str, token: str, size: int, destination: Path) -> None:
    response = get(url, token)
    if destination.exists() and destination.stat().st_size == size:
        print(f"Skipping download, file already exists: {destination}")
        return
    print(f"Downloading: {url}")
    print(f"Into: {destination}")
    downloaded = 0
    with destination.open("wb") as f:
        while True:
            print(f"\r{downloaded / 2**20:.0f}Mo/{size / 2**20:.0f}Mo", end="", flush=True)
            buff = response.read1(2**20)
            if not buff:
                break
            f.write(buff)
            downloaded += len(buff)
    print()


def unzip_artifact(path: Path) -> Path:
    assert path.suffix == ".zip"
    destination = path.parent / path.stem
    if destination.exists():
        shutil.rmtree(destination)
    print(f"Unzipping {path} to {destination}...")
    with ZipFile(str(path), "r") as zip_ref:
        zip_ref.extractall(destination)
    return destination


def get_release_tag(version: str):
    # For some reason, the release tag is:
    # - `refs/tags/v[...]` when the release is in draft
    # - `v[...]` when the release is published
    # Just check which one exists
    for release_tag in (version, f"refs/tags/{version}"):
        process = subprocess.run(
            f"gh release view {release_tag} --repo {REPOSITORY}",
            shell=True,
            text=True,
            capture_output=True,
        )
        if process.returncode == 0:
            return release_tag
    raise RuntimeError(f"Release tag not found for {version} on {REPOSITORY}")


def upload_assets(path: Path, version: str, expected_files: int) -> None:
    files = list(path.iterdir())
    assert len(files) == expected_files
    files_str = " ".join(str(p.resolve()) for p in files)
    print(f"Uploading {expected_files} files to release {version} on repository {REPOSITORY}...")
    release_tag = get_release_tag(version)
    print(f">> gh release upload {release_tag} {files_str} --repo {REPOSITORY}")
    subprocess.run(
        f"gh release upload {release_tag} {files_str} --repo {REPOSITORY}",
        shell=True,
        check=True,
        text=True,
    )


def sign_cli(version: str | None, workdir: Path) -> None:
    # 1) Get back our artifact of interest

    token = get_github_token()
    artifact = get_artifact(CLI_ARTIFACT_NAME, BASE_URL, token, version)
    print(
        f"Found artifact: {artifact.name} version {artifact.version} updated at {artifact.timestamp}"
    )

    # 2) Unzip and check the artifact

    zip_path = workdir / f"{artifact.name}-{artifact.version}.zip"
    download_artifact(artifact.url, token, artifact.size, zip_path)
    destination = unzip_artifact(zip_path)
    target = None
    for item in destination.iterdir():
        if re.match(CLI_EXECUTABLE_PATTERN, item.name):
            target = item
            break
    if target is None:
        raise RuntimeError(
            f"Executable not found in the artifact (contains: {destination.iterdir()})"
        )

    # 3) Sign

    # Sign executable
    signtool_exe = "signtool.exe"
    if shutil.which(signtool_exe) is None:
        print(
            f"signtool.exe not found in $PATH, looking for it in SDKs in {WINDOWS_KITS_BIN_DIR}..."
        )
        signtool_exe = None
        for sdk in WINDOWS_KITS_BIN_DIR.absolute().iterdir():
            candidate = sdk / "x64/signtool.exe"
            if candidate.exists():
                signtool_exe = str(candidate)
                print(f"Found {signtool_exe} !")
                break
        if signtool_exe is None:
            raise RuntimeError("signtool.exe not found :(")
    subprocess.run(
        [
            # see https://learn.microsoft.com/en-us/windows/win32/seccrypto/signtool#sign-command-options
            signtool_exe,
            "sign",
            "/a",
            "/n",
            CLI_SIGN_CERTIFICATE_SUBJECT_NAME,
            "/t",
            CLI_SIGN_TIMESTAMP_SERVER,
            "/sha1",
            CLI_SIGN_CERTIFICATE_SHA1,
            "/fd",
            "sha256",
            "/d",
            f"Parsec CLI {artifact.version}",
            "/v",
            f"{target}",
        ],
        check=True,
    )

    # 4) Create assets directory containing the signed executable & its sha256 file

    assets_directory = destination / "upload"
    try:
        shutil.rmtree(assets_directory)
    except FileNotFoundError:
        pass
    assets_directory.mkdir(parents=True, exist_ok=True)
    shutil.copy(target, assets_directory)
    with (assets_directory / f"{target.name}.sha256").open("w") as f:
        f.write(sha256((assets_directory / target.name).read_bytes()).hexdigest())

    # 5) Upload assets to Github release

    upload_assets(assets_directory, artifact.version, expected_files=2)
    print("Done!")


def sign_gui(version: str | None, workdir: Path) -> None:
    # 1) Get back our artifact of interest

    token = get_github_token()
    artifact = get_artifact(GUI_ARTIFACT_NAME, BASE_URL, token, version)
    print(
        f"Found artifact: {artifact.name} version {artifact.version} updated at {artifact.timestamp}"
    )

    # 2) Unzip and check the artifact

    zip_path = workdir / f"{artifact.name}-{artifact.version}.zip"
    download_artifact(artifact.url, token, artifact.size, zip_path)
    destination = unzip_artifact(zip_path)
    with (destination / "package.json").open() as f:
        package = json.load(f)
    if f"v{package['version']}" != artifact.version:
        raise RuntimeError(f"Version mismatch: {artifact.version} != v{package['version']}")

    # 3) Build & sign using the sub-script embedded in the artifact

    script = (destination / GUI_SIGN_SCRIPT_NAME).resolve()
    subprocess.run([script], check=True, cwd=destination)
    assets_directory = destination / "upload"
    assert assets_directory.exists()

    # 4) Upload to Github release

    upload_assets(assets_directory, artifact.version, expected_files=6)
    print("Done!")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Sign (and upload) the latest GUI & CLI release artifacts."
    )
    parser.add_argument(
        "what",
        choices=["cli", "gui"],
        nargs="*",
        help="Which artifact to sign (default: cli & gui).",
    )
    parser.add_argument(
        "--version",
        type=str,
        help="Specify the version of the artifact to sign (e.g., v1.2.3). If not provided, the latest version will be used.",
    )
    parser.add_argument(
        "--workdir",
        type=str,
        default="./sign_windows_release_tmp",
    )
    namespace = parser.parse_args()
    version: str | None = namespace.version
    # Add `v` if it was omitted
    if version is not None and version[:1].isdigit():
        version = f"v{version}"

    workdir = Path(namespace.workdir)
    workdir.mkdir(parents=True, exist_ok=True)

    if not namespace.what or "cli" in namespace.what:
        sign_cli(version, workdir)

    if not namespace.what or "gui" in namespace.what:
        sign_gui(version, workdir)
