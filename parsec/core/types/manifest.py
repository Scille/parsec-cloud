# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import TypeVar, Union

from parsec.serde import fields
from parsec.api.protocol import RealmRole, BlockID
from parsec.api.data import (
    WorkspaceEntry,
    BlockAccess,
    UserManifest as RemoteUserManifest,
    WorkspaceManifest as RemoteWorkspaceManifest,
    FolderManifest as RemoteFolderManifest,
    FileManifest as RemoteFileManifest,
    AnyRemoteManifest,
)
from enum import Enum

from parsec._parsec import (
    ChunkID,
    Chunk,
    LocalFileManifest,
    LocalFolderManifest,
    LocalUserManifest,
    LocalWorkspaceManifest,
    DateTime,
    local_manifest_decrypt_and_load,
    Regex,
)

AnyLocalManifest = Union[
    LocalFileManifest,
    LocalFolderManifest,
    LocalWorkspaceManifest,
    LocalUserManifest,
]

__all__ = (
    "WorkspaceEntry",
    "BlockAccess",
    "BlockID",
    "WorkspaceRole",
    "Chunk",
    "LocalFileManifest",
    "LocalFolderManifest",
    "LocalUserManifest",
    "LocalWorkspaceManifest",
    "AnyLocalManifest",
    "local_manifest_decrypt_and_load",
)


DEFAULT_BLOCK_SIZE = 512 * 1024  # 512 KB


# Cheap rename
WorkspaceRole = RealmRole


ChunkIDField = fields.uuid_based_field_factory(ChunkID)


class LocalManifestType(Enum):
    LOCAL_FILE_MANIFEST = "local_file_manifest"
    LOCAL_FOLDER_MANIFEST = "local_folder_manifest"
    LOCAL_WORKSPACE_MANIFEST = "local_workspace_manifest"
    LOCAL_USER_MANIFEST = "local_user_manifest"


LocalFileManifestTypeVar = TypeVar("LocalFileManifestTypeVar", bound="LocalFileManifest")
LocalFolderManifestTypeVar = TypeVar("LocalFolderManifestTypeVar", bound="LocalFolderManifest")
LocalWorkspaceManifestTypeVar = TypeVar(
    "LocalWorkspaceManifestTypeVar", bound="LocalWorkspaceManifest"
)
LocalUserManifestTypeVar = TypeVar("LocalUserManifestTypeVar", bound="LocalUserManifest")


def manifest_from_remote(
    remote: AnyRemoteManifest,
    prevent_sync_pattern: Regex,
) -> AnyLocalManifest:
    if isinstance(remote, RemoteFileManifest):
        return LocalFileManifest.from_remote(remote)
    elif isinstance(remote, RemoteFolderManifest):
        return LocalFolderManifest.from_remote(remote, prevent_sync_pattern)
    elif isinstance(remote, RemoteWorkspaceManifest):
        return LocalWorkspaceManifest.from_remote(remote, prevent_sync_pattern)
    elif isinstance(remote, RemoteUserManifest):
        return LocalUserManifest.from_remote(remote)
    raise ValueError("Wrong remote type")


def manifest_from_remote_with_local_context(
    remote: AnyRemoteManifest,
    prevent_sync_pattern: Regex,
    local_manifest: AnyLocalManifest,
    timestamp: DateTime,
) -> AnyLocalManifest:
    if isinstance(remote, RemoteFileManifest):
        return LocalFileManifest.from_remote(remote)
    elif isinstance(remote, RemoteFolderManifest):
        return LocalFolderManifest.from_remote_with_local_context(
            remote, prevent_sync_pattern, local_manifest, timestamp=timestamp
        )
    elif isinstance(remote, RemoteWorkspaceManifest):
        return LocalWorkspaceManifest.from_remote_with_local_context(
            remote, prevent_sync_pattern, local_manifest, timestamp=timestamp
        )
    elif isinstance(remote, RemoteUserManifest):
        return LocalUserManifest.from_remote(remote)
    raise ValueError("Wrong remote type")
