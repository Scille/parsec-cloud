# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
from typing import Tuple, Dict
from collections import defaultdict
from uuid import UUID

from parsec.types import DeviceID, UserID, OrganizationID
from parsec.event_bus import EventBus
from parsec.backend.beacon import BaseBeaconComponent, BeaconNotFound, BeaconAccessError


@attr.s
class Beacon:
    checkpoint = attr.ib(default=0)
    rights = attr.ib(factory=dict)
    changes = attr.ib(factory=dict)


class MemoryBeaconComponent(BaseBeaconComponent):
    def __init__(self, event_bus: EventBus):
        self.event_bus = event_bus
        self._organizations = defaultdict(dict)

    def _lazy_init(self, organization_id, id, author):
        beacons = self._organizations[organization_id]
        if id in beacons:
            return
        beacons[id] = Beacon(rights={author: (True, True, True)})

    def _get(self, organization_id, id):
        beacons = self._organizations[organization_id]
        try:
            return beacons[id]

        except KeyError:
            raise BeaconNotFound(f"Unknown beacon `{id}`")

    def _can_read(self, organization_id, user_id, id):
        try:
            beacon = self._get(organization_id, id)
            return beacon.rights[user_id][1]
        except (BeaconNotFound, KeyError):
            return False

    def _can_write(self, organization_id, user_id, id):
        try:
            beacon = self._get(organization_id, id)
            return beacon.rights[user_id][2]
        except (BeaconNotFound, KeyError):
            return False

    def _vlob_updated(self, organization_id, author, id, src_id, src_version=1):
        beacon = self._get(organization_id, id)
        beacon.checkpoint += 1
        beacon.changes[src_id] = (author, beacon.checkpoint, src_version)
        self.event_bus.send(
            "beacon.updated",
            organization_id=organization_id,
            author=author,
            beacon_id=id,
            checkpoint=beacon.checkpoint,
            src_id=src_id,
            src_version=src_version,
        )

    async def get_rights(
        self, organization_id: OrganizationID, author: UserID, id: UUID
    ) -> Dict[DeviceID, Tuple[bool, bool, bool]]:
        beacon = self._get(organization_id, id)
        if not beacon.rights.get(author, (False, False, False))[1]:
            raise BeaconAccessError()
        return beacon.rights.copy()

    async def set_rights(
        self,
        organization_id: OrganizationID,
        author: UserID,
        id: UUID,
        user: UserID,
        admin_access: bool,
        read_access: bool,
        write_access: bool,
    ) -> None:
        beacon = self._get(organization_id, id)
        if not beacon.rights.get(author, (False, False, False))[0]:
            raise BeaconAccessError()
        if not read_access and not write_access:
            beacon.rights.pop(user, None)
        else:
            beacon.rights[user] = (admin_access, read_access, write_access)

    async def poll(
        self, organization_id: OrganizationID, author: UserID, id: UUID, checkpoint: int
    ) -> Tuple[int, Dict[UUID, int]]:
        beacon = self._get(organization_id, id)
        if not beacon.rights.get(author, (False, False, False))[1]:
            raise BeaconAccessError()
        changes_since_checkpoint = {
            src_id: src_version
            for src_id, (_, change_checkpoint, src_version) in beacon.changes.items()
            if change_checkpoint > checkpoint
        }
        return (beacon.checkpoint, changes_since_checkpoint)
