# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from uuid import UUID
from collections import defaultdict

from parsec.types import DeviceID, OrganizationID
from parsec.backend.blockstore import BaseBlockStoreComponent
from parsec.backend.block import (
    BaseBlockComponent,
    BlockAlreadyExistsError,
    BlockAccessError,
    BlockNotFoundError,
)


class MemoryBlockComponent(BaseBlockComponent):
    def __init__(self, blockstore_component, vlob_component):
        self._organizations = defaultdict(dict)
        self._blockstore_component = blockstore_component
        self._vlob_component = vlob_component

    async def read(self, organization_id: OrganizationID, author: DeviceID, id: UUID) -> bytes:
        try:
            vlob_group = self._organizations[organization_id][id]

        except KeyError:
            raise BlockNotFoundError()

        if not self._vlob_component._can_read(organization_id, author.user_id, vlob_group):
            raise BlockAccessError()

        return await self._blockstore_component.read(organization_id, id)

    async def create(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        id: UUID,
        vlob_group: UUID,
        block: bytes,
    ) -> None:
        if not self._vlob_component._can_write(organization_id, author.user_id, vlob_group):
            raise BlockAccessError()

        await self._blockstore_component.create(organization_id, id, block)
        self._organizations[organization_id][id] = vlob_group


class MemoryBlockStoreComponent(BaseBlockStoreComponent):
    def __init__(self):
        self._organizations = defaultdict(dict)

    async def read(self, organization_id: OrganizationID, id: UUID) -> bytes:
        blocks = self._organizations[organization_id]

        try:
            return blocks[id]

        except KeyError:
            raise BlockNotFoundError()

    async def create(self, organization_id: OrganizationID, id: UUID, block: bytes) -> None:
        blocks = self._organizations[organization_id]

        if id in blocks:
            # Should not happen if client play with uuid randomness
            raise BlockAlreadyExistsError()

        blocks[id] = block
