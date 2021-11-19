# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import typing
from uuid import UUID, uuid4


K = typing.TypeVar("K")
V = typing.TypeVar("V")


class FrozenDict(typing.Dict[K, V]):
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


class UUID4(UUID):
    __slots__ = ()

    def __init__(self, *args, **kwargs):
        if not args and not kwargs:
            super().__init__(bytes=uuid4().bytes)
        else:
            if len(args) == 1 and isinstance(args[0], UUID):
                super().__init__(bytes=args[0].bytes)
            else:
                super().__init__(*args, **kwargs)


# Cheap typing
typing.FrozenDict = FrozenDict
