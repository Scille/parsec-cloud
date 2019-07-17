# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.core.types import (
    FileManifest,
    FolderManifest,
    LocalFileManifest,
    LocalFolderManifest,
    LocalManifest,
    LocalUserManifest,
    LocalWorkspaceManifest,
    Manifest,
    UserManifest,
    WorkspaceManifest,
)


def is_placeholder_manifest(manifest: LocalManifest) -> bool:
    return manifest.is_placeholder


def is_file_manifest(manifest: Manifest) -> bool:
    return isinstance(manifest, (FileManifest, LocalFileManifest))


def is_folder_manifest(manifest: Manifest) -> bool:
    return isinstance(manifest, (FolderManifest, LocalFolderManifest))


def is_folderish_manifest(manifest: Manifest) -> bool:
    return isinstance(
        manifest,
        (
            FolderManifest,
            LocalFolderManifest,
            WorkspaceManifest,
            LocalWorkspaceManifest,
            UserManifest,  # TODO: remove when migration is over
            LocalUserManifest,
        ),
    )


def is_workspace_manifest(manifest: Manifest) -> bool:
    return isinstance(manifest, (WorkspaceManifest, LocalWorkspaceManifest))


def is_user_manifest(manifest: Manifest) -> bool:
    return isinstance(manifest, (UserManifest, LocalUserManifest))
