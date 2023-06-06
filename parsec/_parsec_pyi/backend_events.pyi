# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from __future__ import annotations

from parsec._parsec import (
    DateTime,
    DeviceID,
    InvitationStatus,
    InvitationToken,
    OrganizationID,
    RealmID,
    RealmRole,
    UserID,
    UserProfile,
    VlobID,
)

class BackendEvent:
    def dump(self) -> bytes: ...
    @staticmethod
    def load(raw: bytes) -> BackendEvent: ...
    @property
    def organization_id(self) -> OrganizationID: ...

class BackendEventCertificatesUpdated(BackendEvent):
    def __init__(
        self,
        organization_id: OrganizationID,
        timestamp: DateTime,
    ) -> None: ...
    @property
    def timestamp(self) -> DateTime: ...

class BackendEventInviteConduitUpdated(BackendEvent):
    def __init__(
        self,
        organization_id: OrganizationID,
        token: InvitationToken,
    ) -> None: ...
    @property
    def token(self) -> InvitationToken: ...

class BackendEventUserUpdatedOrRevoked(BackendEvent):
    def __init__(
        self,
        organization_id: OrganizationID,
        user_id: UserID,
        # `profile=None` for revoked user
        profile: UserProfile | None,
    ) -> None: ...
    @property
    def user_id(self) -> UserID: ...
    @property
    def profile(self) -> UserProfile | None: ...

class BackendEventOrganizationExpired(BackendEvent):
    def __init__(
        self,
        organization_id: OrganizationID,
    ) -> None: ...

class BackendEventPinged(BackendEvent):
    def __init__(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        ping: str,
    ) -> None: ...
    @property
    def author(self) -> DeviceID: ...
    @property
    def ping(self) -> str: ...

class BackendEventMessageReceived(BackendEvent):
    def __init__(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        recipient: UserID,
        index: int,
        message: bytes,
    ) -> None: ...
    @property
    def author(self) -> DeviceID: ...
    @property
    def recipient(self) -> UserID: ...
    @property
    def index(self) -> int: ...
    @property
    def message(self) -> bytes: ...

class BackendEventInviteStatusChanged(BackendEvent):
    def __init__(
        self,
        organization_id: OrganizationID,
        greeter: UserID,
        token: InvitationToken,
        status: InvitationStatus,
    ) -> None: ...
    @property
    def greeter(self) -> UserID: ...
    @property
    def token(self) -> InvitationToken: ...
    @property
    def status(self) -> InvitationStatus: ...

class BackendEventRealmMaintenanceFinished(BackendEvent):
    def __init__(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: RealmID,
        encryption_revision: int,
    ) -> None: ...
    @property
    def author(self) -> DeviceID: ...
    @property
    def realm_id(self) -> RealmID: ...
    @property
    def encryption_revision(self) -> int: ...

class BackendEventRealmMaintenanceStarted(BackendEvent):
    def __init__(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: RealmID,
        encryption_revision: int,
    ) -> None: ...
    @property
    def author(self) -> DeviceID: ...
    @property
    def realm_id(self) -> RealmID: ...
    @property
    def encryption_revision(self) -> int: ...

class BackendEventRealmVlobsUpdated(BackendEvent):
    def __init__(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: RealmID,
        checkpoint: int,
        src_id: VlobID,
        src_version: int,
    ) -> None: ...
    @property
    def author(self) -> DeviceID: ...
    @property
    def realm_id(self) -> RealmID: ...
    @property
    def checkpoint(self) -> int: ...
    @property
    def src_id(self) -> VlobID: ...
    @property
    def src_version(self) -> int: ...

class BackendEventRealmRolesUpdated(BackendEvent):
    def __init__(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: RealmID,
        user: UserID,
        role: RealmRole | None,
    ) -> None: ...
    @property
    def author(self) -> DeviceID: ...
    @property
    def realm_id(self) -> RealmID: ...
    @property
    def user(self) -> UserID: ...
    @property
    def role(self) -> RealmRole | None: ...

class BackendEventPkiEnrollmentUpdated(BackendEvent):
    def __init__(
        self,
        organization_id: OrganizationID,
    ) -> None: ...
