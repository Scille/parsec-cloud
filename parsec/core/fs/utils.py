# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
from parsec.api.data import WorkspaceManifest, FileManifest, FolderManifest, Manifest
from parsec.core.types import (
    LocalUserManifest,
    LocalWorkspaceManifest,
    LocalFolderManifest,
    LocalFileManifest,
    LocalManifest,
)

# Cross-plateform windows error enumeration

if os.name == "nt":
    from winfspy.plumbing.winstuff import NTSTATUS as ntstatus
else:

    class NTSTATUS:
        def __getattr__(self, key):
            assert key.startswith("STATUS_")
            return None

    ntstatus = NTSTATUS()


# TODO: remove those methods ?


def is_placeholder_manifest(manifest: LocalManifest) -> bool:
    return manifest.is_placeholder


def is_file_manifest(manifest: Manifest) -> bool:
    return isinstance(manifest, (FileManifest, LocalFileManifest))


def is_folder_manifest(manifest: Manifest) -> bool:
    return isinstance(manifest, (FolderManifest, LocalFolderManifest))


def is_workspace_manifest(manifest: Manifest) -> bool:
    return isinstance(manifest, (WorkspaceManifest, LocalWorkspaceManifest))


def is_folderish_manifest(manifest: Manifest) -> bool:
    return isinstance(
        manifest,
        (
            FolderManifest,
            LocalFolderManifest,
            WorkspaceManifest,
            LocalWorkspaceManifest,
            LocalUserManifest,
        ),
    )
