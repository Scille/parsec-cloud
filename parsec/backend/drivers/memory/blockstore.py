# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from uuid import UUID
from collections import defaultdict

from parsec.types import DeviceID, OrganizationID
from parsec.backend.blockstore import (
    BaseBlockstoreComponent,
    BlockstoreAlreadyExistsError,
    BlockstoreNotFoundError,
)


class MemoryBlockstoreComponent(BaseBlockstoreComponent):
    def __init__(self):
        self._organizations = defaultdict(dict)

    async def read(self, organization_id: OrganizationID, id: UUID) -> bytes:
        blocks = self._organizations[organization_id]

        try:
            return blocks[id][0]

        except KeyError:
            raise BlockstoreNotFoundError()

    async def create(
        self, organization_id: OrganizationID, id: UUID, block: bytes, author: DeviceID
    ) -> None:
        blocks = self._organizations[organization_id]

        if id in blocks:
            # Should never happen
            raise BlockstoreAlreadyExistsError()

        blocks[id] = (block, author)
