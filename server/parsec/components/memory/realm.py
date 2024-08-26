# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import override

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    RealmKeyRotationCertificate,
    RealmNameCertificate,
    RealmRoleCertificate,
    UserID,
    UserProfile,
    VerifyKey,
    VlobID,
)
from parsec.ballpark import RequireGreaterTimestamp, TimestampOutOfBallpark
from parsec.components.events import EventBus
from parsec.components.memory.datamodel import (
    MemoryDatamodel,
    MemoryRealm,
    MemoryRealmKeyRotation,
    MemoryRealmRename,
    MemoryRealmUserRole,
)
from parsec.components.realm import (
    BadKeyIndex,
    BaseRealmComponent,
    CertificateBasedActionIdempotentOutcome,
    KeysBundle,
    RealmCreateStoreBadOutcome,
    RealmCreateValidateBadOutcome,
    RealmDumpRealmsGrantedRolesBadOutcome,
    RealmGetCurrentRealmsForUserBadOutcome,
    RealmGetKeysBundleBadOutcome,
    RealmGetStatsAsUserBadOutcome,
    RealmGrantedRole,
    RealmRenameStoreBadOutcome,
    RealmRenameValidateBadOutcome,
    RealmRole,
    RealmRotateKeyStoreBadOutcome,
    RealmRotateKeyValidateBadOutcome,
    RealmShareStoreBadOutcome,
    RealmShareValidateBadOutcome,
    RealmStats,
    RealmUnshareStoreBadOutcome,
    RealmUnshareValidateBadOutcome,
    realm_create_validate,
    realm_rename_validate,
    realm_rotate_key_validate,
    realm_share_validate,
    realm_unshare_validate,
)
from parsec.events import EventRealmCertificate


class MemoryRealmComponent(BaseRealmComponent):
    def __init__(self, data: MemoryDatamodel, event_bus: EventBus) -> None:
        self._data = data
        self._event_bus = event_bus

    @override
    async def create(
        self,
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
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return RealmCreateStoreBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return RealmCreateStoreBadOutcome.ORGANIZATION_EXPIRED

        async with org.topics_lock(read=["common"]) as (common_topic_last_timestamp,):
            try:
                author_device = org.devices[author]
            except KeyError:
                return RealmCreateStoreBadOutcome.AUTHOR_NOT_FOUND
            author_user_id = author_device.cooked.user_id

            author_user = org.users[author_user_id]
            if author_user.is_revoked:
                return RealmCreateStoreBadOutcome.AUTHOR_REVOKED

            assert author_verify_key == author_device.cooked.verify_key
            match realm_create_validate(
                now=now,
                expected_author_user_id=author_user_id,
                expected_author_device_id=author,
                author_verify_key=author_verify_key,
                realm_role_certificate=realm_role_certificate,
            ):
                case RealmRoleCertificate() as certif:
                    pass
                case error:
                    return error

            if certif.realm_id in org.realms:
                return CertificateBasedActionIdempotentOutcome(
                    certificate_timestamp=org.per_topic_last_timestamp[("realm", certif.realm_id)]
                )

            realm_topic = ("realm", certif.realm_id)

            async with org.topics_lock(write=[realm_topic]) as (realm_topic_last_timestamp,):
                # Ensure we are not breaking causality by adding a newer timestamp.

                last_certificate = max(common_topic_last_timestamp, realm_topic_last_timestamp)
                if last_certificate >= certif.timestamp:
                    return RequireGreaterTimestamp(strictly_greater_than=last_certificate)

                # All checks are good, now we do the actual insertion

                org.per_topic_last_timestamp[realm_topic] = certif.timestamp

                org.realms[certif.realm_id] = MemoryRealm(
                    realm_id=certif.realm_id,
                    created_on=now,
                    roles=[
                        MemoryRealmUserRole(
                            cooked=certif, realm_role_certificate=realm_role_certificate
                        )
                    ],
                )

                await self._event_bus.send(
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
    async def share(
        self,
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
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return RealmShareStoreBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return RealmShareStoreBadOutcome.ORGANIZATION_EXPIRED

        async with org.topics_lock(read=["common"]) as (common_topic_last_timestamp,):
            try:
                author_device = org.devices[author]
            except KeyError:
                return RealmShareStoreBadOutcome.AUTHOR_NOT_FOUND
            author_user_id = author_device.cooked.user_id

            author_user = org.users[author_user_id]
            if author_user.is_revoked:
                return RealmShareStoreBadOutcome.AUTHOR_REVOKED

            assert author_verify_key == author_device.cooked.verify_key
            match realm_share_validate(
                now=now,
                expected_author_user_id=author_user_id,
                expected_author_device_id=author,
                author_verify_key=author_verify_key,
                realm_role_certificate=realm_role_certificate,
            ):
                case RealmRoleCertificate() as certif:
                    pass
                case error:
                    return error

            try:
                user = org.users[certif.user_id]
            except KeyError:
                return RealmShareStoreBadOutcome.RECIPIENT_NOT_FOUND

            if user.is_revoked:
                return RealmShareStoreBadOutcome.RECIPIENT_REVOKED

            try:
                realm = org.realms[certif.realm_id]
            except KeyError:
                return RealmShareStoreBadOutcome.REALM_NOT_FOUND

            realm_topic = ("realm", certif.realm_id)

            async with org.topics_lock(write=[realm_topic]) as (realm_topic_last_timestamp,):
                owner_only = (RealmRole.OWNER,)
                owner_or_manager = (RealmRole.OWNER, RealmRole.MANAGER)
                existing_user_role = realm.get_current_role_for(certif.user_id)
                new_user_role = certif.role
                needed_roles: tuple[RealmRole, ...]
                if existing_user_role in owner_or_manager or new_user_role in owner_or_manager:
                    needed_roles = owner_only
                else:
                    needed_roles = owner_or_manager

                author_role = realm.get_current_role_for(author_user_id)
                if author_role not in needed_roles:
                    return RealmShareStoreBadOutcome.AUTHOR_NOT_ALLOWED

                if user.current_profile == UserProfile.OUTSIDER and certif.role in (
                    RealmRole.MANAGER,
                    RealmRole.OWNER,
                ):
                    return RealmShareStoreBadOutcome.ROLE_INCOMPATIBLE_WITH_OUTSIDER

                if existing_user_role == new_user_role:
                    return CertificateBasedActionIdempotentOutcome(
                        certificate_timestamp=realm_topic_last_timestamp
                    )

                # Ensure we are not breaking causality by adding a newer timestamp.

                if realm.last_vlob_timestamp is not None:
                    last_timestamp = max(
                        common_topic_last_timestamp,
                        realm_topic_last_timestamp,
                        realm.last_vlob_timestamp,
                    )
                else:
                    last_timestamp = max(common_topic_last_timestamp, realm_topic_last_timestamp)
                if last_timestamp >= certif.timestamp:
                    return RequireGreaterTimestamp(strictly_greater_than=last_timestamp)

                try:
                    last_key_rotation = realm.key_rotations[-1]
                except IndexError:
                    return BadKeyIndex(last_realm_certificate_timestamp=realm_topic_last_timestamp)
                if key_index != last_key_rotation.cooked.key_index:
                    return BadKeyIndex(last_realm_certificate_timestamp=realm_topic_last_timestamp)

                # All checks are good, now we do the actual insertion

                org.per_topic_last_timestamp[realm_topic] = certif.timestamp

                realm.roles.append(
                    MemoryRealmUserRole(
                        cooked=certif, realm_role_certificate=realm_role_certificate
                    )
                )

                last_key_rotation.per_participant_keys_bundle_access[certif.user_id] = (
                    recipient_keys_bundle_access
                )

                await self._event_bus.send(
                    EventRealmCertificate(
                        organization_id=organization_id,
                        timestamp=certif.timestamp,
                        realm_id=certif.realm_id,
                        user_id=certif.user_id,
                        role_removed=False,
                    )
                )

                return certif

    @override
    async def unshare(
        self,
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
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return RealmUnshareStoreBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return RealmUnshareStoreBadOutcome.ORGANIZATION_EXPIRED

        async with org.topics_lock(read=["common"]) as (common_topic_last_timestamp,):
            try:
                author_device = org.devices[author]
            except KeyError:
                return RealmUnshareStoreBadOutcome.AUTHOR_NOT_FOUND
            author_user_id = author_device.cooked.user_id

            author_user = org.users[author_user_id]
            if author_user.is_revoked:
                return RealmUnshareStoreBadOutcome.AUTHOR_REVOKED

            assert author_verify_key == author_device.cooked.verify_key
            match realm_unshare_validate(
                now=now,
                expected_author_user_id=author_user_id,
                expected_author_device_id=author,
                author_verify_key=author_verify_key,
                realm_role_certificate=realm_role_certificate,
            ):
                case RealmRoleCertificate() as certif:
                    pass
                case error:
                    return error

            # Note we don't check if the recipient is revoked here: it is indeed allowed
            # to unshare with a revoked user. This allows for client to only check for
            # unshare event to detect when key rotation is needed.
            if certif.user_id not in org.users:
                return RealmUnshareStoreBadOutcome.RECIPIENT_NOT_FOUND

            try:
                realm = org.realms[certif.realm_id]
            except KeyError:
                return RealmUnshareStoreBadOutcome.REALM_NOT_FOUND

            realm_topic = ("realm", certif.realm_id)

            async with org.topics_lock(write=[realm_topic]) as (realm_topic_last_timestamp,):
                owner_only = (RealmRole.OWNER,)
                owner_or_manager = (RealmRole.OWNER, RealmRole.MANAGER)
                existing_user_role = realm.get_current_role_for(certif.user_id)
                new_user_role = certif.role
                needed_roles: tuple[RealmRole, ...]
                if existing_user_role in owner_or_manager or new_user_role in owner_or_manager:
                    needed_roles = owner_only
                else:
                    needed_roles = owner_or_manager

                author_role = realm.get_current_role_for(author_user_id)
                if author_role not in needed_roles:
                    return RealmUnshareStoreBadOutcome.AUTHOR_NOT_ALLOWED

                if existing_user_role == new_user_role:
                    return CertificateBasedActionIdempotentOutcome(
                        certificate_timestamp=realm_topic_last_timestamp
                    )

                # Ensure we are not breaking causality by adding a newer timestamp.

                if realm.last_vlob_timestamp is not None:
                    last_timestamp = max(
                        common_topic_last_timestamp,
                        realm_topic_last_timestamp,
                        realm.last_vlob_timestamp,
                    )
                else:
                    last_timestamp = max(common_topic_last_timestamp, realm_topic_last_timestamp)
                if last_timestamp >= certif.timestamp:
                    return RequireGreaterTimestamp(strictly_greater_than=last_timestamp)

                # All checks are good, now we do the actual insertion

                org.per_topic_last_timestamp[realm_topic] = certif.timestamp

                realm.roles.append(
                    MemoryRealmUserRole(
                        cooked=certif, realm_role_certificate=realm_role_certificate
                    )
                )

                await self._event_bus.send(
                    EventRealmCertificate(
                        organization_id=organization_id,
                        timestamp=certif.timestamp,
                        realm_id=certif.realm_id,
                        user_id=certif.user_id,
                        role_removed=True,
                    )
                )

                return certif

    @override
    async def rename(
        self,
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
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return RealmRenameStoreBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return RealmRenameStoreBadOutcome.ORGANIZATION_EXPIRED

        async with org.topics_lock(read=["common"]) as (common_topic_last_timestamp,):
            try:
                author_device = org.devices[author]
            except KeyError:
                return RealmRenameStoreBadOutcome.AUTHOR_NOT_FOUND
            author_user_id = author_device.cooked.user_id

            author_user = org.users[author_user_id]
            if author_user.is_revoked:
                return RealmRenameStoreBadOutcome.AUTHOR_REVOKED

            assert author_verify_key == author_device.cooked.verify_key
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

            try:
                realm = org.realms[certif.realm_id]
            except KeyError:
                return RealmRenameStoreBadOutcome.REALM_NOT_FOUND

            realm_topic = ("realm", certif.realm_id)

            async with org.topics_lock(write=[realm_topic]) as (realm_topic_last_timestamp,):
                if realm.get_current_role_for(author_user_id) != RealmRole.OWNER:
                    return RealmRenameStoreBadOutcome.AUTHOR_NOT_ALLOWED

                # We only accept the last key
                if len(realm.key_rotations) != certif.key_index:
                    return BadKeyIndex(last_realm_certificate_timestamp=realm_topic_last_timestamp)

                if initial_name_or_fail and realm.renames:
                    return CertificateBasedActionIdempotentOutcome(
                        certificate_timestamp=realm_topic_last_timestamp
                    )

                # Ensure we are not breaking causality by adding a newer timestamp.

                last_certificate = max(common_topic_last_timestamp, realm_topic_last_timestamp)
                if last_certificate >= certif.timestamp:
                    return RequireGreaterTimestamp(strictly_greater_than=last_certificate)

                # All checks are good, now we do the actual insertion

                org.per_topic_last_timestamp[realm_topic] = certif.timestamp

                realm.renames.append(
                    MemoryRealmRename(
                        cooked=certif,
                        realm_name_certificate=realm_name_certificate,
                    )
                )

                await self._event_bus.send(
                    EventRealmCertificate(
                        organization_id=organization_id,
                        timestamp=certif.timestamp,
                        realm_id=certif.realm_id,
                        user_id=author_user_id,
                        role_removed=False,
                    )
                )

                return certif

    @override
    async def rotate_key(
        self,
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
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return RealmRotateKeyStoreBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return RealmRotateKeyStoreBadOutcome.ORGANIZATION_EXPIRED

        async with org.topics_lock(read=["common"]) as (common_topic_last_timestamp,):
            try:
                author_device = org.devices[author]
            except KeyError:
                return RealmRotateKeyStoreBadOutcome.AUTHOR_NOT_FOUND
            author_user_id = author_device.cooked.user_id

            author_user = org.users[author_user_id]
            if author_user.is_revoked:
                return RealmRotateKeyStoreBadOutcome.AUTHOR_REVOKED

            assert author_verify_key == author_device.cooked.verify_key
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

            try:
                realm = org.realms[certif.realm_id]
            except KeyError:
                return RealmRotateKeyStoreBadOutcome.REALM_NOT_FOUND

            realm_topic = ("realm", certif.realm_id)

            async with org.topics_lock(write=[realm_topic]) as (realm_topic_last_timestamp,):
                if realm.get_current_role_for(author_user_id) != RealmRole.OWNER:
                    return RealmRotateKeyStoreBadOutcome.AUTHOR_NOT_ALLOWED

                last_index = len(realm.key_rotations)
                if certif.key_index != last_index + 1:
                    return BadKeyIndex(last_realm_certificate_timestamp=realm_topic_last_timestamp)

                participants = set()
                for role in realm.roles:
                    if role.cooked.role is None:
                        participants.discard(role.cooked.user_id)
                    else:
                        participants.add(role.cooked.user_id)

                if per_participant_keys_bundle_access.keys() != participants:
                    return RealmRotateKeyStoreBadOutcome.PARTICIPANT_MISMATCH

                # Ensure we are not breaking causality by adding a newer timestamp.

                last_certificate = max(common_topic_last_timestamp, realm_topic_last_timestamp)
                if last_certificate >= certif.timestamp:
                    return RequireGreaterTimestamp(strictly_greater_than=last_certificate)

                # All checks are good, now we do the actual insertion

                org.per_topic_last_timestamp[realm_topic] = certif.timestamp

                realm.key_rotations.append(
                    MemoryRealmKeyRotation(
                        cooked=certif,
                        realm_key_rotation_certificate=realm_key_rotation_certificate,
                        per_participant_keys_bundle_access=per_participant_keys_bundle_access,
                        keys_bundle=keys_bundle,
                    )
                )

                await self._event_bus.send(
                    EventRealmCertificate(
                        organization_id=organization_id,
                        timestamp=certif.timestamp,
                        realm_id=certif.realm_id,
                        user_id=author_user_id,
                        role_removed=False,
                    )
                )

                return certif

    @override
    async def get_keys_bundle(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: VlobID,
        key_index: int | None,
    ) -> KeysBundle | RealmGetKeysBundleBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return RealmGetKeysBundleBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return RealmGetKeysBundleBadOutcome.ORGANIZATION_EXPIRED

        try:
            author_device = org.devices[author]
        except KeyError:
            return RealmGetKeysBundleBadOutcome.AUTHOR_NOT_FOUND
        author_user_id = author_device.cooked.user_id

        author_user = org.users[author_user_id]
        if author_user.is_revoked:
            return RealmGetKeysBundleBadOutcome.AUTHOR_REVOKED

        try:
            realm = org.realms[realm_id]
        except KeyError:
            return RealmGetKeysBundleBadOutcome.REALM_NOT_FOUND

        if realm.get_current_role_for(author_user_id) is None:
            return RealmGetKeysBundleBadOutcome.AUTHOR_NOT_ALLOWED

        # `key_index` starts at 1, but the array starts at 0
        match key_index:
            case 0:
                return RealmGetKeysBundleBadOutcome.BAD_KEY_INDEX
            case None:
                # Take the last one
                key_rotations_array_index = -1
            case key_index:
                key_rotations_array_index = key_index - 1
        try:
            key_rotation = realm.key_rotations[key_rotations_array_index]
        except IndexError:
            return RealmGetKeysBundleBadOutcome.BAD_KEY_INDEX

        try:
            keys_bundle_access = key_rotation.per_participant_keys_bundle_access[author_user_id]
        except KeyError:
            return RealmGetKeysBundleBadOutcome.ACCESS_NOT_AVAILABLE_FOR_AUTHOR

        return KeysBundle(
            key_index=key_rotation.cooked.key_index,
            keys_bundle_access=keys_bundle_access,
            keys_bundle=key_rotation.keys_bundle,
        )

    @override
    async def get_stats(
        self, organization_id: OrganizationID, author: DeviceID, realm_id: VlobID
    ) -> RealmStats | RealmGetStatsAsUserBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return RealmGetStatsAsUserBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return RealmGetStatsAsUserBadOutcome.ORGANIZATION_EXPIRED

        try:
            author_device = org.devices[author]
        except KeyError:
            return RealmGetStatsAsUserBadOutcome.AUTHOR_NOT_FOUND
        author_user_id = author_device.cooked.user_id

        author_user = org.users[author_user_id]
        if author_user.is_revoked:
            return RealmGetStatsAsUserBadOutcome.AUTHOR_REVOKED

        try:
            realm = org.realms[realm_id]
        except KeyError:
            return RealmGetStatsAsUserBadOutcome.REALM_NOT_FOUND

        if realm.get_current_role_for(author_user_id) is None:
            return RealmGetStatsAsUserBadOutcome.AUTHOR_NOT_ALLOWED

        block_size = 0
        vlob_size = 0

        for vlob in org.vlobs.values():
            for vlob_atom in vlob:
                vlob_size += len(vlob_atom.blob)

        for block in org.blocks.values():
            if block.realm_id == realm_id:
                block_size += block.block_size

        return RealmStats(
            blocks_size=block_size,
            vlobs_size=vlob_size,
        )

    @override
    async def get_current_realms_for_user(
        self, organization_id: OrganizationID, user: UserID
    ) -> dict[VlobID, RealmRole] | RealmGetCurrentRealmsForUserBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return RealmGetCurrentRealmsForUserBadOutcome.ORGANIZATION_NOT_FOUND

        if user not in org.users:
            return RealmGetCurrentRealmsForUserBadOutcome.USER_NOT_FOUND

        user_realms = {}
        for realm in org.realms.values():
            role = realm.get_current_role_for(user)
            if role is not None:
                user_realms[realm.realm_id] = role

        return user_realms

    @override
    async def dump_realms_granted_roles(
        self, organization_id: OrganizationID
    ) -> list[RealmGrantedRole] | RealmDumpRealmsGrantedRolesBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return RealmDumpRealmsGrantedRolesBadOutcome.ORGANIZATION_NOT_FOUND

        granted_roles = []
        for realm in org.realms.values():
            for role in realm.roles:
                granted_roles.append(
                    RealmGrantedRole(
                        realm_id=realm.realm_id,
                        certificate=role.realm_role_certificate,
                        user_id=role.cooked.user_id,
                        role=role.cooked.role,
                        granted_by=role.cooked.author,
                        granted_on=role.cooked.timestamp,
                    )
                )

        return granted_roles
