import os
from uuid import UUID
from pathlib import PurePosixPath, PureWindowsPath
from typing import NewType, Union


BeaconId = NewType("BeaconId", UUID)

FileDescriptor = NewType("FileDescriptor", int)


class Path(PurePosixPath):
    def __init__(self, raw):
        if not self.is_absolute():
            raise ValueError("Path must be absolute")

    @classmethod
    def _from_parts(cls, args, init=True):
        self = object.__new__(cls)
        if os.name == "nt":
            drv, root, parts = PureWindowsPath._parse_args(args)
        else:
            drv, root, parts = PurePosixPath._parse_args(args)
        self._drv = drv
        self._root = root
        self._parts = parts
        if init:
            self._init()
        return self

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
