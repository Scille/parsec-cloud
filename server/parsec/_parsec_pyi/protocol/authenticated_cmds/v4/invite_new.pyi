# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from parsec._parsec import InvitationToken

class UserOrDevice:
    pass

class UserOrDeviceUser(UserOrDevice):
    def __init__(self, claimer_email: str, send_email: bool) -> None: ...
    @property
    def claimer_email(self) -> str: ...
    @property
    def send_email(self) -> bool: ...

class UserOrDeviceDevice(UserOrDevice):
    def __init__(self, send_email: bool) -> None: ...
    @property
    def send_email(self) -> bool: ...

class InvitationEmailSentStatus:
    VALUES: tuple[InvitationEmailSentStatus]
    SUCCESS: InvitationEmailSentStatus
    NOT_AVAILABLE: InvitationEmailSentStatus
    BAD_RECIPIENT: InvitationEmailSentStatus

    @classmethod
    def from_str(cls, value: str) -> InvitationEmailSentStatus: ...
    @property
    def str(self) -> str: ...

class Req:
    def __init__(self, unit: UserOrDevice) -> None: ...
    def dump(self) -> bytes: ...
    @property
    def unit(self) -> UserOrDevice: ...

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
    def __init__(self, token: InvitationToken, email_sent: InvitationEmailSentStatus) -> None: ...
    @property
    def token(self) -> InvitationToken: ...
    @property
    def email_sent(self) -> InvitationEmailSentStatus: ...

class RepNotAllowed(Rep):
    def __init__(
        self,
    ) -> None: ...

class RepAlreadyMember(Rep):
    def __init__(
        self,
    ) -> None: ...

class RepNotAvailable(Rep):
    def __init__(
        self,
    ) -> None: ...
