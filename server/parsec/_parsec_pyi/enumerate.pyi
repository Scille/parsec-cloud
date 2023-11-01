# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

class InvitationStatus:
    IDLE: InvitationStatus
    READY: InvitationStatus
    CANCELLED: InvitationStatus
    FINISHED: InvitationStatus
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
