# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Dict, TypeVar, Any

K = TypeVar("K")
V = TypeVar("V")


class FrozenDict(Dict[K, V]):
    __slots__ = ()

    def __repr__(self) -> str:
        return f"{self.__class__.__name__}({dict.__repr__(self)})"

    def _ro_guard(*args: Any, **kwargs: Any) -> None:
        raise AttributeError("FrozenDict doesn't allow modifications")

    __setitem__ = _ro_guard
    __delitem__ = _ro_guard
    pop = _ro_guard  # type: ignore[assignment]
    clear = _ro_guard
    popitem = _ro_guard  # type: ignore[assignment]
    setdefault = _ro_guard  # type: ignore[assignment]
    update = _ro_guard

    def evolve(self, **data: V) -> FrozenDict[K, V]:
        return FrozenDict(**self, **data)


# Cheap typing
import typing

typing.FrozenDict = FrozenDict  # type: ignore[attr-defined]
