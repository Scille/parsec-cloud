from typing import Union

from parsec.core.types.base import TrustSeed, AccessID, EntryName, FileDescriptor, FsPath
from parsec.core.types.access import Access, ManifestAccess, BlockAccess, DirtyBlockAccess
from parsec.core.types.local_device import LocalDevice, local_device_serializer
from parsec.core.types.local_manifests import (
    LocalFileManifest,
    LocalFolderManifest,
    LocalWorkspaceManifest,
    LocalUserManifest,
    LocalManifest,
    local_manifest_serializer,
)
from parsec.core.types.remote_device import RemoteDevice, RemoteDevicesMapping, RemoteUser
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
    "TrustSeed",
    "AccessID",
    "EntryName",
    "FileDescriptor",
    "FsPath",
    "Access",
    "ManifestAccess",
    "BlockAccess",
    "DirtyBlockAccess",
    "LocalDevice",
    "local_device_serializer",
    "LocalFileManifest",
    "LocalFolderManifest",
    "LocalWorkspaceManifest",
    "LocalUserManifest",
    "LocalManifest",
    "local_manifest_serializer",
    "RemoteDevice",
    "RemoteDevicesMapping",
    "RemoteUser",
    "FileManifest",
    "FolderManifest",
    "WorkspaceManifest",
    "UserManifest",
    "RemoteManifest",
    "remote_manifest_serializer",
    "Manifest",
)
