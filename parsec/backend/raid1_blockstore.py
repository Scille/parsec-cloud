# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

from typing import List

from parsec.utils import open_service_nursery
from parsec.api.protocol import OrganizationID, BlockID
from parsec.backend.blockstore import BaseBlockStoreComponent
from parsec.backend.block import BlockAlreadyExistsError, BlockNotFoundError, BlockTimeoutError


class RAID1BlockStoreComponent(BaseBlockStoreComponent):
    def __init__(self, blockstores: List[BaseBlockStoreComponent]):
        self.blockstores = blockstores

    async def read(self, organization_id: OrganizationID, id: BlockID) -> bytes:
        value = None

        async def _single_blockstore_read(nursery, blockstore: BaseBlockStoreComponent) -> None:
            nonlocal value
            try:
                value = await blockstore.read(organization_id, id)
                nursery.cancel_scope.cancel()
            except (BlockNotFoundError, BlockTimeoutError):
                pass

        async with open_service_nursery() as nursery:
            for blockstore in self.blockstores:
                nursery.start_soon(_single_blockstore_read, nursery, blockstore)

        if not value:
            raise BlockNotFoundError()

        return value

    async def create(self, organization_id: OrganizationID, id: BlockID, block: bytes) -> None:
        async def _single_blockstore_create(blockstore: BaseBlockStoreComponent) -> None:
            try:
                await blockstore.create(organization_id, id, block)
            except BlockAlreadyExistsError:
                # It's possible a previous tentative to upload this block has
                # failed due to another blockstore not available. In such case
                # a retrial will raise AlreadyExistsError on all the blockstores
                # that sucessfully uploaded the block during last attempt.
                # Only solution to solve this is to ignore AlreadyExistsError.
                pass

        async with open_service_nursery() as nursery:
            for blockstore in self.blockstores:
                nursery.start_soon(_single_blockstore_create, blockstore)
