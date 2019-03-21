# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from typing import Union

from parsec.core.types.base import TrustSeed, AccessID, EntryName, FileDescriptor, FsPath
from parsec.core.types.access import (
    Access,
    UserAccess,
    ManifestAccess,
    BlockAccess,
    DirtyBlockAccess,
    WorkspaceEntry,
)
from parsec.core.types.local_device import LocalDevice, local_device_serializer
from parsec.core.types.local_manifests import (
    LocalFileManifest,
    LocalFolderManifest,
    LocalWorkspaceManifest,
    LocalUserManifest,
    LocalManifest,
    local_manifest_serializer,
)
from parsec.core.types.remote_device import (
    UnverifiedRemoteUser,
    UnverifiedRemoteDevice,
    VerifiedRemoteUser,
    VerifiedRemoteDevice,
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


__all__ = (
    # base
    "TrustSeed",
    "AccessID",
    "EntryName",
    "FileDescriptor",
    "FsPath",
    # access
    "Access",
    "UserAccess",
    "ManifestAccess",
    "BlockAccess",
    "DirtyBlockAccess",
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
    # remote_device
    "UnverifiedRemoteUser",
    "UnverifiedRemoteDevice",
    "VerifiedRemoteUser",
    "VerifiedRemoteDevice",
    # remote_manifests
    "FileManifest",
    "FolderManifest",
    "WorkspaceManifest",
    "UserManifest",
    "RemoteManifest",
    "remote_manifest_serializer",
    "Manifest",
)
