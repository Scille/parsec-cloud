# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from collections import defaultdict
from contextlib import asynccontextmanager
from pathlib import Path
from typing import TYPE_CHECKING, AsyncIterator, NoReturn, Type, cast

import trio
from structlog import get_logger

from parsec._parsec import (
    BlockID,
    ChunkID,
    DateTime,
    EntryID,
    LocalDevice,
    LocalFileManifest,
    LocalWorkspaceManifest,
    Regex,
)
from parsec._parsec import WorkspaceStorage as _SyncWorkspaceStorage
from parsec._parsec import WorkspaceStorageSnapshot as _SyncWorkspaceStorageSnapshot
from parsec._parsec import (
    workspace_storage_non_speculative_init as _sync_workspace_storage_non_speculative_init,
)
from parsec.core.config import DEFAULT_WORKSPACE_STORAGE_CACHE_SIZE
from parsec.core.fs.exceptions import FSError
from parsec.core.types import FileDescriptor
from parsec.core.types.manifest import AnyLocalManifest

logger = get_logger()

if TYPE_CHECKING:
    from parsec._parsec import PseudoFileDescriptor
else:
    PseudoFileDescriptor = Type["PseudoFileDescriptor"]

DEFAULT_CHUNK_VACUUM_THRESHOLD = 512 * 1024 * 1024

FAILSAFE_PATTERN_FILTER = Regex.from_regex_str(
    r"^\b$"
)  # Matches nothing (https://stackoverflow.com/a/2302992/2846140)

AnyWorkspaceStorage = "WorkspaceStorage" | "WorkspaceStorageSnapshot"

__all__ = [
    "WorkspaceStorage",
    "AnyWorkspaceStorage",
    "PseudoFileDescriptor",
]


async def workspace_storage_non_speculative_init(
    data_base_dir: Path,
    device: LocalDevice,
    workspace_id: EntryID,
) -> None:
    return await trio.to_thread.run_sync(
        _sync_workspace_storage_non_speculative_init,
        data_base_dir,
        device,
        workspace_id,
        device.timestamp(),
    )


class WorkspaceStorage:
    def __init__(
        self,
        data_base_dir: Path,
        device: LocalDevice,
        workspace_id: EntryID,
        cache_size: int,
    ):
        self.sync_instance = _SyncWorkspaceStorage(
            data_base_dir,
            device,
            workspace_id,
            cache_size,
        )
        # Locking structures
        self.locking_tasks: dict[EntryID, trio.lowlevel.Task] = {}
        self.entry_locks: dict[EntryID, trio.Lock] = defaultdict(trio.Lock)

    @property
    def device(self) -> LocalDevice:
        return self.sync_instance.device

    @property
    def workspace_id(self) -> EntryID:
        return self.sync_instance.workspace_id

    @classmethod
    @asynccontextmanager
    async def run(
        cls,
        data_base_dir: Path,
        device: LocalDevice,
        workspace_id: EntryID,
        prevent_sync_pattern: Regex = FAILSAFE_PATTERN_FILTER,
        cache_size: int = DEFAULT_WORKSPACE_STORAGE_CACHE_SIZE,
    ) -> AsyncIterator["WorkspaceStorage"]:
        instance = cls(
            data_base_dir,
            device,
            workspace_id,
            cache_size,
        )
        try:

            # Load "prevent sync" pattern
            await instance.set_prevent_sync_pattern(prevent_sync_pattern)

            yield instance
        finally:
            instance.sync_instance.clear_memory_cache(flush=True)

    # Helpers

    async def clear_memory_cache(self, flush: bool = True) -> None:
        return await trio.to_thread.run_sync(self.sync_instance.clear_memory_cache, flush)

    # Locking helpers

    @asynccontextmanager
    async def lock_entry_id(self, entry_id: EntryID) -> AsyncIterator[EntryID]:
        async with self.entry_locks[entry_id]:
            try:
                self.locking_tasks[entry_id] = trio.lowlevel.current_task()
                yield entry_id
            finally:
                del self.locking_tasks[entry_id]

    @asynccontextmanager
    async def lock_manifest(self, entry_id: EntryID) -> AsyncIterator[AnyLocalManifest]:
        async with self.lock_entry_id(entry_id):
            yield await self.get_manifest(entry_id)

    def _check_lock_status(self, entry_id: EntryID) -> None:
        task = self.locking_tasks.get(entry_id)
        if task != trio.lowlevel.current_task():
            raise RuntimeError(f"Entry `{entry_id.hex}` modified without being locked")

    # File management interface

    def create_file_descriptor(self, manifest: LocalFileManifest) -> FileDescriptor:
        return cast(FileDescriptor, self.sync_instance.create_file_descriptor(manifest))

    async def load_file_descriptor(self, fd: PseudoFileDescriptor) -> LocalFileManifest:
        return await trio.to_thread.run_sync(self.sync_instance.load_file_descriptor, fd)

    def remove_file_descriptor(self, fd: int) -> None:
        return self.sync_instance.remove_file_descriptor(fd)

    # Block interface

    async def set_clean_block(self, block_id: BlockID, blocks: bytes) -> None:
        return await trio.to_thread.run_sync(self.sync_instance.set_clean_block, block_id, blocks)

    async def clear_clean_block(self, block_id: BlockID) -> None:
        return await trio.to_thread.run_sync(self.sync_instance.clear_clean_block, block_id)

    async def get_dirty_block(self, block_id: BlockID) -> bytes:
        return await trio.to_thread.run_sync(self.sync_instance.get_dirty_block, block_id)

    # Chunk interface

    async def get_chunk(self, chunk_id: ChunkID) -> bytes:
        return await trio.to_thread.run_sync(self.sync_instance.get_chunk, chunk_id)

    async def set_chunk(self, chunk_id: ChunkID, block: bytes) -> None:
        return await trio.to_thread.run_sync(self.sync_instance.set_chunk, chunk_id, block)

    async def clear_chunk(self, chunk_id: ChunkID, miss_ok: bool = False) -> None:
        return await trio.to_thread.run_sync(self.sync_instance.clear_chunk, chunk_id, miss_ok)

    # "Prevent sync" pattern interface

    def get_prevent_sync_pattern(self) -> Regex:
        return self.sync_instance.get_prevent_sync_pattern()

    def get_prevent_sync_pattern_fully_applied(self) -> bool:
        return self.sync_instance.get_prevent_sync_pattern_fully_applied()

    async def set_prevent_sync_pattern(self, pattern: Regex) -> None:
        """set the "prevent sync" pattern for the corresponding workspace

        This operation is idempotent,
        i.e it does not reset the `fully_applied` flag if the pattern hasn't changed.
        """
        return await trio.to_thread.run_sync(self.sync_instance.set_prevent_sync_pattern, pattern)

    async def mark_prevent_sync_pattern_fully_applied(self, pattern: Regex) -> None:
        """Mark the provided pattern as fully applied.

        This is meant to be called after one made sure that all the manifests in the
        workspace are compliant with the new pattern. The applied pattern is provided
        as an argument in order to avoid concurrency issues.
        """
        return await trio.to_thread.run_sync(
            self.sync_instance.mark_prevent_sync_pattern_fully_applied, pattern
        )

    async def get_local_chunk_ids(self, chunk_ids: list[ChunkID]) -> tuple[ChunkID, ...]:
        return await trio.to_thread.run_sync(self.sync_instance.get_local_chunk_ids, chunk_ids)

    async def get_local_block_ids(self, chunk_ids: list[ChunkID]) -> tuple[ChunkID, ...]:
        return await trio.to_thread.run_sync(self.sync_instance.get_local_block_ids, chunk_ids)

    # Manifest interface

    def get_workspace_manifest(self) -> LocalWorkspaceManifest:
        return self.sync_instance.get_workspace_manifest()

    async def get_manifest(self, entry_id: EntryID) -> AnyLocalManifest:
        return await trio.to_thread.run_sync(self.sync_instance.get_manifest, entry_id)

    async def set_manifest(
        self,
        entry_id: EntryID,
        manifest: AnyLocalManifest,
        cache_only: bool = False,
        check_lock_status: bool = True,
        removed_ids: set[ChunkID] | None = None,
    ) -> None:
        if check_lock_status:
            self._check_lock_status(entry_id)
        await trio.to_thread.run_sync(
            self.sync_instance.set_manifest, entry_id, manifest, cache_only, removed_ids
        )

    async def ensure_manifest_persistent(self, entry_id: EntryID) -> None:
        self._check_lock_status(entry_id)
        await trio.to_thread.run_sync(self.sync_instance.ensure_manifest_persistent, entry_id)

    async def clear_manifest(self, entry_id: EntryID) -> None:
        self._check_lock_status(entry_id)
        await trio.to_thread.run_sync(self.sync_instance.clear_manifest, entry_id)

    # Checkpoint interface

    async def get_realm_checkpoint(self) -> int:
        return await trio.to_thread.run_sync(self.sync_instance.get_realm_checkpoint)

    async def update_realm_checkpoint(
        self, new_checkpoint: int, change_vlobs: dict[EntryID, int]
    ) -> None:
        return await trio.to_thread.run_sync(
            self.sync_instance.update_realm_checkpoint, new_checkpoint, change_vlobs
        )

    async def get_need_sync_entries(self) -> tuple[set[EntryID], set[EntryID]]:
        return await trio.to_thread.run_sync(self.sync_instance.get_need_sync_entries)

    async def run_vacuum(self) -> None:
        return await trio.to_thread.run_sync(self.sync_instance.run_vacuum)

    async def is_manifest_cache_ahead_of_persistance(self, entry_id: EntryID) -> bool:
        return await trio.to_thread.run_sync(
            self.sync_instance.is_manifest_cache_ahead_of_persistance, entry_id
        )

    # Timestamped workspace

    def to_timestamped(self, timestamp: DateTime) -> "WorkspaceStorageSnapshot":
        return WorkspaceStorageSnapshot(self.sync_instance, timestamp)


class WorkspaceStorageSnapshot:
    """Timestamped version to access a local storage as it was at a given timestamp

    That includes:
    - another cache in memory for fast access to deserialized data
    - the timestamped persistent storage to keep serialized data on the disk :
      vlobs are in common, not manifests. Actually only vlobs are used, manifests are mocked
    - the same lock mechanism to protect against race conditions, although it is useless there
    """

    def __init__(
        self,
        workspace_storage: _SyncWorkspaceStorage | _SyncWorkspaceStorageSnapshot,
        timestamp: DateTime,
    ):
        assert isinstance(workspace_storage, (_SyncWorkspaceStorage, _SyncWorkspaceStorageSnapshot))

        if isinstance(workspace_storage, _SyncWorkspaceStorage):
            sync_instance = _SyncWorkspaceStorageSnapshot(workspace_storage)
        elif isinstance(workspace_storage, _SyncWorkspaceStorageSnapshot):
            sync_instance = workspace_storage

        self.timestamp = timestamp
        self.sync_instance: _SyncWorkspaceStorageSnapshot = sync_instance
        # Locking structures
        self.locking_tasks: dict[EntryID, trio.lowlevel.Task] = {}
        self.entry_locks: dict[EntryID, trio.Lock] = defaultdict(trio.Lock)

    @property
    def device(self) -> LocalDevice:
        return self.sync_instance.device

    @property
    def workspace_id(self) -> EntryID:
        return self.sync_instance.workspace_id

    async def get_chunk(self, chunk_id: ChunkID) -> bytes:
        return await trio.to_thread.run_sync(self.sync_instance.get_chunk, chunk_id)

    async def set_chunk(self, chunk_id: ChunkID, block: bytes) -> NoReturn:
        self._throw_permission_error()

    async def clear_chunk(self, chunk_id: ChunkID, miss_ok: bool = False) -> NoReturn:
        self._throw_permission_error()

    async def clear_manifest(self, entry_id: EntryID) -> NoReturn:
        self._throw_permission_error()

    async def run_vacuum(self) -> NoReturn:
        self._throw_permission_error()

    async def get_need_sync_entries(self) -> NoReturn:
        self._throw_permission_error()

    async def get_realm_checkpoint(self) -> NoReturn:
        self._throw_permission_error()

    async def clear_memory_cache(self, flush: bool = True) -> NoReturn:
        self._throw_permission_error()

    async def update_realm_checkpoint(
        self, new_checkpoint: int, changed_vlobs: dict[EntryID, int]
    ) -> NoReturn:
        self._throw_permission_error()

    def _throw_permission_error(self) -> NoReturn:
        raise FSError("Not implemented : WorkspaceStorage is timestamped")

    # Manifest interface

    def get_workspace_manifest(self) -> LocalWorkspaceManifest:
        """
        Raises nothing, workspace manifest is guaranteed to be always available
        """
        return self.sync_instance.get_workspace_manifest()

    async def get_manifest(self, entry_id: EntryID) -> AnyLocalManifest:
        """Raises: FSLocalMissError"""
        manifest = await trio.to_thread.run_sync(self.sync_instance.get_manifest, entry_id)
        return manifest

    async def set_manifest(
        self,
        entry_id: EntryID,
        manifest: AnyLocalManifest,
        cache_only: bool = False,
        check_lock_status: bool = True,
        removed_ids: set[ChunkID] | None = None,
    ) -> None:
        assert isinstance(entry_id, EntryID)
        if manifest.need_sync:
            self._throw_permission_error()
        self._check_lock_status(entry_id)
        return await trio.to_thread.run_sync(self.sync_instance.set_manifest, entry_id, manifest)

    async def ensure_manifest_persistent(self, entry_id: EntryID) -> None:
        pass

    # Block interface

    async def set_clean_block(self, block_id: BlockID, block: bytes) -> None:
        return await trio.to_thread.run_sync(self.sync_instance.set_clean_block, block_id, block)

    async def clear_clean_block(self, block_id: BlockID) -> None:
        return await trio.to_thread.run_sync(self.sync_instance.clear_clean_block, block_id)

    async def get_dirty_block(self, block_id: BlockID) -> bytes:
        return await trio.to_thread.run_sync(self.sync_instance.get_dirty_block, block_id)

    # File management interface

    def create_file_descriptor(self, manifest: LocalFileManifest) -> FileDescriptor:
        return cast(FileDescriptor, self.sync_instance.create_file_descriptor(manifest))

    async def load_file_descriptor(self, fd: FileDescriptor) -> LocalFileManifest:
        manifest = await trio.to_thread.run_sync(self.sync_instance.load_file_descriptor, fd)
        return manifest

    def remove_file_descriptor(self, fd: FileDescriptor) -> None:
        return self.sync_instance.remove_file_descriptor(fd)

    # Locking helpers

    @asynccontextmanager
    async def lock_entry_id(self, entry_id: EntryID) -> AsyncIterator[EntryID]:
        async with self.entry_locks[entry_id]:
            try:
                self.locking_tasks[entry_id] = trio.lowlevel.current_task()
                yield entry_id
            finally:
                del self.locking_tasks[entry_id]

    @asynccontextmanager
    async def lock_manifest(self, entry_id: EntryID) -> AsyncIterator[AnyLocalManifest]:
        async with self.lock_entry_id(entry_id):
            yield await self.get_manifest(entry_id)

    def _check_lock_status(self, entry_id: EntryID) -> None:
        task = self.locking_tasks.get(entry_id)
        if task != trio.lowlevel.current_task():
            raise RuntimeError(f"Entry `{entry_id.hex}` modified without being locked")

    # Prevent sync pattern interface

    async def set_prevent_sync_pattern(self, pattern: Regex) -> None:
        raise NotImplementedError

    async def mark_prevent_sync_pattern_fully_applied(self, pattern: Regex) -> None:
        raise NotImplementedError

    async def get_local_chunk_ids(self, chunk_id: list[ChunkID]) -> list[ChunkID]:
        raise NotImplementedError

    async def get_local_block_ids(self, chunk_id: list[ChunkID]) -> list[ChunkID]:
        raise NotImplementedError

    async def clear_local_cache(self) -> None:
        return await trio.to_thread.run_sync(self.sync_instance.clear_local_cache)

    def get_prevent_sync_pattern(self) -> Regex:
        return self.sync_instance.get_prevent_sync_pattern()

    def get_prevent_sync_pattern_fully_applied(self) -> bool:
        return self.sync_instance.get_prevent_sync_pattern_fully_applied()

    def to_timestamped(self, timestamp: DateTime) -> "WorkspaceStorageSnapshot":
        wss = WorkspaceStorageSnapshot(self.sync_instance.to_timestamp(), timestamp)
        return wss
