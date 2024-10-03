# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

"""
Automate the signing of the latest release artifact.

This script downloads the latest pre-built artifact from the GitHub repository,
unzips it, checks the version, and create a signed windows installer using the
embedded signing script.


Requirements:
- Python 3.9+
- gh (GitHub CLI) installed and authenticated
- tqdm installed (pip install tqdm)
- urllib3>=2 installed (pip install "urllib3>=2")
"""

from __future__ import annotations


import argparse
import json
import pathlib
import shutil
import subprocess
from zipfile import ZipFile

import tqdm
import urllib3

SIGN_SCRIPT = "sign-windows-package.cmd"
ARTIFACT_NAME = "windows-exe-X64-electron-pre-built"
BASE_URL = "https://api.github.com/repos/scille/parsec-cloud"


def get_github_token() -> str:
    process = subprocess.run(
        "gh auth token", capture_output=True, shell=True, check=True, text=True
    )
    return process.stdout.strip()


def get(url: str, token: str) -> urllib3.HTTPResponse:
    headers = {
        "Accept": "application/vnd.github+json",
        "Authorization": f"Bearer {token}",
        "X-GitHub-Api-Version": "2022-11-28",
    }
    return urllib3.request("GET", url, headers=headers, preload_content=False)  # type: ignore


def get_artifact(
    base_url: str, token: str, version: None | str = None
) -> tuple[str, int, str, str, str]:
    url = f"{base_url}/actions/artifacts"
    artefacts = get(url, token).json()["artifacts"]  # type: ignore
    for artefact in artefacts:
        name = artefact["name"]
        size = artefact["size_in_bytes"]
        url = artefact["archive_download_url"]
        timestamp = artefact["updated_at"]
        head = artefact["workflow_run"]["head_branch"]
        if name != ARTIFACT_NAME:
            continue
        if not head.startswith("v"):
            continue
        if version is not None and head != version:
            continue
        return name, size, url, timestamp, head
    if version is None:
        raise ValueError(f"Artifact {ARTIFACT_NAME} not found")
    else:
        raise ValueError(f"Artifact {ARTIFACT_NAME} not found for version {version}")


def download_artifact(url: str, token: str, size: int, destination: pathlib.Path) -> None:
    data = get(url, token)
    if destination.exists() and destination.stat().st_size == size:
        print(f"Skipping download, file already exists: {destination}")
        return
    print(f"Downloading: {url}")
    print(f"Into: {destination}")
    with destination.open("wb") as f:
        with tqdm.tqdm(total=size) as progress_bar:
            for chunk in data.stream():
                f.write(chunk)
                progress_bar.update(len(chunk))


def unzip_artifact(path: pathlib.Path) -> pathlib.Path:
    assert path.suffix == ".zip"
    destination = path.parent / path.stem
    if destination.exists():
        shutil.rmtree(destination)
    print(f"Unzipping {path} to {destination}...")
    with ZipFile(str(path), "r") as zip_ref:
        zip_ref.extractall(destination)
    return destination


def check_version(path: pathlib.Path, version: str) -> None:
    with (path / "package.json").open() as f:
        package = json.load(f)
    if f"v{package['version']}" != version:
        raise ValueError(f"Version mismatch: {version} != v{package['version']}")
    print(f"Version check passed on {path}")


def sign_release(path: pathlib.Path) -> None:
    script = (path / SIGN_SCRIPT).resolve()
    subprocess.run([script], check=True, cwd=path)
    upload_dir = (path / "upload").resolve()
    subprocess.Popen(f'explorer "{upload_dir}"', shell=True)


def main(version: str | None = None) -> None:
    token = get_github_token()
    name, size, url, timestamp, version = get_artifact(BASE_URL, token, version)
    print(f"Found artifact: {name} version {version} updated at {timestamp}")
    zip_path = pathlib.Path(f"{name}-{version}.zip")
    download_artifact(url, token, size, zip_path)
    destination = unzip_artifact(zip_path)
    check_version(destination, version)
    sign_release(destination)
    print("Done!")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Sign the latest release artifact.")
    parser.add_argument(
        "--version",
        type=str,
        help="Specify the version of the artifact to sign (e.g., v1.2.3). If not provided, the latest version will be used.",
    )
    args = parser.parse_args()
    version = args.version if args.version.startswith("v") else f"v{args.version}"
    main(version)
