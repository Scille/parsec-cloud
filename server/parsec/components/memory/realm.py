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
    SequesterServiceID,
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
    ParticipantMismatch,
    RealmCreateStoreBadOutcome,
    RealmCreateValidateBadOutcome,
    RealmDumpRealmsGrantedRolesBadOutcome,
    RealmExportBatchOffsetMarker,
    RealmExportBlocksMetadataBatch,
    RealmExportBlocksMetadataBatchItem,
    RealmExportCertificates,
    RealmExportDoBaseInfo,
    RealmExportDoBaseInfoBadOutcome,
    RealmExportDoBlocksBatchMetadataBadOutcome,
    RealmExportDoCertificatesBadOutcome,
    RealmExportDoVlobsBatchBadOutcome,
    RealmExportVlobsBatch,
    RealmExportVlobsBatchItem,
    RealmGetCurrentRealmsForUserBadOutcome,
    RealmGetKeysBundleBadOutcome,
    RealmGrantedRole,
    RealmRenameStoreBadOutcome,
    RealmRenameValidateBadOutcome,
    RealmRole,
    RealmRotateKeyStoreBadOutcome,
    RealmRotateKeyValidateBadOutcome,
    RealmShareStoreBadOutcome,
    RealmShareValidateBadOutcome,
    RealmUnshareStoreBadOutcome,
    RealmUnshareValidateBadOutcome,
    RejectedBySequesterService,
    SequesterServiceMismatch,
    SequesterServiceUnavailable,
    realm_create_validate,
    realm_rename_validate,
    realm_rotate_key_validate,
    realm_share_validate,
    realm_unshare_validate,
)
from parsec.components.sequester import SequesterServiceType
from parsec.events import EventRealmCertificate
from parsec.webhooks import WebhooksComponent


class MemoryRealmComponent(BaseRealmComponent):
    def __init__(
        self, data: MemoryDatamodel, event_bus: EventBus, webhooks: WebhooksComponent
    ) -> None:
        super().__init__(webhooks)
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

            if author_user.current_profile == UserProfile.OUTSIDER:
                return RealmCreateStoreBadOutcome.AUTHOR_NOT_ALLOWED

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

                if certif.user_id not in last_key_rotation.per_participant_keys_bundle_accesses:
                    last_key_rotation.per_participant_keys_bundle_accesses[certif.user_id] = []
                last_key_rotation.per_participant_keys_bundle_accesses[certif.user_id].append(
                    (certif.timestamp, recipient_keys_bundle_access)
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

            async with org.topics_lock(read=["sequester"], write=[realm_topic]) as (
                sequester_topic_last_timestamp,
                realm_topic_last_timestamp,
            ):
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
                    return ParticipantMismatch(
                        last_realm_certificate_timestamp=realm_topic_last_timestamp
                    )

                if org.is_sequestered:
                    assert org.sequester_services is not None
                    if per_sequester_service_keys_bundle_access is None:
                        return SequesterServiceMismatch(
                            last_sequester_certificate_timestamp=sequester_topic_last_timestamp
                        )

                    active_services_ids = {
                        service.cooked.service_id for service in org.active_sequester_services()
                    }
                    provided_services_ids = (
                        per_sequester_service_keys_bundle_access.keys()
                        if per_sequester_service_keys_bundle_access
                        else set()
                    )
                    if active_services_ids != provided_services_ids:
                        return SequesterServiceMismatch(
                            last_sequester_certificate_timestamp=sequester_topic_last_timestamp
                        )
                    for service in org.active_sequester_services():
                        if service.service_type == SequesterServiceType.WEBHOOK:
                            assert service.webhook_url is not None

                            keys_bundle_access = per_sequester_service_keys_bundle_access[
                                service.cooked.service_id
                            ]
                            match await self.webhooks.sequester_service_on_realm_rotate_key(
                                webhook_url=service.webhook_url,
                                service_id=service.cooked.service_id,
                                organization_id=organization_id,
                                keys_bundle=keys_bundle,
                                keys_bundle_access=keys_bundle_access,
                                author=certif.author,
                                timestamp=certif.timestamp,
                                realm_id=certif.realm_id,
                                key_index=certif.key_index,
                                encryption_algorithm=certif.encryption_algorithm,
                                hash_algorithm=certif.hash_algorithm,
                                key_canary=certif.key_canary,
                            ):
                                case None:
                                    pass
                                case error:
                                    return error

                elif per_sequester_service_keys_bundle_access is not None:
                    return RealmRotateKeyStoreBadOutcome.ORGANIZATION_NOT_SEQUESTERED

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
                        per_participant_keys_bundle_accesses={
                            k: [(certif.timestamp, v)]
                            for k, v in per_participant_keys_bundle_access.items()
                        },
                        per_sequester_service_keys_bundle_access=per_sequester_service_keys_bundle_access,
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
            _, keys_bundle_access = key_rotation.per_participant_keys_bundle_accesses[
                author_user_id
            ][-1]
        except KeyError:
            return RealmGetKeysBundleBadOutcome.ACCESS_NOT_AVAILABLE_FOR_AUTHOR

        return KeysBundle(
            key_index=key_rotation.cooked.key_index,
            keys_bundle_access=keys_bundle_access,
            keys_bundle=key_rotation.keys_bundle,
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

        granted_roles: list[RealmGrantedRole] = []
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

    @override
    async def export_do_base_info(
        self,
        organization_id: OrganizationID,
        realm_id: VlobID,
        snapshot_timestamp: DateTime,
    ) -> RealmExportDoBaseInfo | RealmExportDoBaseInfoBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return RealmExportDoBaseInfoBadOutcome.ORGANIZATION_NOT_FOUND

        if not org.is_bootstrapped:
            return RealmExportDoBaseInfoBadOutcome.ORGANIZATION_NOT_FOUND

        root_verify_key = org.root_verify_key
        assert root_verify_key is not None

        if realm_id not in org.realms:
            return RealmExportDoBaseInfoBadOutcome.REALM_NOT_FOUND

        vlob_offset_marker_upper_bound = 0
        vlobs_total_bytes = 0
        for sequential_row_id, row in org.simulate_postgresql_vlob_atom_table():
            if row.created_on > snapshot_timestamp:
                break
            if row.realm_id == realm_id:
                vlob_offset_marker_upper_bound = sequential_row_id
                vlobs_total_bytes += len(row.blob)

        blocks_total_bytes = 0
        block_offset_marker_upper_bound = 0
        for sequential_row_id, row in org.simulate_postgresql_block_table():
            if row.created_on > snapshot_timestamp:
                break
            if row.realm_id == realm_id:
                block_offset_marker_upper_bound = sequential_row_id
                blocks_total_bytes += row.block_size

        common_certificate_timestamp_upper_bound = None
        for ts, _, _ in org.ordered_common_certificates():
            if ts > snapshot_timestamp:
                break
            common_certificate_timestamp_upper_bound = ts
        # The organization has been bootstrapped, there *must* be some common certificates
        assert common_certificate_timestamp_upper_bound is not None

        realm_certificate_timestamp_upper_bound = None
        for ts, _, _ in org.ordered_realm_certificates(realm_id):
            if ts > snapshot_timestamp:
                break
            realm_certificate_timestamp_upper_bound = ts
        if realm_certificate_timestamp_upper_bound is None:
            return RealmExportDoBaseInfoBadOutcome.REALM_DIDNT_EXIST_AT_SNAPSHOT_TIMESTAMP

        sequester_certificate_timestamp_upper_bound = None
        for ts, _, _ in org.ordered_sequester_certificates():
            if ts > snapshot_timestamp:
                break
            sequester_certificate_timestamp_upper_bound = ts
        # It is possible to export a non-sequestered organization, in which case
        # there is no sequester certificate at all.

        return RealmExportDoBaseInfo(
            root_verify_key=root_verify_key,
            vlob_offset_marker_upper_bound=vlob_offset_marker_upper_bound,
            block_offset_marker_upper_bound=block_offset_marker_upper_bound,
            blocks_total_bytes=blocks_total_bytes,
            vlobs_total_bytes=vlobs_total_bytes,
            common_certificate_timestamp_upper_bound=common_certificate_timestamp_upper_bound,
            realm_certificate_timestamp_upper_bound=realm_certificate_timestamp_upper_bound,
            sequester_certificate_timestamp_upper_bound=sequester_certificate_timestamp_upper_bound,
        )

    @override
    async def export_do_certificates(
        self,
        organization_id: OrganizationID,
        realm_id: VlobID,
        common_certificate_timestamp_upper_bound: DateTime,
        realm_certificate_timestamp_upper_bound: DateTime,
        sequester_certificate_timestamp_upper_bound: DateTime | None,
    ) -> RealmExportCertificates | RealmExportDoCertificatesBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return RealmExportDoCertificatesBadOutcome.ORGANIZATION_NOT_FOUND

        try:
            realm = org.realms[realm_id]
        except KeyError:
            return RealmExportDoCertificatesBadOutcome.REALM_NOT_FOUND

        common_certificates: list[bytes] = []
        for ts, raw, _ in org.ordered_common_certificates():
            if ts > common_certificate_timestamp_upper_bound:
                break
            common_certificates.append(raw)
        # The organization has been bootstrapped, there *must* be some common certificates
        assert common_certificates

        realm_certificates: list[bytes] = []
        for ts, raw, _ in org.ordered_realm_certificates(realm_id):
            if ts > realm_certificate_timestamp_upper_bound:
                break
            realm_certificates.append(raw)
        # The realm existence has already been checked, there *must* be some realm certificates
        assert realm_certificates

        sequester_certificates: list[bytes] = []
        if sequester_certificate_timestamp_upper_bound is not None:
            for ts, raw, _ in org.ordered_sequester_certificates():
                if ts > sequester_certificate_timestamp_upper_bound:
                    break
                sequester_certificates.append(raw)

        realm_keys_bundles: list[tuple[int, bytes]] = []
        realm_keys_bundle_user_accesses: list[tuple[UserID, int, bytes]] = []
        realm_keys_bundle_sequester_accesses: list[tuple[SequesterServiceID, int, bytes]] = []
        for key_rotation in realm.key_rotations:
            if key_rotation.cooked.timestamp > realm_certificate_timestamp_upper_bound:
                break
            realm_keys_bundles.append((key_rotation.cooked.key_index, key_rotation.keys_bundle))
            for user_id, accesses in key_rotation.per_participant_keys_bundle_accesses.items():
                for access_timestamp, access in accesses:
                    if access_timestamp > realm_certificate_timestamp_upper_bound:
                        break
                    realm_keys_bundle_user_accesses.append(
                        (user_id, key_rotation.cooked.key_index, access)
                    )
            if key_rotation.per_sequester_service_keys_bundle_access is not None:
                for (
                    sequester_id,
                    access,
                ) in key_rotation.per_sequester_service_keys_bundle_access.items():
                    realm_keys_bundle_sequester_accesses.append(
                        (sequester_id, key_rotation.cooked.key_index, access)
                    )

        return RealmExportCertificates(
            common_certificates=common_certificates,
            sequester_certificates=sequester_certificates,
            realm_certificates=realm_certificates,
            realm_keys_bundles=realm_keys_bundles,
            realm_keys_bundle_user_accesses=realm_keys_bundle_user_accesses,
            realm_keys_bundle_sequester_accesses=realm_keys_bundle_sequester_accesses,
        )

    @override
    async def export_do_vlobs_batch(
        self,
        organization_id: OrganizationID,
        realm_id: VlobID,
        batch_offset_marker: RealmExportBatchOffsetMarker,
        batch_size: int,
    ) -> RealmExportVlobsBatch | RealmExportDoVlobsBatchBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return RealmExportDoVlobsBatchBadOutcome.ORGANIZATION_NOT_FOUND

        if realm_id not in org.realms:
            return RealmExportDoVlobsBatchBadOutcome.REALM_NOT_FOUND

        items: list[RealmExportVlobsBatchItem] = []
        for sequential_row_id, row in org.simulate_postgresql_vlob_atom_table():
            # Skip already processed rows
            if sequential_row_id <= batch_offset_marker:
                continue

            if row.realm_id != realm_id:
                continue

            items.append(
                RealmExportVlobsBatchItem(
                    sequential_id=sequential_row_id,
                    vlob_id=row.vlob_id,
                    version=row.version,
                    key_index=row.key_index,
                    blob=row.blob,
                    size=len(row.blob),
                    author=row.author,
                    timestamp=row.created_on,
                )
            )

            if len(items) >= batch_size:
                break

        return RealmExportVlobsBatch(
            items=items,
        )

    async def export_do_blocks_metadata_batch(
        self,
        organization_id: OrganizationID,
        realm_id: VlobID,
        batch_offset_marker: RealmExportBatchOffsetMarker,
        batch_size: int,
    ) -> RealmExportBlocksMetadataBatch | RealmExportDoBlocksBatchMetadataBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return RealmExportDoBlocksBatchMetadataBadOutcome.ORGANIZATION_NOT_FOUND

        if realm_id not in org.realms:
            return RealmExportDoBlocksBatchMetadataBadOutcome.REALM_NOT_FOUND

        items: list[RealmExportBlocksMetadataBatchItem] = []
        for sequential_row_id, row in org.simulate_postgresql_block_table():
            # Skip already processed rows
            if sequential_row_id <= batch_offset_marker:
                continue

            if row.realm_id != realm_id:
                continue

            items.append(
                RealmExportBlocksMetadataBatchItem(
                    sequential_id=sequential_row_id,
                    block_id=row.block_id,
                    author=row.author,
                    key_index=row.key_index,
                    size=row.block_size,
                )
            )

            if len(items) >= batch_size:
                break

        return RealmExportBlocksMetadataBatch(
            items=items,
        )
