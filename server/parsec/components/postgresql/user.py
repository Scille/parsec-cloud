# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import assert_never, override

import asyncpg

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
)
from parsec.ballpark import RequireGreaterTimestamp, TimestampOutOfBallpark
from parsec.components.events import EventBus
from parsec.components.organization import Organization, OrganizationGetBadOutcome
from parsec.components.postgresql.organization import PGOrganizationComponent
from parsec.components.postgresql.user_queries import (
    query_dump_users,
)
from parsec.components.postgresql.user_queries.create import (
    q_create_device,
    q_create_user,
    q_take_user_device_write_lock,
)
from parsec.components.postgresql.user_queries.get import (
    _q_get_device,
    _q_get_user,
    _q_get_user_info,
    _q_get_user_info_from_email,
    query_list_users,
)
from parsec.components.postgresql.user_queries.revoke import (
    _q_revoke_user,
    q_freeze_user,
)
from parsec.components.postgresql.utils import (
    Q,
    q_device_internal_id,
    q_user_internal_id,
    transaction,
)
from parsec.components.realm import CertificateBasedActionIdempotentOutcome
from parsec.components.user import (
    BaseUserComponent,
    CheckDeviceBadOutcome,
    CheckUserBadOutcome,
    UserCreateDeviceStoreBadOutcome,
    UserCreateDeviceValidateBadOutcome,
    UserCreateUserStoreBadOutcome,
    UserCreateUserValidateBadOutcome,
    UserDump,
    UserFreezeUserBadOutcome,
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


class PGUserComponent(BaseUserComponent):
    def __init__(self, pool: asyncpg.Pool, event_bus: EventBus) -> None:
        super().__init__()
        self.pool = pool
        self.event_bus = event_bus
        self.organization: PGOrganizationComponent

    def register_components(self, organization: PGOrganizationComponent, **kwargs) -> None:
        self.organization = organization

    async def _check_device(
        self,
        conn: asyncpg.Connection,
        organization_id: OrganizationID,
        device_id: DeviceID,
    ) -> UserProfile | CheckDeviceBadOutcome:
        d_row = await conn.fetchrow(
            *_q_get_device(organization_id=organization_id.str, device_id=device_id.str)
        )
        if not d_row:
            return CheckDeviceBadOutcome.DEVICE_NOT_FOUND
        match await self._check_user(conn, organization_id, device_id.user_id):
            case CheckUserBadOutcome.USER_NOT_FOUND:
                return CheckDeviceBadOutcome.USER_NOT_FOUND
            case CheckUserBadOutcome.USER_REVOKED:
                return CheckDeviceBadOutcome.USER_REVOKED
            case UserProfile() as profile:
                return profile
            case unknown:
                assert_never(unknown)

    async def _check_user(
        self,
        conn: asyncpg.Connection,
        organization_id: OrganizationID,
        user_id: UserID,
    ) -> UserProfile | CheckUserBadOutcome:
        u_row = await conn.fetchrow(
            *_q_get_user(organization_id=organization_id.str, user_id=user_id.str)
        )
        if not u_row:
            return CheckUserBadOutcome.USER_NOT_FOUND
        if u_row["revoked_on"] is not None:
            return CheckUserBadOutcome.USER_REVOKED
        initial_profile = UserProfile.from_str(u_row["profile"])
        # TODO: return the actual profile
        return initial_profile

    @override
    @transaction
    async def create_user(
        self,
        conn: asyncpg.Connection,
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
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return UserCreateUserStoreBadOutcome.ORGANIZATION_NOT_FOUND
            case Organization() as organization:
                pass
            case unknown:
                assert_never(unknown)
        if organization.is_expired:
            return UserCreateUserStoreBadOutcome.ORGANIZATION_EXPIRED

        match await self._check_device(conn, organization_id, author):
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return UserCreateUserStoreBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return UserCreateUserStoreBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return UserCreateUserStoreBadOutcome.AUTHOR_REVOKED
            case UserProfile() as profile:
                if profile != UserProfile.ADMIN:
                    return UserCreateUserStoreBadOutcome.AUTHOR_NOT_ALLOWED
            case unknown:
                assert_never(unknown)

        match user_create_user_validate(
            now=now,
            expected_author=author,
            author_verify_key=author_verify_key,
            user_certificate=user_certificate,
            device_certificate=device_certificate,
            redacted_user_certificate=redacted_user_certificate,
            redacted_device_certificate=redacted_device_certificate,
        ):
            case (u_certif, d_certif):
                pass
            case error:
                return error

        match await q_create_user(
            conn,
            organization_id,
            user_certificate_cooked=u_certif,
            user_certificate=user_certificate,
            user_certificate_redacted=redacted_user_certificate,
            device_certificate_cooked=d_certif,
            device_certificate=device_certificate,
            device_certificate_redacted=redacted_device_certificate,
        ):
            case UserCreateUserStoreBadOutcome() as error:
                return error
            case UserCreateDeviceStoreBadOutcome() as error:
                assert False, f"Unexpected {error}, the device creation should not fail"
            case None:
                pass
            case unknown:
                assert_never(unknown)

        await self.event_bus.send(
            EventCommonCertificate(organization_id=organization_id, timestamp=u_certif.timestamp)
        )

        return u_certif, d_certif

    @override
    @transaction
    async def create_device(
        self,
        conn: asyncpg.Connection,
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
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return UserCreateDeviceStoreBadOutcome.ORGANIZATION_NOT_FOUND
            case Organization() as organization:
                pass
            case unknown:
                assert_never(unknown)
        if organization.is_expired:
            return UserCreateDeviceStoreBadOutcome.ORGANIZATION_EXPIRED

        match await self._check_device(conn, organization_id, author):
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return UserCreateDeviceStoreBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return UserCreateDeviceStoreBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return UserCreateDeviceStoreBadOutcome.AUTHOR_REVOKED
            case UserProfile():
                pass
            case unknown:
                assert_never(unknown)

        match user_create_device_validate(
            now=now,
            expected_author=author,
            author_verify_key=author_verify_key,
            device_certificate=device_certificate,
            redacted_device_certificate=redacted_device_certificate,
        ):
            case DeviceCertificate() as certif:
                pass
            case error:
                return error

        await q_create_device(
            conn,
            organization_id,
            device_certificate_cooked=certif,
            device_certificate=device_certificate,
            device_certificate_redacted=redacted_device_certificate,
        )

        await self.event_bus.send(
            EventCommonCertificate(
                organization_id=organization_id,
                timestamp=certif.timestamp,
            )
        )

        return certif

    @override
    @transaction
    async def update_user(
        self,
        conn: asyncpg.Connection,
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
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return UserUpdateUserStoreBadOutcome.ORGANIZATION_NOT_FOUND
            case Organization() as organization:
                pass
            case unknown:
                assert_never(unknown)
        if organization.is_expired:
            return UserUpdateUserStoreBadOutcome.ORGANIZATION_EXPIRED

        match await self._check_device(conn, organization_id, author):
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return UserUpdateUserStoreBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return UserUpdateUserStoreBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return UserUpdateUserStoreBadOutcome.AUTHOR_REVOKED
            case UserProfile() as profile:
                if profile != UserProfile.ADMIN:
                    return UserUpdateUserStoreBadOutcome.AUTHOR_NOT_ALLOWED
            case unknown:
                assert_never(unknown)

        match user_update_user_validate(
            now=now,
            expected_author=author,
            author_verify_key=author_verify_key,
            user_update_certificate=user_update_certificate,
        ):
            case UserUpdateCertificate() as certif:
                pass
            case error:
                return error

        match await self._check_user(conn, organization_id, certif.user_id):
            case CheckUserBadOutcome.USER_NOT_FOUND:
                return UserUpdateUserStoreBadOutcome.USER_NOT_FOUND
            case CheckUserBadOutcome.USER_REVOKED:
                return UserUpdateUserStoreBadOutcome.USER_REVOKED
            case UserProfile() as current_profile:
                if current_profile == certif.new_profile:
                    return UserUpdateUserStoreBadOutcome.USER_NO_CHANGES
                pass
            case unknown:
                assert_never(unknown)

        # Ensure certificate consistency: our certificate must be the newest thing on the server.
        #
        # Strictly speaking consistency only requires to ensure the profile change didn't
        # remove rights that have been used to add certificates/vlobs with posterior timestamp
        # (e.g. switching from OWNER to READER while a vlob has been created).
        #
        # However doing such precise checks is complex and error prone, so we take a simpler
        # approach by considering certificates don't change often so it's no big deal to
        # have a much more coarse approach.

        # TODO: implement consistency
        # if org.last_certificate_or_vlob_timestamp >= certif.timestamp:
        #     return RequireGreaterTimestamp(
        #         strictly_greater_than=org.last_certificate_or_vlob_timestamp
        #     )

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

        await q_take_user_device_write_lock(conn, organization_id)
        result = await conn.execute(
            *_q_update_user(
                organization_id=organization_id.str,
                user_id=certif.user_id.str,
                profile=certif.new_profile.str,
                profile_certificate=user_update_certificate,
                certified_by=author.str,
                certified_on=now,
            )
        )

        # This should not fail as the proper checks have already been performed
        assert result == "INSERT 0 1", f"Unexpected {result}"

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
        self, conn: asyncpg.Connection, organization_id: OrganizationID, user_id: UserID
    ) -> UserInfo | None:
        row = await conn.fetchrow(
            *_q_get_user_info(organization_id=organization_id.str, user_id=user_id.str)
        )
        if row is None:
            return None
        human_handle = HumanHandle(row["email"], row["label"])
        return UserInfo(user_id, human_handle, bool(row["frozen"]))

    async def get_user_info_from_email(
        self, conn: asyncpg.Connection, organization_id: OrganizationID, email: str
    ) -> UserInfo | None:
        row = await conn.fetchrow(
            *_q_get_user_info_from_email(organization_id=organization_id.str, email=email)
        )
        if row is None:
            return None
        user_id = UserID(row["user_id"])
        human_handle = HumanHandle(row["email"], row["label"])
        return UserInfo(user_id, human_handle, bool(row["frozen"]))

    @override
    @transaction
    async def list_users(
        self, conn: asyncpg.Connection, organization_id: OrganizationID
    ) -> list[UserInfo] | UserListUsersBadOutcome:
        match await self.organization._get(conn, organization_id):
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return UserListUsersBadOutcome.ORGANIZATION_NOT_FOUND
            case Organization():
                pass
            case unknown:
                assert_never(unknown)

        return await query_list_users(conn, organization_id)

    # async def get_user_with_trustchain(
    #     self, organization_id: OrganizationID, user_id: UserID
    # ) -> tuple[User, Trustchain]:
    #     raise NotImplementedError
    #     async with self.dbh.pool.acquire() as conn:
    #         return await query_get_user_with_trustchain(conn, organization_id, user_id)

    # async def get_user_with_device_and_trustchain(
    #     self, organization_id: OrganizationID, device_id: DeviceID
    # ) -> tuple[User, Device, Trustchain]:
    #     raise NotImplementedError
    #     async with self.dbh.pool.acquire() as conn:
    #         return await query_get_user_with_device_and_trustchain(conn, organization_id, device_id)

    # async def get_user_with_devices_and_trustchain(
    #     self, organization_id: OrganizationID, user_id: UserID, redacted: bool = False
    # ) -> GetUserAndDevicesResult:
    #     raise NotImplementedError
    #     async with self.dbh.pool.acquire() as conn:
    #         return await query_get_user_with_devices_and_trustchain(
    #             conn, organization_id, user_id, redacted=redacted
    #         )

    # async def _get_user_with_device(
    #     self, conn: asyncpg.Connection, organization_id: OrganizationID, device_id: DeviceID
    # ) -> tuple[User, Device]:
    #     raise NotImplementedError
    #     return await query_get_user_with_device(conn, organization_id, device_id)

    # async def find_humans(
    #     self,
    #     organization_id: OrganizationID,
    #     query: str | None = None,
    #     page: int = 1,
    #     per_page: int = 100,
    #     omit_revoked: bool = False,
    #     omit_non_human: bool = False,
    # ) -> tuple[list[HumanFindResultItem], int]:
    #     async with self.dbh.pool.acquire() as conn:
    #         return await query_find_humans(
    #             conn,
    #             organization_id=organization_id,
    #             query=query,
    #             page=page,
    #             per_page=per_page,
    #             omit_revoked=omit_revoked,
    #             omit_non_human=omit_non_human,
    #         )

    @override
    @transaction
    async def revoke_user(
        self,
        conn: asyncpg.Connection,
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
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return UserRevokeUserStoreBadOutcome.ORGANIZATION_NOT_FOUND
            case Organization() as organization:
                pass
            case unknown:
                assert_never(unknown)
        if organization.is_expired:
            return UserRevokeUserStoreBadOutcome.ORGANIZATION_EXPIRED

        match await self._check_device(conn, organization_id, author):
            case CheckDeviceBadOutcome.DEVICE_NOT_FOUND:
                return UserRevokeUserStoreBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_NOT_FOUND:
                return UserRevokeUserStoreBadOutcome.AUTHOR_NOT_FOUND
            case CheckDeviceBadOutcome.USER_REVOKED:
                return UserRevokeUserStoreBadOutcome.AUTHOR_REVOKED
            case UserProfile() as profile:
                if profile != UserProfile.ADMIN:
                    return UserRevokeUserStoreBadOutcome.AUTHOR_NOT_ALLOWED
            case unknown:
                assert_never(unknown)

        match user_revoke_user_validate(
            now=now,
            expected_author=author,
            author_verify_key=author_verify_key,
            revoked_user_certificate=revoked_user_certificate,
        ):
            case RevokedUserCertificate() as certif:
                pass
            case error:
                return error

        match await self._check_user(conn, organization_id, certif.user_id):
            case CheckUserBadOutcome.USER_NOT_FOUND:
                return UserRevokeUserStoreBadOutcome.USER_NOT_FOUND
            case CheckUserBadOutcome.USER_REVOKED:
                return CertificateBasedActionIdempotentOutcome(
                    certificate_timestamp=certif.timestamp  # TODO: Wrong timestamp!
                )
            case UserProfile():
                pass
            case unknown:
                assert_never(unknown)

        # Ensure certificate consistency: our certificate must be the newest thing on the server.
        #
        # Strictly speaking consistency only requires the certificate to be more recent than
        # the the certificates involving the realm and/or the recipient user; and, similarly,
        # the vlobs created/updated by the recipient.
        #
        # However doing such precise checks is complex and error prone, so we take a simpler
        # approach by considering certificates don't change often so it's no big deal to
        # have a much more coarse approach.

        # TODO: implement consistency
        # if org.last_certificate_or_vlob_timestamp >= certif.timestamp:
        #     return RequireGreaterTimestamp(
        #         strictly_greater_than=org.last_certificate_or_vlob_timestamp
        #     )

        # All checks are good, now we do the actual insertion
        await q_take_user_device_write_lock(conn, organization_id)
        result = await conn.execute(
            *_q_revoke_user(
                organization_id=organization_id.str,
                user_id=certif.user_id.str,
                revoked_user_certificate=revoked_user_certificate,
                revoked_user_certifier=author.str,
                revoked_on=now,
            )
        )

        # This should not fail as the proper checks have already been performed
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
        conn: asyncpg.Connection,
        organization_id: OrganizationID,
        user_id: UserID | None,
        user_email: str | None,
        frozen: bool,
    ) -> UserInfo | UserFreezeUserBadOutcome:
        match await self.organization._get(conn, organization_id):
            case OrganizationGetBadOutcome.ORGANIZATION_NOT_FOUND:
                return UserFreezeUserBadOutcome.ORGANIZATION_NOT_FOUND
            case Organization():
                pass
            case unknown:
                assert_never(unknown)

        match (user_id, user_email):
            case (None, None):
                return UserFreezeUserBadOutcome.NO_USER_ID_NOR_EMAIL
            case (UserID() as user_id, None):
                match await self.get_user_info(conn, organization_id, user_id):
                    case UserInfo() as info:
                        pass
                    case None:
                        return UserFreezeUserBadOutcome.USER_NOT_FOUND
                    case unknown:
                        assert_never(unknown)
            case (None, str() as user_email):
                match await self.get_user_info_from_email(conn, organization_id, user_email):
                    case UserInfo() as info:
                        pass
                    case None:
                        return UserFreezeUserBadOutcome.USER_NOT_FOUND
                    case unknown:
                        assert_never(unknown)
            case (UserID(), str()):
                return UserFreezeUserBadOutcome.BOTH_USER_ID_AND_EMAIL
            case _:
                assert (
                    False
                )  # Can't use assert_never here due to https://github.com/python/mypy/issues/16650

        await conn.execute(
            *q_freeze_user(
                organization_id=organization_id.str, user_id=info.user_id.str, frozen=frozen
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
        self, conn: asyncpg.Connection, organization_id: OrganizationID
    ) -> dict[UserID, UserDump]:
        raise NotImplementedError
        return await query_dump_users(conn, organization_id)
