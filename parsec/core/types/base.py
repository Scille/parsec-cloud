import re
from uuid import UUID
from typing import NewType
from pathlib import PurePosixPath

from parsec.schema_fields import str_based_field_factory


__all__ = (
    "SchemaSerializationError",
    "TrustSeed",
    "AccessID",
    "EntryName",
    "EntryNameField",
    "FileDescriptor",
    "Path",
)


class SchemaSerializationError(Exception):
    pass


TrustSeed = NewType("TrustSeed", str)
AccessID = NewType("AccessID", UUID)
FileDescriptor = NewType("FileDescriptor", int)


class EntryName(str):
    __slots__ = ()
    regex = re.compile(r"^\w{1,256}$")

    def __init__(self, raw):
        if not isinstance(raw, str) or not self.regex.match(raw):
            raise ValueError("Invalid entry name")


EntryNameField = str_based_field_factory(EntryName)
TrustSeedField = str_based_field_factory(TrustSeed)


class Path(PurePosixPath):
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

    def walk_from_path(self):
        parent = None
        curr = self
        while curr != parent:
            yield curr
            parent, curr = curr, curr.parent

    def walk_to_path(self):
        return reversed(list(self.walk_from_path()))
