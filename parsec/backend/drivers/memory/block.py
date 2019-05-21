# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from uuid import UUID
import attr

from parsec.types import DeviceID, OrganizationID
from parsec.backend.realm import BaseRealmComponent
from parsec.backend.blockstore import BaseBlockStoreComponent
from parsec.backend.block import (
    BaseBlockComponent,
    BlockAlreadyExistsError,
    BlockAccessError,
    BlockNotFoundError,
    BlockInMaintenanceError,
)


@attr.s(auto_attribs=True)
class BlockMeta:
    realm_id: UUID


class MemoryBlockComponent(BaseBlockComponent):
    def __init__(
        self, blockstore_component: BaseBlockStoreComponent, realm_component: BaseRealmComponent
    ):
        self._blockmetas = {}
        self._blockstore_component = blockstore_component
        self._realm_component = realm_component

    async def read(
        self, organization_id: OrganizationID, author: DeviceID, block_id: UUID
    ) -> bytes:
        try:
            blockmeta = self._blockmetas[(organization_id, block_id)]

        except KeyError:
            raise BlockNotFoundError()

        self._realm_component._check_read_access_and_maintenance(
            organization_id,
            blockmeta.realm_id,
            author.user_id,
            not_found_exc=BlockNotFoundError,
            access_error_exc=BlockAccessError,
            in_maintenance_exc=BlockInMaintenanceError,
        )

        return await self._blockstore_component.read(organization_id, block_id)

    async def create(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        block_id: UUID,
        realm_id: UUID,
        block: bytes,
    ) -> None:
        self._realm_component._check_write_access_and_maintenance(
            organization_id,
            realm_id,
            author.user_id,
            not_found_exc=BlockNotFoundError,
            access_error_exc=BlockAccessError,
            in_maintenance_exc=BlockInMaintenanceError,
        )

        await self._blockstore_component.create(organization_id, block_id, block)
        self._blockmetas[(organization_id, block_id)] = BlockMeta(realm_id)


class MemoryBlockStoreComponent(BaseBlockStoreComponent):
    def __init__(self):
        self._blocks = {}

    async def read(self, organization_id: OrganizationID, block_id: UUID) -> bytes:
        try:
            return self._blocks[(organization_id, block_id)]

        except KeyError:
            raise BlockNotFoundError()

    async def create(self, organization_id: OrganizationID, block_id: UUID, block: bytes) -> None:
        key = (organization_id, block_id)
        if key in self._blocks:
            # Should not happen if client play with uuid randomness
            raise BlockAlreadyExistsError()

        self._blocks[key] = block
