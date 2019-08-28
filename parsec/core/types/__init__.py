# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from typing import Union, NewType

from parsec.core.types.base import EntryID, EntryIDField, EntryName, EntryNameField, FsPath
from parsec.core.types.backend_address import (
    BackendAddr,
    BackendOrganizationAddr,
    BackendOrganizationBootstrapAddr,
    BackendOrganizationAddrField,
)
from parsec.core.types.local_device import LocalDevice, local_device_serializer
from parsec.core.types.local_manifests import (
    LocalFileManifest,
    LocalFolderManifest,
    LocalWorkspaceManifest,
    local_manifest_serializer,
    DEFAULT_BLOCK_SIZE,
)
from parsec.core.types.manifest import (
    LocalUserManifest,
    LocalManifest,
    WorkspaceEntry,
    WorkspaceRole,
    BlockAccess,
    BlockID,
    Chunk,
    ChunkID,
)
from parsec.core.types.remote_device import (
    UnverifiedRemoteUser,
    UnverifiedRemoteDevice,
    UnverifiedRealmRole,
    VerifiedRemoteUser,
    VerifiedRemoteDevice,
    VerifiedRealmRole,
)

# from parsec.core.types.remote_manifests import (
#     FileManifest,
#     FolderManifest,
#     WorkspaceManifest,
#     UserManifest,
#     RemoteManifest,
#     remote_manifest_serializer,
# )

# Manifest = Union[LocalManifest, RemoteManifest]
FileDescriptor = NewType("FileDescriptor", int)
LocalFolderishManifests = Union[LocalFolderManifest, LocalWorkspaceManifest]


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
    # local_device
    "LocalDevice",
    "local_device_serializer",
    # local_manifests
    "LocalFileManifest",
    "LocalFolderManifest",
    "LocalWorkspaceManifest",
    "local_manifest_serializer",
    # Local manifest
    "LocalUserManifest",
    "LocalManifest",
    "WorkspaceEntry",
    "WorkspaceRole",
    "local_manifest_serializer",
    "DEFAULT_BLOCK_SIZE",
    "BlockAccess",
    "Chunk",
    "ChunkID",
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
    # "Manifest",
    # local file
    "FileDescriptor",
)
