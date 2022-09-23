# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import struct
from structlog import get_logger
from sys import byteorder
from typing import List, Optional, Union

from parsec.utils import open_service_nursery
from parsec.api.protocol import OrganizationID, BlockID
from parsec.backend.block import BlockStoreError
from parsec.backend.blockstore import BaseBlockStoreComponent


logger = get_logger()


def _xor_buffers(*buffers: bytes) -> bytes:
    buff_len = len(buffers[0])
    xored = int.from_bytes(buffers[0], byteorder)
    for buff in buffers[1:]:
        assert len(buff) == buff_len
        xored ^= int.from_bytes(buff, byteorder)
    return xored.to_bytes(buff_len, byteorder)


def split_block_in_chunks(block: bytes, nb_chunks: int) -> List[bytes]:
    payload_size = len(block) + 4  # encode block len as a uint32
    chunk_len = payload_size // nb_chunks
    if nb_chunks * chunk_len < payload_size:
        chunk_len += 1
    padding_len = chunk_len * nb_chunks - payload_size

    payload = struct.pack("!I", len(block)) + block + b"\x00" * padding_len

    return [payload[chunk_len * i : chunk_len * (i + 1)] for i in range(nb_chunks)]


def generate_checksum_chunk(chunks: List[bytes]) -> bytes:
    return _xor_buffers(*chunks)


def rebuild_block_from_chunks(
    chunks: List[Optional[bytes]], checksum_chunk: Optional[bytes]
) -> bytes:
    valid_chunks = [chunk for chunk in chunks if chunk is not None]
    assert len(chunks) - len(valid_chunks) <= 1  # Cannot correct more than 1 chunk
    try:
        missing_chunk_id = next(index for index, chunk in enumerate(chunks) if chunk is None)
        assert checksum_chunk is not None
        chunks[missing_chunk_id] = _xor_buffers(*valid_chunks, checksum_chunk)
    except StopIteration:
        pass
    # By now, all chunks are valid
    chunks: List[bytes]
    payload = b"".join(chunks)
    (block_len,) = struct.unpack("!I", payload[:4])
    return payload[4 : 4 + block_len]


class RAID5BlockStoreComponent(BaseBlockStoreComponent):
    def __init__(
        self,
        blockstores: List[BaseBlockStoreComponent],
        partial_create_ok: bool = False,
    ):
        self.blockstores = blockstores
        self._partial_create_ok = partial_create_ok
        self._logger = logger.bind(blockstore_type="RAID5", partial_create_ok=partial_create_ok)

    async def read(self, organization_id: OrganizationID, block_id: BlockID) -> bytes:
        error_count = 0
        fetch_results: List[Union[Exception, Optional[bytes]]] = [None] * len(self.blockstores)

        async def _partial_blockstore_read(nursery, blockstore_index: int) -> None:
            nonlocal error_count
            nonlocal fetch_results
            try:
                fetch_results[blockstore_index] = await self.blockstores[blockstore_index].read(
                    organization_id, block_id
                )
            except BlockStoreError as exc:
                fetch_results[blockstore_index] = exc
                error_count += 1
                if error_count > 1:
                    nursery.cancel_scope.cancel()
                else:
                    # Try to fetch the checksum to rebuild the current missing chunk...
                    nursery.start_soon(_partial_blockstore_read, nursery, len(self.blockstores) - 1)

        async with open_service_nursery() as nursery:
            # Don't fetch the checksum by default
            for blockstore_index in range(len(self.blockstores) - 1):
                nursery.start_soon(_partial_blockstore_read, nursery, blockstore_index)

        if error_count == 0:
            # Sanity check: no errors and we didn't fetch the checksum
            assert len([res for res in fetch_results if res is None]) == 1
            assert fetch_results[-1] is None
            assert not len([res for res in fetch_results if isinstance(res, Exception)])

            return rebuild_block_from_chunks(fetch_results[:-1], None)  # type: ignore

        elif error_count == 1:
            checksum = fetch_results[-1]
            # Sanity check: one error and we have fetched the checksum
            assert len([res for res in fetch_results if res is None]) == 0
            assert isinstance(checksum, (bytes, bytearray))
            assert len([res for res in fetch_results if isinstance(res, Exception)]) == 1

            return rebuild_block_from_chunks(
                [
                    res if isinstance(res, (bytes, bytearray)) else None
                    for res in fetch_results[:-1]
                ],
                checksum,
            )

        else:
            # No need to log the detail of the nodes errors, they should have
            # already been logged before raising their exceptions
            self._logger.warning(
                "Block read error: More than 1 nodes have failed",
                organization_id=organization_id.str,
                block_id=block_id.str,
            )
            raise BlockStoreError("More than 1 RAID5 nodes have failed")

    async def create(
        self, organization_id: OrganizationID, block_id: BlockID, block: bytes
    ) -> None:
        nb_chunks = len(self.blockstores) - 1
        chunks = split_block_in_chunks(block, nb_chunks)
        assert len(chunks) == nb_chunks
        checksum_chunk = generate_checksum_chunk(chunks)

        # Actually do the upload
        error_count = 0

        async def _subblockstore_create(
            nursery, blockstore_index: int, chunk_or_checksum: bytes
        ) -> None:
            nonlocal error_count
            try:
                await self.blockstores[blockstore_index].create(
                    organization_id, block_id, chunk_or_checksum
                )
            except BlockStoreError:
                error_count += 1
                if error_count > 1 or not self._partial_create_ok:
                    # Early exit
                    nursery.cancel_scope.cancel()

        async with open_service_nursery() as nursery:
            for i, chunk_or_checksum in enumerate([*chunks, checksum_chunk]):
                nursery.start_soon(_subblockstore_create, nursery, i, chunk_or_checksum)

        if self._partial_create_ok:
            # Only a single blockstore is allowed to fail
            # Note it's possible to have error_count > 1 and still have some blokstore nodes
            # that have written the block. This is no big deal given we consider the create
            # operation to be idempotent (and two create with the same orgID/ID couple are
            # expected to have the same block data).
            if error_count > 1:
                # No need to log the detail of the nodes errors, they should have
                # already been logged before raising their exceptions
                self._logger.warning(
                    "Block create error: More than 1 nodes have failed",
                    organization_id=organization_id.str,
                    block_id=block_id.str,
                )
                raise BlockStoreError("More than 1 RAID5 nodes have failed")

        else:
            if error_count:
                self._logger.warning(
                    "Block create error: A node have failed",
                    organization_id=organization_id.str,
                    block_id=block_id.str,
                )
                raise BlockStoreError("A RAID5 node have failed")
