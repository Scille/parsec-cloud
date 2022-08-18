# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import attr
from typing import TypeVar, Type, Union, Pattern

from parsec._parsec import SecretKey
from parsec.serde import fields
from parsec.api.protocol import RealmRole, BlockID
from parsec.api.data import (
    WorkspaceEntry,
    BlockAccess,
    UserManifest as RemoteUserManifest,
    WorkspaceManifest as RemoteWorkspaceManifest,
    FolderManifest as RemoteFolderManifest,
    FileManifest as RemoteFileManifest,
    AnyManifest as RemoteAnyManifest,
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
)

__all__ = (
    "WorkspaceEntry",  # noqa: Republishing
    "BlockAccess",  # noqa: Republishing
    "BlockID",  # noqa: Republishing
    "WorkspaceRole",
    "Chunk",
    "LocalFileManifest",
    "LocalFolderManifest",
    "LocalUserManifest",
    "LocalWorkspaceManifest",
    "AnyLocalManifest",
)


DEFAULT_BLOCK_SIZE = 512 * 1024  # 512 KB


# Cheap rename
WorkspaceRole = RealmRole


ChunkIDField: Type[fields.Field] = fields.uuid_based_field_factory(ChunkID)


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
AnyLocalManifest = Union[
    LocalFileManifest,
    LocalFolderManifest,
    LocalWorkspaceManifest,
    LocalUserManifest,
]


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False, init=False)
class BaseLocalManifest:
    def __init__(self) -> None:
        raise RuntimeError(f"Trying to initialize {BaseLocalManifest.__name__}")

    @staticmethod
    def decrypt_and_load(encrypted: bytes, key: SecretKey) -> AnyLocalManifest:
        """
        Raises:
            DataError
        """
        from parsec._parsec import local_manifest_decrypt_and_load

        return local_manifest_decrypt_and_load(encrypted, key)

    @staticmethod
    def from_remote(
        remote: RemoteAnyManifest,
        prevent_sync_pattern: Pattern,
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

    @staticmethod
    def from_remote_with_local_context(
        remote: RemoteAnyManifest,
        prevent_sync_pattern: Pattern,
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
