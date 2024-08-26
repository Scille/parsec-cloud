# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import Any, Literal, override

from typing_extensions import assert_never

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
from parsec.components.memory.datamodel import (
    MemoryDatamodel,
    MemoryDevice,
    MemoryUser,
    MemoryUserProfileUpdate,
)
from parsec.components.realm import CertificateBasedActionIdempotentOutcome
from parsec.components.user import (
    BaseUserComponent,
    CertificatesBundle,
    UserCreateDeviceStoreBadOutcome,
    UserCreateDeviceValidateBadOutcome,
    UserCreateUserStoreBadOutcome,
    UserCreateUserValidateBadOutcome,
    UserDump,
    UserFreezeUserBadOutcome,
    UserGetActiveDeviceVerifyKeyBadOutcome,
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


class MemoryUserComponent(BaseUserComponent):
    def __init__(
        self,
        data: MemoryDatamodel,
        event_bus: EventBus,
        *args: Any,
        **kwargs: Any,
    ) -> None:
        super().__init__(*args, **kwargs)
        self._data = data
        self._event_bus = event_bus

    @override
    async def create_user(
        self,
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
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return UserCreateUserStoreBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return UserCreateUserStoreBadOutcome.ORGANIZATION_EXPIRED

        async with org.topics_lock(write=["common"]) as (common_topic_last_timestamp,):
            try:
                author_device = org.devices[author]
            except KeyError:
                return UserCreateUserStoreBadOutcome.AUTHOR_NOT_FOUND

            author_user = org.users[author_device.cooked.user_id]
            if author_user.is_revoked:
                return UserCreateUserStoreBadOutcome.AUTHOR_REVOKED
            if author_user.current_profile != UserProfile.ADMIN:
                return UserCreateUserStoreBadOutcome.AUTHOR_NOT_ALLOWED

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

            if org.active_user_limit_reached():
                return UserCreateUserStoreBadOutcome.ACTIVE_USERS_LIMIT_REACHED

            if u_certif.user_id in org.users:
                return UserCreateUserStoreBadOutcome.USER_ALREADY_EXISTS

            if d_certif.device_id in org.devices:
                return UserCreateUserStoreBadOutcome.USER_ALREADY_EXISTS

            if any(
                True for u in org.active_users() if u.cooked.human_handle == u_certif.human_handle
            ):
                return UserCreateUserStoreBadOutcome.HUMAN_HANDLE_ALREADY_TAKEN

            # Ensure we are not breaking causality by adding a newer timestamp.

            # We already ensured user and device certificates' timestamps are consistent,
            # so only need to check one of them here
            if common_topic_last_timestamp >= u_certif.timestamp:
                return RequireGreaterTimestamp(strictly_greater_than=common_topic_last_timestamp)

            # All checks are good, now we do the actual insertion

            org.per_topic_last_timestamp["common"] = u_certif.timestamp

            org.users[u_certif.user_id] = MemoryUser(
                cooked=u_certif,
                user_certificate=user_certificate,
                redacted_user_certificate=redacted_user_certificate,
            )

            org.devices[d_certif.device_id] = MemoryDevice(
                cooked=d_certif,
                device_certificate=device_certificate,
                redacted_device_certificate=redacted_device_certificate,
            )

            await self._event_bus.send(
                EventCommonCertificate(
                    organization_id=organization_id, timestamp=u_certif.timestamp
                )
            )

            return u_certif, d_certif

    @override
    async def create_device(
        self,
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
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return UserCreateDeviceStoreBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return UserCreateDeviceStoreBadOutcome.ORGANIZATION_EXPIRED

        async with org.topics_lock(write=["common"]) as (common_topic_last_timestamp,):
            try:
                author_device = org.devices[author]
            except KeyError:
                return UserCreateDeviceStoreBadOutcome.AUTHOR_NOT_FOUND

            author_user_id = author_device.cooked.user_id
            author_user = org.users[author_user_id]
            if author_user.is_revoked:
                return UserCreateDeviceStoreBadOutcome.AUTHOR_REVOKED

            match user_create_device_validate(
                now=now,
                expected_author_user_id=author_user_id,
                expected_author_device_id=author,
                author_verify_key=author_verify_key,
                device_certificate=device_certificate,
                redacted_device_certificate=redacted_device_certificate,
            ):
                case DeviceCertificate() as certif:
                    pass
                case error:
                    return error

            if certif.device_id in org.devices:
                return UserCreateDeviceStoreBadOutcome.DEVICE_ALREADY_EXISTS

            # Ensure we are not breaking causality by adding a newer timestamp.

            if common_topic_last_timestamp >= certif.timestamp:
                return RequireGreaterTimestamp(strictly_greater_than=common_topic_last_timestamp)

            # All checks are good, now we do the actual insertion

            org.per_topic_last_timestamp["common"] = certif.timestamp

            org.devices[certif.device_id] = MemoryDevice(
                cooked=certif,
                device_certificate=device_certificate,
                redacted_device_certificate=redacted_device_certificate,
            )

            await self._event_bus.send(
                EventCommonCertificate(
                    organization_id=organization_id,
                    timestamp=certif.timestamp,
                )
            )

            return certif

    @override
    async def revoke_user(
        self,
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
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return UserRevokeUserStoreBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return UserRevokeUserStoreBadOutcome.ORGANIZATION_EXPIRED

        async with org.topics_lock(write=["common"]) as (common_topic_last_timestamp,):
            try:
                author_device = org.devices[author]
            except KeyError:
                return UserRevokeUserStoreBadOutcome.AUTHOR_NOT_FOUND
            author_user_id = author_device.cooked.user_id

            author_user = org.users[author_user_id]
            if author_user.is_revoked:
                return UserRevokeUserStoreBadOutcome.AUTHOR_REVOKED
            if author_user.current_profile != UserProfile.ADMIN:
                return UserRevokeUserStoreBadOutcome.AUTHOR_NOT_ALLOWED

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

            try:
                target_user = org.users[certif.user_id]
            except KeyError:
                return UserRevokeUserStoreBadOutcome.USER_NOT_FOUND

            if target_user.is_revoked:
                assert target_user.cooked_revoked is not None
                return CertificateBasedActionIdempotentOutcome(
                    certificate_timestamp=target_user.cooked_revoked.timestamp
                )

            # Ensure we are not breaking causality by adding a newer timestamp.
            # Given a revoked user is not allowed to modify a realm, we must check timestamp on:
            # - The common topic
            # - For each realm the user is part of: the realm topic
            # - For each realm the user is part of: the last vlob

            realms_user_is_part_of = [
                realm
                for realm in org.realms.values()
                if realm.get_current_role_for(certif.user_id) is not None
            ]

            realm_topics_user_is_part_of: list[tuple[Literal["realm"], VlobID]] = [
                ("realm", realm.realm_id) for realm in realms_user_is_part_of
            ]
            async with org.topics_lock(
                read=realm_topics_user_is_part_of
            ) as realm_topics_last_timestamp:
                last_timestamp = max(
                    (
                        common_topic_last_timestamp,
                        *realm_topics_last_timestamp,
                        *(
                            ts
                            for realm in realms_user_is_part_of
                            if (ts := realm.last_vlob_timestamp) is not None
                        ),
                    )
                )

                if last_timestamp >= certif.timestamp:
                    return RequireGreaterTimestamp(strictly_greater_than=last_timestamp)

                # All checks are good, now we do the actual insertion

                org.per_topic_last_timestamp["common"] = certif.timestamp

                target_user.revoked_user_certificate = revoked_user_certificate
                target_user.cooked_revoked = certif

                await self._event_bus.send(
                    EventCommonCertificate(
                        organization_id=organization_id,
                        timestamp=certif.timestamp,
                    )
                )
                await self._event_bus.send(
                    EventUserRevokedOrFrozen(
                        organization_id=organization_id,
                        user_id=certif.user_id,
                    )
                )

                return certif

    @override
    async def update_user(
        self,
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
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return UserUpdateUserStoreBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return UserUpdateUserStoreBadOutcome.ORGANIZATION_EXPIRED

        async with org.topics_lock(write=["common"]) as (common_topic_last_timestamp,):
            try:
                author_device = org.devices[author]
            except KeyError:
                return UserUpdateUserStoreBadOutcome.AUTHOR_NOT_FOUND
            author_user_id = author_device.cooked.user_id

            author_user = org.users[author_user_id]
            if author_user.is_revoked:
                return UserUpdateUserStoreBadOutcome.AUTHOR_REVOKED
            if author_user.current_profile != UserProfile.ADMIN:
                return UserUpdateUserStoreBadOutcome.AUTHOR_NOT_ALLOWED

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

            try:
                target_user = org.users[certif.user_id]
            except KeyError:
                return UserUpdateUserStoreBadOutcome.USER_NOT_FOUND

            if target_user.is_revoked:
                return UserUpdateUserStoreBadOutcome.USER_REVOKED

            if target_user.current_profile == certif.new_profile:
                return UserUpdateUserStoreBadOutcome.USER_NO_CHANGES

            # Ensure we are not breaking causality by adding a newer timestamp.

            if common_topic_last_timestamp >= certif.timestamp:
                return RequireGreaterTimestamp(strictly_greater_than=common_topic_last_timestamp)

            # All checks are good, now we do the actual insertion

            # TODO: validate it's okay not to check this
            # Note an OUTSIDER is not supposed to be OWNER/MANAGER of a shared realm. However this
            # is possible if the user's profile is updated to OUTSIDER here.
            # We don't try to prevent this given:
            # - It is complex and error prone to check.
            # - It is a very niche case.
            # - It is puzzling for the end user to understand why he cannot change a profile,
            #   and that he have to find somebody with access to a seemingly unrelated realm
            #   to change a role in order to be able to do it !

            org.per_topic_last_timestamp["common"] = certif.timestamp

            target_user.profile_updates.append(
                MemoryUserProfileUpdate(
                    cooked=certif,
                    user_update_certificate=user_update_certificate,
                )
            )

            await self._event_bus.send(
                EventCommonCertificate(
                    organization_id=organization_id,
                    timestamp=certif.timestamp,
                )
            )
            await self._event_bus.send(
                EventUserUpdated(
                    organization_id=organization_id,
                    user_id=certif.user_id,
                    new_profile=certif.new_profile,
                )
            )

            return certif

    @override
    async def get_certificates(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        common_after: DateTime | None,
        sequester_after: DateTime | None,
        shamir_recovery_after: DateTime | None,
        realm_after: dict[VlobID, DateTime],
    ) -> CertificatesBundle | UserGetCertificatesAsUserBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return UserGetCertificatesAsUserBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return UserGetCertificatesAsUserBadOutcome.ORGANIZATION_EXPIRED

        try:
            device = org.devices[author]
            author_user_id = device.cooked.user_id
            user = org.users[author_user_id]
        except KeyError:
            return UserGetCertificatesAsUserBadOutcome.AUTHOR_NOT_FOUND
        redacted = user.current_profile == UserProfile.OUTSIDER

        # Bootstrap must have occurred if author is present !
        assert org.bootstrapped_on is not None

        # 1) Common certificates (i.e. user/device/revoked/update)

        # Certificates must be returned ordered by timestamp, however there is a trick
        # for the common certificates: when a new user is created, the corresponding
        # user and device certificates have the same timestamp, but we must return
        # the user certificate first (given device references the user).
        # So to achieve this we use a tuple (timestamp, priority, certificate) where
        # only the first two field should be used for sorting (the priority field
        # handling the case where user and device have the same timestamp).

        common_certificates_unordered: list[tuple[DateTime, int, bytes]] = []
        for user in org.users.values():
            if redacted:
                common_certificates_unordered.append(
                    (user.cooked.timestamp, 0, user.redacted_user_certificate)
                )
            else:
                common_certificates_unordered.append(
                    (user.cooked.timestamp, 0, user.user_certificate)
                )

            if user.is_revoked:
                assert user.cooked_revoked is not None
                assert user.revoked_user_certificate is not None
                common_certificates_unordered.append(
                    (user.cooked_revoked.timestamp, 1, user.revoked_user_certificate)
                )

            for update in user.profile_updates:
                common_certificates_unordered.append(
                    (update.cooked.timestamp, 1, update.user_update_certificate)
                )

        for device in org.devices.values():
            if redacted:
                common_certificates_unordered.append(
                    (device.cooked.timestamp, 1, device.redacted_device_certificate)
                )
            else:
                common_certificates_unordered.append(
                    (device.cooked.timestamp, 1, device.device_certificate)
                )

        common_certificates = [
            c
            for ts, _, c in sorted(common_certificates_unordered)
            if not common_after or common_after < ts
        ]

        # 2) Sequester certificates

        sequester_certificates: list[bytes] = []
        if org.sequester_authority_certificate is not None:
            assert org.cooked_sequester_authority is not None
            assert org.sequester_services is not None
            if (
                sequester_after is None
                or sequester_after < org.cooked_sequester_authority.timestamp
            ):
                sequester_certificates.append(org.sequester_authority_certificate)
            for service in org.sequester_services.values():
                if sequester_after is None or sequester_after < service.cooked.timestamp:
                    sequester_certificates.append(service.sequester_service_certificate)

        # 3) Realm certificates

        per_realm_certificates: dict[VlobID, list[bytes]] = {}
        for realm in org.realms.values():
            last_role = None
            for role in reversed(realm.roles):
                if role.cooked.user_id == author_user_id:
                    last_role = role.cooked
                    break
            if not last_role:
                # User never had access to this realm, just ignore it
                continue

            # Collect all the certificates related to the realm
            realm_certificates_unordered: list[tuple[DateTime, bytes]] = []
            realm_certificates_unordered += [
                (role.cooked.timestamp, role.realm_role_certificate) for role in realm.roles
            ]
            realm_certificates_unordered += [
                (role.cooked.timestamp, role.realm_key_rotation_certificate)
                for role in realm.key_rotations
            ]
            realm_certificates_unordered += [
                (role.cooked.timestamp, role.realm_name_certificate) for role in realm.renames
            ]
            # TODO: support archiving here !

            if last_role.role is None:
                # User used to have access to the realm, only provide the certificate that
                # was available back when he had access
                realm_certificates_unordered = [
                    (ts, c) for ts, c in realm_certificates_unordered if ts <= last_role.timestamp
                ]

            current_realm_after = realm_after.get(realm.realm_id)
            realm_certificates = [
                c
                for ts, c in sorted(realm_certificates_unordered)
                if not current_realm_after or current_realm_after < ts
            ]
            # Omit realm with no new certificates
            if realm_certificates:
                per_realm_certificates[realm.realm_id] = realm_certificates

        # 4) Shamir certificates

        shamir_recovery_certificates: list[bytes] = []

        for user_id, shamir in sorted(org.shamir_setup.items(), key=lambda x: x[1].brief.timestamp):
            # filter on timestamp
            if shamir_recovery_after is not None and shamir.brief.timestamp < shamir_recovery_after:
                continue

            # if it is user's certificate keep brief
            if user_id == author_user_id:
                shamir_recovery_certificates.append(shamir.brief_bytes)

            # if user is a share recipient keep share and brief
            if author_user_id in shamir.shares.keys():
                shamir_recovery_certificates.append(shamir.shares[author_user_id])
                shamir_recovery_certificates.append(shamir.brief_bytes)

        return CertificatesBundle(
            common=common_certificates,
            sequester=sequester_certificates,
            shamir_recovery=shamir_recovery_certificates,
            realm=per_realm_certificates,
        )

    @override
    async def get_active_device_verify_key(
        self, organization_id: OrganizationID, device_id: DeviceID
    ) -> VerifyKey | UserGetActiveDeviceVerifyKeyBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return UserGetActiveDeviceVerifyKeyBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return UserGetActiveDeviceVerifyKeyBadOutcome.ORGANIZATION_EXPIRED

        try:
            device = org.devices[device_id]
            user = org.users[device.cooked.user_id]

        except KeyError:
            return UserGetActiveDeviceVerifyKeyBadOutcome.DEVICE_NOT_FOUND

        if user.is_revoked:
            return UserGetActiveDeviceVerifyKeyBadOutcome.USER_REVOKED

        return device.cooked.verify_key

    @override
    async def test_dump_current_users(
        self, organization_id: OrganizationID
    ) -> dict[UserID, UserDump]:
        org = self._data.organizations[organization_id]
        items = {}
        for user in org.users.values():
            user_id = user.cooked.user_id
            items[user_id] = UserDump(
                user_id=user_id,
                devices=[
                    d.cooked.device_id for d in org.devices.values() if d.cooked.user_id == user_id
                ],
                current_profile=user.current_profile,
                created_on=user.cooked.timestamp,
                revoked_on=user.cooked_revoked.timestamp
                if user.cooked_revoked is not None
                else None,
                human_handle=user.cooked.human_handle,
            )

        return items

    @override
    async def list_users(
        self, organization_id: OrganizationID
    ) -> list[UserInfo] | UserListUsersBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return UserListUsersBadOutcome.ORGANIZATION_NOT_FOUND

        users = []
        for user in org.users.values():
            users.append(
                UserInfo(
                    user_id=user.cooked.user_id,
                    human_handle=user.cooked.human_handle,
                    frozen=user.is_frozen,
                )
            )

        return users

    @override
    async def freeze_user(
        self,
        organization_id: OrganizationID,
        user_id: UserID | None,
        user_email: str | None,
        frozen: bool,
    ) -> UserInfo | UserFreezeUserBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return UserFreezeUserBadOutcome.ORGANIZATION_NOT_FOUND

        match (user_id, user_email):
            case (None, None):
                return UserFreezeUserBadOutcome.NO_USER_ID_NOR_EMAIL
            case (UserID() as user_id, None):
                try:
                    user = org.users[user_id]
                except KeyError:
                    return UserFreezeUserBadOutcome.USER_NOT_FOUND
            case (None, str() as user_email):
                for user in org.users.values():
                    if user.cooked.human_handle.email == user_email:
                        break
                else:
                    return UserFreezeUserBadOutcome.USER_NOT_FOUND
            case (UserID(), str()):
                return UserFreezeUserBadOutcome.BOTH_USER_ID_AND_EMAIL
            case never:  # pyright: ignore [reportUnnecessaryComparison]
                assert_never(never)

        user.is_frozen = frozen
        if user.is_frozen:
            await self._event_bus.send(
                EventUserRevokedOrFrozen(
                    organization_id=organization_id,
                    user_id=user.cooked.user_id,
                )
            )
        else:
            await self._event_bus.send(
                EventUserUnfrozen(
                    organization_id=organization_id,
                    user_id=user.cooked.user_id,
                )
            )

        return UserInfo(
            user_id=user.cooked.user_id,
            human_handle=user.cooked.human_handle,
            frozen=user.is_frozen,
        )
