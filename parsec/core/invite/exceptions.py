# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


class InviteError(Exception):
    pass


class InvitePeerResetError(InviteError):
    def __init__(self, msg="Claim operation reset by peer"):
        super().__init__(msg)


class InviteNotFoundError(InviteError):
    def __init__(self, msg="Invitation not found"):
        super().__init__(msg)


class InviteAlreadyUsedError(InviteError):
    def __init__(self, msg="Invitation already used"):
        super().__init__(msg)


class InviteAlreadyMemberError(InviteError):
    def __init__(self, msg="Invited user is already a member of the organization"):
        super().__init__(msg)


class InviteTimestampError(InviteError):
    def __init__(self, msg="Invalid timestamp"):
        super().__init__(msg)
