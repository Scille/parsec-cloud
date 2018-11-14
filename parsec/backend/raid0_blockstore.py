from uuid import UUID

from parsec.backend.blockstore import BaseBlockStoreComponent


class RAID0BlockStoreComponent(BaseBlockStoreComponent):
    def __init__(self, blockstores):
        self.blockstores = blockstores

    def _get_blockstore(self, id: UUID):
        return self.blockstores[id.int % len(self.blockstores)]

    async def get(self, id: UUID) -> bytes:
        blockstore = self._get_blockstore(id)
        return await blockstore.get(id)

    async def post(self, id, block):
        blockstore = self._get_blockstore(id)
        return await blockstore.post(id, block)
