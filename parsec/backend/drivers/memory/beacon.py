from typing import List, Tuple
from collections import defaultdict
from uuid import UUID

from parsec.types import DeviceID, OrganizationID
from parsec.event_bus import EventBus
from parsec.backend.beacon import BaseBeaconComponent


class MemoryBeaconComponent(BaseBeaconComponent):
    def __init__(self, event_bus: EventBus):
        self.event_bus = event_bus
        self._organizations = defaultdict(lambda: defaultdict(list))

    async def read(
        self, organization_id: OrganizationID, id: UUID, offset: int
    ) -> List[Tuple[UUID, int]]:
        beacons = self._organizations[organization_id]
        return beacons[id][offset:]

    async def update(
        self,
        organization_id: OrganizationID,
        id: UUID,
        src_id: UUID,
        src_version: int,
        author: DeviceID = None,
    ) -> None:
        beacons = self._organizations[organization_id]
        beacons[id].append((src_id, src_version))
        index = len(beacons[id])
        if author:
            self.event_bus.send(
                "beacon.updated",
                organization_id=organization_id,
                author=author,
                beacon_id=id,
                index=index,
                src_id=src_id,
                src_version=src_version,
            )
