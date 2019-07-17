# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from typing import Optional

import attr
from async_generator import asynccontextmanager

from parsec.core.fs.buffer_ordering import (
    Buffer,
    merge_buffers_with_limits,
    quick_filter_block_accesses,
)
from parsec.core.fs.remote_loader import RemoteLoader
from parsec.core.local_storage import (
    FSInvalidFileDescriptor,
    LocalStorage,
    LocalStorageMissingError,
)
from parsec.core.types import BlockAccess, EntryID, FileDescriptor, LocalFileManifest
from parsec.event_bus import EventBus

__all__ = ("FSInvalidFileDescriptor", "FileTransactions")


# Helpers


def normalize_offset(offset, cursor, manifest):
    if offset is None:
        return cursor.offset
    if offset < 0:
        return manifest.size
    return offset


def pad_content(offset: int, size: int, content: bytes = b""):
    empty_gap = offset - size
    if empty_gap <= 0:
        return offset, content
    padded_content = b"\x00" * empty_gap + content
    return size, padded_content


def merge_buffers(manifest: LocalFileManifest, start: int, end: int):
    dirty_blocks = quick_filter_block_accesses(manifest.dirty_blocks, start, end)
    dirty_buffers = [DirtyBlockBuffer(*args) for args in dirty_blocks]
    blocks = quick_filter_block_accesses(manifest.blocks, start, end)
    buffers = [BlockBuffer(*args) for args in blocks]
    return merge_buffers_with_limits(buffers + dirty_buffers, start, end)


@attr.s(slots=True)
class DirtyBlockBuffer(Buffer):
    access = attr.ib(type=BlockAccess)
    data = attr.ib(default=None, type=Optional[bytes])


@attr.s(slots=True)
class BlockBuffer(Buffer):
    access = attr.ib(type=BlockAccess)
    data = attr.ib(default=None, type=Optional[bytes])


class FileTransactions:
    """A stateless class to centralize all file transactions.

    The actual state is stored in the local storage and file transactions
    have access to the remote loader to download missing resources.

    The exposed transactions all take a file descriptor as first argument.
    The file descriptors correspond to a file cursor which points to a file
    on the file system (using access and manifest).

    The corresponding file is locked while performing the change (i.e. between
    the reading and writing of the file cursor and manifest) in order to avoid
    race conditions and data corruption.

    The table below lists the effects of the 6 file transactions:
    - close    -> remove file descriptor from local storage
    - seek     -> affects cursor offset
    - write    -> affects cursor offset, file content and possibly file size
    - truncate -> affects file size and possibly file content
    - read     -> affects cursor offset
    - flush    -> no-op
    """

    def __init__(
        self,
        workspace_id: EntryID,
        local_storage: LocalStorage,
        remote_loader: RemoteLoader,
        event_bus: EventBus,
    ):
        self.workspace_id = workspace_id
        self.local_storage = local_storage
        self.remote_loader = remote_loader
        self.event_bus = event_bus

    # Event helper

    def _send_event(self, event, **kwargs):
        self.event_bus.send(event, workspace_id=self.workspace_id, **kwargs)

    # Locking helper

    @asynccontextmanager
    async def _load_and_lock_file(self, fd: FileDescriptor):
        # The LocalStorageMissingError exception is not considered here.
        # This is because we should be able to assume that the manifest
        # corresponding to valid file descriptor is always available locally

        # Get the corresponding entry_id
        cursor, _ = self.local_storage.load_file_descriptor(fd)
        entry_id = cursor.entry_id

        # Lock the entry_id
        async with self.local_storage.lock_manifest(entry_id):
            yield self.local_storage.load_file_descriptor(fd)

    # Helpers

    def _attempt_read(self, manifest: LocalFileManifest, start: int, end: int):
        missing = []
        data = bytearray(end - start)
        merged = merge_buffers(manifest, start, end)
        for cs in merged.spaces:
            for bs in cs.buffers:
                block_access = bs.buffer.access

                if isinstance(bs.buffer, DirtyBlockBuffer):
                    try:
                        buff = self.local_storage.get_block(block_access.id)
                    except LocalStorageMissingError as exc:
                        raise RuntimeError(f"Unknown local block `{block_access.id}`") from exc

                elif isinstance(bs.buffer, BlockBuffer):
                    try:
                        buff = self.local_storage.get_block(block_access.id)
                    except LocalStorageMissingError:
                        missing.append(block_access)
                        continue

                data[bs.start - cs.start : bs.end - cs.start] = buff[
                    bs.buffer_slice_start : bs.buffer_slice_end
                ]

        return data, missing

    # Atomic transactions

    async def fd_close(self, fd: FileDescriptor) -> None:
        # Fetch and lock
        async with self._load_and_lock_file(fd) as (cursor, manifest):

            # Atomic change
            self.local_storage.remove_file_descriptor(fd, manifest)

    async def fd_seek(self, fd: FileDescriptor, offset: int) -> None:
        # Fetch and lock
        async with self._load_and_lock_file(fd) as (cursor, manifest):

            # Prepare
            offset = normalize_offset(offset, cursor, manifest)

            # Atomic change
            cursor.offset = offset

    async def fd_write(self, fd: FileDescriptor, content: bytes, offset: int = None) -> int:
        # Fetch and lock
        async with self._load_and_lock_file(fd) as (cursor, manifest):

            # No-op
            if not content:
                return 0

            # Prepare
            offset = normalize_offset(offset, cursor, manifest)
            start, padded_content = pad_content(offset, manifest.size, content)
            block_access = BlockAccess.from_block(padded_content, start)

            new_offset = offset + len(content)
            new_size = max(manifest.size, new_offset)

            manifest = manifest.evolve_and_mark_updated(
                dirty_blocks=(*manifest.dirty_blocks, block_access), size=new_size
            )

            # Atomic change
            self.local_storage.set_dirty_block(block_access.id, padded_content)
            self.local_storage.set_manifest(cursor.entry_id, manifest)
            cursor.offset = new_offset

        # Notify
        self._send_event("fs.entry.updated", id=cursor.entry_id)
        return len(content)

    async def fd_resize(self, fd: FileDescriptor, length: int) -> None:
        # Fetch and lock
        async with self._load_and_lock_file(fd) as (cursor, manifest):

            # No-op
            if manifest.size == length:
                return

            # Prepare
            dirty_blocks = manifest.dirty_blocks
            start, padded_content = pad_content(length, manifest.size)
            block_access = BlockAccess.from_block(padded_content, start)
            if padded_content:
                dirty_blocks += (block_access,)
            manifest = manifest.evolve_and_mark_updated(dirty_blocks=dirty_blocks, size=length)

            # Atomic change
            if padded_content:
                self.local_storage.set_dirty_block(block_access.id, padded_content)
            self.local_storage.set_manifest(cursor.entry_id, manifest)

        # Notify
        self._send_event("fs.entry.updated", id=cursor.entry_id)

    async def fd_read(self, fd: FileDescriptor, size: int = -1, offset: int = None) -> bytes:
        # Loop over attemps
        missing = []
        while True:

            # Load missing blocks
            # TODO: add a `load_blocks` method to the remote loader
            # to download the blocks in a concurrent way.
            for block_access in missing:
                await self.remote_loader.load_block(block_access)

            # Fetch and lock
            async with self._load_and_lock_file(fd) as (cursor, manifest):

                # No-op
                offset = normalize_offset(offset, cursor, manifest)
                if offset is not None and offset > manifest.size:
                    return b""

                # Prepare
                start = offset
                size = manifest.size if size < 0 else size
                end = min(offset + size, manifest.size)
                data, missing = self._attempt_read(manifest, start, end)

                # Retry
                if missing:
                    continue

                # Atomic change
                cursor.offset = end

                return data

    async def fd_flush(self, fd: FileDescriptor) -> None:
        # No-op
        pass
