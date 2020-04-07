# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


class InviteError(Exception):
    pass


class InvitePeerResetError(InviteError):
    pass


class InviteNotAvailableError(InviteError):
    pass
