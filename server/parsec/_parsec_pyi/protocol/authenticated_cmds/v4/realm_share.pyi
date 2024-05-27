# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

# /!\ Autogenerated by misc/gen_protocol_typings.py, any modification will be lost !

from __future__ import annotations

from parsec._parsec import DateTime

class Req:
    def __init__(
        self, realm_role_certificate: bytes, recipient_keys_bundle_access: bytes, key_index: int
    ) -> None: ...
    def dump(self) -> bytes: ...
    @property
    def key_index(self) -> int: ...
    @property
    def realm_role_certificate(self) -> bytes: ...
    @property
    def recipient_keys_bundle_access(self) -> bytes: ...

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

class RepRealmNotFound(Rep):
    def __init__(
        self,
    ) -> None: ...

class RepRecipientNotFound(Rep):
    def __init__(
        self,
    ) -> None: ...

class RepRecipientRevoked(Rep):
    def __init__(
        self,
    ) -> None: ...

class RepRoleIncompatibleWithOutsider(Rep):
    def __init__(
        self,
    ) -> None: ...

class RepRoleAlreadyGranted(Rep):
    def __init__(self, last_realm_certificate_timestamp: DateTime) -> None: ...
    @property
    def last_realm_certificate_timestamp(self) -> DateTime: ...

class RepBadKeyIndex(Rep):
    def __init__(self, last_realm_certificate_timestamp: DateTime) -> None: ...
    @property
    def last_realm_certificate_timestamp(self) -> DateTime: ...

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
    def server_timestamp(self) -> DateTime: ...
    @property
    def client_timestamp(self) -> DateTime: ...

class RepRequireGreaterTimestamp(Rep):
    def __init__(self, strictly_greater_than: DateTime) -> None: ...
    @property
    def strictly_greater_than(self) -> DateTime: ...
