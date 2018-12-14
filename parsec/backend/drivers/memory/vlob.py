from uuid import UUID
from typing import List, Tuple

from parsec.types import DeviceID
from parsec.event_bus import EventBus
from parsec.backend.beacon import BaseBeaconComponent
from parsec.backend.vlob import (
    BaseVlobComponent,
    VlobTrustSeedError,
    VlobVersionError,
    VlobNotFoundError,
    VlobAlreadyExistsError,
)


class MemoryVlob:
    def __init__(self, id, rts, wts, blob):
        self.id = id
        self.rts = rts
        self.wts = wts
        self.blob_versions = [blob]


class MemoryVlobComponent(BaseVlobComponent):
    def __init__(self, event_bus: EventBus, beacon_component: BaseBeaconComponent):
        self.event_bus = event_bus
        self.beacon_component = beacon_component
        self.vlobs = {}

    async def group_check(self, to_check: List[dict]) -> List[dict]:
        changed = []
        for item in to_check:
            id = item["id"]
            rts = item["rts"]
            version = item["version"]
            if version == 0:
                changed.append({"id": id, "version": version})
            else:
                try:
                    current_version, _ = await self.read(id, rts)
                except (VlobNotFoundError, VlobTrustSeedError):
                    continue
                if current_version != version:
                    changed.append({"id": id, "version": current_version})
        return changed

    async def create(
        self,
        id: UUID,
        rts: str,
        wts: str,
        blob: bytes,
        notify_beacon: UUID = None,
        author: DeviceID = None,
    ) -> None:
        vlob = MemoryVlob(id, rts, wts, blob)
        if vlob.id in self.vlobs:
            raise VlobAlreadyExistsError()
        self.vlobs[vlob.id] = vlob

        if notify_beacon and author:
            await self.beacon_component.update(notify_beacon, id, 1, author)

    async def read(self, id: UUID, rts: str, version: int = None) -> Tuple[int, bytes]:
        try:
            vlob = self.vlobs[id]
            if vlob.rts != rts:
                raise VlobTrustSeedError()

        except KeyError:
            raise VlobNotFoundError()

        version = version or len(vlob.blob_versions)
        try:
            return version, vlob.blob_versions[version - 1]

        except IndexError:
            raise VlobVersionError()

    async def update(
        self,
        id: UUID,
        wts: str,
        version: int,
        blob: bytes,
        notify_beacon: UUID = None,
        author: DeviceID = None,
    ) -> None:
        try:
            vlob = self.vlobs[id]
            if vlob.wts != wts:
                raise VlobTrustSeedError()

        except KeyError:
            raise VlobNotFoundError()

        if version - 1 == len(vlob.blob_versions):
            vlob.blob_versions.append(blob)
        else:
            raise VlobVersionError()

        if notify_beacon and author:
            await self.beacon_component.update(notify_beacon, id, version, author)
