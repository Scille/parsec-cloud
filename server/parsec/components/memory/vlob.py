# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import override

from parsec._parsec import (
    DateTime,
    DeviceID,
    OrganizationID,
    RealmRole,
    SequesterServiceID,
    VlobID,
)
from parsec.ballpark import (
    RequireGreaterTimestamp,
    TimestampOutOfBallpark,
    timestamps_in_the_ballpark,
)
from parsec.components.events import EventBus
from parsec.components.memory.datamodel import (
    MemoryDatamodel,
    MemoryRealmVlobUpdate,
    MemoryVlobAtom,
)
from parsec.components.realm import BadKeyIndex
from parsec.components.sequester import SequesterServiceType
from parsec.components.vlob import (
    BaseVlobComponent,
    RejectedBySequesterService,
    SequesterServiceUnavailable,
    VlobCreateBadOutcome,
    VlobPollChangesAsUserBadOutcome,
    VlobReadAsUserBadOutcome,
    VlobReadResult,
    VlobUpdateBadOutcome,
)
from parsec.events import EVENT_VLOB_MAX_BLOB_SIZE, EventVlob
from parsec.webhooks import WebhooksComponent

# Tuple contains: blob, author, timestamp, certificate index at the time of creation
type VlobData = list[tuple[bytes, DeviceID, DateTime, int]]
type SequesteredVlobData = list[dict[SequesterServiceID, bytes]]


class MemoryVlobComponent(BaseVlobComponent):
    def __init__(
        self,
        data: MemoryDatamodel,
        event_bus: EventBus,
        webhooks: WebhooksComponent,
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
        realm_id: VlobID,
        vlob_id: VlobID,
        key_index: int,
        timestamp: DateTime,
        blob: bytes,
    ) -> (
        None
        | BadKeyIndex
        | VlobCreateBadOutcome
        | TimestampOutOfBallpark
        | RequireGreaterTimestamp
        | RejectedBySequesterService
        | SequesterServiceUnavailable
    ):
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return VlobCreateBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return VlobCreateBadOutcome.ORGANIZATION_EXPIRED

        async with org.topics_lock(read=["common", ("realm", realm_id)]) as (
            common_topic_last_timestamp,
            realm_topic_last_timestamp,
        ):
            try:
                author_device = org.devices[author]
            except KeyError:
                return VlobCreateBadOutcome.AUTHOR_NOT_FOUND
            author_user_id = author_device.cooked.user_id

            author_user = org.users[author_user_id]
            if author_user.is_revoked:
                return VlobCreateBadOutcome.AUTHOR_REVOKED

            try:
                realm = org.realms[realm_id]
            except KeyError:
                return VlobCreateBadOutcome.REALM_NOT_FOUND

            match realm.get_current_role_for(author_user_id):
                case RealmRole.READER | None:
                    return VlobCreateBadOutcome.AUTHOR_NOT_ALLOWED

                case RealmRole.OWNER | RealmRole.MANAGER | RealmRole.CONTRIBUTOR:
                    pass

                case unknown:
                    # TODO: Implement `Enum` on `RealmRole` so we can use `assert_never` here
                    assert False, unknown

            # We only accept the last key
            if len(realm.key_rotations) != key_index:
                return BadKeyIndex(last_realm_certificate_timestamp=realm_topic_last_timestamp)

            if vlob_id in realm.vlobs:
                return VlobCreateBadOutcome.VLOB_ALREADY_EXISTS

            maybe_error = timestamps_in_the_ballpark(timestamp, now)
            if maybe_error is not None:
                return maybe_error

            # Ensure we are not breaking causality by adding a newer timestamp.

            last_certificate = max(common_topic_last_timestamp, realm_topic_last_timestamp)
            if last_certificate >= timestamp:
                return RequireGreaterTimestamp(strictly_greater_than=last_certificate)

            if org.is_sequestered:
                assert org.sequester_services is not None
                for service_id, service in org.sequester_services.items():
                    if service.is_revoked:
                        continue
                    if service.service_type == SequesterServiceType.WEBHOOK:
                        assert service.webhook_url is not None
                        match await self.webhooks.sequester_service_on_vlob_create_or_update(
                            webhook_url=service.webhook_url,
                            service_id=service_id,
                            organization_id=organization_id,
                            author=author,
                            realm_id=realm_id,
                            vlob_id=vlob_id,
                            key_index=key_index,
                            version=1,
                            timestamp=timestamp,
                            blob=blob,
                        ):
                            case None:
                                pass
                            case error:
                                return error

            # All checks are good, now we do the actual insertion

            match realm.last_vlob_timestamp:
                case None:
                    realm.last_vlob_timestamp = timestamp
                case previous_last_vlob_timestamp:
                    realm.last_vlob_timestamp = max(previous_last_vlob_timestamp, timestamp)

            vlob_atom = MemoryVlobAtom(
                realm_id=realm_id,
                vlob_id=vlob_id,
                key_index=key_index,
                version=1,
                blob=blob,
                author=author,
                created_on=timestamp,
            )
            realm.vlobs[vlob_id] = [vlob_atom]
            realm_change_checkpoint = len(realm.vlob_updates) + 1
            realm.vlob_updates.append(
                MemoryRealmVlobUpdate(
                    index=realm_change_checkpoint,
                    vlob_atom=vlob_atom,
                )
            )

            await self._event_bus.send(
                EventVlob(
                    organization_id=organization_id,
                    author=author,
                    realm_id=realm_id,
                    timestamp=timestamp,
                    vlob_id=vlob_id,
                    version=1,
                    blob=blob if len(blob) < EVENT_VLOB_MAX_BLOB_SIZE else None,
                    last_common_certificate_timestamp=common_topic_last_timestamp,
                    last_realm_certificate_timestamp=realm_topic_last_timestamp,
                )
            )

    async def update(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: VlobID,
        vlob_id: VlobID,
        key_index: int,
        version: int,
        timestamp: DateTime,
        blob: bytes,
    ) -> (
        None
        | BadKeyIndex
        | VlobUpdateBadOutcome
        | TimestampOutOfBallpark
        | RequireGreaterTimestamp
        | RejectedBySequesterService
        | SequesterServiceUnavailable
    ):
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return VlobUpdateBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return VlobUpdateBadOutcome.ORGANIZATION_EXPIRED

        async with org.topics_lock(read=["common"]) as (common_topic_last_timestamp,):
            try:
                author_device = org.devices[author]
            except KeyError:
                return VlobUpdateBadOutcome.AUTHOR_NOT_FOUND
            author_user_id = author_device.cooked.user_id

            author_user = org.users[author_user_id]
            if author_user.is_revoked:
                return VlobUpdateBadOutcome.AUTHOR_REVOKED

            try:
                realm = org.realms[realm_id]
            except KeyError:
                return VlobUpdateBadOutcome.REALM_NOT_FOUND

            try:
                vlobs = realm.vlobs[vlob_id]
            except KeyError:
                return VlobUpdateBadOutcome.VLOB_NOT_FOUND

            async with org.topics_lock(read=[("realm", realm_id)]) as (realm_topic_last_timestamp,):
                realm = org.realms[realm_id]
                match realm.get_current_role_for(author_user_id):
                    case RealmRole.READER | None:
                        return VlobUpdateBadOutcome.AUTHOR_NOT_ALLOWED

                    case RealmRole.OWNER | RealmRole.MANAGER | RealmRole.CONTRIBUTOR:
                        pass

                    case unknown:
                        # TODO: Implement `Enum` on `RealmRole` so we can use `assert_never` here
                        assert False, unknown

                # We only accept the last key
                if len(realm.key_rotations) != key_index:
                    return BadKeyIndex(last_realm_certificate_timestamp=realm_topic_last_timestamp)

                maybe_error = timestamps_in_the_ballpark(timestamp, now)
                if maybe_error is not None:
                    return maybe_error

                # Ensure we are not breaking causality by adding a newer timestamp.

                last_certificate = max(common_topic_last_timestamp, realm_topic_last_timestamp)
                if last_certificate >= timestamp:
                    return RequireGreaterTimestamp(strictly_greater_than=last_certificate)

                if org.is_sequestered:
                    assert org.sequester_services is not None
                    for service_id, service in org.sequester_services.items():
                        if service.is_revoked:
                            continue
                        if service.service_type == SequesterServiceType.WEBHOOK:
                            assert service.webhook_url is not None
                            match await self.webhooks.sequester_service_on_vlob_create_or_update(
                                webhook_url=service.webhook_url,
                                service_id=service_id,
                                organization_id=organization_id,
                                author=author,
                                realm_id=realm_id,
                                vlob_id=vlob_id,
                                key_index=key_index,
                                version=version,
                                timestamp=timestamp,
                                blob=blob,
                            ):
                                case None:
                                    pass
                                case error:
                                    return error

                if version != len(vlobs) + 1:
                    return VlobUpdateBadOutcome.BAD_VLOB_VERSION

                # All checks are good, now we do the actual insertion

                match realm.last_vlob_timestamp:
                    case None:
                        realm.last_vlob_timestamp = timestamp
                    case previous_last_vlob_timestamp:
                        realm.last_vlob_timestamp = max(previous_last_vlob_timestamp, timestamp)

                version = len(vlobs) + 1
                vlob_atom = MemoryVlobAtom(
                    realm_id=realm_id,
                    vlob_id=vlob_id,
                    key_index=key_index,
                    version=version,
                    blob=blob,
                    author=author,
                    created_on=timestamp,
                )
                vlobs.append(vlob_atom)
                realm_change_checkpoint = len(realm.vlob_updates) + 1
                realm.vlob_updates.append(
                    MemoryRealmVlobUpdate(
                        index=realm_change_checkpoint,
                        vlob_atom=vlob_atom,
                    )
                )

                await self._event_bus.send(
                    EventVlob(
                        organization_id=organization_id,
                        author=author,
                        realm_id=realm.realm_id,
                        timestamp=timestamp,
                        vlob_id=vlob_id,
                        version=version,
                        blob=blob if len(blob) < EVENT_VLOB_MAX_BLOB_SIZE else None,
                        last_common_certificate_timestamp=common_topic_last_timestamp,
                        last_realm_certificate_timestamp=realm_topic_last_timestamp,
                    )
                )

    @override
    async def read_versions(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: VlobID,
        items: list[tuple[VlobID, int]],
    ) -> VlobReadResult | VlobReadAsUserBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return VlobReadAsUserBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return VlobReadAsUserBadOutcome.ORGANIZATION_EXPIRED

        try:
            author_device = org.devices[author]
        except KeyError:
            return VlobReadAsUserBadOutcome.AUTHOR_NOT_FOUND
        author_user_id = author_device.cooked.user_id

        try:
            author_user = org.users[author_user_id]
        except KeyError:
            return VlobReadAsUserBadOutcome.AUTHOR_NOT_FOUND
        if author_user.is_revoked:
            return VlobReadAsUserBadOutcome.AUTHOR_REVOKED

        try:
            realm = org.realms[realm_id]
        except KeyError:
            return VlobReadAsUserBadOutcome.REALM_NOT_FOUND

        match realm.get_current_role_for(author_user_id):
            case None:
                return VlobReadAsUserBadOutcome.AUTHOR_NOT_ALLOWED

            case RealmRole.OWNER | RealmRole.MANAGER | RealmRole.CONTRIBUTOR | RealmRole.READER:
                pass

            case unknown:
                # TODO: Implement `Enum` on `RealmRole` so we can use `assert_never` here
                assert False, unknown

        output = []
        for vlob_id, version in items:
            try:
                vlob = realm.vlobs[vlob_id]
            except KeyError:
                # An unkown vlob ID was provided
                continue

            if version < 1:
                continue
            try:
                atom = vlob[version - 1]
            except IndexError:
                # An unknown version was provided
                continue

            output.append(
                (
                    vlob_id,
                    atom.key_index,
                    atom.author,
                    atom.version,
                    atom.created_on,
                    atom.blob,
                )
            )

        return VlobReadResult(
            items=output,
            needed_common_certificate_timestamp=org.per_topic_last_timestamp["common"],
            needed_realm_certificate_timestamp=org.per_topic_last_timestamp[("realm", realm_id)],
        )

    @override
    async def read_batch(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: VlobID,
        vlobs: list[VlobID],
        at: DateTime | None,
    ) -> VlobReadResult | VlobReadAsUserBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return VlobReadAsUserBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return VlobReadAsUserBadOutcome.ORGANIZATION_EXPIRED

        try:
            author_device = org.devices[author]
        except KeyError:
            return VlobReadAsUserBadOutcome.AUTHOR_NOT_FOUND
        author_user_id = author_device.cooked.user_id

        try:
            author_user = org.users[author_user_id]
        except KeyError:
            return VlobReadAsUserBadOutcome.AUTHOR_NOT_FOUND
        if author_user.is_revoked:
            return VlobReadAsUserBadOutcome.AUTHOR_REVOKED

        try:
            realm = org.realms[realm_id]
        except KeyError:
            return VlobReadAsUserBadOutcome.REALM_NOT_FOUND

        match realm.get_current_role_for(author_user_id):
            case None:
                return VlobReadAsUserBadOutcome.AUTHOR_NOT_ALLOWED

            case RealmRole.OWNER | RealmRole.MANAGER | RealmRole.CONTRIBUTOR | RealmRole.READER:
                pass

            case unknown:
                # TODO: Implement `Enum` on `RealmRole` so we can use `assert_never` here
                assert False, unknown

        output = []
        for vlob_id in vlobs:
            try:
                vlob = realm.vlobs[vlob_id]
            except KeyError:
                # An unkown vlob ID was provided
                continue

            if at is None:
                atom = vlob[-1]
            else:
                for atom in reversed(vlob):
                    if atom.created_on <= at:
                        break
                else:
                    # The vlob didn't exists at the considered time
                    continue

            output.append(
                (
                    vlob_id,
                    atom.key_index,
                    atom.author,
                    atom.version,
                    atom.created_on,
                    atom.blob,
                )
            )

        return VlobReadResult(
            items=output,
            needed_common_certificate_timestamp=org.per_topic_last_timestamp["common"],
            needed_realm_certificate_timestamp=org.per_topic_last_timestamp[("realm", realm_id)],
        )

    @override
    async def poll_changes(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: VlobID,
        checkpoint: int,
    ) -> tuple[int, list[tuple[VlobID, int]]] | VlobPollChangesAsUserBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return VlobPollChangesAsUserBadOutcome.ORGANIZATION_NOT_FOUND
        if org.is_expired:
            return VlobPollChangesAsUserBadOutcome.ORGANIZATION_EXPIRED

        try:
            author_device = org.devices[author]
        except KeyError:
            return VlobPollChangesAsUserBadOutcome.AUTHOR_NOT_FOUND
        author_user_id = author_device.cooked.user_id

        try:
            author_user = org.users[author_user_id]
        except KeyError:
            return VlobPollChangesAsUserBadOutcome.AUTHOR_NOT_FOUND
        if author_user.is_revoked:
            return VlobPollChangesAsUserBadOutcome.AUTHOR_REVOKED

        try:
            realm = org.realms[realm_id]
        except KeyError:
            return VlobPollChangesAsUserBadOutcome.REALM_NOT_FOUND

        match realm.get_current_role_for(author_user_id):
            case None:
                return VlobPollChangesAsUserBadOutcome.AUTHOR_NOT_ALLOWED

            case RealmRole.OWNER | RealmRole.MANAGER | RealmRole.CONTRIBUTOR | RealmRole.READER:
                pass

            case unknown:
                # TODO: Implement `Enum` on `RealmRole` so we can use `assert_never` here
                assert False, unknown

        items = {}
        for vlob_update in realm.vlob_updates[checkpoint:]:
            items[vlob_update.vlob_atom.vlob_id] = (
                vlob_update.vlob_atom.vlob_id,
                vlob_update.vlob_atom.version,
            )

        return len(realm.vlob_updates), list(items.values())

    @override
    async def test_dump_vlobs(
        self, organization_id: OrganizationID
    ) -> dict[VlobID, dict[VlobID, list[tuple[DeviceID, DateTime, bytes]]]]:
        org = self._data.organizations[organization_id]
        return {
            realm_id: {
                vlob_id: [(atom.author, atom.created_on, atom.blob) for atom in vlob_atoms]
                for vlob_id, vlob_atoms in realm.vlobs.items()
            }
            for realm_id, realm in org.realms.items()
        }
