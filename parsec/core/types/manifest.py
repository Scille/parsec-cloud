# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from enum import Enum
from typing import TypeVar, Union

from parsec._parsec import (
    Chunk,
    ChunkID,
    DateTime,
    LocalFileManifest,
    LocalFolderManifest,
    LocalUserManifest,
    LocalWorkspaceManifest,
    Regex,
    local_manifest_decrypt_and_load,
)
from parsec.api.data import AnyRemoteManifest, BlockAccess, WorkspaceEntry
from parsec.api.data import FileManifest as RemoteFileManifest
from parsec.api.data import FolderManifest as RemoteFolderManifest
from parsec.api.data import UserManifest as RemoteUserManifest
from parsec.api.data import WorkspaceManifest as RemoteWorkspaceManifest
from parsec.api.protocol import BlockID, RealmRole
from parsec.serde import fields

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
