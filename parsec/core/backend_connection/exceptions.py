# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations


class BackendConnectionError(Exception):
    pass


class BackendProtocolError(BackendConnectionError):
    pass


class BackendNotAvailable(BackendConnectionError):
    pass


class BackendConnectionRefused(BackendConnectionError):
    pass


class BackendInvitationAlreadyUsed(BackendConnectionRefused):
    pass


class BackendInvitationNotFound(BackendConnectionRefused):
    pass


# TODO: hack needed by `LoggedCore.get_user_info`
class BackendNotFoundError(BackendConnectionError):
    pass


# TODO: hack needed by `LoggedCore.new_user_invitation`
class BackendInvitationOnExistingMember(BackendConnectionError):
    pass


class BackendOutOfBallparkError(BackendConnectionError):
    pass
