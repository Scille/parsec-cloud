# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from __future__ import annotations

from parsec._parsec import BlockID

class Req:
    def __init__(self, block_id: BlockID) -> None: ...
    def dump(self) -> bytes: ...
    @property
    def block_id(self) -> BlockID: ...

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
    def __init__(self, block: bytes) -> None: ...
    @property
    def block(self) -> bytes: ...

class RepNotFound(Rep):
    def __init__(
        self,
    ) -> None: ...

class RepTimeout(Rep):
    def __init__(
        self,
    ) -> None: ...

class RepNotAllowed(Rep):
    def __init__(
        self,
    ) -> None: ...

class RepInMaintenance(Rep):
    def __init__(
        self,
    ) -> None: ...
