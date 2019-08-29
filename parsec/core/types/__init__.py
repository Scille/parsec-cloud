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
from parsec.core.types.manifest import (
    DEFAULT_BLOCK_SIZE,
    LocalFileManifest,
    LocalFolderManifest,
    LocalWorkspaceManifest,
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


FileDescriptor = NewType("FileDescriptor", int)
LocalFolderishManifests = Union[LocalFolderManifest, LocalWorkspaceManifest]


__all__ = (
    "FileDescriptor",
    "LocalFolderishManifests",
    # base
    "EntryID",
    "EntryIDField",
    "EntryName",
    "EntryNameField",
    "FsPath",
    # Backend address
    "BackendAddr",
    "BackendOrganizationAddr",
    "BackendOrganizationBootstrapAddr",
    "BackendOrganizationAddrField",
    # local_device
    "LocalDevice",
    "local_device_serializer",
    # "manifest"
    "DEFAULT_BLOCK_SIZE",
    "LocalFileManifest",
    "LocalFolderManifest",
    "LocalWorkspaceManifest",
    "LocalUserManifest",
    "LocalManifest",
    "WorkspaceEntry",
    "WorkspaceRole",
    "BlockAccess",
    "BlockID",
    "Chunk",
    "ChunkID",
    # remote_device
    "UnverifiedRemoteUser",
    "UnverifiedRemoteDevice",
    "UnverifiedRealmRole",
    "VerifiedRemoteUser",
    "VerifiedRemoteDevice",
    "VerifiedRealmRole",
)
