# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

class Req:
    def __init__(self, offset: int) -> None: ...
    def dump(self) -> bytes: ...
    @property
    def offset(self) -> int: ...

class Rep:
    @staticmethod
    def load(raw: bytes) -> Rep: ...
    def dump(self) -> bytes: ...

class RepUnknownStatus(Rep):
    def __init__(self, status: str, reason: str | None) -> None: ...
    @property
    def status(self) -> str: ...
    @property
    def reason(self) -> str | None: ...

class RepOk(Rep):
    def __init__(self, certificates: list[bytes]) -> None: ...
    @property
    def certificates(self) -> list[bytes]: ...
