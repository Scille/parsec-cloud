# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from typing import List

from parsec.api.protocol import OrganizationID, BlockID
from parsec.backend.blockstore import BaseBlockStoreComponent


class RAID0BlockStoreComponent(BaseBlockStoreComponent):
    def __init__(self, blockstores: List[BaseBlockStoreComponent]):
        self.blockstores = blockstores

    def _get_blockstore(self, block_id: BlockID) -> BaseBlockStoreComponent:
        return self.blockstores[block_id.int % len(self.blockstores)]

    async def read(self, organization_id: OrganizationID, block_id: BlockID) -> bytes:
        blockstore = self._get_blockstore(block_id)
        return await blockstore.read(organization_id, block_id)

    async def create(
        self, organization_id: OrganizationID, block_id: BlockID, block: bytes
    ) -> None:
        blockstore = self._get_blockstore(block_id)
        await blockstore.create(organization_id, block_id, block)
