# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
from uuid import UUID
from typing import List, Tuple
from collections import defaultdict

from parsec.types import DeviceID, UserID, OrganizationID
from parsec.event_bus import EventBus
from parsec.backend.beacon import BaseBeaconComponent
from parsec.backend.vlob import (
    BaseVlobComponent,
    VlobAccessError,
    VlobVersionError,
    VlobNotFoundError,
    VlobAlreadyExistsError,
)


@attr.s
class Vlob:
    beacon = attr.ib()
    data = attr.ib(factory=list)

    @property
    def current_version(self):
        return len(self.data)


class MemoryVlobComponent(BaseVlobComponent):
    def __init__(self, event_bus: EventBus, beacon_component: BaseBeaconComponent):
        self.event_bus = event_bus
        self.beacon_component = beacon_component
        self._organizations = defaultdict(dict)

    def _get(self, organization_id, id):
        vlobs = self._organizations[organization_id]

        try:
            return vlobs[id]

        except KeyError:
            raise VlobNotFoundError()

    async def group_check(
        self, organization_id: OrganizationID, user_id: UserID, to_check: List[dict]
    ) -> List[dict]:
        changed = []
        for item in to_check:
            id = item["id"]
            version = item["version"]
            if version == 0:
                changed.append({"id": id, "version": version})
            else:
                try:
                    vlob = self._get(organization_id, id)
                except VlobNotFoundError:
                    continue

                if not self.beacon_component._can_read(organization_id, user_id, vlob.beacon):
                    continue

                if vlob.current_version != version:
                    changed.append({"id": id, "version": vlob.current_version})

        return changed

    async def create(
        self, organization_id: OrganizationID, author: DeviceID, beacon: UUID, id: UUID, blob: bytes
    ) -> None:
        self.beacon_component._lazy_init(organization_id, beacon, author.user_id)
        if not self.beacon_component._can_write(organization_id, author.user_id, beacon):
            raise VlobAccessError()

        vlobs = self._organizations[organization_id]

        if id in vlobs:
            raise VlobAlreadyExistsError()

        vlobs[id] = Vlob(beacon, [(blob, author)])

        self.beacon_component._vlob_updated(organization_id, author, beacon, id)

    async def read(
        self, organization_id: OrganizationID, user_id: UserID, id: UUID, version: int = None
    ) -> Tuple[int, bytes]:
        vlob = self._get(organization_id, id)

        if not self.beacon_component._can_read(organization_id, user_id, vlob.beacon):
            raise VlobAccessError()

        if version is None:
            version = vlob.current_version
        try:
            return (version, vlob.data[version - 1][0])

        except IndexError:
            raise VlobVersionError()

    async def update(
        self, organization_id: OrganizationID, author: DeviceID, id: UUID, version: int, blob: bytes
    ) -> None:
        vlob = self._get(organization_id, id)

        if not self.beacon_component._can_write(organization_id, author.user_id, vlob.beacon):
            raise VlobAccessError()

        if version - 1 == vlob.current_version:
            vlob.data.append((blob, author))
        else:
            raise VlobVersionError()

        self.beacon_component._vlob_updated(organization_id, author, vlob.beacon, id, version)
