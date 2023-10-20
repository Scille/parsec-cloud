# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from parsec._parsec import InvitationToken

class InvitationDeletedReason:
    VALUES: tuple[InvitationDeletedReason]
    FINISHED: InvitationDeletedReason
    CANCELLED: InvitationDeletedReason
    ROTTEN: InvitationDeletedReason

    @classmethod
    def from_str(cls, value: str) -> InvitationDeletedReason: ...
    @property
    def str(self) -> str: ...

class Req:
    def __init__(self, token: InvitationToken, reason: InvitationDeletedReason) -> None: ...
    def dump(self) -> bytes: ...
    @property
    def token(self) -> InvitationToken: ...
    @property
    def reason(self) -> InvitationDeletedReason: ...

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

class RepNotFound(Rep):
    def __init__(
        self,
    ) -> None: ...

class RepAlreadyDeleted(Rep):
    def __init__(
        self,
    ) -> None: ...
