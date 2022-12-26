# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from __future__ import annotations

class ClientType:
    AUTHENTICATED: ClientType
    INVITED: ClientType
    ANONYMOUS: ClientType
    APIV1_ANONYMOUS: ClientType
    APIV1_ADMINISTRATION: ClientType
    def __hash__(self) -> int: ...

class InvitationDeletedReason:
    FINISHED: InvitationDeletedReason
    CANCELLED: InvitationDeletedReason
    ROTTEN: InvitationDeletedReason
    VALUES: tuple[InvitationDeletedReason, ...]
    @classmethod
    def from_str(cls, value: str) -> InvitationDeletedReason: ...
    @property
    def str(self) -> str: ...

class InvitationEmailSentStatus:
    SUCCESS: InvitationEmailSentStatus
    NOT_AVAILABLE: InvitationEmailSentStatus
    BAD_RECIPIENT: InvitationEmailSentStatus
    VALUES: tuple[InvitationEmailSentStatus, ...]

    @classmethod
    def from_str(cls, value: str) -> InvitationEmailSentStatus: ...
    @property
    def str(self) -> str: ...

class InvitationStatus:
    IDLE: InvitationStatus
    READY: InvitationStatus
    DELETED: InvitationStatus
    VALUES: tuple[InvitationStatus, ...]

    @classmethod
    def from_str(cls, value: str) -> InvitationStatus: ...
    @property
    def str(self) -> str: ...

class InvitationType:
    DEVICE: InvitationType
    USER: InvitationType
    VALUES: tuple[InvitationType, ...]

    @classmethod
    def from_str(cls, value: str) -> InvitationType: ...
    @property
    def str(self) -> str: ...

class RealmRole:
    OWNER: RealmRole
    MANAGER: RealmRole
    CONTRIBUTOR: RealmRole
    READER: RealmRole
    VALUES: tuple[RealmRole, ...]

    @classmethod
    def from_str(cls, value: str) -> RealmRole: ...
    @property
    def str(self) -> str: ...

class UserProfile:
    ADMIN: UserProfile
    STANDARD: UserProfile
    OUTSIDER: UserProfile
    VALUES: tuple[UserProfile, ...]

    @classmethod
    def from_str(cls, value: str) -> UserProfile: ...
    @property
    def str(self) -> str: ...
