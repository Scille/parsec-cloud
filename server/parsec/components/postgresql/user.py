# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import override

from parsec._parsec import (
    DateTime,
    DeviceCertificate,
    DeviceID,
    OrganizationID,
    RevokedUserCertificate,
    UserCertificate,
    UserID,
    UserProfile,
    UserUpdateCertificate,
    VerifyKey,
    VlobID,
)
from parsec.ballpark import RequireGreaterTimestamp, TimestampOutOfBallpark
from parsec.components.events import EventBus
from parsec.components.postgresql import AsyncpgConnection, AsyncpgPool
from parsec.components.postgresql.user_create_device import user_create_device
from parsec.components.postgresql.user_create_user import user_create_user
from parsec.components.postgresql.user_freeze_user import user_freeze_user
from parsec.components.postgresql.user_get_certificates import user_get_certificates
from parsec.components.postgresql.user_get_user_info import (
    user_get_user_info,
    user_get_user_info_from_email,
)
from parsec.components.postgresql.user_list_users import user_list_users
from parsec.components.postgresql.user_revoke_user import user_revoke_user
from parsec.components.postgresql.user_test_dump_current_users import user_test_dump_current_users
from parsec.components.postgresql.user_update_user import user_update_user
from parsec.components.postgresql.utils import (
    Q,
    no_transaction,
    q_organization_internal_id,
    transaction,
)
from parsec.components.realm import CertificateBasedActionIdempotentOutcome
from parsec.components.user import (
    BaseUserComponent,
    CertificatesBundle,
    CheckDeviceBadOutcome,
    GetProfileForUserUserBadOutcome,
    UserCreateDeviceStoreBadOutcome,
    UserCreateDeviceValidateBadOutcome,
    UserCreateUserStoreBadOutcome,
    UserCreateUserValidateBadOutcome,
    UserDump,
    UserFreezeUserBadOutcome,
    UserGetCertificatesAsUserBadOutcome,
    UserInfo,
    UserListUsersBadOutcome,
    UserRevokeUserStoreBadOutcome,
    UserRevokeUserValidateBadOutcome,
    UserUpdateUserStoreBadOutcome,
    UserUpdateUserValidateBadOutcome,
)

_q_get_device = Q(
    f"""
SELECT
    user_.user_id,
    device.verify_key
FROM device
INNER JOIN user_
ON user_._id = device.user_
WHERE
    device.organization = { q_organization_internal_id("$organization_id") }
    AND device.device_id = $device_id
"""
)

_q_get_profile_for_user = Q(
    f"""
SELECT
    COALESCE(user_.current_profile, user_.initial_profile) AS profile,
    user_.revoked_on
FROM user_
WHERE
    organization = { q_organization_internal_id("$organization_id") }
    AND user_id = $user_id
"""
)


def _make_q_lock_common_topic(for_update: bool = False, for_share=False) -> Q:
    assert for_update ^ for_share
    share_or_update = "SHARE" if for_share else "UPDATE"
    return Q(f"""
SELECT last_timestamp
FROM common_topic
JOIN organization ON common_topic.organization = organization._id
WHERE organization_id = $organization_id
FOR {share_or_update}
""")


_q_check_common_topic = _make_q_lock_common_topic(for_share=True)


class PGUserComponent(BaseUserComponent):
    def __init__(self, pool: AsyncpgPool, event_bus: EventBus) -> None:
        super().__init__()
        self.pool = pool
        self.event_bus = event_bus

    async def _check_common_topic(
        self, conn: AsyncpgConnection, organization_id: OrganizationID
    ) -> DateTime:
        common_timestamp = await conn.fetchval(
            *_q_check_common_topic(organization_id=organization_id.str)
        )
        if common_timestamp is None:
            common_timestamp = DateTime.from_timestamp_micros(0)
        return common_timestamp

    async def _check_device(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        device_id: DeviceID,
    ) -> tuple[UserID, UserProfile, DateTime] | CheckDeviceBadOutcome:
        common_timestamp = await self._check_common_topic(conn, organization_id)
        d_row = await conn.fetchrow(
            *_q_get_device(organization_id=organization_id.str, device_id=device_id)
        )
        if not d_row:
            return CheckDeviceBadOutcome.DEVICE_NOT_FOUND
        user_id = UserID.from_hex(d_row["user_id"])
        u_row = await conn.fetchrow(
            *_q_get_profile_for_user(organization_id=organization_id.str, user_id=user_id)
        )
        if not u_row:
            return CheckDeviceBadOutcome.USER_NOT_FOUND
        if u_row["revoked_on"] is not None:
            return CheckDeviceBadOutcome.USER_REVOKED
        return (
            user_id,
            UserProfile.from_str(u_row["profile"]),
            common_timestamp,
        )

    async def _get_profile_for_user(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        user_id: UserID,
        check_common_topic: bool = True,
    ) -> UserProfile | GetProfileForUserUserBadOutcome:
        if check_common_topic:
            await self._check_common_topic(conn, organization_id)
        u_row = await conn.fetchrow(
            *_q_get_profile_for_user(organization_id=organization_id.str, user_id=user_id)
        )
        if not u_row:
            return GetProfileForUserUserBadOutcome.USER_NOT_FOUND
        if u_row["revoked_on"] is not None:
            return GetProfileForUserUserBadOutcome.USER_REVOKED
        return UserProfile.from_str(u_row["profile"])

    @override
    @transaction
    async def create_user(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        author_verify_key: VerifyKey,
        user_certificate: bytes,
        redacted_user_certificate: bytes,
        device_certificate: bytes,
        redacted_device_certificate: bytes,
    ) -> (
        tuple[UserCertificate, DeviceCertificate]
        | UserCreateUserValidateBadOutcome
        | UserCreateUserStoreBadOutcome
        | TimestampOutOfBallpark
        | RequireGreaterTimestamp
    ):
        return await user_create_user(
            self.event_bus,
            conn,
            now,
            organization_id,
            author,
            author_verify_key,
            user_certificate,
            redacted_user_certificate,
            device_certificate,
            redacted_device_certificate,
        )

    @override
    @transaction
    async def create_device(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        author_verify_key: VerifyKey,
        device_certificate: bytes,
        redacted_device_certificate: bytes,
    ) -> (
        DeviceCertificate
        | UserCreateDeviceValidateBadOutcome
        | UserCreateDeviceStoreBadOutcome
        | TimestampOutOfBallpark
        | RequireGreaterTimestamp
    ):
        return await user_create_device(
            self.event_bus,
            conn,
            now,
            organization_id,
            author,
            author_verify_key,
            device_certificate,
            redacted_device_certificate,
        )

    @override
    @transaction
    async def update_user(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        author_verify_key: VerifyKey,
        user_update_certificate: bytes,
    ) -> (
        UserUpdateCertificate
        | UserUpdateUserValidateBadOutcome
        | UserUpdateUserStoreBadOutcome
        | TimestampOutOfBallpark
        | RequireGreaterTimestamp
    ):
        return await user_update_user(
            self.event_bus,
            conn,
            now,
            organization_id,
            author,
            author_verify_key,
            user_update_certificate,
        )

    async def get_user_info(
        self, conn: AsyncpgConnection, organization_id: OrganizationID, user_id: UserID
    ) -> UserInfo | None:
        return await user_get_user_info(conn, organization_id, user_id)

    async def get_user_info_from_email(
        self, conn: AsyncpgConnection, organization_id: OrganizationID, email: str
    ) -> UserInfo | None:
        return await user_get_user_info_from_email(conn, organization_id, email)

    @override
    @no_transaction
    async def list_users(
        self, conn: AsyncpgConnection, organization_id: OrganizationID
    ) -> list[UserInfo] | UserListUsersBadOutcome:
        return await user_list_users(conn, organization_id)

    @override
    @no_transaction
    async def get_certificates(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        author: DeviceID,
        common_after: DateTime | None,
        sequester_after: DateTime | None,
        shamir_recovery_after: DateTime | None,
        realm_after: dict[VlobID, DateTime],
    ) -> CertificatesBundle | UserGetCertificatesAsUserBadOutcome:
        return await user_get_certificates(
            conn,
            organization_id,
            author,
            common_after,
            sequester_after,
            shamir_recovery_after,
            realm_after,
        )

    @override
    @transaction
    async def revoke_user(
        self,
        conn: AsyncpgConnection,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        author_verify_key: VerifyKey,
        revoked_user_certificate: bytes,
    ) -> (
        RevokedUserCertificate
        | CertificateBasedActionIdempotentOutcome
        | UserRevokeUserValidateBadOutcome
        | UserRevokeUserStoreBadOutcome
        | TimestampOutOfBallpark
        | RequireGreaterTimestamp
    ):
        return await user_revoke_user(
            self.event_bus,
            conn,
            now,
            organization_id,
            author,
            author_verify_key,
            revoked_user_certificate,
        )

    @override
    @transaction
    async def freeze_user(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        user_id: UserID | None,
        user_email: str | None,
        frozen: bool,
    ) -> UserInfo | UserFreezeUserBadOutcome:
        return await user_freeze_user(
            self.event_bus, conn, organization_id, user_id, user_email, frozen
        )

    @override
    @no_transaction
    async def test_dump_current_users(
        self, conn: AsyncpgConnection, organization_id: OrganizationID
    ) -> dict[UserID, UserDump]:
        return await user_test_dump_current_users(conn, organization_id)
