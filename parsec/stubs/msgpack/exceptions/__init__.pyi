# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from __future__ import annotations

class UnpackException(Exception): ...

class ExtraData(ValueError):
    def __init__(self, unpacked: object, extra: object) -> None: ...
    def __str__(self) -> str: ...

class FormatError(ValueError, UnpackException):
    def __init__(self, *args: object, **kwargs: object) -> None: ...
    def __str__(self) -> str: ...

class StackError(ValueError, UnpackException):
    def __init__(self, *args: object, **kwargs: object) -> None: ...
