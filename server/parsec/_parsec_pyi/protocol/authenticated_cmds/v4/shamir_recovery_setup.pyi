# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

# /!\ Autogenerated by misc/gen_protocol_typings.py, any modification will be lost !

from __future__ import annotations

from parsec._parsec import DateTime, InvitationToken

class ShamirRecoverySetup:
    def __init__(
        self, ciphered_data: bytes, reveal_token: InvitationToken, brief: bytes, shares: list[bytes]
    ) -> None: ...
    @property
    def brief(self) -> bytes: ...
    @property
    def ciphered_data(self) -> bytes: ...
    @property
    def reveal_token(self) -> InvitationToken: ...
    @property
    def shares(self) -> list[bytes]: ...

class Req:
    def __init__(self, setup: ShamirRecoverySetup | None) -> None: ...
    def dump(self) -> bytes: ...
    @property
    def setup(self) -> ShamirRecoverySetup | None: ...

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

class RepBriefInvalidData(Rep):
    def __init__(
        self,
    ) -> None: ...

class RepShareInvalidData(Rep):
    def __init__(
        self,
    ) -> None: ...

class RepInvalidRecipient(Rep):
    def __init__(
        self,
    ) -> None: ...

class RepShareRecipientNotInBrief(Rep):
    def __init__(
        self,
    ) -> None: ...

class RepDuplicateShareForRecipient(Rep):
    def __init__(
        self,
    ) -> None: ...

class RepAuthorIncludedAsRecipient(Rep):
    def __init__(
        self,
    ) -> None: ...

class RepMissingShareForRecipient(Rep):
    def __init__(
        self,
    ) -> None: ...

class RepShareInconsistentTimestamp(Rep):
    def __init__(
        self,
    ) -> None: ...

class RepShamirSetupAlreadyExists(Rep):
    def __init__(self, last_shamir_certificate_timestamp: DateTime) -> None: ...
    @property
    def last_shamir_certificate_timestamp(self) -> DateTime: ...

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
