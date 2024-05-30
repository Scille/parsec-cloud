# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import override

import anyio
from anyio.abc import CancelScope, TaskGroup

from parsec._parsec import BlockID, OrganizationID
from parsec.components.blockstore import (
    BaseBlockStoreComponent,
    BlockStoreCreateBadOutcome,
    BlockStoreReadBadOutcome,
)
from parsec.logging import get_logger

logger = get_logger()


class RAID1BlockStoreComponent(BaseBlockStoreComponent):
    def __init__(
        self,
        blockstores: list[BaseBlockStoreComponent],
        partial_create_ok: bool = False,
    ):
        self.blockstores = blockstores
        self._partial_create_ok = partial_create_ok
        self._logger = logger.bind(blockstore_type="RAID1", partial_create_ok=partial_create_ok)

    @override
    async def read(
        self, organization_id: OrganizationID, block_id: BlockID
    ) -> bytes | BlockStoreReadBadOutcome:
        value = None

        async def _single_blockstore_read(
            task_group: TaskGroup, blockstore: BaseBlockStoreComponent
        ) -> None:
            nonlocal value
            outcome = await blockstore.read(organization_id, block_id)
            if isinstance(outcome, bytes):
                value = outcome
                task_group.cancel_scope.cancel()

        async with anyio.create_task_group() as task_group:
            for blockstore in self.blockstores:
                task_group.start_soon(_single_blockstore_read, task_group, blockstore)

        if not value:
            self._logger.warning(
                "Block read error: All nodes have failed",
                organization_id=organization_id,
                block_id=block_id,
            )
            return BlockStoreReadBadOutcome.STORE_UNAVAILABLE

        return value

    @override
    async def create(
        self, organization_id: OrganizationID, block_id: BlockID, block: bytes
    ) -> None | BlockStoreCreateBadOutcome:
        at_least_one_success = False
        at_least_one_error = False

        async def _single_blockstore_create(
            cancel_scope: CancelScope, blockstore: BaseBlockStoreComponent
        ) -> None:
            nonlocal at_least_one_success
            nonlocal at_least_one_error
            outcome = await blockstore.create(organization_id, block_id, block)
            match outcome:
                case None:
                    at_least_one_success = True
                case BlockStoreCreateBadOutcome():
                    at_least_one_error = True
                    if not self._partial_create_ok:
                        # Early exit given the create cannot succeed
                        cancel_scope.cancel()

        async with anyio.create_task_group() as task_group:
            for blockstore in self.blockstores:
                task_group.start_soon(
                    _single_blockstore_create, task_group.cancel_scope, blockstore
                )

        if self._partial_create_ok:
            if not at_least_one_success:
                self._logger.warning(
                    "Block create error: All nodes have failed",
                    organization_id=organization_id.str,
                    block_id=block_id.hex,
                )
                return BlockStoreCreateBadOutcome.STORE_UNAVAILABLE
        else:
            if at_least_one_error:
                self._logger.warning(
                    "Block create error: A node have failed",
                    organization_id=organization_id.str,
                    block_id=block_id.hex,
                )
                return BlockStoreCreateBadOutcome.STORE_UNAVAILABLE
