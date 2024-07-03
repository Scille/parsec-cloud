# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import itertools
from typing import TYPE_CHECKING, override

from asyncpg import UniqueViolationError

from parsec._parsec import (
    DateTime,
    DeviceCertificate,
    DeviceID,
    HumanHandle,
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
from parsec.components.organization import Organization, OrganizationGetBadOutcome
from parsec.components.postgresql import AsyncpgConnection, AsyncpgPool
from parsec.components.postgresql.utils import (
    Q,
    q_device_internal_id,
    q_human_internal_id,
    q_organization_internal_id,
    q_user_internal_id,
    transaction,
)
from parsec.components.realm import CertificateBasedActionIdempotentOutcome
from parsec.components.user import (
    BaseUserComponent,
    CertificatesBundle,
    CheckDeviceBadOutcome,
    CheckUserBadOutcome,
    CheckUserForAuthenticationBadOutcome,
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
    user_create_device_validate,
    user_create_user_validate,
    user_revoke_user_validate,
    user_update_user_validate,
)
from parsec.events import (
    EventCommonCertificate,
    EventUserRevokedOrFrozen,
    EventUserUnfrozen,
    EventUserUpdated,
)

if TYPE_CHECKING:
    from parsec.components.postgresql.organization import PGOrganizationComponent
    from parsec.components.postgresql.realm import PGRealmComponent

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
    COALESCE(profile.profile, user_.initial_profile) AS profile,
    user_.revoked_on
FROM user_
LEFT JOIN profile ON user_._id = profile.user_
WHERE
    organization = { q_organization_internal_id("$organization_id") }
    AND user_id = $user_id
ORDER BY profile.certified_on DESC LIMIT 1
"""
)


def _make_q_get_user_for(condition: str) -> Q:
    return Q(f"""
SELECT
    user_.user_id,
    user_.frozen,
    user_.revoked_on,
    human.email,
    human.label
FROM user_
INNER JOIN human ON user_.human = human._id
WHERE
    user_.organization = { q_organization_internal_id("$organization_id") }
    AND {condition}
""")


_q_get_user = _make_q_get_user_for("user_.user_id = $user_id")
_q_get_user_for_email = _make_q_get_user_for("human.email = $email")

_q_freeze_user = Q(
    f"""
UPDATE user_ SET
    frozen = $frozen
WHERE
    organization = { q_organization_internal_id("$organization_id") }
    AND user_id = $user_id
"""
)

_q_revoke_user = Q(
    f"""
UPDATE user_ SET
    revoked_user_certificate = $revoked_user_certificate,
    revoked_user_certifier = { q_device_internal_id(organization_id="$organization_id", device_id="$revoked_user_certifier") },
    revoked_on = $revoked_on
WHERE
    organization = { q_organization_internal_id("$organization_id") }
    AND user_id = $user_id
    AND revoked_on IS NULL
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
_q_lock_common_topic = _make_q_lock_common_topic(for_update=True)

_q_update_common_topic = Q(
    f"""
UPDATE common_topic
SET last_timestamp = $last_timestamp
WHERE organization = { q_organization_internal_id("$organization_id") }
"""
)

_q_update_user = Q(
    f"""
INSERT INTO profile (user_, profile, profile_certificate, certified_by, certified_on)
VALUES (
    { q_user_internal_id(organization_id="$organization_id", user_id="$user_id") },
    $profile,
    $profile_certificate,
    { q_device_internal_id(organization_id="$organization_id", device_id="$certified_by") },
    $certified_on
)
"""
)

_q_get_user_certificates = Q(
    f"""
SELECT
    created_on,
    user_certificate
FROM user_
WHERE organization = { q_organization_internal_id("$organization_id") }
AND COALESCE(created_on > $after, TRUE)
"""
)

_q_get_redacted_user_certificates = Q(
    f"""
SELECT
    created_on,
    redacted_user_certificate AS user_certificate
FROM user_
WHERE organization = { q_organization_internal_id("$organization_id") }
AND COALESCE(created_on > $after, TRUE)
"""
)

_q_get_user_revoked_certificates = Q(
    f"""
SELECT
    revoked_on,
    revoked_user_certificate
FROM user_
WHERE organization = { q_organization_internal_id("$organization_id") }
AND revoked_on IS NOT NULL
AND COALESCE(revoked_on > $after, TRUE)
"""
)

_q_get_user_update_certificates = Q(
    f"""
SELECT
    certified_on,
    profile_certificate
FROM profile
INNER JOIN user_ ON profile.user_ = user_._id
WHERE user_.organization = { q_organization_internal_id("$organization_id") }
AND COALESCE(certified_on > $after, TRUE)
"""
)

_q_get_device_certificates = Q(
    f"""
SELECT
    created_on,
    device_certificate
FROM device
WHERE organization = { q_organization_internal_id("$organization_id") }
AND COALESCE(created_on > $after, TRUE)
"""
)

_q_get_device_redacted_certificates = Q(
    f"""
SELECT
    created_on,
    redacted_device_certificate AS device_certificate
FROM device
WHERE organization = { q_organization_internal_id("$organization_id") }
AND COALESCE(created_on > $after, TRUE)
"""
)

_q_get_organization_users = Q(
    f"""
SELECT DISTINCT ON(user_._id)
    user_.user_id,
    user_.frozen,
    user_.created_on,
    user_.revoked_on,
    COALESCE(profile.profile, user_.initial_profile) AS current_profile,
    human.email AS human_email,
    human.label AS human_label
FROM user_
LEFT JOIN profile ON user_._id = profile.user_
INNER JOIN human ON human._id = user_.human
WHERE
    user_.organization = { q_organization_internal_id("$organization_id") }
ORDER BY user_._id, profile.certified_on DESC
"""
)

_q_get_organization_devices = Q(
    f"""
SELECT
    user_.user_id,
    device.device_id,
    device.created_on
FROM device
INNER JOIN user_ ON user_._id = device.user_
WHERE
    device.organization = { q_organization_internal_id("$organization_id") }
"""
)

_q_check_active_users_limit = Q(
    """
    SELECT
        (
            organization.active_users_limit IS NULL
            OR (
                SELECT
                    count(*)
                FROM
                    user_
                WHERE
                    user_.organization = organization._id AND
                    user_.revoked_on IS NULL
            ) < organization.active_users_limit
        ) AS allowed
    FROM
        organization
    WHERE
        organization.organization_id = $organization_id
"""
)

_q_insert_human_if_not_exists = Q(
    f"""
INSERT INTO human (organization, email, label)
VALUES (
    { q_organization_internal_id("$organization_id") },
    $email,
    $label
)
ON CONFLICT DO NOTHING
"""
)

_q_insert_user_with_human_handle = Q(
    f"""
INSERT INTO user_ (
    organization,
    user_id,
    initial_profile,
    user_certificate,
    redacted_user_certificate,
    user_certifier,
    created_on,
    human
)
VALUES (
    { q_organization_internal_id("$organization_id") },
    $user_id,
    $initial_profile,
    $user_certificate,
    $redacted_user_certificate,
    { q_device_internal_id(organization_id="$organization_id", device_id="$user_certifier") },
    $created_on,
    { q_human_internal_id(organization_id="$organization_id", email="$email") }
)
"""
)

_q_get_active_users_for_human = Q(
    f"""
SELECT user_id
FROM user_
WHERE
    human = { q_human_internal_id(organization_id="$organization_id", email="$email") }
    AND (
        revoked_on IS NULL
        OR revoked_on > $now
    )
"""
)

_q_get_devices_for_user = Q(
    f"""
SELECT device_id
FROM device
WHERE user_ = { q_user_internal_id(organization_id="$organization_id", user_id="$user_id") }
"""
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

_q_insert_device = Q(
    f"""
INSERT INTO device (
    organization,
    user_,
    device_id,
    device_label,
    verify_key,
    device_certificate,
    redacted_device_certificate,
    device_certifier,
    created_on
)
VALUES (
    { q_organization_internal_id("$organization_id") },
    { q_user_internal_id(organization_id="$organization_id", user_id="$user_id") },
    $device_id,
    $device_label,
    $verify_key,
    $device_certificate,
    $redacted_device_certificate,
    { q_device_internal_id(organization_id="$organization_id", device_id="$device_certifier") },
    $created_on
)
"""
)


async def query_list_users(
    conn: AsyncpgConnection, organization_id: OrganizationID
) -> list[UserInfo]:
    users = []

    rows = await conn.fetch(*_q_get_organization_users(organization_id=organization_id.str))
    for row in rows:
        users.append(
            UserInfo(
                user_id=UserID.from_hex(row["user_id"]),
                human_handle=HumanHandle(email=row["human_email"], label=row["human_label"]),
                frozen=row["frozen"],
            )
        )
    return users


class PGUserComponent(BaseUserComponent):
    def __init__(self, pool: AsyncpgPool, event_bus: EventBus) -> None:
        super().__init__()
        self.pool = pool
        self.event_bus = event_bus
        self.organization: PGOrganizationComponent
        self.realm: PGRealmComponent

    def register_components(
        self, organization: PGOrganizationComponent, realm: PGRealmComponent, **kwargs
    ) -> None:
        self.organization = organization
        self.realm = realm

    async def _create_user(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        user_certificate_cooked: UserCertificate,
        user_certificate: bytes,
        user_certificate_redacted: bytes,
    ) -> None | UserCreateUserStoreBadOutcome:
        assert user_certificate_cooked.human_handle is not None
        # Create human handle if needed
        await conn.execute(
            *_q_insert_human_if_not_exists(
                organization_id=organization_id.str,
                email=user_certificate_cooked.human_handle.email,
                label=user_certificate_cooked.human_handle.label,
            )
        )

        # Now insert the new user
        try:
            user_certifier = (
                user_certificate_cooked.author if user_certificate_cooked.author else None
            )
            result = await conn.execute(
                *_q_insert_user_with_human_handle(
                    organization_id=organization_id.str,
                    user_id=user_certificate_cooked.user_id,
                    initial_profile=user_certificate_cooked.profile.str,
                    user_certificate=user_certificate,
                    redacted_user_certificate=user_certificate_redacted,
                    user_certifier=user_certifier,
                    created_on=user_certificate_cooked.timestamp,
                    email=user_certificate_cooked.human_handle.email,
                )
            )

        except UniqueViolationError:
            return UserCreateUserStoreBadOutcome.USER_ALREADY_EXISTS

        if result != "INSERT 0 1":
            assert False, f"Insertion error: {result}"

        # Finally make sure there is only one active user with this human handle
        now = DateTime.now()
        active_users = await conn.fetch(
            *_q_get_active_users_for_human(
                organization_id=organization_id.str,
                email=user_certificate_cooked.human_handle.email,
                now=now,
            )
        )
        if (
            len(active_users) != 1
            or UserID.from_hex(active_users[0]["user_id"]) != user_certificate_cooked.user_id
        ):
            # Exception cancels the transaction so the user insertion is automatically cancelled
            return UserCreateUserStoreBadOutcome.HUMAN_HANDLE_ALREADY_TAKEN

        # Update the common topic
        result = await conn.execute(
            *_q_update_common_topic(
                organization_id=organization_id.str,
                last_timestamp=user_certificate_cooked.timestamp,
            )
        )
        assert result == "UPDATE 1", f"Unexpected {result}"

    async def _create_device(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        device_certificate_cooked: DeviceCertificate,
        device_certificate: bytes,
        device_certificate_redacted: bytes,
        first_device: bool = False,
    ) -> None | UserCreateDeviceStoreBadOutcome:
        if not first_device:
            existing_devices = await conn.fetch(
                *_q_get_devices_for_user(
                    organization_id=organization_id.str,
                    user_id=device_certificate_cooked.user_id,
                )
            )
            if not existing_devices:
                return UserCreateDeviceStoreBadOutcome.AUTHOR_NOT_FOUND

            if device_certificate_cooked.device_id in itertools.chain(*existing_devices):
                return UserCreateDeviceStoreBadOutcome.DEVICE_ALREADY_EXISTS

        try:
            device_certifier = (
                device_certificate_cooked.author if device_certificate_cooked.author else None
            )
            result = await conn.execute(
                *_q_insert_device(
                    organization_id=organization_id.str,
                    user_id=device_certificate_cooked.user_id,
                    device_id=device_certificate_cooked.device_id,
                    device_label=device_certificate_cooked.device_label.str,
                    verify_key=device_certificate_cooked.verify_key.encode(),
                    device_certificate=device_certificate,
                    redacted_device_certificate=device_certificate_redacted,
                    device_certifier=device_certifier,
                    created_on=device_certificate_cooked.timestamp,
                )
            )
        except UniqueViolationError:
            return UserCreateDeviceStoreBadOutcome.DEVICE_ALREADY_EXISTS

        assert result == "INSERT 0 1", f"Insertion error: {result}"

        # Update the common topic
        result = await conn.execute(
            *_q_update_common_topic(
                organization_id=organization_id.str,
                last_timestamp=device_certificate_cooked.timestamp,
            )
        )
        assert result == "UPDATE 1", f"Unexpected {result}"

    async def _check_device(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        device_id: DeviceID,
    ) -> tuple[UserID, UserProfile, DateTime] | CheckDeviceBadOutcome:
        common_timestamp = await conn.fetchval(
            *_q_check_common_topic(organization_id=organization_id.str)
        )
        if common_timestamp is None:
            common_timestamp = DateTime.from_timestamp_micros(0)
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

    async def _check_user(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        user_id: UserID,
    ) -> tuple[UserProfile, DateTime] | CheckUserBadOutcome:
        common_timestamp = await conn.fetchval(
            *_q_check_common_topic(organization_id=organization_id.str)
        )
        if common_timestamp is None:
            common_timestamp = DateTime.from_timestamp_micros(0)
        u_row = await conn.fetchrow(
            *_q_get_profile_for_user(organization_id=organization_id.str, user_id=user_id)
        )
        if not u_row:
            return CheckUserBadOutcome.USER_NOT_FOUND
        if u_row["revoked_on"] is not None:
            return CheckUserBadOutcome.USER_REVOKED
        return UserProfile.from_str(u_row["profile"]), common_timestamp

    # TODO: This is used by the auth component and is somewhat duplicated with _check_device and _check_user
    #       See https://github.com/Scille/parsec-cloud/issues/7119
    async def _check_user_for_authentication(
        self,
        conn: AsyncpgConnection,
        organization_id: OrganizationID,
        device_id: DeviceID,
    ) -> tuple[UserID, VerifyKey] | CheckUserForAuthenticationBadOutcome:
        d_row = await conn.fetchrow(
            *_q_get_device(organization_id=organization_id.str, device_id=device_id)
        )
        if not d_row:
            return CheckUserForAuthenticationBadOutcome.DEVICE_NOT_FOUND

        user_id = UserID.from_hex(d_row["user_id"])
        verify_key = VerifyKey(d_row["verify_key"])

        u_row = await conn.fetchrow(
            *_q_get_user(organization_id=organization_id.str, user_id=user_id)
        )
        if not u_row:
            return CheckUserForAuthenticationBadOutcome.USER_NOT_FOUND
        if u_row["revoked_on"] is not None:
            return CheckUserForAuthenticationBadOutcome.USER_REVOKED
        if u_row["frozen"]:
            return CheckUserForAuthenticationBadOutcome.USER_FROZEN

        return (user_id, verify_key)

    async def _lock_common_topic(
        self, conn: AsyncpgConnection, organization_id: OrganizationID
    ) -> DateTime:
        value = await conn.fetchval(*_q_lock_common_topic(organization_id=organization_id.str))
        if value is None:
            return DateTime.from_timestamp_micros(0)
        return value

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
        match await self.organization._get(conn, organization_id):
            case Organization() as organization:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return UserCreateUserStoreBadOutcome.ORGANIZATION_NOT_FOUND

        if organization.is_expired:
            return UserCreateUserStoreBadOutcome.ORGANIZATION_EXPIRED

        common_topic_timestamp = await self._lock_common_topic(conn, organization_id)

        match await self._check_device(conn, organization_id, author):
            case (_, profile, _):
                if profile != UserProfile.ADMIN:
                    return UserCreateUserStoreBadOutcome.AUTHOR_NOT_ALLOWED
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return UserCreateUserStoreBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return UserCreateUserStoreBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return UserCreateUserStoreBadOutcome.AUTHOR_REVOKED

        match user_create_user_validate(
            now=now,
            expected_author=author,
            author_verify_key=author_verify_key,
            user_certificate=user_certificate,
            device_certificate=device_certificate,
            redacted_user_certificate=redacted_user_certificate,
            redacted_device_certificate=redacted_device_certificate,
        ):
            case (user_certificate_cooked, device_certificate_cooked):
                pass
            case error:
                return error

        if common_topic_timestamp >= user_certificate_cooked.timestamp:
            return RequireGreaterTimestamp(strictly_greater_than=common_topic_timestamp)

        row = await conn.fetchrow(*_q_check_active_users_limit(organization_id=organization_id.str))
        # Note that with the user/device write lock held, we have guarantee that
        # the active users limit won't change behind our back.
        if row is not None and not row["allowed"]:
            return UserCreateUserStoreBadOutcome.ACTIVE_USERS_LIMIT_REACHED

        if not user_certificate_cooked.human_handle:
            assert False, "User creation without human handle is not supported anymore"

        match await self._create_user(
            conn,
            organization_id,
            user_certificate_cooked,
            user_certificate,
            redacted_user_certificate,
        ):
            case UserCreateUserStoreBadOutcome() as bad_outcome:
                return bad_outcome
            case None:
                pass

        match await self._create_device(
            conn,
            organization_id,
            device_certificate_cooked,
            device_certificate,
            redacted_device_certificate,
            first_device=True,
        ):
            case UserCreateDeviceStoreBadOutcome() as bad_outcome:
                assert False, f"Unexpected {bad_outcome}"
            case None:
                pass

        await self.event_bus.send(
            EventCommonCertificate(
                organization_id=organization_id, timestamp=user_certificate_cooked.timestamp
            )
        )

        return user_certificate_cooked, device_certificate_cooked

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
        match await self.organization._get(conn, organization_id):
            case Organization() as organization:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return UserCreateDeviceStoreBadOutcome.ORGANIZATION_NOT_FOUND

        if organization.is_expired:
            return UserCreateDeviceStoreBadOutcome.ORGANIZATION_EXPIRED

        common_topic_timestamp = await self._lock_common_topic(conn, organization_id)

        match await self._check_device(conn, organization_id, author):
            case (author_user_id, _, _):
                pass
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return UserCreateDeviceStoreBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return UserCreateDeviceStoreBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return UserCreateDeviceStoreBadOutcome.AUTHOR_REVOKED

        match user_create_device_validate(
            now=now,
            expected_author_user_id=author_user_id,
            expected_author_device_id=author,
            author_verify_key=author_verify_key,
            device_certificate=device_certificate,
            redacted_device_certificate=redacted_device_certificate,
        ):
            case DeviceCertificate() as device_certificate_cooked:
                pass
            case error:
                return error

        if device_certificate_cooked.timestamp <= common_topic_timestamp:
            return RequireGreaterTimestamp(strictly_greater_than=common_topic_timestamp)

        match await self._create_device(
            conn,
            organization_id,
            device_certificate_cooked,
            device_certificate,
            redacted_device_certificate,
            first_device=False,
        ):
            case None:
                pass
            case UserCreateDeviceStoreBadOutcome() as bad_outcome:
                return bad_outcome

        await self.event_bus.send(
            EventCommonCertificate(
                organization_id=organization_id,
                timestamp=device_certificate_cooked.timestamp,
            )
        )

        return device_certificate_cooked

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
        match await self.organization._get(conn, organization_id):
            case Organization() as organization:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return UserUpdateUserStoreBadOutcome.ORGANIZATION_NOT_FOUND
        if organization.is_expired:
            return UserUpdateUserStoreBadOutcome.ORGANIZATION_EXPIRED

        common_topic_timestamp = await self._lock_common_topic(conn, organization_id)

        match await self._check_device(conn, organization_id, author):
            case (author_user_id, profile, _):
                if profile != UserProfile.ADMIN:
                    return UserUpdateUserStoreBadOutcome.AUTHOR_NOT_ALLOWED
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return UserUpdateUserStoreBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return UserUpdateUserStoreBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return UserUpdateUserStoreBadOutcome.AUTHOR_REVOKED

        match user_update_user_validate(
            now=now,
            expected_author_user_id=author_user_id,
            expected_author_device_id=author,
            author_verify_key=author_verify_key,
            user_update_certificate=user_update_certificate,
        ):
            case UserUpdateCertificate() as certif:
                pass
            case error:
                return error

        match await self._check_user(conn, organization_id, certif.user_id):
            case (current_profile, _):
                if current_profile == certif.new_profile:
                    return UserUpdateUserStoreBadOutcome.USER_NO_CHANGES
                pass
            case CheckUserBadOutcome.USER_NOT_FOUND:
                return UserUpdateUserStoreBadOutcome.USER_NOT_FOUND
            case CheckUserBadOutcome.USER_REVOKED:
                return UserUpdateUserStoreBadOutcome.USER_REVOKED

        # Ensure certificate consistency: our certificate must be the newest thing on the server.
        #
        # Strictly speaking consistency only requires to ensure the profile change didn't
        # remove rights that have been used to add certificates/vlobs with posterior timestamp
        # (e.g. switching from OWNER to READER while a vlob has been created).
        #
        # However doing such precise checks is complex and error prone, so we take a simpler
        # approach by considering certificates don't change often so it's no big deal to
        # have a much more coarse approach.

        if certif.timestamp <= common_topic_timestamp:
            return RequireGreaterTimestamp(strictly_greater_than=common_topic_timestamp)

        # TODO: validate it's okay not to check this
        # All checks are good, now we do the actual insertion

        # Note an OUTSIDER is not supposed to be OWNER/MANAGER of a shared realm. However this
        # is possible if the user's profile is updated to OUTSIDER here.
        # We don't try to prevent this given:
        # - It is complex and error prone to check.
        # - It is a very niche case.
        # - It is puzzling for the end user to understand why he cannot change a profile,
        #   and that he have to find somebody with access to a seemingly unrelated realm
        #   to change a role in order to be able to do it !

        result = await conn.execute(
            *_q_update_user(
                organization_id=organization_id.str,
                user_id=certif.user_id,
                profile=certif.new_profile.str,
                profile_certificate=user_update_certificate,
                certified_by=author,
                certified_on=now,
            )
        )

        # This should not fail as the proper checks have already been performed
        assert result == "INSERT 0 1", f"Unexpected {result}"

        # Update the common topic
        result = await conn.execute(
            *_q_update_common_topic(
                organization_id=organization_id.str,
                last_timestamp=certif.timestamp,
            )
        )
        assert result == "UPDATE 1", f"Unexpected {result}"

        await self.event_bus.send(
            EventCommonCertificate(
                organization_id=organization_id,
                timestamp=certif.timestamp,
            )
        )
        await self.event_bus.send(
            EventUserUpdated(
                organization_id=organization_id,
                user_id=certif.user_id,
                new_profile=certif.new_profile,
            )
        )

        return certif

    async def get_user_info(
        self, conn: AsyncpgConnection, organization_id: OrganizationID, user_id: UserID
    ) -> UserInfo | None:
        row = await conn.fetchrow(
            *_q_get_user(organization_id=organization_id.str, user_id=user_id)
        )
        if row is None:
            return None
        human_handle = HumanHandle(row["email"], row["label"])
        return UserInfo(user_id, human_handle, bool(row["frozen"]))

    async def get_user_info_from_email(
        self, conn: AsyncpgConnection, organization_id: OrganizationID, email: str
    ) -> UserInfo | None:
        row = await conn.fetchrow(
            *_q_get_user_for_email(organization_id=organization_id.str, email=email)
        )
        if row is None:
            return None
        user_id = UserID.from_hex(row["user_id"])
        human_handle = HumanHandle(row["email"], row["label"])
        return UserInfo(user_id, human_handle, bool(row["frozen"]))

    @override
    @transaction
    async def list_users(
        self, conn: AsyncpgConnection, organization_id: OrganizationID
    ) -> list[UserInfo] | UserListUsersBadOutcome:
        match await self.organization._get(conn, organization_id):
            case Organization():
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return UserListUsersBadOutcome.ORGANIZATION_NOT_FOUND

        return await query_list_users(conn, organization_id)

    @override
    @transaction
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
        match await self.organization._get(conn, organization_id):
            case Organization() as organization:
                assert organization.bootstrapped_on is not None
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return UserGetCertificatesAsUserBadOutcome.ORGANIZATION_NOT_FOUND
        if organization.is_expired:
            return UserGetCertificatesAsUserBadOutcome.ORGANIZATION_EXPIRED

        match await self._check_device(conn, organization_id, author):
            case (author_user_id, profile, _):
                redacted = profile == UserProfile.OUTSIDER
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return UserGetCertificatesAsUserBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return UserGetCertificatesAsUserBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return UserGetCertificatesAsUserBadOutcome.AUTHOR_REVOKED

        # 1) Common certificates (i.e. user/device/revoked/update)

        # Certificates must be returned ordered by timestamp, however there is a trick
        # for the common certificates: when a new user is created, the corresponding
        # user and device certificates have the same timestamp, but we must return
        # the user certificate first (given device references the user).
        # So to achieve this we use a tuple (timestamp, priority, certificate) where
        # only the first two field should be used for sorting (the priority field
        # handling the case where user and device have the same timestamp).

        common_items: list[tuple[DateTime, int, bytes]] = []
        user_priority = 0
        revoked_priority = 1
        update_priority = 2
        device_priority = 3

        query = _q_get_redacted_user_certificates if redacted else _q_get_user_certificates
        for row in await conn.fetch(
            *query(organization_id=organization_id.str, after=common_after)
        ):
            user_item = (
                row["created_on"],
                user_priority,
                row["user_certificate"],
            )
            common_items.append(user_item)

        for row in await conn.fetch(
            *_q_get_user_revoked_certificates(
                organization_id=organization_id.str, after=common_after
            )
        ):
            if row["revoked_on"] is not None:
                revoked_item = (
                    row["revoked_on"],
                    revoked_priority,
                    row["revoked_user_certificate"],
                )
                common_items.append(revoked_item)

        for row in await conn.fetch(
            *_q_get_user_update_certificates(
                organization_id=organization_id.str, after=common_after
            )
        ):
            update_item = (
                row["certified_on"],
                update_priority,
                row["profile_certificate"],
            )
            common_items.append(update_item)

        query = _q_get_device_redacted_certificates if redacted else _q_get_device_certificates
        for row in await conn.fetch(
            *query(organization_id=organization_id.str, after=common_after)
        ):
            device_item = (
                row["created_on"],
                device_priority,
                row["device_certificate"],
            )
            common_items.append(device_item)

        # Sort certificates
        common_items.sort()
        common_certificates = [certificate for _, _, certificate in common_items]

        # 2) Sequester certificates

        # TODO: implement sequester certificates
        sequester_certificates: list[bytes] = []

        # 3) Realm certificates

        realm_items = await self.realm._get_realm_certificates_for_user(
            conn, organization_id, author_user_id, realm_after
        )

        # 4) Shamir certificates

        # TODO: Shamir not currently implemented !
        shamir_recovery_certificates: list[bytes] = []

        return CertificatesBundle(
            common=common_certificates,
            sequester=sequester_certificates,
            shamir_recovery=shamir_recovery_certificates,
            realm=realm_items,
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
        match await self.organization._get(conn, organization_id):
            case Organization() as organization:
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return UserRevokeUserStoreBadOutcome.ORGANIZATION_NOT_FOUND
        if organization.is_expired:
            return UserRevokeUserStoreBadOutcome.ORGANIZATION_EXPIRED

        common_topic_timestamp = await self._lock_common_topic(conn, organization_id)

        match await self._check_device(conn, organization_id, author):
            case (author_user_id, profile, _):
                if profile != UserProfile.ADMIN:
                    return UserRevokeUserStoreBadOutcome.AUTHOR_NOT_ALLOWED
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return UserRevokeUserStoreBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return UserRevokeUserStoreBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return UserRevokeUserStoreBadOutcome.AUTHOR_REVOKED

        match user_revoke_user_validate(
            now=now,
            expected_author_user_id=author_user_id,
            expected_author_device_id=author,
            author_verify_key=author_verify_key,
            revoked_user_certificate=revoked_user_certificate,
        ):
            case RevokedUserCertificate() as certif:
                pass
            case error:
                return error

        match await self._check_user(conn, organization_id, certif.user_id):
            case tuple():
                pass
            case CheckUserBadOutcome.USER_NOT_FOUND:
                return UserRevokeUserStoreBadOutcome.USER_NOT_FOUND
            case CheckUserBadOutcome.USER_REVOKED:
                return CertificateBasedActionIdempotentOutcome(
                    certificate_timestamp=common_topic_timestamp
                )

        # Ensure certificate consistency: our certificate must be the newest thing on the server.
        #
        # Strictly speaking consistency only requires the certificate to be more recent than
        # the the certificates involving the realm and/or the recipient user; and, similarly,
        # the vlobs created/updated by the recipient.
        #
        # However doing such precise checks is complex and error prone, so we take a simpler
        # approach by considering certificates don't change often so it's no big deal to
        # have a much more coarse approach.

        if certif.timestamp <= common_topic_timestamp:
            return RequireGreaterTimestamp(strictly_greater_than=common_topic_timestamp)

        # All checks are good, now we do the actual insertion
        result = await conn.execute(
            *_q_revoke_user(
                organization_id=organization_id.str,
                user_id=certif.user_id,
                revoked_user_certificate=revoked_user_certificate,
                revoked_user_certifier=author,
                revoked_on=certif.timestamp,
            )
        )

        # This should not fail as the proper checks have already been performed
        assert result == "UPDATE 1", f"Unexpected {result}"

        # Update the common topic
        result = await conn.execute(
            *_q_update_common_topic(
                organization_id=organization_id.str,
                last_timestamp=certif.timestamp,
            )
        )
        assert result == "UPDATE 1", f"Unexpected {result}"

        await self.event_bus.send(
            EventCommonCertificate(
                organization_id=organization_id,
                timestamp=certif.timestamp,
            )
        )
        await self.event_bus.send(
            EventUserRevokedOrFrozen(
                organization_id=organization_id,
                user_id=certif.user_id,
            )
        )

        return certif

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
        match await self.organization._get(conn, organization_id):
            case Organization():
                pass
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return UserFreezeUserBadOutcome.ORGANIZATION_NOT_FOUND

        match (user_id, user_email):
            case (None, None):
                return UserFreezeUserBadOutcome.NO_USER_ID_NOR_EMAIL
            case (UserID() as user_id, None):
                match await self.get_user_info(conn, organization_id, user_id):
                    case UserInfo() as info:
                        pass
                    case None:
                        return UserFreezeUserBadOutcome.USER_NOT_FOUND
            case (None, str() as user_email):
                match await self.get_user_info_from_email(conn, organization_id, user_email):
                    case UserInfo() as info:
                        pass
                    case None:
                        return UserFreezeUserBadOutcome.USER_NOT_FOUND
            case (UserID(), str()):
                return UserFreezeUserBadOutcome.BOTH_USER_ID_AND_EMAIL
            case _:
                assert (
                    False
                )  # Can't use assert_never here due to https://github.com/python/mypy/issues/16650

        await conn.execute(
            *_q_freeze_user(
                organization_id=organization_id.str, user_id=info.user_id, frozen=frozen
            )
        )
        info.frozen = frozen

        if info.frozen:
            await self.event_bus.send(
                EventUserRevokedOrFrozen(
                    organization_id=organization_id,
                    user_id=info.user_id,
                )
            )
        else:
            await self.event_bus.send(
                EventUserUnfrozen(
                    organization_id=organization_id,
                    user_id=info.user_id,
                )
            )

        return info

    @override
    @transaction
    async def test_dump_current_users(
        self, conn: AsyncpgConnection, organization_id: OrganizationID
    ) -> dict[UserID, UserDump]:
        rows = await conn.fetch(*_q_get_organization_users(organization_id=organization_id.str))
        items: dict[UserID, UserDump] = {}
        for row in rows:
            user_id = UserID.from_hex(row["user_id"])
            human_handle = HumanHandle(email=row["human_email"], label=row["human_label"])
            items[user_id] = UserDump(
                user_id=user_id,
                human_handle=human_handle,
                created_on=row["created_on"],
                revoked_on=row["revoked_on"],
                devices=[],
                current_profile=UserProfile.from_str(row["current_profile"]),
            )
        rows = await conn.fetch(*_q_get_organization_devices(organization_id=organization_id.str))
        for row in sorted(rows, key=lambda row: row["created_on"]):
            user_id = UserID.from_hex(row["user_id"])
            device_id = DeviceID.from_hex(row["device_id"])
            items[user_id].devices.append(device_id)
        return items
