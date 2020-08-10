# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from typing import Union, NewType

from parsec.api.data import EntryID, EntryIDField, EntryName, EntryNameField

from parsec.core.types.base import FsPath

from parsec.core.types.backend_address import (
    BackendAddr,
    BackendOrganizationAddr,
    BackendActionAddr,
    BackendOrganizationBootstrapAddr,
    BackendOrganizationClaimUserAddr,
    BackendOrganizationClaimDeviceAddr,
    BackendOrganizationAddrField,
    BackendOrganizationFileLinkAddr,
    BackendInvitationAddr,
)
from parsec.core.types.local_device import LocalDevice, UserInfo, DeviceInfo
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


FileDescriptor = NewType("FileDescriptor", int)
LocalFolderishManifests = Union[LocalFolderManifest, LocalWorkspaceManifest]


__all__ = (
    "FileDescriptor",
    "LocalFolderishManifests",
    # Base
    "FsPath",
    # Entry
    "EntryID",
    "EntryIDField",
    "EntryName",
    "EntryNameField",
    # Backend address
    "BackendAddr",
    "BackendOrganizationAddr",
    "BackendActionAddr",
    "BackendOrganizationBootstrapAddr",
    "BackendOrganizationClaimUserAddr",
    "BackendOrganizationClaimDeviceAddr",
    "BackendOrganizationAddrField",
    "BackendOrganizationFileLinkAddr",
    "BackendInvitationAddr",
    # local_device
    "LocalDevice",
    "UserInfo",
    "DeviceInfo",
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
)
