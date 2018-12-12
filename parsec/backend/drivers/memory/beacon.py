from typing import List, Tuple
from collections import defaultdict
from uuid import UUID

from parsec.types import DeviceID
from parsec.backend.beacon import BaseBeaconComponent


class MemoryBeaconComponent(BaseBeaconComponent):
    def __init__(self, event_bus):
        self.event_bus = event_bus
        self.beacons = defaultdict(list)

    async def read(self, id: UUID, offset: int) -> List[Tuple[UUID, int]]:
        return self.beacons[id][offset:]

    async def update(
        self, id: UUID, src_id: UUID, src_version: int, author: DeviceID = None
    ) -> None:
        self.beacons[id].append((src_id, src_version))
        index = len(self.beacons[id])
        if author:
            self.event_bus.send(
                "beacon.updated",
                author=author,
                beacon_id=id,
                index=index,
                src_id=src_id,
                src_version=src_version,
            )
