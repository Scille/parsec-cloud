# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from pathlib import PurePosixPath


__all__ = ("FsPath",)


class FsPath(PurePosixPath):
    @classmethod
    def _from_parts(cls, args, init=True):
        self = object.__new__(cls)
        args = [x.replace("\\", "/") if isinstance(x, str) else x for x in args]

        drv, root, parts = PurePosixPath._parse_args(args)
        if not root:
            raise ValueError("Path must be absolute")

        if drv:
            raise ValueError("Path must be Posix style")

        # Posix style root can be `/` or `//` (yeah, this is silly...)
        root = parts[0] = "/"
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
