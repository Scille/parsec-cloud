# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from typing import Tuple, List, Callable

from collections import defaultdict
from async_generator import asynccontextmanager

from parsec.event_bus import EventBus
from parsec.core.types import FileDescriptor, EntryID, LocalDevice

from parsec.core.fs.remote_loader import RemoteLoader
from parsec.core.fs.local_storage import LocalStorage
from parsec.core.fs.exceptions import FSLocalMissError, FSInvalidFileDescriptor
from parsec.core.types import Chunk, Chunks, BlockID, LocalFileManifest
from parsec.core.fs.workspacefs.file_operations import (
    prepare_read,
    prepare_write,
    prepare_resize,
    prepare_reshape,
)

__all__ = ("FSInvalidFileDescriptor", "FileTransactions")


# Helpers


def normalize_argument(arg, manifest):
    return manifest.size if arg < 0 else arg


def padded_data(data: bytes, start: int, stop: int) -> bytes:
    """Return the data between the start and stop index.

    The data is treated as padded with an infinite amount of null bytes before index 0.
    """
    assert start <= stop
    if start <= stop <= 0:
        return b"\x00" * (stop - start)
    if 0 <= start <= stop:
        return data[start:stop]
    return b"\x00" * (0 - start) + data[0:stop]


class FileTransactions:
    """A stateless class to centralize all file transactions.

    The actual state is stored in the local storage and file transactions
    have access to the remote loader to download missing resources.

    The exposed transactions all take a file descriptor as first argument.
    The file descriptors correspond to an entry id which points to a file
    on the file system (i.e. a file manifest).

    The corresponding file is locked while performing the change (i.e. between
    the reading and writing of the corresponding manifest) in order to avoid
    race conditions and data corruption.

    The table below lists the effects of the 6 file transactions:
    - close    -> remove file descriptor from local storage
    - write    -> affects file content and possibly file size
    - truncate -> affects file size and possibly file content
    - read     -> no side effect
    - flush    -> no-op
    """

    def __init__(
        self,
        workspace_id: EntryID,
        get_workspace_entry: Callable,
        device: LocalDevice,
        local_storage: LocalStorage,
        remote_loader: RemoteLoader,
        event_bus: EventBus,
    ):
        self.workspace_id = workspace_id
        self.get_workspace_entry = get_workspace_entry
        self.local_author = device.device_id
        self.local_storage = local_storage
        self.remote_loader = remote_loader
        self.event_bus = event_bus
        self._write_count = defaultdict(int)

    # Event helper

    def _send_event(self, event, **kwargs):
        self.event_bus.send(event, workspace_id=self.workspace_id, **kwargs)

    # Helper

    def _read_chunk(self, chunk: Chunk) -> bytes:
        data = self.local_storage.get_chunk(chunk.id)
        return data[chunk.start - chunk.raw_offset : chunk.stop - chunk.raw_offset]

    def _write_chunk(self, chunk: Chunk, content: bytes, offset: int = 0) -> None:
        data = padded_data(content, offset, offset + chunk.stop - chunk.start)
        self.local_storage.set_chunk(chunk.id, data)
        return len(data)

    def _build_data(self, chunks: Chunks) -> Tuple[bytes, List[BlockID]]:
        # Empty array
        if not chunks:
            return bytearray(), []

        # Build byte array
        missing = []
        start, stop = chunks[0].start, chunks[-1].stop
        result = bytearray(stop - start)
        for chunk in chunks:
            try:
                result[chunk.start - start : chunk.stop - start] = self._read_chunk(chunk)
            except FSLocalMissError:
                assert chunk.access is not None
                missing.append(chunk.access)

        # Return byte array
        return result, missing

    # Locking helper

    @asynccontextmanager
    async def _load_and_lock_file(self, fd: FileDescriptor):
        # The FSLocalMissError exception is not considered here.
        # This is because we should be able to assume that the manifest
        # corresponding to valid file descriptor is always available locally

        # Get the corresponding entry_id
        manifest = self.local_storage.load_file_descriptor(fd)

        # Lock the entry_id
        async with self.local_storage.lock_manifest(manifest.entry_id):
            yield self.local_storage.load_file_descriptor(fd)

    # Atomic transactions

    async def fd_close(self, fd: FileDescriptor) -> None:
        # Fetch and lock
        async with self._load_and_lock_file(fd) as manifest:

            # Force writing to disk
            self.local_storage.ensure_manifest_persistent(manifest.entry_id)

            # Atomic change
            self.local_storage.remove_file_descriptor(fd, manifest)

            # Clear fd
            self._write_count.pop(fd, None)

    async def fd_write(self, fd: FileDescriptor, content: bytes, offset: int) -> int:
        # Fetch and lock
        async with self._load_and_lock_file(fd) as manifest:

            # No-op
            if not content:
                return 0

            # Prepare
            offset = normalize_argument(offset, manifest)
            manifest, write_operations, removed_ids = prepare_write(manifest, len(content), offset)

            # Writing
            for chunk, offset in write_operations:
                self._write_count[fd] += self._write_chunk(chunk, content, offset)

            # Atomic change
            self.local_storage.set_manifest(manifest.entry_id, manifest, cache_only=True)

            # Clean up
            for removed_id in removed_ids:
                self.local_storage.clear_chunk(removed_id, miss_ok=True)

            # Reshaping
            if self._write_count[fd] >= manifest.blocksize:
                self._manifest_reshape(manifest, cache_only=True)
                self._write_count.pop(fd, None)

        # Notify
        self._send_event("fs.entry.updated", id=manifest.entry_id)
        return len(content)

    async def fd_resize(self, fd: FileDescriptor, length: int) -> None:
        # Fetch and lock
        async with self._load_and_lock_file(fd) as manifest:

            # Perform the resize operation
            self._manifest_resize(manifest, length)

        # Notify
        self._send_event("fs.entry.updated", id=manifest.entry_id)

    async def fd_read(self, fd: FileDescriptor, size: int, offset: int) -> bytes:
        # Loop over attemps
        missing = []
        while True:

            # Load missing blocks
            await self.remote_loader.load_blocks(missing)

            # Fetch and lock
            async with self._load_and_lock_file(fd) as manifest:

                # Normalize
                offset = normalize_argument(offset, manifest)
                size = normalize_argument(size, manifest)

                # No-op
                if offset > manifest.size:
                    return b""

                # Prepare
                chunks = prepare_read(manifest, size, offset)
                data, missing = self._build_data(chunks)

                # Return the data
                if not missing:
                    return data

    async def fd_flush(self, fd: FileDescriptor) -> None:
        async with self._load_and_lock_file(fd) as manifest:
            self._manifest_reshape(manifest)
            self.local_storage.ensure_manifest_persistent(manifest.entry_id)

    # Transaction helpers

    def _manifest_resize(self, manifest: LocalFileManifest, length: int) -> None:
        """This internal helper does not perform any locking."""
        # No-op
        if manifest.size == length:
            return

        # Prepare
        manifest, write_operations, removed_ids = prepare_resize(manifest, length)

        # Writing
        for chunk, offset in write_operations:
            self._write_chunk(chunk, b"", offset)

        # Atomic change
        self.local_storage.set_manifest(manifest.entry_id, manifest, cache_only=True)

        # Clean up
        for removed_id in removed_ids:
            self.local_storage.clear_chunk(removed_id, miss_ok=True)

    def _manifest_reshape(
        self, manifest: LocalFileManifest, cache_only: bool = False
    ) -> List[BlockID]:
        """This internal helper does not perform any locking."""

        # Prepare
        getter, operations = prepare_reshape(manifest)

        # No-op
        if not operations:
            return []

        # Prepare data structures
        missing = []
        result_dict = {}
        removed_ids = set()

        # Perform operations
        for block, (source, destination, cleanup) in operations.items():

            # Build data block
            data, extra_missing = self._build_data(source)

            # Missing data
            if extra_missing:
                missing += extra_missing
                continue

            # Write data if necessary
            new_chunk = destination.evolve_as_block(data)
            if source != (destination,):
                self._write_chunk(new_chunk, data)

            # Update structures
            removed_ids |= cleanup
            result_dict[block] = new_chunk

        # Craft and set new manifest
        new_manifest = getter(result_dict)
        self.local_storage.set_manifest(new_manifest.entry_id, new_manifest, cache_only=cache_only)

        # Perform cleanup
        for removed_id in removed_ids:
            self.local_storage.clear_chunk(removed_id, miss_ok=True)

        # Return missing block ids
        return missing
