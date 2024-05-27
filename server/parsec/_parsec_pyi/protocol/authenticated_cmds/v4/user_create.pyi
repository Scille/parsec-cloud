# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

# /!\ Autogenerated by misc/gen_protocol_typings.py, any modification will be lost !

from __future__ import annotations

from parsec._parsec import DateTime

class Req:
    def __init__(
        self,
        user_certificate: bytes,
        device_certificate: bytes,
        redacted_user_certificate: bytes,
        redacted_device_certificate: bytes,
    ) -> None: ...
    def dump(self) -> bytes: ...
    @property
    def device_certificate(self) -> bytes: ...
    @property
    def redacted_device_certificate(self) -> bytes: ...
    @property
    def redacted_user_certificate(self) -> bytes: ...
    @property
    def user_certificate(self) -> bytes: ...

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
    ) -> None: ...

class RepAuthorNotAllowed(Rep):
    def __init__(
        self,
    ) -> None: ...

class RepActiveUsersLimitReached(Rep):
    def __init__(
        self,
    ) -> None: ...

class RepUserAlreadyExists(Rep):
    def __init__(
        self,
    ) -> None: ...

class RepHumanHandleAlreadyTaken(Rep):
    def __init__(
        self,
    ) -> None: ...

class RepInvalidCertificate(Rep):
    def __init__(
        self,
    ) -> None: ...

class RepTimestampOutOfBallpark(Rep):
    def __init__(
        self,
        ballpark_client_early_offset: float,
        ballpark_client_late_offset: float,
        server_timestamp: DateTime,
        client_timestamp: DateTime,
    ) -> None: ...
    @property
    def ballpark_client_early_offset(self) -> float: ...
    @property
    def ballpark_client_late_offset(self) -> float: ...
    @property
    def client_timestamp(self) -> DateTime: ...
    @property
    def server_timestamp(self) -> DateTime: ...

class RepRequireGreaterTimestamp(Rep):
    def __init__(self, strictly_greater_than: DateTime) -> None: ...
    @property
    def strictly_greater_than(self) -> DateTime: ...
