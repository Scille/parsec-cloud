# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

# Imports

import bisect
from typing import Tuple, List, Set, Iterator

from parsec.core.types import EntryID, LocalFileManifest, Chunk, Chunks


# Helpers


def locate(offset: int, blocksize: int) -> Tuple[int, int]:
    return divmod(offset, blocksize)


def locate_range(start: int, stop: int, blocksize: int) -> Iterator[Tuple[int, int, int]]:
    start_block, _ = locate(start, blocksize)
    stop_block, _ = locate(stop - 1, blocksize)
    for block in range(start_block, stop_block + 1):
        blockstart = block * blocksize
        substart = max(start, blockstart)
        substop = min(stop, blockstart + blocksize)
        yield block, substart, substop


def index_of_chunk_before_start(chunks: Chunks, start: int) -> int:
    return bisect.bisect_right(chunks, start) - 1


def index_of_chunk_after_stop(chunks: Chunks, stop: int) -> int:
    return bisect.bisect_left(chunks, stop)


def dirty_id_set(chunks):
    return {chunk.id for chunk in chunks if chunk.dirty}


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
) -> Tuple[Chunks, Set[EntryID]]:
    # Init
    stop = start + size

    # Edge case
    if not chunks:
        return (new_chunk,), set()

    # Bisect
    start_index = index_of_chunk_before_start(chunks, start)
    stop_index = index_of_chunk_after_stop(chunks, stop)

    # Prepare result
    result = list(chunks[:start_index])

    # Test start chunk
    start_chunk = chunks[start_index]
    if start_chunk.start < start:
        result.append(start_chunk.evolve(stop=start))

    # Add new buffer
    result.append(new_chunk)

    # Test stop_chunk
    stop_chunk = chunks[stop_index - 1]
    if stop_chunk.stop > stop:
        result.append(stop_chunk.evolve(start=stop))

    # Fill up
    result += chunks[stop_index:]

    # Clean up
    removed_ids = dirty_id_set(chunks[start_index:stop_index])
    removed_ids -= dirty_id_set(result)

    # Return immutable result
    return tuple(result), removed_ids


def prepare_write(
    manifest: LocalFileManifest, size: int, offset: int
) -> Tuple[LocalFileManifest, List[Tuple[Chunk, int]], Set[EntryID]]:
    # Prepare
    padding = 0
    removed_ids: Set[EntryID] = set()
    write_operations: List[Tuple[Chunk, int]] = []

    # Padding
    if offset > manifest.size:
        padding = offset - manifest.size
        size += padding
        offset = manifest.size

    # Copy buffers
    blocks = list(manifest.blocks)

    # Loop over blocks
    for block, subsize, start, content_offset in split_write(size, offset, manifest.blocksize):

        # Prepare new chunk
        new_chunk = Chunk.new_chunk(start, start + subsize)
        write_operations.append((new_chunk, content_offset - padding))

        # Lazy block write
        chunks = manifest.get_chunks(block)
        new_chunks, more_removed_ids = block_write(chunks, subsize, start, new_chunk)

        # Update data structures
        removed_ids |= more_removed_ids
        if len(blocks) == block:
            blocks.append(new_chunks)
        else:
            blocks[block] = new_chunks

    # Evolve manifest
    new_size = max(manifest.size, offset + size)
    new_manifest = manifest.evolve_and_mark_updated(size=new_size, blocks=tuple(blocks))

    # Return write result
    return new_manifest, write_operations, removed_ids


# Resize


def prepare_truncate(
    manifest: LocalFileManifest, size: int
) -> Tuple[LocalFileManifest, Set[EntryID]]:
    # Prepare
    block, remainder = locate(size, manifest.blocksize)
    removed_ids = dirty_id_set(manifest.blocks[block])

    # Truncate buffers
    blocks = manifest.blocks[:block]
    if remainder:
        chunks = manifest.blocks[block]
        stop_index = index_of_chunk_after_stop(chunks, size)
        last_chunk = chunks[stop_index - 1]
        chunks = chunks[: stop_index - 1]
        chunks += (last_chunk.evolve(stop=size),)
        blocks += (chunks,)
        removed_ids -= dirty_id_set(chunks)

    # Clean up
    for chunks in manifest.blocks[block + 1 :]:
        removed_ids |= dirty_id_set(chunks)

    # Craft new manifest
    new_manifest = manifest.evolve_and_mark_updated(size=size, blocks=blocks)

    # Return truncate result
    return new_manifest, removed_ids


def prepare_resize(
    manifest: LocalFileManifest, size: int
) -> Tuple[LocalFileManifest, List[Tuple[Chunk, int]], Set[EntryID]]:
    if size >= manifest.size:
        return prepare_write(manifest, 0, size)
    manifest, removed_ids = prepare_truncate(manifest, size)
    return manifest, [], removed_ids
