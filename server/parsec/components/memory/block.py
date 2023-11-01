# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import assert_never, override

from parsec._parsec import BlockID, DateTime, DeviceID, OrganizationID, RealmRole, UserID, VlobID
from parsec.components.block import BaseBlockComponent, BlockCreateBadOutcome, BlockReadBadOutcome
from parsec.components.blockstore import (
    BaseBlockStoreComponent,
    BlockStoreCreateBadOutcome,
    BlockStoreReadBadOutcome,
)
from parsec.components.memory.datamodel import MemoryBlock, MemoryDatamodel


class MemoryBlockComponent(BaseBlockComponent):
    def __init__(self, data: MemoryDatamodel, blockstore: BaseBlockStoreComponent) -> None:
        self._data = data
        self._blockstore_component = blockstore

    @override
    async def read_as_user(
        self, organization_id: OrganizationID, author: UserID, block_id: BlockID
    ) -> bytes | BlockReadBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return BlockReadBadOutcome.ORGANIZATION_NOT_FOUND

        if author not in org.users:
            return BlockReadBadOutcome.AUTHOR_NOT_FOUND

        try:
            block = org.blocks[block_id]
        except KeyError:
            return BlockReadBadOutcome.BLOCK_NOT_FOUND

        realm = org.realms.get(block.realm_id)
        assert realm is not None  # Sanity check, this consistency is enforced by the database

        current_role = realm.get_current_role_for(author)
        if current_role is None:
            return BlockReadBadOutcome.AUTHOR_NOT_ALLOWED

        outcome = await self._blockstore_component.read(organization_id, block_id)
        match outcome:
            case bytes() | bytearray() | memoryview() as block:
                return block
            case (
                BlockStoreReadBadOutcome.BLOCK_NOT_FOUND
                | BlockStoreReadBadOutcome.STORE_UNAVAILABLE
            ):
                return BlockReadBadOutcome.STORE_UNAVAILABLE
            case unknown:
                assert_never(unknown)

    @override
    async def create(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        block_id: BlockID,
        realm_id: VlobID,
        block: bytes,
    ) -> None | BlockCreateBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return BlockCreateBadOutcome.ORGANIZATION_NOT_FOUND

        if author not in org.devices:
            return BlockCreateBadOutcome.AUTHOR_NOT_FOUND

        try:
            realm = org.realms[realm_id]
        except KeyError:
            return BlockCreateBadOutcome.REALM_NOT_FOUND

        match realm.get_current_role_for(author.user_id):
            case RealmRole.OWNER | RealmRole.MANAGER | RealmRole.CONTRIBUTOR:
                pass
            case None | RealmRole.READER:
                return BlockCreateBadOutcome.AUTHOR_NOT_ALLOWED
            case unknown:
                assert False, unknown  # TODO: Cannot user assert_never with `RealmRole`

        if block_id in org.blocks:
            return BlockCreateBadOutcome.BLOCK_ALREADY_EXISTS

        outcome = await self._blockstore_component.create(organization_id, block_id, block)
        if outcome is not None:
            return BlockCreateBadOutcome.STORE_UNAVAILABLE

        org.blocks[block_id] = MemoryBlock(
            realm_id=realm_id,
            block_id=block_id,
            author=author,
            block_size=len(block),
            created_on=now,
        )

    @override
    async def test_dump_blocks(
        self, organization_id: OrganizationID
    ) -> dict[BlockID, tuple[DateTime, DeviceID, VlobID, int]]:
        org = self._data.organizations[organization_id]

        items = {}
        for block in org.blocks.values():
            items[block.block_id] = (
                block.created_on,
                block.author,
                block.realm_id,
                block.block_size,
            )

        return items


class MemoryBlockStoreComponent(BaseBlockStoreComponent):
    def __init__(self, data: MemoryDatamodel) -> None:
        self._data = data

    @override
    async def read(
        self, organization_id: OrganizationID, block_id: BlockID
    ) -> bytes | BlockStoreReadBadOutcome:
        try:
            return self._data.block_store[(organization_id, block_id)]
        except KeyError:
            return BlockStoreReadBadOutcome.BLOCK_NOT_FOUND

    @override
    async def create(
        self, organization_id: OrganizationID, block_id: BlockID, block: bytes
    ) -> None | BlockStoreCreateBadOutcome:
        key = (organization_id, block_id)
        self._data.block_store[key] = block
