from uuid import UUID
from typing import List, Tuple

from parsec.types import DeviceID
from parsec.api.protocole import beacon_read_serializer
from parsec.backend.utils import catch_protocole_errors


class BaseBeaconComponent:
    @catch_protocole_errors
    async def api_beacon_read(self, client_ctx, msg):
        msg = beacon_read_serializer.req_load(msg)

        # TODO: raise error if too many events since offset ?
        items = await self.read(msg["id"], msg["offset"])

        return beacon_read_serializer.rep_dump(
            {
                "status": "ok",
                "items": [
                    {"src_id": src_id, "src_version": src_version} for src_id, src_version in items
                ],
            }
        )

    async def read(self, id: UUID, offset: int) -> List[Tuple[UUID, int]]:
        raise NotImplementedError()

    async def update(
        self, id: UUID, src_id: UUID, src_version: int, author: DeviceID = None
    ) -> None:
        raise NotImplementedError()
