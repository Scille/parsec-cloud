# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

# /!\ Autogenerated by misc/gen_protocol_typings.py, any modification will be lost !

from __future__ import annotations

from parsec._parsec import DateTime, SequesterServiceID, UserID

class Req:
    def __init__(
        self,
        realm_key_rotation_certificate: bytes,
        per_participant_keys_bundle_access: dict[UserID, bytes],
        per_sequester_service_keys_bundle_access: dict[SequesterServiceID, bytes] | None,
        keys_bundle: bytes,
    ) -> None: ...
    def dump(self) -> bytes: ...
    @property
    def keys_bundle(self) -> bytes: ...
    @property
    def per_participant_keys_bundle_access(self) -> dict[UserID, bytes]: ...
    @property
    def per_sequester_service_keys_bundle_access(
        self,
    ) -> dict[SequesterServiceID, bytes] | None: ...
    @property
    def realm_key_rotation_certificate(self) -> bytes: ...

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

class RepBadKeyIndex(Rep):
    def __init__(self, last_realm_certificate_timestamp: DateTime) -> None: ...
    @property
    def last_realm_certificate_timestamp(self) -> DateTime: ...

class RepParticipantMismatch(Rep):
    def __init__(self, last_realm_certificate_timestamp: DateTime) -> None: ...
    @property
    def last_realm_certificate_timestamp(self) -> DateTime: ...

class RepSequesterServiceMismatch(Rep):
    def __init__(self, last_sequester_certificate_timestamp: DateTime) -> None: ...
    @property
    def last_sequester_certificate_timestamp(self) -> DateTime: ...

class RepRejectedBySequesterService(Rep):
    def __init__(self, service_id: SequesterServiceID, reason: str | None) -> None: ...
    @property
    def reason(self) -> str | None: ...
    @property
    def service_id(self) -> SequesterServiceID: ...

class RepSequesterServiceUnavailable(Rep):
    def __init__(self, service_id: SequesterServiceID) -> None: ...
    @property
    def service_id(self) -> SequesterServiceID: ...

class RepOrganizationNotSequestered(Rep):
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
