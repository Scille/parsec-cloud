# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

# /!\ Autogenerated by misc/gen_protocol_typings.py, any modification will be lost !

from __future__ import annotations

from parsec._parsec import DateTime, DeviceID, UserID

class Req:
    def __init__(
        self,
        id: int,
        author: DeviceID,
        timestamp: DateTime,
        recipient: UserID,
        messageEncrypted: bytes,
    ) -> None: ...
    def dump(self) -> bytes: ...
    @property
    def author(self) -> DeviceID: ...
    @property
    def id(self) -> int: ...
    @property
    def messageEncrypted(self) -> bytes: ...
    @property
    def recipient(self) -> UserID: ...
    @property
    def timestamp(self) -> DateTime: ...

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
        id: int,
        author: DeviceID,
        timestamp: DateTime,
        recipient: UserID,
        messageEncrypted: bytes,
    ) -> None: ...
    @property
    def author(self) -> DeviceID: ...
    @property
    def id(self) -> int: ...
    @property
    def messageEncrypted(self) -> bytes: ...
    @property
    def recipient(self) -> UserID: ...
    @property
    def timestamp(self) -> DateTime: ...
