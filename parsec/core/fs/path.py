# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from pathlib import PurePath

from typing import Tuple, List, Iterable, Union, TypeVar
from trio import Path as TrioPath
from parsec.api.data import EntryName, EntryNameTooLongError
from parsec.core.fs.exceptions import FSNameTooLongError


def _entry_name_for_fspath(raw_part: str) -> EntryName:
    try:
        return EntryName(raw_part)
    except EntryNameTooLongError:
        raise FSNameTooLongError(raw_part)


class FsPath:
    __slots__ = ("_parts",)
    """
    Represent an absolute path to access a resource in the FS.

    FsPath must be initialized with a str representing an absolute path (i.e.
    with a leading slash). If it countains `.` and/or `..` parts the path will
    be resolved.
    """

    def __init__(self, raw: Union["FsPath", str, Iterable[EntryName]]):
        # FsPath case
        if isinstance(raw, FsPath):
            self._parts = raw.parts
            return

        # Iterable[EntryName] case
        if isinstance(raw, (list, tuple)):
            assert all(isinstance(x, EntryName) for x in raw)
            self._parts = tuple(raw)
            return

        # str case
        assert isinstance(raw, str)
        if not raw.startswith("/"):
            raise ValueError("Path must be absolute")

        parts: List[EntryName] = []
        for raw_part in raw.split("/"):
            if raw_part in (".", ""):
                continue
            elif raw_part == "..":
                if parts:
                    parts.pop()
                continue
            else:
                parts.append(_entry_name_for_fspath(raw_part))

        self._parts = tuple(parts)

    def __str__(self) -> str:
        return "/" + "/".join([part.str for part in self._parts])

    def __repr__(self) -> str:
        return f"{type(self).__name__}({str(self)!r})"

    def __truediv__(self, entry: Union[str, EntryName]) -> "FsPath":
        if isinstance(entry, str):
            entry = _entry_name_for_fspath(entry)
        return type(self)([*self._parts, entry])

    def __eq__(self, other: object) -> bool:
        if isinstance(other, FsPath):
            return self._parts == other.parts
        else:
            return NotImplemented

    @property
    def name(self) -> EntryName:
        return self._parts[-1]

    @property
    def parent(self) -> "FsPath":
        return type(self)(self._parts[:-1])

    def is_root(self) -> bool:
        return not self._parts

    @property
    def parts(self) -> Tuple[EntryName, ...]:
        return self._parts

    T = TypeVar("T", PurePath, TrioPath)

    def with_mountpoint(self, mountpoint: T) -> T:
        return mountpoint / "/".join([part.str for part in self._parts])


AnyPath = Union[FsPath, str]
