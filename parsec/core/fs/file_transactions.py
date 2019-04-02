# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
from math import inf
from typing import Optional
from collections import defaultdict

from parsec.event_bus import EventBus
from parsec.core.types import FileDescriptor
from parsec.core.fs.remote_loader import RemoteLoader
from parsec.core.types import Access, BlockAccess, LocalFileManifest
from parsec.core.local_storage import LocalStorage, LocalStorageMissingEntry
from parsec.core.fs.buffer_ordering import (
    quick_filter_block_accesses,
    Buffer,
    merge_buffers_with_limits,
)


# Helpers


def normalize_offset(offset, cursor, manifest):
    if offset is None:
        return cursor.offset
    if offset < 0:
        return manifest.size
    return offset


def shorten_data_repr(data: bytes) -> bytes:
    if len(data) > 100:
        return data[:40] + b"..." + data[-40:]
    else:
        return data


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


@attr.s(slots=True)
class FileCursor:
    access = attr.ib(type=Access)
    offset = attr.ib(default=0, type=int)


class FSInvalidFileDescriptor(Exception):
    pass


class FileTransactions:
    def __init__(
        self, local_storage: LocalStorage, remote_loader: RemoteLoader, event_bus: EventBus
    ):
        self.local_storage = local_storage
        self.remote_loader = remote_loader
        self.event_bus = event_bus

        # TODO: move those attributes in the local storage
        # Those transaction classes should be stateless
        # It would also help a lot with the tests.
        self._opened_cursors = {}
        self._references = defaultdict(set)
        self._next_fd = 1

    # Helpers

    def _get_cursor(self, fd: FileDescriptor) -> FileCursor:
        try:
            return self._opened_cursors[fd]
        except KeyError:
            raise FSInvalidFileDescriptor(fd)

    async def _get_cursor_and_manifest(self, fd: FileDescriptor) -> LocalFileManifest:
        cursor = self._get_cursor(fd)
        # Fetch the manifest
        try:
            manifest = self.local_storage.get_manifest(cursor.access)
        except LocalStorageMissingEntry as exc:
            manifest = await self.remote_loader.load_manifest(exc.access)
        # Make sure fd still exists
        assert cursor == self._get_cursor(fd)
        return cursor, manifest

    def _manifest_cleanup(
        self,
        access: Access,
        manifest: LocalFileManifest,
        closing_fd: Optional[FileDescriptor] = None,
    ) -> None:
        # Get current references
        references = self._references[access].copy()
        if closing_fd is not None:
            references -= {closing_fd}

        # A reference still exists
        if references:
            if closing_fd is not None:
                self._references[access].discard(closing_fd)
                self._opened_cursors.pop(closing_fd, None)
            return

        # Clean up the local storage
        self.local_storage.clear_manifest(access)
        for access in manifest.dirty_blocks:
            self.local_storage.clear_dirty_block(access)
        # We probably want to clear the clean blocks too
        # Let's do that after the local_folder_fs refactoring
        # for access in manifest.blocks:
        #     self.local_storage.clear_clean_block(access)

        # Clean up references
        self._references.pop(access, None)
        if closing_fd is not None:
            del self._opened_cursors[closing_fd]

    def _attempt_read(self, manifest: LocalFileManifest, start: int, end: int):
        missing = []
        data = bytearray(end - start)
        merged = merge_buffers(manifest, start, end)
        for cs in merged.spaces:
            for bs in cs.buffers:
                access = bs.buffer.access

                if isinstance(bs.buffer, DirtyBlockBuffer):
                    try:
                        buff = self.local_storage.get_block(access)
                    except LocalStorageMissingEntry as exc:
                        raise RuntimeError(f"Unknown local block `{access.id}`") from exc

                elif isinstance(bs.buffer, BlockBuffer):
                    try:
                        buff = self.local_storage.get_block(access)
                    except LocalStorageMissingEntry:
                        missing.append(access)
                        continue

                data[bs.start - cs.start : bs.end - cs.start] = buff[
                    bs.buffer_slice_start : bs.buffer_slice_end
                ]

        return data, missing

    # Atomic helpers for folder transactions

    def open(self, access: Access) -> FileDescriptor:
        cursor = FileCursor(access)
        fd = self._next_fd
        self._opened_cursors[fd] = cursor
        self._next_fd += 1

        # Add a reference to access as the file exists
        self._references[access].add(access)
        # Add a reference to the open file descriptor
        self._references[access].add(fd)

        return fd

    def deleted(self, access: Access, manifest: LocalFileManifest):
        # Remove the reference to access as the file no longer exists
        self._references[access].discard(access)
        # Perform the clean up routine
        self._manifest_cleanup(access, manifest)

    # Atomic transactions

    async def close(self, fd: FileDescriptor) -> None:
        # Fetch
        cursor, manifest = await self._get_cursor_and_manifest(fd)

        # Atomic change
        self._manifest_cleanup(cursor.access, manifest, closing_fd=fd)

    async def seek(self, fd: FileDescriptor, offset: int) -> None:
        # Fetch
        cursor, manifest = await self._get_cursor_and_manifest(fd)

        # Prepare
        offset = normalize_offset(offset, cursor, manifest)

        # Atomic change
        cursor.offset = offset

    async def write(self, fd: FileDescriptor, content: bytes, offset: int = None) -> int:
        # Fetch
        cursor, manifest = await self._get_cursor_and_manifest(fd)

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
        self.local_storage.set_dirty_block(block_access, padded_content)
        self.local_storage.set_dirty_manifest(cursor.access, manifest)
        cursor.offset = new_offset

        # Notify
        self.event_bus.send("fs.entry.updated", id=cursor.access.id)
        return len(content)

    async def truncate(self, fd: FileDescriptor, length: int) -> None:
        # Fetch
        cursor, manifest = await self._get_cursor_and_manifest(fd)

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
            self.local_storage.set_dirty_block(block_access, padded_content)
        self.local_storage.set_dirty_manifest(cursor.access, manifest)

        # Notify
        self.event_bus.send("fs.entry.updated", id=cursor.access.id)

    async def read(self, fd: FileDescriptor, size: int = inf, offset: int = None) -> bytes:
        # Loop over attemps
        while True:

            # Fetch
            cursor, manifest = await self._get_cursor_and_manifest(fd)

            # No-op
            offset = normalize_offset(offset, cursor, manifest)
            if offset is not None and offset > manifest.size:
                return b""

            # Prepare
            start = offset
            end = min(offset + size, manifest.size)
            data, missing = self._attempt_read(manifest, start, end)

            # Fetch
            for access in missing:
                await self.remote_loader.load_block(access)

            # Retry
            if missing:
                continue

            # Atomic change
            cursor.offset = end

            return data

    async def flush(self, fd: FileDescriptor) -> None:
        # No-op
        pass
