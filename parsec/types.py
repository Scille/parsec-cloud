# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Dict, TypeVar


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


# Cheap typing
import typing

typing.FrozenDict = FrozenDict
