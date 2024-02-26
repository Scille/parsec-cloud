# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import override

import asyncpg

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    RealmKeyRotationCertificate,
    RealmNameCertificate,
    RealmRoleCertificate,
    UserID,
    VerifyKey,
)
from parsec.ballpark import RequireGreaterTimestamp, TimestampOutOfBallpark
from parsec.components.postgresql.utils import transaction
from parsec.components.realm import (
    BadKeyIndex,
    BaseRealmComponent,
    CertificateBasedActionIdempotentOutcome,
    RealmCreateStoreBadOutcome,
    RealmCreateValidateBadOutcome,
    RealmRenameStoreBadOutcome,
    RealmRenameValidateBadOutcome,
    RealmRotateKeyStoreBadOutcome,
    RealmRotateKeyValidateBadOutcome,
    RealmShareStoreBadOutcome,
    RealmShareValidateBadOutcome,
    realm_create_validate,
    realm_rename_validate,
    realm_rotate_key_validate,
    realm_share_validate,
)


class PGRealmComponent(BaseRealmComponent):
    def __init__(self, pool: asyncpg.Pool):
        super().__init__()
        self.pool = pool

    @override
    @transaction
    async def create(
        self,
        conn: asyncpg.Connection,
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
        # TODO: we pretend that the realm has been created to use the CoolOrg template
        # await query_create(conn, organization_id, self_granted_role)

        match realm_create_validate(
            now=now,
            expected_author=author,
            author_verify_key=author_verify_key,
            realm_role_certificate=realm_role_certificate,
        ):
            case RealmRoleCertificate() as certif:
                pass
            case error:
                return error
        return certif

    @override
    @transaction
    async def share(
        self,
        conn: asyncpg.Connection,
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
        # TODO: we pretend that the realm has been shared to use the CoolOrg template

        match realm_share_validate(
            now=now,
            expected_author=author,
            author_verify_key=author_verify_key,
            realm_role_certificate=realm_role_certificate,
        ):
            case RealmRoleCertificate() as certif:
                pass
            case error:
                return error
        return certif

    @override
    @transaction
    async def rename(
        self,
        conn: asyncpg.Connection,
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
        # TODO: we pretend that the realm has been renamed to use the CoolOrg template

        match realm_rename_validate(
            now=now,
            expected_author=author,
            author_verify_key=author_verify_key,
            realm_name_certificate=realm_name_certificate,
        ):
            case RealmNameCertificate() as certif:
                pass
            case error:
                return error
        return certif

    @override
    @transaction
    async def rotate_key(
        self,
        conn: asyncpg.Connection,
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
        # TODO: we pretend that the realm has been rotated to use the CoolOrg template
        match realm_rotate_key_validate(
            now=now,
            expected_author=author,
            author_verify_key=author_verify_key,
            realm_key_rotation_certificate=realm_key_rotation_certificate,
        ):
            case RealmKeyRotationCertificate() as certif:
                pass
            case error:
                return error

        return certif

    # async def get_status(
    #     self, organization_id: OrganizationID, author: DeviceID, realm_id: VlobID
    # ) -> RealmStatus:
    #     async with self.dbh.pool.acquire() as conn:
    #         return await query_get_status(conn, organization_id, author, realm_id)

    # async def get_stats(
    #     self, organization_id: OrganizationID, author: DeviceID, realm_id: VlobID
    # ) -> RealmStats:
    #     async with self.dbh.pool.acquire() as conn:
    #         return await query_get_stats(conn, organization_id, author, realm_id)

    # async def get_current_roles(
    #     self, organization_id: OrganizationID, realm_id: VlobID
    # ) -> dict[UserID, RealmRole]:
    #     async with self.dbh.pool.acquire() as conn:
    #         return await query_get_current_roles(conn, organization_id, realm_id)

    # async def get_role_certificates(
    #     self, organization_id: OrganizationID, author: DeviceID, realm_id: VlobID
    # ) -> list[bytes]:
    #     async with self.dbh.pool.acquire() as conn:
    #         return await query_get_role_certificates(conn, organization_id, author, realm_id)

    # async def get_realms_for_user(
    #     self, organization_id: OrganizationID, user: UserID
    # ) -> dict[VlobID, RealmRole]:
    #     async with self.dbh.pool.acquire() as conn:
    #         return await query_get_realms_for_user(conn, organization_id, user)

    # async def update_roles(
    #     self,
    #     organization_id: OrganizationID,
    #     new_role: RealmGrantedRole,
    #     recipient_message: bytes | None = None,
    # ) -> None:
    #     async with self.dbh.pool.acquire() as conn:
    #         await query_update_roles(conn, organization_id, new_role, recipient_message)

    # async def dump_realms_granted_roles(
    #     self, organization_id: OrganizationID
    # ) -> list[RealmGrantedRole]:
    #     async with self.dbh.pool.acquire() as conn:
    #         return await query_dump_realms_granted_roles(conn, organization_id)
