import os
from uuid import UUID
from pathlib import PurePosixPath, PureWindowsPath
from typing import NewType, Union


BeaconId = NewType("BeaconId", UUID)

FileDescriptor = NewType("FileDescriptor", int)


class Path(PureWindowsPath if os.name == "nt" else PurePosixPath):
    def __init__(self, raw):
        if not self.is_absolute():
            raise ValueError("Path must be absolute")

    def is_root(self):
        return self.parent == self

    def walk_from_path(self):
        parent = None
        curr = self
        while curr != parent:
            yield curr
            parent, curr = curr, curr.parent

    def walk_to_path(self):
        return reversed(list(self.walk_from_path()))


Access = NewType("Access", dict)
BlockAccess = NewType("BlockAccess", dict)

LocalUserManifest = NewType("LocalUserManifest", dict)
LocalWorkspaceManifest = NewType("LocalWorkspaceManifest", dict)
LocalFolderManifest = NewType("LocalFolderManifest", dict)
LocalFileManifest = NewType("LocalFileManifest", dict)

RemoteUserManifest = NewType("RemoteUserManifest", dict)
RemoteWorkspaceManifest = NewType("RemoteWorkspaceManifest", dict)
RemoteFolderManifest = NewType("RemoteFolderManifest", dict)
RemoteFileManifest = NewType("RemoteFileManifest", dict)


LocalManifest = Union[
    LocalUserManifest, LocalWorkspaceManifest, LocalFolderManifest, LocalFileManifest
]

RemoteManifest = Union[
    RemoteUserManifest, RemoteWorkspaceManifest, RemoteFolderManifest, RemoteFileManifest
]

Manifest = Union[LocalManifest, RemoteManifest]
