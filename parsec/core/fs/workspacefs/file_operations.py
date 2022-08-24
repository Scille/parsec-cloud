# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

# Imports

import bisect
from typing import Tuple, List, Set, Iterator, Sequence
from parsec._parsec import DateTime, prepare_read, prepare_write, prepare_resize, prepare_reshape

from parsec.core.types import LocalFileManifest, Chunk, ChunkID


Chunks = Tuple[Chunk, ...]
ChunkIDSet = Set[ChunkID]
WriteOperationList = List[Tuple[Chunk, int]]

__all__ = [
    "prepare_read",
    "prepare_write",
    "prepare_resize",
    "prepare_reshape",
]

# Helpers


def locate(offset: int, blocksize: int) -> Tuple[int, int]:
    return divmod(offset, blocksize)


def locate_range(start: int, stop: int, blocksize: int) -> Iterator[Tuple[int, int, int]]:
    start_block, _ = locate(start, blocksize)
    stop_block, _ = locate(stop - 1, blocksize)
    for block in range(start_block, stop_block + 1):
        block_start = block * blocksize
        sub_start = max(start, block_start)
        sub_stop = min(stop, block_start + blocksize)
        yield block, sub_start, sub_stop


def index_of_chunk_before_start(chunks: Chunks, start: int) -> int:
    return bisect.bisect_right(chunks, start) - 1


def index_of_chunk_after_stop(chunks: Chunks, stop: int) -> int:
    return bisect.bisect_left(chunks, stop)


def chunk_id_set(chunks: Sequence[Chunk]) -> ChunkIDSet:
    return {chunk.id for chunk in chunks}


# Read functions


def split_read(length: int, offset: int, blocksize: int) -> Iterator[Tuple[int, int, int]]:
    for block, start, stop in locate_range(offset, offset + length, blocksize):
        yield block, stop - start, start


def block_read(chunks: Chunks, size: int, start: int) -> Iterator[Chunk]:
    # Bisect
    stop = start + size
    start_index = index_of_chunk_before_start(chunks, start)
    stop_index = index_of_chunk_after_stop(chunks, stop)

    # Loop over chunks
    for chunk in chunks[start_index:stop_index]:
        yield chunk.evolve(start=max(chunk.start, start), stop=min(chunk.stop, stop))


# Write functions


def split_write(size: int, offset: int, blocksize: int) -> Iterator[Tuple[int, int, int, int]]:
    for block, start, stop in locate_range(offset, offset + size, blocksize):
        yield block, stop - start, start, start - offset


def block_write(
    chunks: Chunks, size: int, start: int, new_chunk: Chunk
) -> Tuple[Chunks, ChunkIDSet]:
    # Init
    stop = start + size

    # Edge case
    if not chunks:
        return (new_chunk,), set()

    # Bisect
    start_index = index_of_chunk_before_start(chunks, start)
    stop_index = index_of_chunk_after_stop(chunks, stop)

    # Removed ids
    removed_ids = chunk_id_set(chunks[start_index:stop_index])

    # Prepare result
    result = list(chunks[:start_index])

    # Test start chunk
    start_chunk = chunks[start_index]
    if start_chunk.start < start:
        result.append(start_chunk.evolve(stop=start))
        removed_ids.discard(start_chunk.id)

    # Add new buffer
    result.append(new_chunk)

    # Test stop_chunk
    stop_chunk = chunks[stop_index - 1]
    if stop_chunk.stop > stop:
        result.append(stop_chunk.evolve(start=stop))
        removed_ids.discard(stop_chunk.id)

    # Fill up
    result += chunks[stop_index:]

    # IDs might appear multiple times
    if removed_ids:
        removed_ids -= chunk_id_set(result)

    # Return immutable result
    return tuple(result), removed_ids


# Resize


def prepare_truncate(
    manifest: LocalFileManifest, size: int, timestamp: DateTime
) -> Tuple[LocalFileManifest, ChunkIDSet]:
    # Prepare
    block, remainder = locate(size, manifest.blocksize)
    removed_ids = chunk_id_set(manifest.blocks[block])

    # Truncate buffers
    blocks = manifest.blocks[:block]
    if remainder:
        chunks = manifest.blocks[block]
        stop_index = index_of_chunk_after_stop(chunks, size)
        last_chunk = chunks[stop_index - 1]
        chunks = chunks[: stop_index - 1]
        chunks += (last_chunk.evolve(stop=size),)
        blocks += (chunks,)
        removed_ids -= chunk_id_set(chunks)

    # Clean up
    for chunks in manifest.blocks[block + 1 :]:
        removed_ids |= chunk_id_set(chunks)

    # Craft new manifest
    new_manifest = manifest.evolve_and_mark_updated(size=size, blocks=blocks, timestamp=timestamp)

    # Return truncate result
    return new_manifest, removed_ids
