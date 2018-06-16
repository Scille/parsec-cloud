from collections import defaultdict

from parsec.backend.beacon import BaseBeaconComponent


class MemoryBeaconComponent(BaseBeaconComponent):
    def __init__(self, signal_ns):
        self._signal_beacon_updated = signal_ns.signal("beacon.updated")
        self.beacons = defaultdict(list)

    async def read(self, id, offset):
        return self.beacons[id][offset:]

    async def update(self, id, src_id, src_version, author="anonymous"):
        self.beacons[id].append({"src_id": src_id, "src_version": src_version})
        index = len(self.beacons[id])
        self._signal_beacon_updated.send(
            None, author=author, beacon_id=id, index=index, src_id=src_id, src_version=src_version
        )
