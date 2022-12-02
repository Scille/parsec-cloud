# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import NewType, Union

from parsec._parsec import DeviceInfo, LocalDevice, UserInfo
from parsec.api.data import EntryID, EntryIDField, EntryName, EntryNameField
from parsec.api.data import FolderManifest as RemoteFolderManifest
from parsec.api.data import WorkspaceManifest as RemoteWorkspaceManifest
from parsec.core.types.backend_address import (
    BackendActionAddr,
    BackendAddr,
    BackendAddrType,
    BackendInvitationAddr,
    BackendOrganizationAddr,
    BackendOrganizationAddrField,
    BackendOrganizationBootstrapAddr,
    BackendOrganizationFileLinkAddr,
    BackendPkiEnrollmentAddr,
)
from parsec.core.types.manifest import (
    DEFAULT_BLOCK_SIZE,
    AnyLocalManifest,
    BlockAccess,
    BlockID,
    Chunk,
    ChunkID,
    LocalFileManifest,
    LocalFolderManifest,
    LocalUserManifest,
    LocalWorkspaceManifest,
    WorkspaceEntry,
    WorkspaceRole,
)
from parsec.core.types.manifest import manifest_from_remote as local_manifest_from_remote
from parsec.core.types.manifest import (
    manifest_from_remote_with_local_context as local_manifest_from_remote_with_local_context,
)
from parsec.core.types.organizations import (
    OrganizationConfig,
    OrganizationStats,
    UsersPerProfileDetailItem,
)

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
    "WorkspaceEntry",
    "WorkspaceRole",
    "BlockAccess",
    "BlockID",
    "Chunk",
    "ChunkID",
    "local_manifest_from_remote",
    "local_manifest_from_remote_with_local_context",
    "AnyLocalManifest",
    # organizations
    "OrganizationStats",
    "UsersPerProfileDetailItem",
    "OrganizationConfig",
)
