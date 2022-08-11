# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from typing import List
from uuid import UUID

from parsec.api.protocol import OrganizationID, BlockID
from parsec.backend.blockstore import BaseBlockStoreComponent


class RAID0BlockStoreComponent(BaseBlockStoreComponent):
    def __init__(self, blockstores: List[BaseBlockStoreComponent]):
        self.blockstores = blockstores

    def _get_blockstore(self, id: UUID) -> BaseBlockStoreComponent:
        return self.blockstores[id.int % len(self.blockstores)]

    async def read(self, organization_id: OrganizationID, id: BlockID) -> bytes:
        blockstore = self._get_blockstore(id.uuid)
        return await blockstore.read(organization_id, id)

    async def create(self, organization_id: OrganizationID, id: BlockID, block: bytes) -> None:
        blockstore = self._get_blockstore(id.uuid)
        await blockstore.create(organization_id, id, block)
