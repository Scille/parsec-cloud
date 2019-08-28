# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from typing import Union, NewType

from parsec.core.types.base import (
    ChunkID,
    ChunkIDField,
    BlockID,
    BlockIDField,
    EntryID,
    EntryIDField,
    EntryName,
    EntryNameField,
    FsPath,
)
from parsec.core.types.backend_address import (
    BackendAddr,
    BackendOrganizationAddr,
    BackendOrganizationBootstrapAddr,
    BackendOrganizationAddrField,
)
from parsec.core.types.access import (
    BlockAccess,
    BlockAccessSchema,
    Chunk,
    Chunks,
    WorkspaceRole,
    WorkspaceRoleField,
    WorkspaceEntrySchema,
    WorkspaceEntry,
)
from parsec.core.types.local_device import LocalDevice, local_device_serializer
from parsec.core.types.local_manifests import (
    LocalFileManifest,
    LocalFolderManifest,
    LocalWorkspaceManifest,
    local_manifest_serializer,
    DEFAULT_BLOCK_SIZE,
)
from parsec.core.types.manifest import LocalUserManifest, LocalManifest
from parsec.core.types.remote_device import (
    UnverifiedRemoteUser,
    UnverifiedRemoteDevice,
    UnverifiedRealmRole,
    VerifiedRemoteUser,
    VerifiedRemoteDevice,
    VerifiedRealmRole,
)
from parsec.core.types.remote_manifests import (
    FileManifest,
    FolderManifest,
    WorkspaceManifest,
    UserManifest,
    RemoteManifest,
    remote_manifest_serializer,
)


Manifest = Union[LocalManifest, RemoteManifest]
FileDescriptor = NewType("FileDescriptor", int)


__all__ = (
    # base
    "ChunkID",
    "ChunkIDField",
    "BlockID",
    "BlockIDField",
    "EntryID",
    "EntryIDField",
    "EntryName",
    "EntryNameField",
    "FileDescriptor",
    "FsPath",
    # Backend address
    "BackendAddr",
    "BackendOrganizationAddr",
    "BackendOrganizationBootstrapAddr",
    "BackendOrganizationAddrField",
    # access
    "BlockAccess",
    "BlockAccessSchema",
    "Chunk",
    "Chunks",
    "WorkspaceRole",
    "WorkspaceRoleField",
    "WorkspaceEntrySchema",
    "WorkspaceEntry",
    # local_device
    "LocalDevice",
    "local_device_serializer",
    # local_manifests
    "LocalFileManifest",
    "LocalFolderManifest",
    "LocalWorkspaceManifest",
    "LocalUserManifest",
    "LocalManifest",
    "local_manifest_serializer",
    "DEFAULT_BLOCK_SIZE",
    # remote_device
    "UnverifiedRemoteUser",
    "UnverifiedRemoteDevice",
    "UnverifiedRealmRole",
    "VerifiedRemoteUser",
    "VerifiedRemoteDevice",
    "VerifiedRealmRole",
    # remote_manifests
    "FileManifest",
    "FolderManifest",
    "WorkspaceManifest",
    "UserManifest",
    "RemoteManifest",
    "remote_manifest_serializer",
    "Manifest",
    # local file
    "FileDescriptor",
)
