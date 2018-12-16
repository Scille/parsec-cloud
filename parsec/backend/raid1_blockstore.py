import trio
from uuid import UUID

from parsec.types import DeviceID
from parsec.backend.blockstore import (
    BaseBlockstoreComponent,
    BlockstoreAlreadyExistsError,
    BlockstoreNotFoundError,
)


class RAID1BlockstoreComponent(BaseBlockstoreComponent):
    def __init__(self, blockstores):
        self.blockstores = blockstores

    async def read(self, id: UUID) -> bytes:
        async def _single_blockstore_read(nursery, blockstore, id):
            nonlocal value
            try:
                value = await blockstore.read(id)
                nursery.cancel_scope.cancel()
            except BlockstoreNotFoundError:
                pass

        value = None
        async with trio.open_nursery() as nursery:
            for blockstore in self.blockstores:
                nursery.start_soon(_single_blockstore_read, nursery, blockstore, id)

        if not value:
            raise BlockstoreNotFoundError()

        return value

    async def create(self, id: UUID, block: bytes, author: DeviceID) -> None:
        async def _single_blockstore_create(nursery, blockstore, id, block):
            try:
                await blockstore.create(id, block, author)
            except BlockstoreAlreadyExistsError:
                # It's possible a previous tentative to upload this block has
                # failed due to another blockstore not available. In such case
                # a retrial will raise AlreadyExistsError on all the blockstores
                # that sucessfully uploaded the block during last attempt.
                # Only solution to solve this is to ignore AlreadyExistsError.
                pass

        async with trio.open_nursery() as nursery:
            for blockstore in self.blockstores:
                nursery.start_soon(_single_blockstore_create, nursery, blockstore, id, block)
