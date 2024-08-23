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
    SequesterInconsistency,
    SequesterServiceNotAvailable,
    VlobCreateBadOutcome,
    VlobPollChangesAsUserBadOutcome,
    VlobReadAsUserBadOutcome,
    VlobReadResult,
    VlobUpdateBadOutcome,
)
from parsec.events import EVENT_VLOB_MAX_BLOB_SIZE, EventVlob

# Tuple contains: blob, author, timestamp, certificate index at the time of creation
type VlobData = list[tuple[bytes, DeviceID, DateTime, int]]
type SequesteredVlobData = list[dict[SequesterServiceID, bytes]]


class MemoryVlobComponent(BaseVlobComponent):
    def __init__(
        self,
        data: MemoryDatamodel,
        event_bus: EventBus,
        *args,
        **kwargs,
    ) -> None:
        super().__init__(*args, **kwargs)
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
        sequester_blob: dict[SequesterServiceID, bytes] | None = None,
    ) -> (
        None
        | BadKeyIndex
        | VlobCreateBadOutcome
        | TimestampOutOfBallpark
        | RequireGreaterTimestamp
        | RejectedBySequesterService
        | SequesterServiceNotAvailable
        | SequesterInconsistency
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

            if vlob_id in org.vlobs:
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
                if sequester_blob is None or sequester_blob.keys() != org.sequester_services.keys():
                    return SequesterInconsistency(
                        last_common_certificate_timestamp=common_topic_last_timestamp
                    )

                blob_for_storage_sequester_services = {}
                for service_id, service in org.sequester_services.items():
                    match service.service_type:
                        case SequesterServiceType.STORAGE:
                            blob_for_storage_sequester_services[service_id] = sequester_blob[
                                service_id
                            ]
                        case SequesterServiceType.WEBHOOK:
                            assert service.webhook_url is not None
                            match await self._sequester_service_send_webhook(
                                webhook_url=service.webhook_url,
                                organization_id=organization_id,
                                service_id=service_id,
                                sequester_blob=sequester_blob[service_id],
                            ):
                                case None:
                                    pass
                                case error:
                                    return error

            else:
                if sequester_blob is not None:
                    return VlobCreateBadOutcome.ORGANIZATION_NOT_SEQUESTERED
                blob_for_storage_sequester_services = None

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
                blob_for_storage_sequester_services=blob_for_storage_sequester_services,
            )
            org.vlobs[vlob_id] = [vlob_atom]
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
        vlob_id: VlobID,
        key_index: int,
        version: int,
        timestamp: DateTime,
        blob: bytes,
        # Sequester is a special case, so gives it a default version to simplify tests
        sequester_blob: dict[SequesterServiceID, bytes] | None = None,
    ) -> (
        None
        | BadKeyIndex
        | VlobUpdateBadOutcome
        | TimestampOutOfBallpark
        | RequireGreaterTimestamp
        | RejectedBySequesterService
        | SequesterServiceNotAvailable
        | SequesterInconsistency
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
                vlobs = org.vlobs[vlob_id]
            except KeyError:
                return VlobUpdateBadOutcome.VLOB_NOT_FOUND
            realm_id = vlobs[0].realm_id

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
                    if (
                        sequester_blob is None
                        or sequester_blob.keys() != org.sequester_services.keys()
                    ):
                        return SequesterInconsistency(
                            last_common_certificate_timestamp=common_topic_last_timestamp
                        )

                    blob_for_storage_sequester_services = {}
                    for service_id, service in org.sequester_services.items():
                        match service.service_type:
                            case SequesterServiceType.STORAGE:
                                blob_for_storage_sequester_services[service_id] = sequester_blob[
                                    service_id
                                ]
                            case SequesterServiceType.WEBHOOK:
                                assert service.webhook_url is not None
                                match await self._sequester_service_send_webhook(
                                    webhook_url=service.webhook_url,
                                    organization_id=organization_id,
                                    service_id=service_id,
                                    sequester_blob=sequester_blob[service_id],
                                ):
                                    case None:
                                        pass
                                    case error:
                                        return error

                else:
                    if sequester_blob is not None:
                        return VlobUpdateBadOutcome.ORGANIZATION_NOT_SEQUESTERED
                    blob_for_storage_sequester_services = None

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
                    realm_id=realm.realm_id,
                    vlob_id=vlob_id,
                    key_index=key_index,
                    version=version,
                    blob=blob,
                    author=author,
                    created_on=timestamp,
                    blob_for_storage_sequester_services=blob_for_storage_sequester_services,
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
                vlob = org.vlobs[vlob_id]
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
                vlob = org.vlobs[vlob_id]
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
    ) -> dict[VlobID, list[tuple[DeviceID, DateTime, VlobID, bytes]]]:
        org = self._data.organizations[organization_id]
        return {
            vlob_id: [
                (atom.author, atom.created_on, atom.realm_id, atom.blob) for atom in vlob_atoms
            ]
            for vlob_id, vlob_atoms in org.vlobs.items()
        }
