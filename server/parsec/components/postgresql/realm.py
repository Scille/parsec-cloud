# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import assert_never, override

import asyncpg

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    RealmKeyRotationCertificate,
    RealmNameCertificate,
    RealmRole,
    RealmRoleCertificate,
    UserID,
    UserProfile,
    VerifyKey,
    VlobID,
)
from parsec.ballpark import RequireGreaterTimestamp, TimestampOutOfBallpark
from parsec.components.events import EventBus
from parsec.components.organization import Organization, OrganizationGetBadOutcome
from parsec.components.postgresql.organization import PGOrganizationComponent
from parsec.components.postgresql.realm_queries.create import query_create
from parsec.components.postgresql.user import PGUserComponent
from parsec.components.postgresql.utils import (
    Q,
    q_device_internal_id,
    q_organization_internal_id,
    q_realm_internal_id,
    q_user,
    q_user_internal_id,
    transaction,
)
from parsec.components.realm import (
    BadKeyIndex,
    BaseRealmComponent,
    CertificateBasedActionIdempotentOutcome,
    KeyIndex,
    RealmCheckBadOutcome,
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
from parsec.components.user import CheckUserWithDeviceBadOutcome
from parsec.events import EventRealmCertificate


def q_user_role(
    user_id: str,
    realm: str,
    organization: str,
) -> str:
    return f"""
COALESCE(
    (
        SELECT realm_user_role.role
        FROM realm_user_role
        WHERE
            realm_user_role.realm = { realm }
            AND realm_user_role.user_ = { q_user_internal_id(organization=organization, user_id=user_id) }
        ORDER BY certified_on DESC
        LIMIT 1
    ),
    NULL
)
"""


q_check_realm = Q(
    f"""
    SELECT
        { q_user_role(organization="realm.organization", realm="realm._id", user_id="$user_id") } role,
        key_index
    FROM realm
    WHERE
        organization = { q_organization_internal_id("$organization_id") }
        AND realm_id = $realm_id
"""
)

q_get_current_roles = Q(
    f"""
SELECT DISTINCT ON(user_) { q_user(_id="realm_user_role.user_", select="user_id") }, role
FROM  realm_user_role
WHERE realm = { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") }
ORDER BY user_, certified_on DESC
"""
)

q_insert_keys_bundle = Q(
    f"""
INSERT INTO realm_keys_bundle (
    realm,
    key_index,
    realm_key_rotation_certificate,
    certified_by,
    certified_on,
    key_canary,
    keys_bundle
) VALUES (
    { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") },
    $key_index,
    $realm_key_rotation_certificate,
    { q_device_internal_id(organization_id="$organization_id", device_id="$certified_by") },
    $certified_on,
    $key_canary,
    $keys_bundle
)
RETURNING _id
"""
)

q_insert_keys_bundle_access = Q(
    f"""
INSERT INTO realm_keys_bundle_access (
    realm,
    user_,
    realm_keys_bundle,
    access
) VALUES (
    { q_realm_internal_id(organization_id="$organization_id", realm_id="$realm_id") },
    { q_user_internal_id(organization_id="$organization_id", user_id="$user_id") },
    $realm_keys_bundle_internal_id,
    $access
)
"""
)


q_update_key_index = Q(
    f"""
UPDATE realm
SET key_index = $key_index
WHERE
    organization = { q_organization_internal_id("$organization_id") }
    AND realm_id = $realm_id
"""
)


class PGRealmComponent(BaseRealmComponent):
    def __init__(self, pool: asyncpg.Pool, event_bus: EventBus):
        super().__init__()
        self.pool = pool
        self.event_bus = event_bus

    def register_components(
        self, organization: PGOrganizationComponent, user: PGUserComponent, **kwargs
    ) -> None:
        self._organization = organization
        self._user = user

    async def _check_realm(
        self,
        conn: asyncpg.Connection,
        organization_id: OrganizationID,
        realm_id: VlobID,
        author: DeviceID,
    ) -> tuple[RealmRole, KeyIndex] | RealmCheckBadOutcome:
        row = await conn.fetchrow(
            *q_check_realm(
                organization_id=organization_id.str,
                realm_id=realm_id,
                user_id=author.user_id.str,
            )
        )
        if not row:
            return RealmCheckBadOutcome.REALM_NOT_FOUND
        if row["role"] is None:
            return RealmCheckBadOutcome.USER_NOT_IN_REALM
        return RealmRole.from_str(row["role"]), int(row["key_index"])

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
        match await self._organization._get(conn, organization_id):
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return RealmCreateStoreBadOutcome.ORGANIZATION_NOT_FOUND
            case Organization() as org:
                pass
            case unknown:
                assert_never(unknown)

        if org.is_expired:
            return RealmCreateStoreBadOutcome.ORGANIZATION_EXPIRED

        match await self._user._check_user(conn, organization_id, author):
            case CheckUserWithDeviceBadOutcome.DEVICE_NOT_FOUND:
                return RealmCreateStoreBadOutcome.AUTHOR_NOT_FOUND
            case CheckUserWithDeviceBadOutcome.USER_NOT_FOUND:
                return RealmCreateStoreBadOutcome.AUTHOR_NOT_FOUND
            case CheckUserWithDeviceBadOutcome.USER_REVOKED:
                return RealmCreateStoreBadOutcome.AUTHOR_REVOKED
            case UserProfile():
                pass
            case unknown:
                assert_never(unknown)

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

        # All checks are good, now we do the actual insertion
        match await query_create(
            conn=conn,
            organization_id=organization_id,
            realm_role_certificate=realm_role_certificate,
            realm_role_certificate_cooked=certif,
        ):
            case CertificateBasedActionIdempotentOutcome() as outcome:
                return outcome
            case None:
                pass
            case unknown:
                assert_never(unknown)

        await self.event_bus.send(
            EventRealmCertificate(
                organization_id=organization_id,
                timestamp=certif.timestamp,
                realm_id=certif.realm_id,
                user_id=certif.user_id,
                role_removed=certif.role is None,
            )
        )

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
        match await self._organization._get(conn, organization_id):
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return RealmRotateKeyStoreBadOutcome.ORGANIZATION_NOT_FOUND
            case Organization() as org:
                pass
            case unknown:
                assert_never(unknown)

        if org.is_expired:
            return RealmRotateKeyStoreBadOutcome.ORGANIZATION_EXPIRED

        match await self._user._check_user(conn, organization_id, author):
            case CheckUserWithDeviceBadOutcome.DEVICE_NOT_FOUND:
                return RealmRotateKeyStoreBadOutcome.AUTHOR_NOT_FOUND
            case CheckUserWithDeviceBadOutcome.USER_NOT_FOUND:
                return RealmRotateKeyStoreBadOutcome.AUTHOR_NOT_FOUND
            case CheckUserWithDeviceBadOutcome.USER_REVOKED:
                return RealmRotateKeyStoreBadOutcome.AUTHOR_REVOKED
            case UserProfile():
                pass
            case unknown:
                assert_never(unknown)

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

        match await self._check_realm(conn, organization_id, certif.realm_id, author):
            case RealmCheckBadOutcome.REALM_NOT_FOUND:
                return RealmRotateKeyStoreBadOutcome.REALM_NOT_FOUND
            case RealmCheckBadOutcome.USER_NOT_IN_REALM:
                return RealmRotateKeyStoreBadOutcome.AUTHOR_NOT_ALLOWED
            case (RealmRole() as role, KeyIndex() as realm_key_index):
                if role not in (RealmRole.OWNER, RealmRole.MANAGER, RealmRole.CONTRIBUTOR):
                    return RealmRotateKeyStoreBadOutcome.AUTHOR_NOT_ALLOWED
            case unknown:
                assert_never(unknown)

        if certif.key_index != realm_key_index + 1:
            return BadKeyIndex(
                last_realm_certificate_timestamp=certif.timestamp,  # TODO: this is not the right timestamp
            )

        participants = await self._get_users_in_realm(conn, organization_id, certif.realm_id)
        if per_participant_keys_bundle_access.keys() != participants:
            return RealmRotateKeyStoreBadOutcome.PARTICIPANT_MISMATCH

        await self._add_realm_key_rotation_certificate(
            conn=conn,
            organization_id=organization_id,
            realm_key_rotation_certificate_cooked=certif,
            realm_key_rotation_certificate=realm_key_rotation_certificate,
            per_participant_keys_bundle_access=per_participant_keys_bundle_access,
            keys_bundle=keys_bundle,
        )

        await self.event_bus.send(
            EventRealmCertificate(
                organization_id=organization_id,
                timestamp=certif.timestamp,
                realm_id=certif.realm_id,
                user_id=author.user_id,
                role_removed=False,
            )
        )

        return certif

    async def _get_users_in_realm(
        self, conn: asyncpg.Connection, organization_id: OrganizationID, realm_id: VlobID
    ) -> set[UserID]:
        rows = await conn.fetch(
            *q_get_current_roles(organization_id=organization_id.str, realm_id=realm_id)
        )
        return {UserID(row["user_id"]) for row in rows if row["role"] is not None}

    async def _add_realm_key_rotation_certificate(
        self,
        conn: asyncpg.Connection,
        organization_id: OrganizationID,
        realm_key_rotation_certificate_cooked: RealmKeyRotationCertificate,
        realm_key_rotation_certificate: bytes,
        per_participant_keys_bundle_access: dict[UserID, bytes],
        keys_bundle: bytes,
    ) -> None:
        assert realm_key_rotation_certificate_cooked.author is not None
        keys_bundle_internal_id = await conn.fetchval(
            *q_insert_keys_bundle(
                organization_id=organization_id.str,
                realm_id=realm_key_rotation_certificate_cooked.realm_id,
                key_index=realm_key_rotation_certificate_cooked.key_index,
                realm_key_rotation_certificate=realm_key_rotation_certificate,
                certified_by=realm_key_rotation_certificate_cooked.author.str,
                certified_on=realm_key_rotation_certificate_cooked.timestamp,
                key_canary=realm_key_rotation_certificate_cooked.key_canary,
                keys_bundle=keys_bundle,
            )
        )
        assert isinstance(keys_bundle_internal_id, int)

        def arg_gen():
            for user_id, access in per_participant_keys_bundle_access.items():
                x = q_insert_keys_bundle_access.arg_only(
                    organization_id=organization_id.str,
                    realm_id=realm_key_rotation_certificate_cooked.realm_id,
                    realm_keys_bundle_internal_id=keys_bundle_internal_id,
                    user_id=user_id.str,
                    access=access,
                )
                yield x

        await conn.executemany(
            q_insert_keys_bundle_access.sql,
            arg_gen(),
        )
        await conn.execute(
            *q_update_key_index(
                organization_id=organization_id.str,
                realm_id=realm_key_rotation_certificate_cooked.realm_id,
                key_index=realm_key_rotation_certificate_cooked.key_index,
            )
        )

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
