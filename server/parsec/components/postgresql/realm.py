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
    SequesterServiceID,
    UserID,
    VerifyKey,
    VlobID,
)
from parsec.ballpark import RequireGreaterTimestamp, TimestampOutOfBallpark
from parsec.components.postgresql import AsyncpgConnection, AsyncpgPool
from parsec.components.postgresql.realm_create import realm_create
from parsec.components.postgresql.realm_dump_realms_granted_roles import (
    realm_dump_realms_granted_roles,
)
from parsec.components.postgresql.realm_export_do_base_info import (
    realm_export_do_base_info,
)
from parsec.components.postgresql.realm_export_do_blocks_metadata_batch import (
    realm_export_do_blocks_metadata_batch,
)
from parsec.components.postgresql.realm_export_do_certificates import (
    realm_export_do_certificates,
)
from parsec.components.postgresql.realm_export_do_vlobs_batch import (
    realm_export_do_vlobs_batch,
)
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
    ParticipantMismatch,
    RealmCreateStoreBadOutcome,
    RealmCreateValidateBadOutcome,
    RealmDumpRealmsGrantedRolesBadOutcome,
    RealmExportBatchOffsetMarker,
    RealmExportBlocksMetadataBatch,
    RealmExportCertificates,
    RealmExportDoBaseInfo,
    RealmExportDoBaseInfoBadOutcome,
    RealmExportDoBlocksBatchMetadataBadOutcome,
    RealmExportDoCertificatesBadOutcome,
    RealmExportDoVlobsBatchBadOutcome,
    RealmExportVlobsBatch,
    RealmGetCurrentRealmsForUserBadOutcome,
    RealmGetKeysBundleBadOutcome,
    RealmGrantedRole,
    RealmRenameStoreBadOutcome,
    RealmRenameValidateBadOutcome,
    RealmRotateKeyStoreBadOutcome,
    RealmRotateKeyValidateBadOutcome,
    RealmShareStoreBadOutcome,
    RealmShareValidateBadOutcome,
    RealmUnshareStoreBadOutcome,
    RealmUnshareValidateBadOutcome,
    RejectedBySequesterService,
    SequesterServiceMismatch,
    SequesterServiceUnavailable,
)
from parsec.webhooks import WebhooksComponent


class PGRealmComponent(BaseRealmComponent):
    def __init__(self, pool: AsyncpgPool, webhooks: WebhooksComponent):
        super().__init__(webhooks)
        self.pool = pool

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
        # Sequester is a special case, so gives it a default version to simplify tests
        per_sequester_service_keys_bundle_access: dict[SequesterServiceID, bytes] | None = None,
    ) -> (
        RealmKeyRotationCertificate
        | BadKeyIndex
        | RealmRotateKeyValidateBadOutcome
        | TimestampOutOfBallpark
        | RealmRotateKeyStoreBadOutcome
        | RequireGreaterTimestamp
        | ParticipantMismatch
        | SequesterServiceMismatch
        | SequesterServiceUnavailable
        | RejectedBySequesterService
    ):
        return await realm_rotate_key(
            conn,
            now,
            organization_id,
            author,
            author_verify_key,
            realm_key_rotation_certificate,
            per_participant_keys_bundle_access,
            keys_bundle,
            per_sequester_service_keys_bundle_access,
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

    @override
    @no_transaction
    async def dump_realms_granted_roles(
        self, conn: AsyncpgConnection, organization_id: OrganizationID
    ) -> list[RealmGrantedRole] | RealmDumpRealmsGrantedRolesBadOutcome:
        return await realm_dump_realms_granted_roles(conn, organization_id)

    @override
    @no_transaction
    async def export_do_base_info(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        realm_id: VlobID,
        snapshot_timestamp: DateTime,
    ) -> RealmExportDoBaseInfo | RealmExportDoBaseInfoBadOutcome:
        return await realm_export_do_base_info(conn, organization_id, realm_id, snapshot_timestamp)

    @override
    @no_transaction
    async def export_do_certificates(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        realm_id: VlobID,
        common_certificate_timestamp_upper_bound: DateTime,
        realm_certificate_timestamp_upper_bound: DateTime,
        sequester_certificate_timestamp_upper_bound: DateTime | None,
    ) -> RealmExportCertificates | RealmExportDoCertificatesBadOutcome:
        return await realm_export_do_certificates(
            conn,
            organization_id,
            realm_id,
            common_certificate_timestamp_upper_bound,
            realm_certificate_timestamp_upper_bound,
            sequester_certificate_timestamp_upper_bound,
        )

    @override
    @no_transaction
    async def export_do_vlobs_batch(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        realm_id: VlobID,
        batch_offset_marker: RealmExportBatchOffsetMarker,
        batch_size: int,
    ) -> RealmExportVlobsBatch | RealmExportDoVlobsBatchBadOutcome:
        return await realm_export_do_vlobs_batch(
            conn, organization_id, realm_id, batch_offset_marker, batch_size
        )

    @override
    @no_transaction
    async def export_do_blocks_metadata_batch(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        realm_id: VlobID,
        batch_offset_marker: RealmExportBatchOffsetMarker,
        batch_size: int,
    ) -> RealmExportBlocksMetadataBatch | RealmExportDoBlocksBatchMetadataBadOutcome:
        return await realm_export_do_blocks_metadata_batch(
            conn, organization_id, realm_id, batch_offset_marker, batch_size
        )
