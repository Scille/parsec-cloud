# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import re
from uuid import UUID
from typing import NewType
from pathlib import PurePosixPath

from parsec.serde import Serializer, fields


__all__ = ("TrustSeed", "AccessID", "EntryName", "EntryNameField", "FileDescriptor", "FsPath")


def serializer_factory(schema_cls):
    # TODO: add custom exceptions ?
    return Serializer(schema_cls)


TrustSeed = NewType("TrustSeed", str)
AccessID = NewType("AccessID", UUID)
FileDescriptor = NewType("FileDescriptor", int)


class EntryName(str):
    __slots__ = ()
    # TODO: This regex is a bit too loose...
    regex = re.compile(r"^[^/]{1,256}$")

    def __init__(self, raw):
        if not isinstance(raw, str) or not self.regex.match(raw):
            raise ValueError("Invalid entry name")


EntryNameField = fields.str_based_field_factory(EntryName)
TrustSeedField = fields.str_based_field_factory(TrustSeed)


class FsPath(PurePosixPath):
    @classmethod
    def _from_parts(cls, args, init=True):
        self = object.__new__(cls)
        args = [x.replace("\\", "/") for x in args]

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

    def is_workspace(self):
        return not self.is_root() and self.parent.is_root()

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
