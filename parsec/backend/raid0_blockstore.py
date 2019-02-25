# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from uuid import UUID

from parsec.types import DeviceID, OrganizationID
from parsec.backend.blockstore import BaseBlockstoreComponent


class RAID0BlockstoreComponent(BaseBlockstoreComponent):
    def __init__(self, blockstores):
        self.blockstores = blockstores

    def _get_blockstore(self, id: UUID):
        return self.blockstores[id.int % len(self.blockstores)]

    async def read(self, organization_id: OrganizationID, id: UUID) -> bytes:
        blockstore = self._get_blockstore(id)
        return await blockstore.read(organization_id, id)

    async def create(
        self, organization_id: OrganizationID, id: UUID, block: bytes, author: DeviceID
    ) -> None:
        blockstore = self._get_blockstore(id)
        await blockstore.create(organization_id, id, block, author)
