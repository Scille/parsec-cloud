# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import re
from pathlib import PurePosixPath
from uuid import UUID, uuid4

from parsec.serde import Serializer, fields

__all__ = (
    "BlockID",
    "BlockIDField",
    "EntryID",
    "EntryIDField",
    "EntryName",
    "EntryNameField",
    "FsPath",
)


def serializer_factory(schema_cls):
    # TODO: add custom exceptions ?
    return Serializer(schema_cls)


class BlockID(UUID):
    __slots__ = ()

    def __init__(self, init=None):
        init = uuid4() if init is None else init
        if isinstance(init, UUID):
            super().__init__(init.hex)
        else:
            super().__init__(init)


BlockIDField = fields.uuid_based_field_factory(BlockID)


class EntryID(UUID):
    __slots__ = ()

    def __init__(self, init=None):
        init = uuid4() if init is None else init
        if isinstance(init, UUID):
            super().__init__(init.hex)
        else:
            super().__init__(init)


EntryIDField = fields.uuid_based_field_factory(EntryID)


class EntryName(str):
    __slots__ = ()
    # TODO: This regex is a bit too loose...
    regex = re.compile(r"^[^/]{1,256}$")

    def __init__(self, raw):
        if not isinstance(raw, str) or not self.regex.match(raw):
            raise ValueError("Invalid entry name")


EntryNameField = fields.str_based_field_factory(EntryName)


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

    # TODO: this method should be removed with the FS class
    # The concept of workspace path doesn't apply anymore

    def is_workspace(self):
        return not self.is_root() and self.parent.is_root()

    # TODO: this method should be removed with the FS class
    # The concept of workspace path doesn't apply anymore

    @property
    def workspace(self):
        return self.parts[1]

    def walk_from_path(self):
        parent = None
        curr = self
        while curr != parent:
            yield curr
            parent, curr = curr, curr.parent

    def walk_to_path(self):
        return reversed(list(self.walk_from_path()))
