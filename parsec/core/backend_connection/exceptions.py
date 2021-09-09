# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS


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
