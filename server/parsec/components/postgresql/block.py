# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import override

from asyncpg.exceptions import UniqueViolationError

from parsec._parsec import (
    BlockID,
    DateTime,
    DeviceID,
    OrganizationID,
    VlobID,
)
from parsec.components.block import (
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
from parsec.components.postgresql import AsyncpgConnection, AsyncpgPool
from parsec.components.postgresql.block_create import block_create
from parsec.components.postgresql.block_read import block_read
from parsec.components.postgresql.block_test_dump_blocks import block_test_dump_blocks
from parsec.components.postgresql.utils import (
    Q,
    transaction,
)
from parsec.components.realm import BadKeyIndex


class PGBlockComponent(BaseBlockComponent):
    def __init__(self, pool: AsyncpgPool, blockstore: BaseBlockStoreComponent):
        self.pool = pool
        self.blockstore = blockstore

    @override
    async def read(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        block_id: BlockID,
    ) -> BlockReadResult | BlockReadBadOutcome:
        return await block_read(self.blockstore, self.pool, organization_id, author, block_id)

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
        return await block_create(
            self.blockstore,
            self.pool,
            now,
            organization_id,
            author,
            realm_id,
            block_id,
            key_index,
            block,
        )

    @override
    @transaction
    async def test_dump_blocks(
        self, conn: AsyncpgConnection, organization_id: OrganizationID
    ) -> dict[BlockID, tuple[DateTime, DeviceID, VlobID, int, int]]:
        return await block_test_dump_blocks(conn, organization_id)


_q_get_block_data = Q(
    """
SELECT data
FROM block_data
WHERE
    organization_id = $organization_id
    AND block_id = $block_id
"""
)

_q_insert_block_data = Q(
    """
INSERT INTO block_data (organization_id, block_id, data)
VALUES ($organization_id, $block_id, $data)
"""
)


class PGBlockStoreComponent(BaseBlockStoreComponent):
    def __init__(self, pool: AsyncpgPool):
        self.pool = pool

    async def read(
        self, organization_id: OrganizationID, block_id: BlockID
    ) -> bytes | BlockStoreReadBadOutcome:
        async with self.pool.acquire() as conn:
            ret = await conn.fetchrow(
                *_q_get_block_data(organization_id=organization_id.str, block_id=block_id)
            )
            if not ret:
                return BlockStoreReadBadOutcome.BLOCK_NOT_FOUND

            return ret[0]

    async def create(
        self, organization_id: OrganizationID, block_id: BlockID, block: bytes
    ) -> None | BlockStoreCreateBadOutcome:
        async with self.pool.acquire() as conn:
            try:
                ret = await conn.execute(
                    *_q_insert_block_data(
                        organization_id=organization_id.str, block_id=block_id, data=block
                    )
                )
                if ret != "INSERT 0 1":
                    return BlockStoreCreateBadOutcome.STORE_UNAVAILABLE

            except UniqueViolationError:
                # Keep calm and stay idempotent
                pass
