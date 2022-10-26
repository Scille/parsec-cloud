# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations


class InviteError(Exception):
    pass


class InvitePeerResetError(InviteError):
    def __init__(self, msg: str = "Claim operation reset by peer") -> None:
        super().__init__(msg)


class InviteNotFoundError(InviteError):
    def __init__(self, msg: str = "Invitation not found") -> None:
        super().__init__(msg)


class InviteAlreadyUsedError(InviteError):
    def __init__(self, msg: str = "Invitation already used") -> None:
        super().__init__(msg)


class InviteActiveUsersLimitReachedError(InviteError):
    def __init__(self, msg: str = "Active users limit reached") -> None:
        super().__init__(msg)
