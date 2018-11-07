import trio

from parsec.backend.exceptions import AlreadyExistsError, NotFoundError
from parsec.backend.blockstore import BaseBlockStoreComponent


class RAID1BlockStoreComponent(BaseBlockStoreComponent):
    def __init__(self, blockstores):
        self.blockstores = blockstores

    async def get(self, id):
        async def _single_blockstore_get(nursery, blockstore, id):
            nonlocal value
            try:
                value = await blockstore.get(id)
                nursery.cancel_scope.cancel()
            except NotFoundError:
                pass

        value = None
        async with trio.open_nursery() as nursery:
            for blockstore in self.blockstores:
                nursery.start_soon(_single_blockstore_get, nursery, blockstore, id)

        if not value:
            raise NotFoundError("Unknown block id.")

        return value

    async def post(self, id, block):
        async def _single_blockstore_post(nursery, blockstore, id, block):
            try:
                await blockstore.post(id, block)
            except AlreadyExistsError:
                # It's possible a previous tentative to upload this block has
                # failed due another blockstore not available. In such case
                # a retrial will raise AlreadyExistsError on all the blockstores
                # that sucessfully uploaded the block during last attempt.
                # Only solution to solve this is to ignore AlreadyExistsError.
                pass

        async with trio.open_nursery() as nursery:
            for blockstore in self.blockstores:
                nursery.start_soon(_single_blockstore_post, nursery, blockstore, id, block)
