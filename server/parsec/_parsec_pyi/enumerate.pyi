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

class GreeterOrClaimer:
    GREETER: GreeterOrClaimer
    CLAIMER: GreeterOrClaimer
    VALUES: tuple[GreeterOrClaimer, ...]

    @classmethod
    def from_str(cls, value: str) -> GreeterOrClaimer: ...
    @property
    def str(self) -> str: ...

class CancelledGreetingAttemptReason:
    MANUALLY_CANCELLED: CancelledGreetingAttemptReason
    INVALID_NONCE_HASH: CancelledGreetingAttemptReason
    INVALID_SAS_CODE: CancelledGreetingAttemptReason
    UNDECIPHERABLE_PAYLOAD: CancelledGreetingAttemptReason
    UNDESERIALIZABLE_PAYLOAD: CancelledGreetingAttemptReason
    INCONSISTENT_PAYLOAD: CancelledGreetingAttemptReason
    AUTOMATICALLY_CANCELLED: CancelledGreetingAttemptReason
    VALUES: tuple[CancelledGreetingAttemptReason, ...]

    @classmethod
    def from_str(cls, value: str) -> CancelledGreetingAttemptReason: ...
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
