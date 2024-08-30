# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from collections.abc import Buffer
from typing import override

from parsec._parsec import BlockID, DateTime, DeviceID, OrganizationID, RealmRole, VlobID
from parsec.components.block import (
    BadKeyIndex,
    BaseBlockComponent,
    BlockCreateBadOutcome,
    BlockReadBadOutcome,
    BlockReadResult,
)
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
    async def read(
        self, organization_id: OrganizationID, author: DeviceID, block_id: BlockID
    ) -> BlockReadResult | BlockReadBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return BlockReadBadOutcome.ORGANIZATION_NOT_FOUND

        try:
            author_device = org.devices[author]
        except KeyError:
            return BlockReadBadOutcome.AUTHOR_NOT_FOUND
        author_user_id = author_device.cooked.user_id

        try:
            block_info = org.blocks[block_id]
        except KeyError:
            return BlockReadBadOutcome.BLOCK_NOT_FOUND

        realm = org.realms.get(block_info.realm_id)
        assert realm is not None  # Sanity check, this consistency is enforced by the database

        current_role = realm.get_current_role_for(author_user_id)
        if current_role is None:
            return BlockReadBadOutcome.AUTHOR_NOT_ALLOWED

        outcome = await self._blockstore_component.read(organization_id, block_id)
        match outcome:
            case Buffer() as block:
                return BlockReadResult(
                    block=block,
                    key_index=block_info.key_index,
                    needed_realm_certificate_timestamp=org.per_topic_last_timestamp[
                        ("realm", realm.realm_id)
                    ],
                )
            case (
                BlockStoreReadBadOutcome.BLOCK_NOT_FOUND
                | BlockStoreReadBadOutcome.STORE_UNAVAILABLE
            ):
                return BlockReadBadOutcome.STORE_UNAVAILABLE

    @override
    async def create(
        self,
        now: DateTime,
        organization_id: OrganizationID,
        author: DeviceID,
        realm_id: VlobID,
        block_id: BlockID,
        key_index: int,
        block: bytes,
    ) -> None | BadKeyIndex | BlockCreateBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            return BlockCreateBadOutcome.ORGANIZATION_NOT_FOUND

        async with org.topics_lock(read=["common", ("realm", realm_id)]) as (
            _,
            realm_topic_last_timestamp,
        ):
            try:
                author_device = org.devices[author]
            except KeyError:
                return BlockCreateBadOutcome.AUTHOR_NOT_FOUND
            author_user_id = author_device.cooked.user_id

            try:
                realm = org.realms[realm_id]
            except KeyError:
                return BlockCreateBadOutcome.REALM_NOT_FOUND

            match realm.get_current_role_for(author_user_id):
                case RealmRole.OWNER | RealmRole.MANAGER | RealmRole.CONTRIBUTOR:
                    pass
                case None | RealmRole.READER:
                    return BlockCreateBadOutcome.AUTHOR_NOT_ALLOWED
                case unknown:
                    assert False, unknown  # TODO: Cannot user assert_never with `RealmRole`

            # We only accept the last key
            if len(realm.key_rotations) != key_index:
                return BadKeyIndex(last_realm_certificate_timestamp=realm_topic_last_timestamp)

            if block_id in org.blocks:
                return BlockCreateBadOutcome.BLOCK_ALREADY_EXISTS

            outcome = await self._blockstore_component.create(organization_id, block_id, block)
            if outcome is not None:
                return BlockCreateBadOutcome.STORE_UNAVAILABLE

            org.blocks[block_id] = MemoryBlock(
                realm_id=realm_id,
                block_id=block_id,
                key_index=key_index,
                author=author,
                block_size=len(block),
                created_on=now,
            )

    @override
    async def test_dump_blocks(
        self, organization_id: OrganizationID
    ) -> dict[BlockID, tuple[DateTime, DeviceID, VlobID, int, int]]:
        org = self._data.organizations[organization_id]

        items = {}
        for block in org.blocks.values():
            items[block.block_id] = (
                block.created_on,
                block.author,
                block.realm_id,
                block.key_index,
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
            return self._data.organizations[organization_id].block_store[block_id]
        except KeyError:
            return BlockStoreReadBadOutcome.BLOCK_NOT_FOUND

    @override
    async def create(
        self, organization_id: OrganizationID, block_id: BlockID, block: bytes
    ) -> None | BlockStoreCreateBadOutcome:
        try:
            org = self._data.organizations[organization_id]
        except KeyError:
            # In theory the blockstore should happily store blocks of unknown
            # organizations, but it is not something that's expected to happen
            # so we cheat a bit here and just pretend we have stored the data.
            return

        org.block_store[block_id] = block
