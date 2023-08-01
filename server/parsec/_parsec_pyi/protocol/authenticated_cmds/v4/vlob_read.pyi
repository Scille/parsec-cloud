# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from parsec._parsec import DateTime, DeviceID, VlobID

class Req:
    def __init__(
        self,
        encryption_revision: int,
        vlob_id: VlobID,
        version: int | None,
        timestamp: DateTime | None,
    ) -> None: ...
    def dump(self) -> bytes: ...
    @property
    def encryption_revision(self) -> int: ...
    @property
    def vlob_id(self) -> VlobID: ...
    @property
    def version(self) -> int | None: ...
    @property
    def timestamp(self) -> DateTime | None: ...

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
    def __init__(
        self,
        version: int,
        blob: bytes,
        author: DeviceID,
        timestamp: DateTime,
        certificate_index: int,
    ) -> None: ...
    @property
    def version(self) -> int: ...
    @property
    def blob(self) -> bytes: ...
    @property
    def author(self) -> DeviceID: ...
    @property
    def timestamp(self) -> DateTime: ...
    @property
    def certificate_index(self) -> int: ...

class RepNotFound(Rep):
    def __init__(self, reason: str | None) -> None: ...
    @property
    def reason(self) -> str | None: ...

class RepNotAllowed(Rep):
    def __init__(
        self,
    ) -> None: ...

class RepBadVersion(Rep):
    def __init__(
        self,
    ) -> None: ...

class RepBadEncryptionRevision(Rep):
    def __init__(
        self,
    ) -> None: ...

class RepInMaintenance(Rep):
    def __init__(
        self,
    ) -> None: ...
