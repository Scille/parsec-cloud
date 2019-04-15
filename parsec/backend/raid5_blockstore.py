# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
from uuid import UUID
from structlog import get_logger

from parsec.types import OrganizationID
from parsec.backend.blockstore import BaseBlockStoreComponent
from parsec.backend.block import BlockAlreadyExistsError, BlockNotFoundError, BlockTimeoutError

logger = get_logger()


class RAID5BlockStoreComponent(BaseBlockStoreComponent):
    def __init__(self, blockstores):
        self.blockstores = blockstores

    async def read(self, organization_id: OrganizationID, id: UUID) -> bytes:
        async def _partial_blockstore_read(nursery, blockstore, i):
            nonlocal exception_already_triggered
            nonlocal subblocks
            try:
                subblocks[i] = await blockstore.read(organization_id, id)
            except (BlockNotFoundError, BlockTimeoutError):
                if not exception_already_triggered:  # Add the last parity subblock
                    exception_already_triggered = True
                    logger.warning(
                        f"Failed to reach blockstore {i} of RAID 5 for block id {id}"
                    )
                    subblocks += [None]
                    nursery.start_soon(
                        _partial_blockstore_read,
                        nursery,
                        self.blockstores[-1],
                        len(self.blockstores) - 1,
                    )
                else:
                    nursery.cancel_scope.cancel()

        exception_already_triggered = False
        # Don't fetch the parity subblock if not needed
        subblocks = [None] * (len(self.blockstores) - 1)
        async with trio.open_nursery() as nursery:
            for i, blockstore in enumerate(self.blockstores[0:-1]):
                nursery.start_soon(_partial_blockstore_read, nursery, blockstore, i)

        if nursery.cancel_scope.cancelled_caught:
            raise BlockNotFoundError()

        if None in subblocks:
            failed_blockstore_pos = subblocks.index(None)
            recover = None
            for i, subblock in enumerate(subblocks):
                if subblock is None:
                    continue
                elif recover is None:
                    recover = bytearray(subblock)
                    if i == len(subblocks) - 2:
                        recover += bytearray([0] * subblocks[-1][0])
                elif i == len(subblocks) - 1:
                    for j in range(len(subblock) - 1):
                        recover[j] ^= subblock[j + 1]
                else:
                    for j in range(len(subblock)):
                        recover[j] ^= subblock[j]
            if failed_blockstore_pos == len(subblocks) - 2 and subblocks[-1][0] != 0:
                subblocks[failed_blockstore_pos] = bytes(recover[0 : -subblocks[-1][0]])
            else:
                subblocks[failed_blockstore_pos] = recover
            del subblocks[-1]

        block = bytearray()
        for subblock in subblocks:
            block += subblock
        return bytes(block)

    async def create(self, organization_id: OrganizationID, id: UUID, block: bytes) -> None:
        async def _subblockstore_create(blockstore, subblock):
            try:
                await blockstore.create(organization_id, id, subblock)
            except BlockAlreadyExistsError:
                # It's possible a previous tentative to upload this block has
                # failed due to another blockstore not available. In such case
                # a retrial will raise AlreadyExistsError on all the blockstores
                # that sucessfully uploaded the block during last attempt.
                # Only solution to solve this is to ignore AlreadyExistsError.
                pass

        nb_stores = (
            len(self.blockstores) - 1
        )  # number of stores without the one reserved for checksums
        subblock_len = len(block) // nb_stores
        if len(block) % nb_stores != 0:
            subblock_len += 1
        subblocks = []
        for i in range(nb_stores):
            subblocks += [block[subblock_len * i : subblock_len * (i + 1)]]
        # how many bytes misses the last subblock compared to the other ones
        padding = subblock_len * nb_stores - len(block)

        checksums = bytearray([padding])
        for i in range(subblock_len):
            xored = 0
            for j in range(nb_stores):
                try:
                    xored ^= subblocks[j][i]
                except IndexError:
                    pass
            checksums += bytearray([xored])
        subblocks += [bytes(checksums)]

        async with trio.open_nursery() as nursery:
            for i, subblock in enumerate(subblocks):
                nursery.start_soon(_subblockstore_create, self.blockstores[i], subblock)
