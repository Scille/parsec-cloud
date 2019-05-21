# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
import pendulum
from uuid import UUID
from typing import List, Tuple, Dict, Optional
from collections import defaultdict

from parsec.types import DeviceID, UserID, OrganizationID
from parsec.event_bus import EventBus
from parsec.backend.drivers.memory.realm import MemoryRealmComponent, RealmAlreadyExistsError
from parsec.backend.vlob import (
    BaseVlobComponent,
    VlobAccessError,
    VlobVersionError,
    VlobNotFoundError,
    VlobAlreadyExistsError,
    VlobEncryptionRevisionError,
    VlobInMaintenanceError,
)


@attr.s
class Vlob:
    realm_id = attr.ib()
    data = attr.ib(factory=list)

    @property
    def current_version(self):
        return len(self.data)


@attr.s
class Changes:
    checkpoint = attr.ib(default=0)
    encryption_revision = attr.ib(default=1)
    changes = attr.ib(factory=dict)


class MemoryVlobComponent(BaseVlobComponent):
    def __init__(self, event_bus: EventBus, realm_component: MemoryRealmComponent):
        self.event_bus = event_bus
        self._realm_component = realm_component
        self._vlobs = {}
        self._per_realm_changes = defaultdict(Changes)

    def _maintenance_reencryption_start_hook(self, organization_id, realm_id, encryption_revision):
        pass

    def _maintenance_reencryption_is_finished_hook(
        self, organization_id, realm_id, encryption_revision
    ):
        pass

    def _get_vlob(self, organization_id, vlob_id):
        try:
            return self._vlobs[(organization_id, vlob_id)]

        except KeyError:
            raise VlobNotFoundError(f"Vlob `{vlob_id}` doesn't exist")

    def _create_realm_if_needed(self, organization_id, realm_id, author):
        try:
            self._realm_component._create_realm(organization_id, realm_id, author)
        except RealmAlreadyExistsError:
            pass

    def _update_changes(self, organization_id, author, realm_id, src_id, src_version=1):
        changes = self._per_realm_changes[(organization_id, realm_id)]
        changes.checkpoint += 1
        changes.changes[src_id] = (author, changes.checkpoint, src_version)
        self.event_bus.send(
            "realm.vlobs_updated",
            organization_id=organization_id,
            author=author,
            realm_id=realm_id,
            checkpoint=changes.checkpoint,
            src_id=src_id,
            src_version=src_version,
        )

    async def create(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        vlob_id: UUID,
        realm_id: UUID,
        timestamp: pendulum.Pendulum,
        blob: bytes,
        encryption_revision: Optional[int] = None,
    ) -> None:
        self._create_realm_if_needed(organization_id, realm_id, author)
        self._realm_component._check_write_access_and_maintenance(
            organization_id,
            realm_id,
            author.user_id,
            not_found_exc=VlobNotFoundError,
            access_error_exc=VlobAccessError,
            in_maintenance_exc=VlobInMaintenanceError,
        )

        key = (organization_id, vlob_id)
        if key in self._vlobs:
            raise VlobAlreadyExistsError()

        if encryption_revision is not None:
            changes = self._per_realm_changes[key]
            if changes.encryption_revision != encryption_revision:
                raise VlobEncryptionRevisionError()

        self._vlobs[key] = Vlob(realm_id, [(blob, author, timestamp)])

        self._update_changes(organization_id, author, realm_id, vlob_id)

    async def read(
        self,
        organization_id: OrganizationID,
        author: UserID,
        vlob_id: UUID,
        version: Optional[int] = None,
        encryption_revision: Optional[int] = None,
    ) -> Tuple[int, bytes, DeviceID, pendulum.Pendulum]:
        vlob = self._get_vlob(organization_id, vlob_id)

        self._realm_component._check_read_access_and_maintenance(
            organization_id,
            vlob.realm_id,
            author.user_id,
            not_found_exc=VlobNotFoundError,
            access_error_exc=VlobAccessError,
            in_maintenance_exc=VlobInMaintenanceError,
        )

        if encryption_revision is not None:
            changes = self._per_realm_changes[(organization_id, vlob_id)]
            if changes.encryption_revision != encryption_revision:
                raise VlobEncryptionRevisionError()

        if version is None:
            version = vlob.current_version
        try:
            return (version, *vlob.data[version - 1])

        except IndexError:
            raise VlobVersionError()

    async def update(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        vlob_id: UUID,
        version: int,
        timestamp: pendulum.Pendulum,
        blob: bytes,
        encryption_revision: Optional[int] = None,
    ) -> None:
        vlob = self._get_vlob(organization_id, vlob_id)

        self._realm_component._check_write_access_and_maintenance(
            organization_id,
            vlob.realm_id,
            author.user_id,
            not_found_exc=VlobNotFoundError,
            access_error_exc=VlobAccessError,
            in_maintenance_exc=VlobInMaintenanceError,
        )

        if encryption_revision is not None:
            changes = self._per_realm_changes[(organization_id, vlob_id)]
            if changes.encryption_revision != encryption_revision:
                raise VlobEncryptionRevisionError()

        if version - 1 == vlob.current_version:
            vlob.data.append((blob, author, timestamp))
        else:
            raise VlobVersionError()

        self._update_changes(organization_id, author, vlob.realm_id, vlob_id, version)

    async def group_check(
        self, organization_id: OrganizationID, author: DeviceID, to_check: List[dict]
    ) -> List[dict]:
        changed = []
        for item in to_check:
            vlob_id = item["vlob_id"]
            version = item["version"]
            if version == 0:
                changed.append({"vlob_id": vlob_id, "version": version})
            else:
                try:
                    vlob = self._get_vlob(organization_id, vlob_id)
                except VlobNotFoundError:
                    continue

                if not self._realm_component._check_read_access_and_maintenance(
                    organization_id, vlob.realm_id, author.user_id
                ):
                    continue

                if vlob.current_version != version:
                    changed.append({"vlob_id": vlob_id, "version": vlob.current_version})

        return changed

    async def poll_changes(
        self, organization_id: OrganizationID, author: DeviceID, realm_id: UUID, checkpoint: int
    ) -> Tuple[int, Dict[UUID, int]]:
        key = (organization_id, realm_id)
        if key not in self._realm_component._realms:
            raise VlobNotFoundError(f"Realm `{realm_id}` doesn't exist")

        self._realm_component._check_read_access_and_maintenance(
            organization_id,
            realm_id,
            author.user_id,
            not_found_exc=VlobNotFoundError,
            access_error_exc=VlobAccessError,
            in_maintenance_exc=VlobInMaintenanceError,
        )

        changes = self._per_realm_changes[key]
        changes_since_checkpoint = {
            src_id: src_version
            for src_id, (_, change_checkpoint, src_version) in changes.changes.items()
            if change_checkpoint > checkpoint
        }
        return (changes.checkpoint, changes_since_checkpoint)
