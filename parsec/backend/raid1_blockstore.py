# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from typing import List
from structlog import get_logger

from parsec.utils import open_service_nursery
from parsec.api.protocol import OrganizationID, BlockID
from parsec.backend.block import BlockStoreError
from parsec.backend.blockstore import BaseBlockStoreComponent


logger = get_logger()


class RAID1BlockStoreComponent(BaseBlockStoreComponent):
    def __init__(self, blockstores: List[BaseBlockStoreComponent], partial_create_ok: bool = False):
        self.blockstores = blockstores
        self._partial_create_ok = partial_create_ok
        self._logger = logger.bind(blockstore_type="RAID1", partial_create_ok=partial_create_ok)

    async def read(self, organization_id: OrganizationID, block_id: BlockID) -> bytes:
        value = None

        async def _single_blockstore_read(nursery, blockstore: BaseBlockStoreComponent) -> None:
            nonlocal value
            try:
                value = await blockstore.read(organization_id, block_id)
                nursery.cancel_scope.cancel()
            except BlockStoreError:
                pass

        async with open_service_nursery() as nursery:
            for blockstore in self.blockstores:
                nursery.start_soon(_single_blockstore_read, nursery, blockstore)

        if not value:
            self._logger.warning(
                "Block read error: All nodes have failed",
                organization_id=organization_id,
                block_id=block_id,
            )
            raise BlockStoreError("All RAID1 nodes have failed")

        return value

    async def create(
        self, organization_id: OrganizationID, block_id: BlockID, block: bytes
    ) -> None:
        at_least_one_success = False
        at_least_one_error = False

        async def _single_blockstore_create(
            cancel_scope, blockstore: BaseBlockStoreComponent
        ) -> None:
            nonlocal at_least_one_success
            nonlocal at_least_one_error
            try:
                await blockstore.create(organization_id, block_id, block)
                at_least_one_success = True

            except BlockStoreError:
                at_least_one_error = True
                if not self._partial_create_ok:
                    # Early exit given the create cannot succeed
                    cancel_scope.cancel()

        async with open_service_nursery() as nursery:
            for blockstore in self.blockstores:
                nursery.start_soon(_single_blockstore_create, nursery.cancel_scope, blockstore)

        if self._partial_create_ok:
            if not at_least_one_success:
                self._logger.warning(
                    "Block create error: All nodes have failed",
                    organization_id=organization_id.str,
                    block_id=block_id.str,
                )
                raise BlockStoreError("All RAID1 nodes have failed")
        else:
            if at_least_one_error:
                self._logger.warning(
                    "Block create error: A node have failed",
                    organization_id=organization_id.str,
                    block_id=block_id.str,
                )
                raise BlockStoreError("A RAID1 node have failed")
