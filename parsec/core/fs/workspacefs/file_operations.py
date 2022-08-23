# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

# Imports

import bisect
from typing import Tuple, List, Set, Iterator, Union, Sequence, TYPE_CHECKING
from parsec._parsec import DateTime

from parsec.core.types import BlockID, LocalFileManifest, Chunk, ChunkID


Chunks = Tuple[Chunk, ...]
ChunkIDSet = Set[Union[ChunkID, BlockID]]
WriteOperationList = List[Tuple[Chunk, int]]


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


def prepare_read(manifest: LocalFileManifest, size: int, offset: int) -> Chunks:
    # Prepare
    chunks: List[Chunk] = []
    offset = min(offset, manifest.size)
    size = min(size, manifest.size - offset)

    # Loop over blocks
    for block, length, start in split_read(size, offset, manifest.blocksize):

        # Loop over chunks
        block_chunks = manifest.get_chunks(block)
        chunks += block_read(block_chunks, length, start)

    # Return read result
    return tuple(chunks)


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


def prepare_write(
    manifest: LocalFileManifest, size: int, offset: int, timestamp: DateTime
) -> Tuple[LocalFileManifest, WriteOperationList, ChunkIDSet]:
    # Prepare
    padding = 0
    removed_ids: ChunkIDSet = set()
    write_operations: WriteOperationList = []

    # Padding
    if offset > manifest.size:
        padding = offset - manifest.size
        size += padding
        offset = manifest.size

    # Copy buffers
    blocks: list[Tuple[Chunk, ...]] = list(manifest.blocks)

    # Loop over blocks
    for block, sub_size, start, content_offset in split_write(size, offset, manifest.blocksize):

        # Prepare new chunk
        new_chunk = Chunk.new(start, start + sub_size)
        write_operations.append((new_chunk, content_offset - padding))

        # Lazy block write
        chunks = manifest.get_chunks(block)
        new_chunks, more_removed_ids = block_write(chunks, sub_size, start, new_chunk)

        # Update data structures
        removed_ids |= more_removed_ids
        if len(blocks) == block:
            blocks.append(new_chunks)
        else:
            blocks[block] = new_chunks

    # Evolve manifest
    new_size = max(manifest.size, offset + size)
    new_manifest = manifest.evolve_and_mark_updated(
        size=new_size, blocks=tuple(blocks), timestamp=timestamp
    )

    # Return write result
    return new_manifest, write_operations, removed_ids


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


def prepare_resize(
    manifest: LocalFileManifest, size: int, timestamp: DateTime
) -> Tuple[LocalFileManifest, WriteOperationList, ChunkIDSet]:
    if size >= manifest.size:
        return prepare_write(manifest, 0, size, timestamp)
    manifest, removed_ids = prepare_truncate(manifest, size, timestamp)
    return manifest, [], removed_ids


# Reshape


def prepare_reshape(
    manifest: LocalFileManifest,
) -> Iterator[Tuple[int, Chunks, Chunk, bool, ChunkIDSet]]:

    # Loop over blocks
    for block, chunks in enumerate(manifest.blocks):

        # Already a block
        if len(chunks) == 1 and chunks[0].is_block():
            continue

        # Already a pseudo-block
        if len(chunks) == 1 and chunks[0].is_pseudo_block():
            yield (block, chunks, chunks[0], False, set())
            continue

        # Prepare new block
        start, stop = chunks[0].start, chunks[-1].stop
        new_chunk = Chunk.new(start, stop)

        # Cleanup
        removed_ids = chunk_id_set(chunks)

        # Yield operations
        yield (block, chunks, new_chunk, True, removed_ids)


_py_prepare_read = prepare_read
_py_prepare_write = prepare_write
_py_prepare_resize = prepare_resize
_py_prepare_reshape = prepare_reshape
if not TYPE_CHECKING:
    try:
        from libparsec.types import prepare_read as _rs_prepare_read
    except:
        pass
    else:
        prepare_read = _rs_prepare_read

    try:
        from libparsec.types import prepare_write as _rs_prepare_write
    except:
        pass
    else:
        prepare_write = _rs_prepare_write

    try:
        from libparsec.types import prepare_resize as _rs_prepare_resize
    except:
        pass
    else:
        prepare_resize = _rs_prepare_resize

    try:
        from libparsec.types import prepare_reshape as _rs_prepare_reshape
    except:
        pass
    else:
        prepare_reshape = _rs_prepare_reshape
