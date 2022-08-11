# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from typing import Union, NewType

from parsec.api.data import EntryID, EntryIDField, EntryName, EntryNameField

from parsec.core.types.backend_address import (
    BackendAddr,
    BackendOrganizationAddr,
    BackendActionAddr,
    BackendOrganizationBootstrapAddr,
    BackendOrganizationAddrField,
    BackendOrganizationFileLinkAddr,
    BackendInvitationAddr,
    BackendPkiEnrollmentAddr,
    BackendAddrType,
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
    BaseLocalManifest,
    WorkspaceEntry,
    WorkspaceRole,
    BlockAccess,
    BlockID,
    Chunk,
    ChunkID,
)

from parsec.core.types.organizations import (
    OrganizationStats,
    UsersPerProfileDetailItem,
    OrganizationConfig,
)

from parsec.api.data import WorkspaceManifest as RemoteWorkspaceManifest
from parsec.api.data import FolderManifest as RemoteFolderManifest

FileDescriptor = NewType("FileDescriptor", int)
LocalFolderishManifests = Union[LocalFolderManifest, LocalWorkspaceManifest]
RemoteFolderishManifests = Union[RemoteFolderManifest, RemoteWorkspaceManifest]
LocalNonRootManifests = Union[LocalFileManifest, LocalFolderManifest]


__all__ = (
    "FileDescriptor",
    "LocalFolderishManifests",
    "RemoteFolderishManifests",
    "LocalNonRootManifests",
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
    "BackendOrganizationAddrField",
    "BackendOrganizationFileLinkAddr",
    "BackendInvitationAddr",
    "BackendPkiEnrollmentAddr",
    "BackendAddrType",
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
    "BaseLocalManifest",
    "WorkspaceEntry",
    "WorkspaceRole",
    "BlockAccess",
    "BlockID",
    "Chunk",
    "ChunkID",
    # organizations
    "OrganizationStats",
    "UsersPerProfileDetailItem",
    "OrganizationConfig",
)
