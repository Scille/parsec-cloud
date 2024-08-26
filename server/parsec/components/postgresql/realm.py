# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import override

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    RealmKeyRotationCertificate,
    RealmNameCertificate,
    RealmRole,
    RealmRoleCertificate,
    UserID,
    VerifyKey,
    VlobID,
)
from parsec.ballpark import RequireGreaterTimestamp, TimestampOutOfBallpark
from parsec.components.events import EventBus
from parsec.components.postgresql import AsyncpgConnection, AsyncpgPool
from parsec.components.postgresql.realm_create import realm_create
from parsec.components.postgresql.realm_get_current_realms_for_user import (
    realm_get_current_realms_for_user,
)
from parsec.components.postgresql.realm_get_keys_bundle import realm_get_keys_bundle
from parsec.components.postgresql.realm_rename import realm_rename
from parsec.components.postgresql.realm_rotate_key import realm_rotate_key
from parsec.components.postgresql.realm_share import realm_share
from parsec.components.postgresql.realm_unshare import realm_unshare
from parsec.components.postgresql.utils import (
    no_transaction,
    transaction,
)
from parsec.components.realm import (
    BadKeyIndex,
    BaseRealmComponent,
    CertificateBasedActionIdempotentOutcome,
    KeysBundle,
    RealmCreateStoreBadOutcome,
    RealmCreateValidateBadOutcome,
    RealmGetCurrentRealmsForUserBadOutcome,
    RealmGetKeysBundleBadOutcome,
    RealmRenameStoreBadOutcome,
    RealmRenameValidateBadOutcome,
    RealmRotateKeyStoreBadOutcome,
    RealmRotateKeyValidateBadOutcome,
    RealmShareStoreBadOutcome,
    RealmShareValidateBadOutcome,
    RealmUnshareStoreBadOutcome,
    RealmUnshareValidateBadOutcome,
)


class PGRealmComponent(BaseRealmComponent):
    def __init__(self, pool: AsyncpgPool, event_bus: EventBus):
        super().__init__()
        self.pool = pool
        self.event_bus = event_bus

    @override
    @transaction
    async def create(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        author_verify_key: VerifyKey,
        realm_role_certificate: bytes,
    ) -> (
        RealmRoleCertificate
        | CertificateBasedActionIdempotentOutcome
        | RealmCreateValidateBadOutcome
        | TimestampOutOfBallpark
        | RealmCreateStoreBadOutcome
        | RequireGreaterTimestamp
    ):
        return await realm_create(
            self.event_bus,
            conn,
            now,
            organization_id,
            author,
            author_verify_key,
            realm_role_certificate,
        )

    @override
    @transaction
    async def share(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        author_verify_key: VerifyKey,
        realm_role_certificate: bytes,
        recipient_keys_bundle_access: bytes,
        key_index: int,
    ) -> (
        RealmRoleCertificate
        | BadKeyIndex
        | CertificateBasedActionIdempotentOutcome
        | RealmShareValidateBadOutcome
        | TimestampOutOfBallpark
        | RealmShareStoreBadOutcome
        | RequireGreaterTimestamp
    ):
        return await realm_share(
            self.event_bus,
            conn,
            now,
            organization_id,
            author,
            author_verify_key,
            realm_role_certificate,
            recipient_keys_bundle_access,
            key_index,
        )

    @override
    @transaction
    async def unshare(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        author_verify_key: VerifyKey,
        realm_role_certificate: bytes,
    ) -> (
        RealmRoleCertificate
        | CertificateBasedActionIdempotentOutcome
        | RealmUnshareValidateBadOutcome
        | TimestampOutOfBallpark
        | RealmUnshareStoreBadOutcome
        | RequireGreaterTimestamp
    ):
        return await realm_unshare(
            self.event_bus,
            conn,
            now,
            organization_id,
            author,
            author_verify_key,
            realm_role_certificate,
        )

    @override
    @transaction
    async def rename(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        author_verify_key: VerifyKey,
        realm_name_certificate: bytes,
        initial_name_or_fail: bool,
    ) -> (
        RealmNameCertificate
        | BadKeyIndex
        | CertificateBasedActionIdempotentOutcome
        | RealmRenameValidateBadOutcome
        | TimestampOutOfBallpark
        | RealmRenameStoreBadOutcome
        | RequireGreaterTimestamp
    ):
        return await realm_rename(
            self.event_bus,
            conn,
            now,
            organization_id,
            author,
            author_verify_key,
            realm_name_certificate,
            initial_name_or_fail,
        )

    @override
    @transaction
    async def rotate_key(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        author_verify_key: VerifyKey,
        realm_key_rotation_certificate: bytes,
        per_participant_keys_bundle_access: dict[UserID, bytes],
        keys_bundle: bytes,
    ) -> (
        RealmKeyRotationCertificate
        | BadKeyIndex
        | RealmRotateKeyValidateBadOutcome
        | TimestampOutOfBallpark
        | RealmRotateKeyStoreBadOutcome
        | RequireGreaterTimestamp
    ):
        return await realm_rotate_key(
            self.event_bus,
            conn,
            now,
            organization_id,
            author,
            author_verify_key,
            realm_key_rotation_certificate,
            per_participant_keys_bundle_access,
            keys_bundle,
        )

    @override
    @no_transaction
    async def get_keys_bundle(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: VlobID,
        key_index: int | None,
    ) -> KeysBundle | RealmGetKeysBundleBadOutcome:
        return await realm_get_keys_bundle(conn, organization_id, author, realm_id, key_index)

    @override
    @no_transaction
    async def get_current_realms_for_user(
        self, conn: AsyncpgConnection, organization_id: OrganizationID, user: UserID
    ) -> dict[VlobID, RealmRole] | RealmGetCurrentRealmsForUserBadOutcome:
        return await realm_get_current_realms_for_user(conn, organization_id, user)
