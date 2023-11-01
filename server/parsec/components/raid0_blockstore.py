# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import override

from parsec._parsec import BlockID, OrganizationID
from parsec.components.blockstore import (
    BaseBlockStoreComponent,
    BlockStoreCreateBadOutcome,
    BlockStoreReadBadOutcome,
)


class RAID0BlockStoreComponent(BaseBlockStoreComponent):
    def __init__(self, blockstores: list[BaseBlockStoreComponent]):
        self.blockstores = blockstores

    def _get_blockstore(self, block_id: BlockID) -> BaseBlockStoreComponent:
        return self.blockstores[block_id.int % len(self.blockstores)]

    @override
    async def read(
        self, organization_id: OrganizationID, block_id: BlockID
    ) -> bytes | BlockStoreReadBadOutcome:
        blockstore = self._get_blockstore(block_id)
        return await blockstore.read(organization_id, block_id)

    @override
    async def create(
        self, organization_id: OrganizationID, block_id: BlockID, block: bytes
    ) -> None | BlockStoreCreateBadOutcome:
        blockstore = self._get_blockstore(block_id)
        await blockstore.create(organization_id, block_id, block)
