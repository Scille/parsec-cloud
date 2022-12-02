# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from __future__ import annotations

# From docs:
# https://marshmallow.readthedocs.io/en/stable/marshmallow.validate.html#marshmallow.validate.Range
class Range:
    def __init__(
        self,
        min: int | None = None,
        max: int | None = None,
        *,
        min_inclusive: bool = True,
        max_inclusive: bool = True,
        error: str | None = None,
    ): ...
    def __call__(self, value: int) -> bool: ...

# From docs:
# https://marshmallow.readthedocs.io/en/stable/marshmallow.validate.html#marshmallow.validate.Length
class Length:
    def __init__(
        self,
        min: int | None = None,
        max: int | None = None,
        *,
        equal: int | None = None,
        error: str | None = None,
    ): ...
    def __call__(self, value: int) -> bool: ...
