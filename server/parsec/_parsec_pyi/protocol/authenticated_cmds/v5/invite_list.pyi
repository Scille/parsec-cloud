# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

# /!\ Autogenerated by misc/gen_protocol_typings.py, any modification will be lost !

from __future__ import annotations

from parsec._parsec import DateTime, InvitationStatus, InvitationToken, UserID

class InviteListItem:
    pass

class InviteListItemUser(InviteListItem):
    def __init__(
        self,
        token: InvitationToken,
        created_on: DateTime,
        claimer_email: str,
        status: InvitationStatus,
    ) -> None: ...
    @property
    def claimer_email(self) -> str: ...
    @property
    def created_on(self) -> DateTime: ...
    @property
    def status(self) -> InvitationStatus: ...
    @property
    def token(self) -> InvitationToken: ...

class InviteListItemDevice(InviteListItem):
    def __init__(
        self, token: InvitationToken, created_on: DateTime, status: InvitationStatus
    ) -> None: ...
    @property
    def created_on(self) -> DateTime: ...
    @property
    def status(self) -> InvitationStatus: ...
    @property
    def token(self) -> InvitationToken: ...

class InviteListItemShamirRecovery(InviteListItem):
    def __init__(
        self,
        token: InvitationToken,
        created_on: DateTime,
        claimer_user_id: UserID,
        shamir_recovery_created_on: DateTime,
        status: InvitationStatus,
    ) -> None: ...
    @property
    def claimer_user_id(self) -> UserID: ...
    @property
    def created_on(self) -> DateTime: ...
    @property
    def shamir_recovery_created_on(self) -> DateTime: ...
    @property
    def status(self) -> InvitationStatus: ...
    @property
    def token(self) -> InvitationToken: ...

class Req:
    def __init__(
        self,
    ) -> None: ...
    def dump(self) -> bytes: ...

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
    def __init__(self, invitations: list[InviteListItem]) -> None: ...
    @property
    def invitations(self) -> list[InviteListItem]: ...