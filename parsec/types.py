# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Dict, Type, TypeVar
from uuid import UUID, uuid4


K = TypeVar("K")
V = TypeVar("V")


class FrozenDict(Dict[K, V]):
    __slots__ = ()

    def __repr__(self):
        return f"{self.__class__.__name__}({dict.__repr__(self)})"

    def _ro_guard(*args, **kwargs):
        raise AttributeError("FrozenDict doesn't allow modifications")

    __setitem__ = _ro_guard
    __delitem__ = _ro_guard
    pop = _ro_guard
    clear = _ro_guard
    popitem = _ro_guard
    setdefault = _ro_guard
    update = _ro_guard

    def evolve(self, **data):
        return FrozenDict(**self, **data)


EntryIDTypeVar = TypeVar("EntryIDTypeVar", bound="UUID4")


class UUID4:
    __slots__ = ("_uuid",)

    def __init__(self, raw: UUID):
        if not isinstance(raw, UUID):
            raise ValueError("Not a UUID")
        self._uuid = raw

    @classmethod
    def from_bytes(cls: Type[K], raw: bytes) -> K:
        return cls(UUID(bytes=raw))

    @classmethod
    def from_hex(cls: Type[K], raw: str) -> K:
        return cls(UUID(hex=raw))

    def __repr__(self) -> str:
        return f"<{type(self).__name__} {self.hex}>"

    def __str__(self) -> str:
        return self._uuid.hex

    def __eq__(self, other) -> bool:
        if not isinstance(other, type(self)):
            return NotImplemented
        return self._uuid == other._uuid

    def __lt__(self, other) -> bool:
        if not isinstance(other, type(self)):
            return NotImplemented
        return self._uuid < self._uuid

    def __hash__(self) -> int:
        return self._uuid.__hash__()

    @property
    def bytes(self) -> bytes:
        return self._uuid.bytes

    @property
    def uuid(self) -> UUID:
        return self._uuid

    @property
    def hex(self) -> str:
        return self._uuid.hex

    @classmethod
    def new(cls: Type[EntryIDTypeVar]) -> EntryIDTypeVar:
        return cls(uuid4())


# Cheap typing
import typing

typing.FrozenDict = FrozenDict
