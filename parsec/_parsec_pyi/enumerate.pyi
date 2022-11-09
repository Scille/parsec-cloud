from __future__ import annotations

class InvitationType:
    DEVICE: InvitationType
    USER: InvitationType
    @classmethod
    def values(cls) -> list[InvitationType]: ...
    @classmethod
    def from_str(cls, value: str) -> InvitationType: ...
    @property
    def str(self) -> str: ...

class InvitationEmailSentStatus:
    SUCCESS: InvitationEmailSentStatus
    NOT_AVAILABLE: InvitationEmailSentStatus
    BAD_RECIPIENT: InvitationEmailSentStatus
    def __str__(self) -> str: ...
    @classmethod
    def values(cls) -> list[InvitationEmailSentStatus]: ...
    @classmethod
    def from_str(cls, value: str) -> InvitationEmailSentStatus: ...

class RealmRole:
    OWNER: RealmRole
    MANAGER: RealmRole
    CONTRIBUTOR: RealmRole
    READER: RealmRole
    @classmethod
    def values(cls) -> list[RealmRole]: ...
    @classmethod
    def from_str(cls, value: str) -> RealmRole: ...
    @property
    def str(self) -> str: ...

class UserProfile:
    ADMIN: UserProfile
    STANDARD: UserProfile
    OUTSIDER: UserProfile
    @classmethod
    def values(cls) -> list[UserProfile]: ...
    @classmethod
    def from_str(cls, value: str) -> UserProfile: ...
    @property
    def str(self) -> str: ...
