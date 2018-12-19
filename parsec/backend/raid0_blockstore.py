from uuid import UUID

from parsec.types import DeviceID
from parsec.backend.blockstore import BaseBlockstoreComponent


class RAID0BlockstoreComponent(BaseBlockstoreComponent):
    def __init__(self, blockstores):
        self.blockstores = blockstores

    def _get_blockstore(self, id: UUID):
        return self.blockstores[id.int % len(self.blockstores)]

    async def read(self, id: UUID) -> bytes:
        blockstore = self._get_blockstore(id)
        return await blockstore.read(id)

    async def create(self, id: UUID, block: bytes, author: DeviceID) -> None:
        blockstore = self._get_blockstore(id)
        await blockstore.create(id, block, author)
