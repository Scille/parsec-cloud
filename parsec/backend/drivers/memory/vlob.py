# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from uuid import UUID
from typing import List, Tuple
from collections import defaultdict

from parsec.types import DeviceID, OrganizationID
from parsec.event_bus import EventBus
from parsec.backend.beacon import BaseBeaconComponent
from parsec.backend.vlob import (
    BaseVlobComponent,
    VlobTrustSeedError,
    VlobVersionError,
    VlobNotFoundError,
    VlobAlreadyExistsError,
)


class MemoryVlobComponent(BaseVlobComponent):
    def __init__(self, event_bus: EventBus, beacon_component: BaseBeaconComponent):
        self.event_bus = event_bus
        self.beacon_component = beacon_component
        self._organizations = defaultdict(dict)

    async def group_check(
        self, organization_id: OrganizationID, to_check: List[dict]
    ) -> List[dict]:
        changed = []
        for item in to_check:
            id = item["id"]
            version = item["version"]
            if version == 0:
                changed.append({"id": id, "version": version})
            else:
                try:
                    current_version, _ = await self.read(organization_id, id)
                except (VlobNotFoundError, VlobTrustSeedError):
                    continue
                # TODO: check access rights here
                if current_version != version:
                    changed.append({"id": id, "version": current_version})
        return changed

    async def create(
        self,
        organization_id: OrganizationID,
        id: UUID,
        blob: bytes,
        author: DeviceID,
        notify_beacon: UUID = None,
    ) -> None:
        vlobs = self._organizations[organization_id]

        if id in vlobs:
            raise VlobAlreadyExistsError()
        vlobs[id] = [(blob, author)]

        if notify_beacon:
            await self.beacon_component.update(organization_id, notify_beacon, id, 1, author)

    async def read(
        self, organization_id: OrganizationID, id: UUID, version: int = None
    ) -> Tuple[int, bytes]:
        vlobs = self._organizations[organization_id]

        try:
            vlob = vlobs[id]

        except KeyError:
            raise VlobNotFoundError()

        # TODO: check access rights here

        if version is None:
            version = len(vlob)
        try:
            return (version, vlob[version - 1][0])

        except IndexError:
            raise VlobVersionError()

    async def update(
        self,
        organization_id: OrganizationID,
        id: UUID,
        version: int,
        blob: bytes,
        author: DeviceID,
        notify_beacon: UUID = None,
    ) -> None:
        vlobs = self._organizations[organization_id]

        try:
            vlob = vlobs[id]

        except KeyError:
            raise VlobNotFoundError()

        # TODO: check access rights here

        if version - 1 == len(vlob):
            vlob.append((blob, author))
        else:
            raise VlobVersionError()

        if notify_beacon:
            await self.beacon_component.update(organization_id, notify_beacon, id, version, author)
