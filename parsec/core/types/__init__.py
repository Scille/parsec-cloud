# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from typing import Union

from parsec.core.types.base import BlockID, BlockIDField, EntryID, EntryIDField, EntryName, FsPath
from parsec.core.types.access import (
    ManifestAccess,
    BlockAccess,
    DirtyBlockAccess,
    WorkspaceRole,
    WorkspaceRoleField,
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
from parsec.core.types.local_file import FileCursor, FileDescriptor


Manifest = Union[LocalManifest, RemoteManifest]


__all__ = (
    # base
    "BlockID",
    "BlockIDField",
    "EntryID",
    "EntryIDField",
    "EntryName",
    "FileDescriptor",
    "FsPath",
    # access
    "ManifestAccess",
    "BlockAccess",
    "DirtyBlockAccess",
    "WorkspaceRole",
    "WorkspaceRoleField",
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
    "FileCursor",
    "FileDescriptor",
)
