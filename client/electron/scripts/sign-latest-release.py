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

REPOSITORY = "scille/parsec-cloud"
SIGN_SCRIPT = "sign-windows-package.cmd"
ARTIFACT_NAME = "windows-exe-X64-electron-pre-built"
BASE_URL = f"https://api.github.com/repos/{REPOSITORY}"


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
    url = f"{base_url}/actions/artifacts?per_page=100"
    artifacts = get(url, token).json()["artifacts"]  # type: ignore
    for artifact in artifacts:
        name = artifact["name"]
        size = artifact["size_in_bytes"]
        url = artifact["archive_download_url"]
        timestamp = artifact["updated_at"]
        head = artifact["workflow_run"]["head_branch"]
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


def sign_release(path: pathlib.Path) -> pathlib.Path:
    script = (path / SIGN_SCRIPT).resolve()
    subprocess.run([script], check=True, cwd=path)
    assets_directory = path / "upload"
    assert assets_directory.exists()
    return assets_directory


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
    raise ValueError(f"Release tag not found for {version} on {REPOSITORY}")


def upload_assets(path: pathlib.Path, version: str, expected_files: int = 6) -> None:
    files = list(path.iterdir())
    assert len(files) == expected_files
    files_str = " ".join(str(p.resolve()) for p in files)
    print(f"Uploading {expected_files} files to release {version} on repository {REPOSITORY}...")
    release_tag = get_release_tag(version)
    subprocess.run(
        f"gh release upload {release_tag} {files_str} --repo {REPOSITORY}",
        shell=True,
        check=True,
        text=True,
    )


def main(version: str | None = None) -> None:
    token = get_github_token()
    name, size, url, timestamp, version = get_artifact(BASE_URL, token, version)
    print(f"Found artifact: {name} version {version} updated at {timestamp}")
    zip_path = pathlib.Path(f"{name}-{version}.zip")
    download_artifact(url, token, size, zip_path)
    destination = unzip_artifact(zip_path)
    check_version(destination, version)
    assets_directory = sign_release(destination)
    upload_assets(assets_directory, version)
    print("Done!")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Sign the latest release artifact.")
    parser.add_argument(
        "--version",
        type=str,
        help="Specify the version of the artifact to sign (e.g., v1.2.3). If not provided, the latest version will be used.",
    )
    namespace = parser.parse_args()
    version: str | None = namespace.version
    # Add `v` if it was omitted
    if version is not None and version[:1].isdigit():
        version = f"v{version}"
    main(version)
