# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from parsec.core.core_events import CoreEvent
from typing import AsyncIterator, Callable, Dict, List, Tuple, cast
from collections import defaultdict
from contextlib import asynccontextmanager

from parsec.event_bus import EventBus
from parsec.api.data import BlockAccess
from parsec.api.protocol import DeviceID
from parsec.core.types import FileDescriptor, EntryID, LocalDevice
from parsec.core.fs.remote_loader import RemoteLoader
from parsec.core.fs.storage import BaseWorkspaceStorage
from parsec.core.fs.exceptions import FSLocalMissError, FSInvalidFileDescriptor, FSEndOfFileError
from parsec.core.types import (
    Chunk,
    WorkspaceEntry,
    LocalFileManifest,
    LocalFolderishManifests,
    LocalNonRootManifests,
    LocalWorkspaceManifest,
)
from parsec.core.fs.workspacefs.file_operations import (
    prepare_read,
    prepare_write,
    prepare_resize,
    prepare_reshape,
)


__all__ = ("FSInvalidFileDescriptor", "FileTransactions")


# Helpers


def normalize_argument(arg: int, manifest: LocalFileManifest) -> int:
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
        get_workspace_entry: Callable[[], WorkspaceEntry],
        device: LocalDevice,
        local_storage: BaseWorkspaceStorage,
        remote_loader: RemoteLoader,
        event_bus: EventBus,
        preferred_language: str,
    ):
        self.workspace_id = workspace_id
        self.get_workspace_entry = get_workspace_entry
        self.device = device
        self.local_storage = local_storage
        self.remote_loader = remote_loader
        self.event_bus = event_bus
        self._write_count: Dict[FileDescriptor, int] = defaultdict(int)
        self.preferred_language = preferred_language

    @property
    def local_author(self) -> DeviceID:
        return self.device.device_id

    # Event helper

    def _send_event(self, event: CoreEvent, **kwargs: object) -> None:
        self.event_bus.send(event, workspace_id=self.workspace_id, **kwargs)

    # Helper

    async def _read_chunk(self, chunk: Chunk) -> bytes:
        data = await self.local_storage.get_chunk(chunk.id)
        return data[chunk.start - chunk.raw_offset : chunk.stop - chunk.raw_offset]

    async def _write_chunk(self, chunk: Chunk, content: bytes, offset: int = 0) -> int:
        data = padded_data(content, offset, offset + chunk.stop - chunk.start)
        await self.local_storage.set_chunk(chunk.id, data)
        return len(data)

    async def _build_data(self, chunks: Tuple[Chunk, ...]) -> Tuple[bytes, List[BlockAccess]]:
        # Empty array
        if not chunks:
            return bytearray(), []

        # Build byte array
        missing = []
        start, stop = chunks[0].start, chunks[-1].stop
        result = bytearray(stop - start)
        for chunk in chunks:
            try:
                result[chunk.start - start : chunk.stop - start] = await self._read_chunk(chunk)
            except FSLocalMissError:
                assert chunk.access is not None
                missing.append(chunk.access)

        # Return byte array
        return result, missing

    # Locking helper

    @asynccontextmanager
    async def _load_and_lock_file(self, fd: FileDescriptor) -> AsyncIterator[LocalFileManifest]:
        # The FSLocalMissError exception is not considered here.
        # This is because we should be able to assume that the manifest
        # corresponding to valid file descriptor is always available locally

        # Get the corresponding entry_id
        manifest = await self.local_storage.load_file_descriptor(fd)

        # Lock the entry_id
        async with self.local_storage.lock_manifest(manifest.id):
            yield await self.local_storage.load_file_descriptor(fd)

    # Confinement helper

    async def _get_confinement_point(self, entry_id: EntryID) -> EntryID | None:
        # Load the corresponding manifest
        try:
            current_manifest = await self.local_storage.get_manifest(entry_id)
        # A missing entry is never confined
        except FSLocalMissError:
            return None

        # Walk the parent chain until the workspace manifest is reached
        while not isinstance(current_manifest, LocalWorkspaceManifest):
            current_manifest = cast(LocalNonRootManifests, current_manifest)

            try:
                parent_manifest = await self.local_storage.get_manifest(current_manifest.parent)
            # In the very unlikely case where the parent manifest is not present,
            # simply consider the entry to be not confined as having false negative
            # on confinement detection is not a big deal.
            except FSLocalMissError:
                return None

            # A parent manifest is necessarily a local folder/workspace manifest
            parent_manifest = cast(LocalFolderishManifests, parent_manifest)

            # The entry is not confined
            if current_manifest.id in parent_manifest.local_confinement_points:
                return parent_manifest.id

            # Walk down
            current_manifest = parent_manifest

        # The entry is not confined
        return None

    # Atomic transactions

    async def fd_size(self, fd: FileDescriptor) -> int:
        manifest = await self.local_storage.load_file_descriptor(fd)
        return manifest.size

    async def fd_info(self, fd: FileDescriptor) -> Dict[str, object]:
        manifest = await self.local_storage.load_file_descriptor(fd)
        stats = manifest.to_stats()
        stats["confinement_point"] = await self._get_confinement_point(manifest.id)
        return stats

    async def fd_close(self, fd: FileDescriptor) -> None:
        # Fetch and lock
        async with self._load_and_lock_file(fd) as manifest:

            # Force writing to disk
            await self.local_storage.ensure_manifest_persistent(manifest.id)

            # Atomic change
            self.local_storage.remove_file_descriptor(fd)

            # Clear write count
            self._write_count.pop(fd, None)

    async def fd_write(
        self, fd: FileDescriptor, content: bytes, offset: int, constrained: bool = False
    ) -> int:
        # Fetch and lock
        async with self._load_and_lock_file(fd) as manifest:

            # Constrained - truncate content to the right length
            if constrained:
                end_offset = min(manifest.size, offset + len(content))
                length = max(end_offset - offset, 0)
                content = content[:length]

            # No-op
            if not content:
                return 0

            # Prepare
            updated = self.device.timestamp()
            offset = normalize_argument(offset, manifest)
            manifest, write_operations, removed_ids = prepare_write(
                manifest, len(content), offset, updated
            )

            # Writing
            for chunk, offset in write_operations:
                self._write_count[fd] += await self._write_chunk(chunk, content, offset)

            # Atomic change
            await self.local_storage.set_manifest(
                manifest.id, manifest, cache_only=True, removed_ids=removed_ids
            )

            # Reshaping
            if self._write_count[fd] >= manifest.blocksize:
                await self._manifest_reshape(manifest, cache_only=True)
                self._write_count.pop(fd, None)

        # Notify
        self._send_event(CoreEvent.FS_ENTRY_UPDATED, id=manifest.id)
        return len(content)

    async def fd_resize(self, fd: FileDescriptor, length: int, truncate_only: bool = False) -> None:
        # Fetch and lock
        async with self._load_and_lock_file(fd) as manifest:

            # Truncate only
            if truncate_only and manifest.size <= length:
                return

            # Perform the resize operation
            await self._manifest_resize(manifest, length, cache_only=True)

        # Notify
        self._send_event(CoreEvent.FS_ENTRY_UPDATED, id=manifest.id)

    async def fd_read(
        self, fd: FileDescriptor, size: int, offset: int, raise_eof: bool = False
    ) -> bytes:
        # Loop over attempts
        missing: List[BlockAccess] = []
        while True:

            # Load missing blocks
            await self.remote_loader.load_blocks(missing)

            # Fetch and lock
            async with self._load_and_lock_file(fd) as manifest:

                # End of file
                if raise_eof and offset >= manifest.size:
                    raise FSEndOfFileError()

                # Normalize
                offset = normalize_argument(offset, manifest)
                size = normalize_argument(size, manifest)

                # No-op
                if offset > manifest.size:
                    return b""

                # Prepare
                chunks = prepare_read(manifest, size, offset)
                data, missing = await self._build_data(chunks)

                # Return the data
                if not missing:
                    return data

    async def fd_flush(self, fd: FileDescriptor) -> None:
        async with self._load_and_lock_file(fd) as manifest:
            await self._manifest_reshape(manifest)
            await self.local_storage.ensure_manifest_persistent(manifest.id)

    # Transaction helpers

    async def _manifest_resize(
        self, manifest: LocalFileManifest, length: int, cache_only: bool = False
    ) -> None:
        """This internal helper does not perform any locking."""
        # No-op
        if manifest.size == length:
            return

        # Prepare
        updated = self.device.timestamp()
        manifest, write_operations, removed_ids = prepare_resize(manifest, length, updated)

        # Writing
        for chunk, offset in write_operations:
            await self._write_chunk(chunk, b"", offset)

        # Atomic change
        await self.local_storage.set_manifest(
            manifest.id, manifest, removed_ids=removed_ids, cache_only=cache_only
        )

    async def _manifest_reshape(
        self, manifest: LocalFileManifest, cache_only: bool = False
    ) -> List[BlockAccess]:
        """This internal helper does not perform any locking."""

        # Prepare data structures
        missing = []

        # Perform operations
        for block, source, destination, write_back, removed_ids in prepare_reshape(manifest):

            # Build data block
            data, extra_missing = await self._build_data(source)

            # Missing data
            if extra_missing:
                missing += extra_missing
                continue

            # Write data if necessary
            new_chunk = destination.evolve_as_block(data)
            if write_back:
                await self._write_chunk(new_chunk, data)

            # Craft the new manifest
            manifest = manifest.evolve_single_block(block, new_chunk)

            # Set the new manifest, acting as a checkpoint
            await self.local_storage.set_manifest(
                manifest.id, manifest, cache_only=True, removed_ids=removed_ids
            )

        # Flush if necessary
        if not cache_only:
            await self.local_storage.ensure_manifest_persistent(manifest.id)

        # Return missing block ids
        return missing
