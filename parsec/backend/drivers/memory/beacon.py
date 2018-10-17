from collections import defaultdict

from parsec.backend.beacon import BaseBeaconComponent


class MemoryBeaconComponent(BaseBeaconComponent):
    def __init__(self, event_bus):
        self.event_bus = event_bus
        self.beacons = defaultdict(list)

    async def read(self, id, offset):
        return self.beacons[id][offset:]

    async def update(self, id, src_id, src_version, author="anonymous"):
        self.beacons[id].append({"src_id": src_id, "src_version": src_version})
        index = len(self.beacons[id])
        self.event_bus.send(
            "beacon.updated",
            author=author,
            beacon_id=id,
            index=index,
            src_id=src_id,
            src_version=src_version,
        )
