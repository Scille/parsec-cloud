# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


class InviteError(Exception):
    pass


class InvitePeerResetError(InviteError):
    def __init__(self, msg="Claim operation reset by peer"):
        super().__init__(msg)


class InviteNotAvailableError(InviteError):
    def __init__(self, msg="Invitation not avaialble"):
        super().__init__(msg)
