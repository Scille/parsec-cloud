# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import functools
from collections import defaultdict
from contextlib import asynccontextmanager
from pathlib import Path
from typing import TYPE_CHECKING, AsyncIterator, Awaitable, Callable, NoReturn, Type, TypeVar, Union

import trio
from structlog import get_logger
from trio_typing import TaskStatus
from typing_extensions import Concatenate, ParamSpec

from parsec._parsec import (
    BlockID,
    ChunkID,
    DateTime,
    EntryID,
    LocalDevice,
    LocalFileManifest,
    LocalFolderManifest,
    LocalWorkspaceManifest,
    Regex,
)
from parsec._parsec import WorkspaceStorage as _RsWorkspaceStorage
from parsec._parsec import WorkspaceStorageSnapshot as _RsWorkspaceStorageSnapshot
from parsec._parsec import (
    workspace_storage_non_speculative_init as _rs_workspace_storage_non_speculative_init,
)
from parsec.core.fs.exceptions import (
    FSError,
    FSLocalStorageClosedError,
    FSLocalStorageOperationalError,
)
from parsec.core.types import FileDescriptor
from parsec.core.types.manifest import AnyLocalManifest
from parsec.utils import open_service_nursery

logger = get_logger()


if TYPE_CHECKING:
    from parsec._parsec import PseudoFileDescriptor
else:
    PseudoFileDescriptor = Type["PseudoFileDescriptor"]

DEFAULT_CHUNK_VACUUM_THRESHOLD = 512 * 1024 * 1024

FAILSAFE_PATTERN_FILTER = Regex.from_regex_str(
    r"^\b$"
)  # Matches nothing (https://stackoverflow.com/a/2302992/2846140)

AnyWorkspaceStorage = Union["WorkspaceStorage", "WorkspaceStorageSnapshot"]

__all__ = [
    "WorkspaceStorage",
    "AnyWorkspaceStorage",
]

P = ParamSpec("P")
R = TypeVar("R")


async def workspace_storage_non_speculative_init(
    data_base_dir: Path,
    device: LocalDevice,
    workspace_id: EntryID,
) -> None:
    # We need to shield the call to the rust function because during the call,
    # It will open a connection to the database and close it at the end.
    # And if we were cancelled we would leak a database connection.
    with trio.CancelScope(shield=True):
        return await _rs_workspace_storage_non_speculative_init(
            data_base_dir=data_base_dir,
            device=device,
            workspace_id=workspace_id,
        )


def manage_operational_error(
    func: Callable[Concatenate[WorkspaceStorage, P], Awaitable[R]]
) -> Callable[Concatenate[WorkspaceStorage, P], Awaitable[R]]:
    @functools.wraps(func)
    async def wrapper(self: WorkspaceStorage, *args: P.args, **kwargs: P.kwargs) -> R:
        if not hasattr(self, "rs_instance"):
            raise FSLocalStorageClosedError("Service is closed")
        try:
            return await func(self, *args, **kwargs)
        except FSLocalStorageOperationalError as exc:
            print(
                f"[{manage_operational_error.__name__}] local storage op error: `{exc}`({type(exc).__name__})"
            )
            await self._send_abort(exc)
            raise exc

    return wrapper


class WorkspaceStorage:
    def __init__(self, rs_instance: _RsWorkspaceStorage):
        self.rs_instance = rs_instance
        # Locking structures
        self.locking_tasks: dict[EntryID, trio.lowlevel.Task] = {}
        self.entry_locks: dict[EntryID, trio.Lock] = defaultdict(trio.Lock)

    @property
    def device(self) -> LocalDevice:
        return self.rs_instance.device

    @property
    def workspace_id(self) -> EntryID:
        return self.rs_instance.workspace_id

    @asynccontextmanager
    async def _service_abort_context(self) -> AsyncIterator[None]:
        async def _service_abort_task(
            task_status: TaskStatus[trio.MemorySendChannel[Exception]] = trio.TASK_STATUS_IGNORED,
        ) -> None:
            send, receive = trio.open_memory_channel[Exception](0)
            task_status.started(send)
            async with receive:
                async for item in receive:
                    raise item

        async with open_service_nursery() as nursery:
            async with await nursery.start(_service_abort_task) as self._abort_service_send_channel:
                yield
            nursery.cancel_scope.cancel()

    async def _send_abort(self, exception: Exception) -> None:
        # Send the exception to be raised in the `run` context
        try:
            assert self._abort_service_send_channel is not None
            await self._abort_service_send_channel.send(exception)
        # The service has already exited
        except trio.BrokenResourceError:
            pass

    @classmethod
    @asynccontextmanager
    async def run(
        cls,
        data_base_dir: Path,
        device: LocalDevice,
        workspace_id: EntryID,
        prevent_sync_pattern: Regex = FAILSAFE_PATTERN_FILTER,
        data_vacuum_threshold: int | None = None,
        cache_size: int | None = None,
    ) -> AsyncIterator[WorkspaceStorage]:
        # We shield the initialization of the rust instance.
        # During the init phase we open the connections to the database in the tokio runner.
        # If at that moment we were canceled by trio, we would leak those database connections.
        with trio.CancelScope(shield=True):
            rs_instance = await _RsWorkspaceStorage.new(
                data_base_dir=data_base_dir,
                device=device,
                workspace_id=workspace_id,
                prevent_sync_pattern=prevent_sync_pattern,
                cache_size=cache_size,
                data_vacuum_threshold=data_vacuum_threshold,
            )

        try:
            instance = cls(rs_instance)

            async with instance._service_abort_context():
                # Load "prevent sync" pattern
                await instance.set_prevent_sync_pattern(prevent_sync_pattern)

                yield instance
        finally:
            # Dirty canary to ensure no concurrent operation could be run by a rogue
            # coroutine (which may add need-to-be-flushed data to cache right after
            # `clear_memory_cache` as been called -__-')
            # In theory structured concurrency prevent us from failling into this trap
            # however better be extra-careful !
            instance.rs_instance = None  # type: ignore[assignment]

            with trio.CancelScope(shield=True):
                await rs_instance.clear_memory_cache(flush=True)
                await rs_instance.close_connections()

    # Helpers

    @manage_operational_error
    async def clear_memory_cache(self, flush: bool = True) -> None:
        return await self.rs_instance.clear_memory_cache(flush)

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
        return FileDescriptor(self.rs_instance.create_file_descriptor(manifest))

    @manage_operational_error
    async def load_file_descriptor(self, fd: PseudoFileDescriptor) -> LocalFileManifest:
        return await self.rs_instance.load_file_descriptor(fd)

    def remove_file_descriptor(self, fd: int) -> None:
        return self.rs_instance.remove_file_descriptor(fd)

    # Block interface

    @manage_operational_error
    async def set_clean_block(self, block_id: BlockID, blocks: bytes) -> set[BlockID]:
        return await self.rs_instance.set_clean_block(block_id, blocks)

    @manage_operational_error
    async def clear_clean_block(self, block_id: BlockID) -> None:
        return await self.rs_instance.clear_clean_block(block_id)

    @manage_operational_error
    async def get_dirty_block(self, block_id: BlockID) -> bytes:
        return await self.rs_instance.get_dirty_block(block_id)

    @manage_operational_error
    async def is_clean_block(self, block_id: BlockID) -> bool:
        return await self.rs_instance.is_clean_block(block_id)

    # Chunk interface

    @manage_operational_error
    async def get_chunk(self, chunk_id: ChunkID) -> bytes:
        return await self.rs_instance.get_chunk(chunk_id)

    @manage_operational_error
    async def set_chunk(self, chunk_id: ChunkID, block: bytes) -> None:
        return await self.rs_instance.set_chunk(chunk_id, block)

    @manage_operational_error
    async def clear_chunk(self, chunk_id: ChunkID, miss_ok: bool = False) -> None:
        return await self.rs_instance.clear_chunk(chunk_id, miss_ok)

    # "Prevent sync" pattern interface

    def get_prevent_sync_pattern(self) -> Regex:
        return self.rs_instance.get_prevent_sync_pattern()

    def get_prevent_sync_pattern_fully_applied(self) -> bool:
        return self.rs_instance.get_prevent_sync_pattern_fully_applied()

    @manage_operational_error
    async def set_prevent_sync_pattern(self, pattern: Regex) -> None:
        """set the "prevent sync" pattern for the corresponding workspace

        This operation is idempotent,
        i.e it does not reset the `fully_applied` flag if the pattern hasn't changed.
        """
        return await self.rs_instance.set_prevent_sync_pattern(pattern)

    @manage_operational_error
    async def mark_prevent_sync_pattern_fully_applied(self, pattern: Regex) -> None:
        """Mark the provided pattern as fully applied.

        This is meant to be called after one made sure that all the manifests in the
        workspace are compliant with the new pattern. The applied pattern is provided
        as an argument in order to avoid concurrency issues.
        """
        return await self.rs_instance.mark_prevent_sync_pattern_fully_applied(pattern)

    async def get_local_chunk_ids(self, chunk_ids: list[ChunkID]) -> tuple[ChunkID, ...]:
        return await self.rs_instance.get_local_chunk_ids(chunk_ids)

    async def get_local_block_ids(self, chunk_ids: list[ChunkID]) -> tuple[ChunkID, ...]:
        return await self.rs_instance.get_local_block_ids(chunk_ids)

    # Manifest interface

    def get_workspace_manifest(self) -> LocalWorkspaceManifest:
        return self.rs_instance.get_workspace_manifest()

    @manage_operational_error
    async def get_manifest(self, entry_id: EntryID) -> AnyLocalManifest:
        return await self.rs_instance.get_manifest(entry_id)

    @manage_operational_error
    async def set_manifest(
        self,
        entry_id: EntryID,
        manifest: LocalFileManifest | LocalFolderManifest,
        cache_only: bool = False,
        check_lock_status: bool = True,
        removed_ids: set[ChunkID] | None = None,
    ) -> None:
        if check_lock_status:
            self._check_lock_status(entry_id)
        await self.rs_instance.set_manifest(entry_id, manifest, cache_only, removed_ids)

    @manage_operational_error
    async def set_workspace_manifest(
        self,
        manifest: LocalWorkspaceManifest,
    ) -> None:
        self._check_lock_status(manifest.id)
        return await self.rs_instance.set_workspace_manifest(manifest)

    @manage_operational_error
    async def ensure_manifest_persistent(self, entry_id: EntryID) -> None:
        self._check_lock_status(entry_id)
        await self.rs_instance.ensure_manifest_persistent(entry_id)

    @manage_operational_error
    async def clear_manifest(self, entry_id: EntryID) -> None:
        self._check_lock_status(entry_id)
        await self.rs_instance.clear_manifest(entry_id)

    # Checkpoint interface

    @manage_operational_error
    async def get_realm_checkpoint(self) -> int:
        return await self.rs_instance.get_realm_checkpoint()

    @manage_operational_error
    async def update_realm_checkpoint(
        self, new_checkpoint: int, change_vlobs: dict[EntryID, int]
    ) -> None:
        return await self.rs_instance.update_realm_checkpoint(new_checkpoint, change_vlobs)

    async def get_need_sync_entries(self) -> tuple[set[EntryID], set[EntryID]]:
        return await self.rs_instance.get_need_sync_entries()

    @manage_operational_error
    async def run_vacuum(self) -> None:
        return await self.rs_instance.run_vacuum()

    def is_manifest_cache_ahead_of_persistance(self, entry_id: EntryID) -> bool:
        return self.rs_instance.is_manifest_cache_ahead_of_persistance(entry_id)

    # Timestamped workspace

    def to_timestamped(self, timestamp: DateTime) -> "WorkspaceStorageSnapshot":
        return WorkspaceStorageSnapshot(self.rs_instance, timestamp)

    def is_block_remanent(self) -> bool:
        return self.rs_instance.is_block_remanent()

    async def enable_block_remanence(self) -> bool:
        return await self.rs_instance.enable_block_remanence()

    async def disable_block_remanence(self) -> set[BlockID] | None:
        return await self.rs_instance.disable_block_remanence()

    async def remove_clean_blocks(self, block_ids: list[BlockID]) -> None:
        return await self.rs_instance.remove_clean_blocks(block_ids)

    async def clear_unreferenced_blocks(
        self, block_ids: list[BlockID], not_accessed_after: DateTime
    ) -> None:
        return await self.rs_instance.clear_unreferenced_blocks(block_ids, not_accessed_after)


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
        workspace_storage: _RsWorkspaceStorage | _RsWorkspaceStorageSnapshot,
        timestamp: DateTime,
    ):
        assert isinstance(workspace_storage, (_RsWorkspaceStorage, _RsWorkspaceStorageSnapshot))

        if isinstance(workspace_storage, _RsWorkspaceStorage):
            rs_instance = workspace_storage.to_timestamp()
        elif isinstance(workspace_storage, _RsWorkspaceStorageSnapshot):
            rs_instance = workspace_storage

        self.timestamp = timestamp
        self.rs_instance: _RsWorkspaceStorageSnapshot = rs_instance
        # Locking structures
        self.locking_tasks: dict[EntryID, trio.lowlevel.Task] = {}
        self.entry_locks: dict[EntryID, trio.Lock] = defaultdict(trio.Lock)

    @property
    def device(self) -> LocalDevice:
        return self.rs_instance.device

    @property
    def workspace_id(self) -> EntryID:
        return self.rs_instance.workspace_id

    async def get_chunk(self, chunk_id: ChunkID) -> bytes:
        return await self.rs_instance.get_chunk(chunk_id)

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
        raise FSError("Not implemented : WorkspaceStorage is timestamped")

    async def get_manifest(self, entry_id: EntryID) -> AnyLocalManifest:
        """Raises: FSLocalMissError"""
        return await self.rs_instance.get_manifest(entry_id)

    async def set_manifest(
        self,
        entry_id: EntryID,
        manifest: LocalFileManifest | LocalFolderManifest,
        cache_only: bool = False,
        check_lock_status: bool = True,
        removed_ids: set[ChunkID] | None = None,
    ) -> None:
        assert isinstance(entry_id, EntryID)
        if manifest.need_sync:
            self._throw_permission_error()
        self._check_lock_status(entry_id)
        return await self.rs_instance.set_manifest(entry_id, manifest)

    async def set_workspace_manifest(
        self,
        _manifest: LocalWorkspaceManifest,
    ) -> None:
        """
        We do nothing here, because we have nothing to do ;)
        We need to have a blanked implementation of `set_workspace_manifest` that is alike `WorkspaceStorage::set_workspace_manifest`.
        Because this method will be used by `SyncTransaction::apply_prevent_sync_pattern`.
        """
        pass

    async def ensure_manifest_persistent(self, entry_id: EntryID) -> None:
        pass

    # Block interface

    async def set_clean_block(self, block_id: BlockID, block: bytes) -> set[BlockID]:
        return await self.rs_instance.set_clean_block(block_id, block)

    async def clear_clean_block(self, block_id: BlockID) -> None:
        return await self.rs_instance.clear_clean_block(block_id)

    async def get_dirty_block(self, block_id: BlockID) -> bytes:
        return await self.rs_instance.get_dirty_block(block_id)

    # File management interface

    def create_file_descriptor(self, manifest: LocalFileManifest) -> FileDescriptor:
        return FileDescriptor(self.rs_instance.create_file_descriptor(manifest))

    async def load_file_descriptor(self, fd: FileDescriptor) -> LocalFileManifest:
        return await self.rs_instance.load_file_descriptor(fd)

    def remove_file_descriptor(self, fd: FileDescriptor) -> None:
        return self.rs_instance.remove_file_descriptor(fd)

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
        return await self.rs_instance.clear_local_cache()

    def get_prevent_sync_pattern(self) -> Regex:
        return self.rs_instance.get_prevent_sync_pattern()

    def get_prevent_sync_pattern_fully_applied(self) -> bool:
        return self.rs_instance.get_prevent_sync_pattern_fully_applied()

    def to_timestamped(self, timestamp: DateTime) -> "WorkspaceStorageSnapshot":
        wss = WorkspaceStorageSnapshot(self.rs_instance.to_timestamp(), timestamp)
        return wss

    def is_block_remanent(self) -> bool:
        return self.rs_instance.is_block_remanent()

    async def enable_block_remanence(self) -> bool:
        return await self.rs_instance.enable_block_remanence()

    async def disable_block_remanence(self) -> set[BlockID] | None:
        return await self.rs_instance.disable_block_remanence()

    async def remove_clean_blocks(self, block_ids: list[BlockID]) -> None:
        return await self.rs_instance.remove_clean_blocks(block_ids)

    async def is_clean_block(self, block_id: BlockID) -> bool:
        return await self.rs_instance.is_clean_block(block_id)

    async def clear_unreferenced_blocks(
        self, block_ids: list[BlockID], not_accessed_after: DateTime
    ) -> None:
        return await self.rs_instance.clear_unreferenced_blocks(block_ids, not_accessed_after)
