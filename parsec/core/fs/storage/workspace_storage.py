# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from pathlib import Path
from collections import defaultdict
from typing import Dict, Tuple, Set

import trio
from trio import hazmat
from pendulum import Pendulum
from structlog import get_logger
from async_generator import asynccontextmanager

from parsec.core.types import (
    EntryID,
    BlockID,
    ChunkID,
    LocalDevice,
    FileDescriptor,
    LocalManifest,
    LocalFileManifest,
)
from parsec.core.fs.exceptions import FSError, FSLocalMissError, FSInvalidFileDescriptor

from parsec.core.fs.storage.manifest_storage import ManifestStorage
from parsec.core.fs.storage.chunk_storage import ChunkStorage, BlockStorage
from parsec.core.fs.storage.version import (
    MANIFEST_STORAGE_NAME,
    CHUNK_STORAGE_NAME,
    BLOCK_STORAGE_NAME,
)


logger = get_logger()

# TODO: should be in config.py
DEFAULT_BLOCK_CACHE_SIZE = 128 * 1024 * 1024
DEFAULT_CHUNK_VACUUM_THRESHOLD = 128 * 1024 * 1024


class WorkspaceStorage:
    """Manage the access to the local storage.

    That includes:
    - a cache in memory for fast access to deserialized data
    - the persistent storage to keep serialized data on the disk
    - a lock mecanism to protect against race conditions
    """

    def __init__(
        self,
        device: LocalDevice,
        path: Path,
        workspace_id: EntryID,
        manifest_storage: ManifestStorage,
        block_storage: ChunkStorage,
        chunk_storage: ChunkStorage,
    ):
        self.path = path
        self.device = device
        self.device_id = device.device_id
        self.workspace_id = workspace_id

        # File descriptors
        self.open_fds: Dict[FileDescriptor, EntryID] = {}
        self.fd_counter = 0

        # Locking structures
        self.locking_tasks = {}
        self.entry_locks = defaultdict(trio.Lock)

        # Manifest and block storage
        self.manifest_storage = manifest_storage
        self.block_storage = block_storage
        self.chunk_storage = chunk_storage

    @classmethod
    @asynccontextmanager
    async def run(
        cls,
        device: LocalDevice,
        path: Path,
        workspace_id: EntryID,
        cache_size=DEFAULT_BLOCK_CACHE_SIZE,
        vacuum_threshold=DEFAULT_CHUNK_VACUUM_THRESHOLD,
    ):
        manifest_storage_context = ManifestStorage.run(
            device, path / MANIFEST_STORAGE_NAME, workspace_id
        )
        block_storage_context = BlockStorage.run(
            device, path / BLOCK_STORAGE_NAME, cache_size=cache_size
        )
        chunk_storage_context = ChunkStorage.run(
            device, path / CHUNK_STORAGE_NAME, vacuum_threshold=vacuum_threshold
        )
        async with manifest_storage_context as manifest_storage:
            async with block_storage_context as block_storage:
                async with chunk_storage_context as chunk_storage:
                    yield cls(
                        device, path, workspace_id, manifest_storage, block_storage, chunk_storage
                    )

    # Helpers

    def _get_next_fd(self) -> FileDescriptor:
        self.fd_counter += 1
        return FileDescriptor(self.fd_counter)

    def clear_memory_cache(self):
        self.manifest_storage.clear_memory_cache()

    # Locking helpers

    @asynccontextmanager
    async def lock_entry_id(self, entry_id: EntryID):
        async with self.entry_locks[entry_id]:
            try:
                self.locking_tasks[entry_id] = hazmat.current_task()
                yield entry_id
            finally:
                del self.locking_tasks[entry_id]

    @asynccontextmanager
    async def lock_manifest(self, entry_id: EntryID):
        async with self.lock_entry_id(entry_id):
            yield await self.get_manifest(entry_id)

    def _check_lock_status(self, entry_id: EntryID) -> None:
        task = self.locking_tasks.get(entry_id)
        if task != hazmat.current_task():
            raise RuntimeError(f"Entry `{entry_id}` modified without beeing locked")

    # Checkpoint interface

    async def get_realm_checkpoint(self) -> int:
        return await self.manifest_storage.get_realm_checkpoint()

    async def update_realm_checkpoint(
        self, new_checkpoint: int, changed_vlobs: Dict[EntryID, int]
    ) -> None:
        """
        Raises: Nothing !
        """
        await self.manifest_storage.update_realm_checkpoint(new_checkpoint, changed_vlobs)

    async def get_need_sync_entries(self) -> Tuple[Set[EntryID], Set[EntryID]]:
        return await self.manifest_storage.get_need_sync_entries()

    # Manifest interface

    async def get_manifest(self, entry_id: EntryID) -> LocalManifest:
        """Raises: FSLocalMissError"""
        return await self.manifest_storage.get_manifest(entry_id)

    async def set_manifest(
        self,
        entry_id: EntryID,
        manifest: LocalManifest,
        cache_only: bool = False,
        check_lock_status=True,
    ) -> None:
        if check_lock_status:
            self._check_lock_status(entry_id)
        await self.manifest_storage.set_manifest(entry_id, manifest, cache_only=cache_only)

    async def ensure_manifest_persistent(self, entry_id: EntryID) -> None:
        self._check_lock_status(entry_id)
        await self.manifest_storage.ensure_manifest_persistent(entry_id)

    async def clear_manifest(self, entry_id: EntryID) -> None:
        self._check_lock_status(entry_id)
        await self.manifest_storage.clear_manifest(entry_id)

    # Block interface

    async def set_clean_block(self, block_id: BlockID, block: bytes) -> None:
        assert isinstance(block_id, BlockID)
        return await self.block_storage.set_chunk(ChunkID(block_id), block)

    async def clear_clean_block(self, block_id: BlockID) -> None:
        assert isinstance(block_id, BlockID)
        try:
            await self.block_storage.clear_chunk(ChunkID(block_id))
        except FSLocalMissError:
            pass

    async def get_dirty_block(self, block_id: BlockID) -> bytes:
        return await self.chunk_storage.get_chunk(ChunkID(block_id))

    # Chunk interface

    async def get_chunk(self, chunk_id: ChunkID) -> bytes:
        assert isinstance(chunk_id, ChunkID)
        try:
            return await self.chunk_storage.get_chunk(chunk_id)
        except FSLocalMissError:
            return await self.block_storage.get_chunk(chunk_id)

    async def set_chunk(self, chunk_id: ChunkID, block: bytes) -> None:
        assert isinstance(chunk_id, ChunkID)
        return await self.chunk_storage.set_chunk(chunk_id, block)

    async def clear_chunk(self, chunk_id: ChunkID, miss_ok: bool = False) -> None:
        assert isinstance(chunk_id, ChunkID)
        try:
            await self.chunk_storage.clear_chunk(chunk_id)
        except FSLocalMissError:
            if not miss_ok:
                raise

    # File management interface

    def create_file_descriptor(self, manifest: LocalFileManifest) -> FileDescriptor:
        assert isinstance(manifest, LocalFileManifest)
        fd = self._get_next_fd()
        self.open_fds[fd] = manifest.id
        return fd

    async def load_file_descriptor(self, fd: FileDescriptor) -> LocalFileManifest:
        try:
            entry_id = self.open_fds[fd]
        except KeyError:
            raise FSInvalidFileDescriptor(fd)
        manifest = await self.get_manifest(entry_id)
        assert isinstance(manifest, LocalFileManifest)
        return manifest

    def remove_file_descriptor(self, fd: FileDescriptor) -> None:
        try:
            self.open_fds.pop(fd)
        except KeyError:
            raise FSInvalidFileDescriptor(fd)

    # Vacuum

    async def run_vacuum(self):
        await self.chunk_storage.run_vacuum()

    # Timestamped workspace

    def to_timestamped(self, timestamp: Pendulum):
        return WorkspaceStorageTimestamped(self, timestamp)


class WorkspaceStorageTimestamped(WorkspaceStorage):
    """Timestamped version to access a local storage as it was at a given timestamp

    That includes:
    - another cache in memory for fast access to deserialized data
    - the timestamped persistent storage to keep serialized data on the disk :
      vlobs are in common, not manifests. Actually only vlobs are used, manifests are mocked
    - the same lock mecanism to protect against race conditions, although it is useless there
    """

    def __init__(self, workspace_storage: WorkspaceStorage, timestamp: Pendulum):
        super().__init__(
            workspace_storage.device,
            workspace_storage.path,
            workspace_storage.workspace_id,
            None,
            workspace_storage.block_storage,
            workspace_storage.chunk_storage,
        )

        self._cache = {}
        self.timestamp = timestamp

        self.set_chunk = self._throw_permission_error
        self.clear_chunk = self._throw_permission_error
        self.clear_manifest = self._throw_permission_error

    def _throw_permission_error(*args, **kwargs):
        raise FSError("Not implemented : WorkspaceStorage is timestamped")

    # Manifest interface

    async def get_manifest(self, entry_id: EntryID) -> LocalManifest:
        """Raises: FSLocalMissError"""
        assert isinstance(entry_id, EntryID)
        try:
            return self._cache[entry_id]
        except KeyError:
            raise FSLocalMissError(entry_id)

    async def set_manifest(
        self, entry_id: EntryID, manifest: LocalManifest, cache_only: bool = False
    ) -> None:  # initially for clean
        assert isinstance(entry_id, EntryID)
        if manifest.need_sync:
            return self._throw_permission_error()
        self._check_lock_status(entry_id)
        self._cache[entry_id] = manifest

    async def ensure_manifest_persistent(self, entry_id: EntryID) -> None:
        pass
