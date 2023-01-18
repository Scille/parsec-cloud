# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import secrets
import time
from collections import defaultdict
from contextlib import asynccontextmanager
from pathlib import Path
from typing import AsyncContextManager, AsyncIterator, NoReturn, TypeVar, cast

import trio
from structlog import get_logger
from trio import lowlevel

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
from parsec.core.config import DEFAULT_WORKSPACE_STORAGE_CACHE_SIZE
from parsec.core.fs.exceptions import (
    FSError,
    FSInvalidFileDescriptor,
    FSLocalMissError,
    FSLocalStorageClosedError,
)
from parsec.core.types import DEFAULT_BLOCK_SIZE, FileDescriptor
from parsec.core.types.manifest import AnyLocalManifest
from tests.core.fs.old_storage.local_database import Cursor, LocalDatabase
from tests.core.fs.old_storage.manifest_storage import ManifestStorage
from tests.core.fs.old_storage.version import (
    get_workspace_cache_storage_db_path,
    get_workspace_data_storage_db_path,
)

logger = get_logger()

DEFAULT_CHUNK_VACUUM_THRESHOLD = 512 * 1024 * 1024

FAILSAFE_PATTERN_FILTER = Regex.from_regex_str(
    r"^\b$"
)  # Matches nothing (https://stackoverflow.com/a/2302992/2846140)


async def workspace_storage_non_speculative_init(
    data_base_dir: Path, device: LocalDevice, workspace_id: EntryID, timestamp: DateTime
) -> None:
    db_path = get_workspace_data_storage_db_path(data_base_dir, device, workspace_id)

    # Local data storage service
    async with LocalDatabase.run(db_path) as data_localdb:

        # Manifest storage service
        async with ManifestStorage.run(device, data_localdb, workspace_id) as manifest_storage:

            manifest = LocalWorkspaceManifest.new_placeholder(
                author=device.device_id, id=workspace_id, timestamp=timestamp, speculative=False
            )
            await manifest_storage.set_manifest(workspace_id, manifest)


class BaseWorkspaceStorage:
    """Common base class for WorkspaceStorage and WorkspaceStorageTimestamped
    Can not be instantiated
    """

    def __init__(
        self,
        device: LocalDevice,
        workspace_id: EntryID,
        block_storage: ChunkStorage,
        chunk_storage: ChunkStorage,
    ):
        self.device = device
        self.device_id = device.device_id
        self.workspace_id = workspace_id

        # File descriptors
        self.open_fds: dict[FileDescriptor, EntryID] = {}
        self.fd_counter = 0

        # Locking structures
        self.locking_tasks: dict[EntryID, lowlevel.Task] = {}
        self.entry_locks: dict[EntryID, trio.Lock] = defaultdict(trio.Lock)

        # Manifest and block storage
        self.block_storage = block_storage
        self.chunk_storage = chunk_storage

        # Pattern attributes
        # set by `_load_prevent_sync_pattern` in WorkspaceStorage.run()
        self._prevent_sync_pattern: Regex
        self._prevent_sync_pattern_fully_applied: bool

    def _get_next_fd(self) -> FileDescriptor:
        self.fd_counter += 1
        return FileDescriptor(self.fd_counter)

    # Manifest interface

    def get_workspace_manifest(self) -> LocalWorkspaceManifest:
        """
        Raises nothing, workspace manifest is guaranteed to be always available
        """
        raise NotImplementedError

    async def get_manifest(self, entry_id: EntryID) -> AnyLocalManifest:
        raise NotImplementedError

    async def set_manifest(
        self,
        entry_id: EntryID,
        manifest: AnyLocalManifest,
        cache_only: bool = False,
        check_lock_status: bool = True,
        removed_ids: set[ChunkID] | None = None,
    ) -> None:
        raise NotImplementedError

    async def ensure_manifest_persistent(self, entry_id: EntryID) -> None:
        raise NotImplementedError

    # Prevent sync pattern interface

    async def set_prevent_sync_pattern(self, pattern: Regex) -> None:
        raise NotImplementedError

    async def mark_prevent_sync_pattern_fully_applied(self, pattern: Regex) -> None:
        raise NotImplementedError

    async def get_local_chunk_ids(self, chunk_id: list[ChunkID]) -> list[ChunkID]:
        raise NotImplementedError

    async def get_local_block_ids(self, chunk_id: list[ChunkID]) -> list[ChunkID]:
        raise NotImplementedError

    # Locking helpers

    @asynccontextmanager
    async def lock_entry_id(self, entry_id: EntryID) -> AsyncIterator[EntryID]:
        async with self.entry_locks[entry_id]:
            try:
                self.locking_tasks[entry_id] = lowlevel.current_task()
                yield entry_id
            finally:
                del self.locking_tasks[entry_id]

    @asynccontextmanager
    async def lock_manifest(self, entry_id: EntryID) -> AsyncIterator[AnyLocalManifest]:
        async with self.lock_entry_id(entry_id):
            yield await self.get_manifest(entry_id)

    def _check_lock_status(self, entry_id: EntryID) -> None:
        task = self.locking_tasks.get(entry_id)
        if task != lowlevel.current_task():
            raise RuntimeError(f"Entry `{entry_id.hex}` modified without being locked")

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

    # Block interface

    async def set_clean_block(self, block_id: BlockID, block: bytes) -> None:
        assert isinstance(block_id, BlockID)
        return await self.block_storage.set_chunk(ChunkID.from_block_id(block_id), block)

    async def clear_clean_block(self, block_id: BlockID) -> None:
        assert isinstance(block_id, BlockID)
        try:
            await self.block_storage.clear_chunk(ChunkID.from_block_id(block_id))
        except FSLocalMissError:
            pass

    async def get_dirty_block(self, block_id: BlockID) -> bytes:
        return await self.chunk_storage.get_chunk(ChunkID.from_block_id(block_id))

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

    # "Prevent sync" pattern interface

    def get_prevent_sync_pattern(self) -> Regex:
        return self._prevent_sync_pattern

    def get_prevent_sync_pattern_fully_applied(self) -> bool:
        return self._prevent_sync_pattern_fully_applied

    # Timestamped workspace

    def to_timestamped(self, timestamp: DateTime) -> "WorkspaceStorageTimestamped":
        return WorkspaceStorageTimestamped(self, timestamp)


class WorkspaceStorage(BaseWorkspaceStorage):
    """Manage the access to the local storage.

    That includes:
    - a cache in memory for fast access to deserialized data
    - the persistent storage to keep serialized data on the disk
    - a lock mechanism to protect against race conditions
    """

    def __init__(
        self,
        device: LocalDevice,
        workspace_id: EntryID,
        data_localdb: LocalDatabase,
        cache_localdb: LocalDatabase,
        block_storage: ChunkStorage,
        chunk_storage: ChunkStorage,
        manifest_storage: ManifestStorage,
    ):
        super().__init__(device, workspace_id, block_storage, chunk_storage)
        self.data_localdb = data_localdb
        self.cache_localdb = cache_localdb
        self.manifest_storage = manifest_storage

    @classmethod
    @asynccontextmanager
    async def run(
        cls,
        data_base_dir: Path,
        device: LocalDevice,
        workspace_id: EntryID,
        prevent_sync_pattern: Regex = FAILSAFE_PATTERN_FILTER,
        cache_size: int = DEFAULT_WORKSPACE_STORAGE_CACHE_SIZE,
        data_vacuum_threshold: int = DEFAULT_CHUNK_VACUUM_THRESHOLD,
    ) -> AsyncIterator["WorkspaceStorage"]:
        data_path = get_workspace_data_storage_db_path(data_base_dir, device, workspace_id)
        cache_path = get_workspace_cache_storage_db_path(data_base_dir, device, workspace_id)

        # The cache database usually doesn't require vacuuming as it already has a maximum size.
        # However, vacuuming might still be necessary after a change in the configuration.
        # The cache size plus 10% seems like a reasonable configuration to avoid false positive.
        cache_localdb_vacuum_threshold = int(cache_size * 1.1)

        # Local cache storage service
        async with LocalDatabase.run(
            cache_path, vacuum_threshold=cache_localdb_vacuum_threshold
        ) as cache_localdb:

            # Local data storage service
            async with LocalDatabase.run(
                data_path, vacuum_threshold=data_vacuum_threshold
            ) as data_localdb:

                # Block storage service
                async with BlockStorage.run(
                    device, cache_localdb, cache_size=cache_size
                ) as block_storage:

                    # Clean up block storage and run vacuum if necessary
                    # (e.g after changing the cache size)
                    await block_storage.cleanup()
                    await cache_localdb.run_vacuum()

                    # Manifest storage service
                    async with ManifestStorage.run(
                        device, data_localdb, workspace_id
                    ) as manifest_storage:

                        # Chunk storage service
                        async with ChunkStorage.run(device, data_localdb) as chunk_storage:

                            # Instantiate workspace storage
                            instance = cls(
                                device,
                                workspace_id,
                                data_localdb=data_localdb,
                                cache_localdb=cache_localdb,
                                block_storage=block_storage,
                                chunk_storage=chunk_storage,
                                manifest_storage=manifest_storage,
                            )

                            # Populate the cache with the workspace manifest to be able to
                            # access it synchronously at all time
                            await instance._load_workspace_manifest()
                            assert instance.workspace_id in instance.manifest_storage._cache

                            # Load "prevent sync" pattern
                            await instance.set_prevent_sync_pattern(prevent_sync_pattern)

                            # Yield point
                            yield instance

    # Helpers

    async def clear_memory_cache(self, flush: bool = True) -> None:
        await self.manifest_storage.clear_memory_cache(flush=flush)

    # Checkpoint interface

    async def get_realm_checkpoint(self) -> int:
        return await self.manifest_storage.get_realm_checkpoint()

    async def update_realm_checkpoint(
        self, new_checkpoint: int, changed_vlobs: dict[EntryID, int]
    ) -> None:
        """
        Raises: Nothing !
        """
        await self.manifest_storage.update_realm_checkpoint(new_checkpoint, changed_vlobs)

    async def get_need_sync_entries(self) -> tuple[set[EntryID], set[EntryID]]:
        return await self.manifest_storage.get_need_sync_entries()

    # Manifest interface

    async def _load_workspace_manifest(self) -> None:
        try:
            await self.manifest_storage.get_manifest(self.workspace_id)

        except FSLocalMissError:
            # It is possible to lack the workspace manifest in local if our
            # device hasn't tried to access it yet (and we are not the creator
            # of the workspace, in which case the workspacefs local db is
            # initialized with a non-speculative local manifest placeholder).
            # In such case it is easy to fall back on an empty manifest
            # which is a good enough approximation of the very first version
            # of the manifest (field `created` is invalid, but it will be
            # correction by the merge during sync).
            # This approach also guarantees the workspace root folder is always
            # consistent (ls/touch/mkdir always works on it), which is not the
            # case for the others files and folders (as their access may
            # require communication with the backend).
            # This is especially important when the workspace is accessed from
            # file system mountpoint given having a weird error popup when clicking
            # on the mountpoint from the file explorer really feel like a bug :/
            timestamp = self.device.timestamp()
            manifest = LocalWorkspaceManifest.new_placeholder(
                author=self.device.device_id,
                id=self.workspace_id,
                timestamp=timestamp,
                speculative=True,
            )
            await self.manifest_storage.set_manifest(self.workspace_id, manifest)

    def get_workspace_manifest(self) -> LocalWorkspaceManifest:
        """
        Raises nothing, workspace manifest is guaranteed to be always available
        """
        return cast(LocalWorkspaceManifest, self.manifest_storage._cache[self.workspace_id])

    async def get_manifest(self, entry_id: EntryID) -> AnyLocalManifest:
        """Raises: FSLocalMissError"""
        return await self.manifest_storage.get_manifest(entry_id)

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
        await self.manifest_storage.set_manifest(
            entry_id, manifest, cache_only=cache_only, removed_ids=removed_ids
        )

    async def ensure_manifest_persistent(self, entry_id: EntryID) -> None:
        self._check_lock_status(entry_id)
        await self.manifest_storage.ensure_manifest_persistent(entry_id)

    async def clear_manifest(self, entry_id: EntryID) -> None:
        self._check_lock_status(entry_id)
        await self.manifest_storage.clear_manifest(entry_id)

    # "Prevent sync" pattern interface

    async def set_prevent_sync_pattern(self, pattern: Regex) -> None:
        """set the "prevent sync" pattern for the corresponding workspace

        This operation is idempotent,
        i.e it does not reset the `fully_applied` flag if the pattern hasn't changed.
        """
        self._prevent_sync_pattern = pattern
        self._prevent_sync_pattern_fully_applied = (
            await self.manifest_storage.set_prevent_sync_pattern(pattern)
        )

    async def mark_prevent_sync_pattern_fully_applied(self, pattern: Regex) -> None:
        """Mark the provided pattern as fully applied.

        This is meant to be called after one made sure that all the manifests in the
        workspace are compliant with the new pattern. The applied pattern is provided
        as an argument in order to avoid concurrency issues.
        """
        self._prevent_sync_pattern_fully_applied = (
            await self.manifest_storage.mark_prevent_sync_pattern_fully_applied(pattern)
        )

    async def get_local_chunk_ids(self, chunk_id: list[ChunkID]) -> list[ChunkID]:
        return await self.chunk_storage.get_local_chunk_ids(chunk_id)

    async def get_local_block_ids(self, chunk_id: list[ChunkID]) -> list[ChunkID]:
        return await self.block_storage.get_local_chunk_ids(chunk_id)

    # Vacuum

    async def run_vacuum(self) -> None:
        # Only the data storage needs to get vacuumed
        await self.data_localdb.run_vacuum()


_PyWorkspaceStorage = WorkspaceStorage


class WorkspaceStorageTimestamped(BaseWorkspaceStorage):
    """Timestamped version to access a local storage as it was at a given timestamp

    That includes:
    - another cache in memory for fast access to deserialized data
    - the timestamped persistent storage to keep serialized data on the disk :
      vlobs are in common, not manifests. Actually only vlobs are used, manifests are mocked
    - the same lock mechanism to protect against race conditions, although it is useless there
    """

    def __init__(self, workspace_storage: BaseWorkspaceStorage, timestamp: DateTime):
        super().__init__(
            workspace_storage.device,
            workspace_storage.workspace_id,
            block_storage=workspace_storage.block_storage,
            chunk_storage=workspace_storage.chunk_storage,
        )

        self._cache: dict[EntryID, AnyLocalManifest] = {}
        self.workspace_storage = workspace_storage
        self.timestamp = timestamp
        self.manifest_storage = None

        self._prevent_sync_pattern = workspace_storage._prevent_sync_pattern
        self._prevent_sync_pattern_fully_applied = (
            workspace_storage._prevent_sync_pattern_fully_applied
        )

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
        return self.workspace_storage.get_workspace_manifest()

    async def get_manifest(self, entry_id: EntryID) -> AnyLocalManifest:
        """Raises: FSLocalMissError"""
        assert isinstance(entry_id, EntryID)
        try:
            return self._cache[entry_id]
        except KeyError:
            raise FSLocalMissError(entry_id)

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
        self._cache[entry_id] = manifest

    async def ensure_manifest_persistent(self, entry_id: EntryID) -> None:
        pass

    # def to_timestamped(self, timestamp: DateTime) -> "WorkspaceStorageTimestamped":
    #     return WorkspaceStorageTimestamped(self, timestamp)


T = TypeVar("T", bound="ChunkStorage")


class ChunkStorage:
    """Interface to access the local chunks of data."""

    def __init__(self, device: LocalDevice, localdb: LocalDatabase):
        self.local_symkey = device.local_symkey
        self.localdb = localdb

    @property
    def path(self) -> Path:
        return Path(self.localdb.path)

    @classmethod
    @asynccontextmanager
    async def run(
        cls, device: LocalDevice, localdb: LocalDatabase
    ) -> AsyncIterator["ChunkStorage"]:
        async with cls(device, localdb)._run() as self:
            yield self

    @asynccontextmanager
    async def _run(self: T) -> AsyncIterator[T]:
        await self._create_db()
        try:
            yield self
        finally:
            with trio.CancelScope(shield=True):
                # Commit the pending changes in the local database
                try:
                    await self.localdb.commit()
                # Ignore storage closed exceptions, since it follows an operational error
                except FSLocalStorageClosedError:
                    pass

    def _open_cursor(self) -> AsyncContextManager[Cursor]:
        # There is no point in committing dirty chunks:
        # they are referenced by a manifest that will get committed
        # soon after them. This greatly improves the performance of
        # writing file using the mountpoint as the OS will typically
        # writes data as blocks of 4K. The manifest being kept in
        # memory during the writing, this means that the data and
        # metadata is typically not flushed to the disk until an
        # an actual flush operation is performed.
        return self.localdb.open_cursor(commit=False)

    # Database initialization

    async def _create_db(self) -> None:
        async with self._open_cursor() as cursor:
            cursor.execute(
                """CREATE TABLE IF NOT EXISTS chunks
                    (chunk_id BLOB PRIMARY KEY NOT NULL, -- UUID
                     size INTEGER NOT NULL,
                     offline INTEGER NOT NULL,  -- Boolean
                     accessed_on REAL, -- Timestamp
                     data BLOB NOT NULL
                );"""
            )

    # Size and chunks

    async def get_nb_blocks(self) -> int:
        async with self._open_cursor() as cursor:
            cursor.execute("SELECT COUNT(*) FROM chunks")
            (result,) = cursor.fetchone()
            return result

    async def get_total_size(self) -> int:
        async with self._open_cursor() as cursor:
            cursor.execute("SELECT COALESCE(SUM(size), 0) FROM chunks")
            (result,) = cursor.fetchone()
            return result

    # Generic chunk operations

    async def is_chunk(self, chunk_id: ChunkID) -> bool:
        async with self._open_cursor() as cursor:
            cursor.execute("SELECT chunk_id FROM chunks WHERE chunk_id = ?", (chunk_id.bytes,))
            manifest_row = cursor.fetchone()
        return bool(manifest_row)

    async def get_local_chunk_ids(self, chunk_id: list[ChunkID]) -> list[ChunkID]:

        bytes_id_list = [(id.bytes,) for id in chunk_id]

        async with self._open_cursor() as cursor:
            # Can't use execute many with SELECT so we have to make a temporary table filled with the needed chunk_id
            # and intersect it with the normal table
            # create random name for the temporary table to avoid asynchronous errors
            table_name = "temp" + secrets.token_hex(12)
            assert table_name.isalnum()

            cursor.execute(f"""DROP TABLE IF EXISTS {table_name}""")
            cursor.execute(
                f"""CREATE TABLE IF NOT EXISTS {table_name}
                    (chunk_id BLOB PRIMARY KEY NOT NULL -- UUID
                );"""
            )

            cursor.executemany(
                f"""INSERT OR REPLACE INTO
            {table_name} (chunk_id)
            VALUES (?)""",
                iter(bytes_id_list),
            )

            cursor.execute(
                f"""SELECT chunk_id FROM chunks INTERSECT SELECT chunk_id FROM {table_name}"""
            )

            intersect_rows = cursor.fetchall()
            cursor.execute(f"""DROP TABLE IF EXISTS {table_name}""")

        return [ChunkID.from_bytes(id_bytes) for (id_bytes,) in intersect_rows]

    async def get_chunk(self, chunk_id: ChunkID) -> bytes:
        async with self._open_cursor() as cursor:
            # Use a thread as executing a statement that modifies the content of the database might,
            # in some case, block for several hundreds of milliseconds
            await self.localdb.run_in_thread(
                cursor.execute,
                """
                UPDATE chunks SET accessed_on = ? WHERE chunk_id = ?;
                """,
                (time.time(), chunk_id.bytes),
            )
            cursor.execute("SELECT changes()")
            (changes,) = cursor.fetchone()
            if not changes:
                raise FSLocalMissError(chunk_id)
            cursor.execute("""SELECT data FROM chunks WHERE chunk_id = ?""", (chunk_id.bytes,))
            (ciphered,) = cursor.fetchone()

        return self.local_symkey.decrypt(ciphered)

    async def set_chunk(self, chunk_id: ChunkID, raw: bytes) -> None:
        ciphered = self.local_symkey.encrypt(raw)

        # Update database
        async with self._open_cursor() as cursor:
            # Use a thread as executing a statement that modifies the content of the database might,
            # in some case, block for several hundreds of milliseconds
            await self.localdb.run_in_thread(
                cursor.execute,
                """INSERT OR REPLACE INTO
                chunks (chunk_id, size, offline, accessed_on, data)
                VALUES (?, ?, ?, ?, ?)""",
                (chunk_id.bytes, len(ciphered), False, time.time(), ciphered),
            )

    async def clear_chunk(self, chunk_id: ChunkID) -> None:
        async with self._open_cursor() as cursor:
            # Use a thread as executing a statement that modifies the content of the database might,
            # in some case, block for several hundreds of milliseconds
            await self.localdb.run_in_thread(
                cursor.execute, "DELETE FROM chunks WHERE chunk_id = ?", (chunk_id.bytes,)
            )
            cursor.execute("SELECT changes()")
            (changes,) = cursor.fetchone()

        if not changes:
            raise FSLocalMissError(chunk_id)


class BlockStorage(ChunkStorage):
    """Interface for caching the data blocks."""

    def __init__(self, device: LocalDevice, localdb: LocalDatabase, cache_size: int):
        super().__init__(device, localdb)
        self.cache_size = cache_size

    @classmethod
    @asynccontextmanager
    async def run(  # type: ignore[override]
        cls, device: LocalDevice, localdb: LocalDatabase, cache_size: int
    ) -> AsyncIterator["BlockStorage"]:
        async with cls(device, localdb, cache_size)._run() as self:
            yield self

    def _open_cursor(self) -> AsyncContextManager[Cursor]:
        # It doesn't matter for blocks to be committed as soon as they're added
        # since they exists in the remote storage anyway. But it's simply more
        # convenient to perform the commit right away as it does't cost much (at
        # least compare to the downloading of the block).
        return self.localdb.open_cursor(commit=True)

    @asynccontextmanager
    async def _reenter_cursor(self, cursor: Cursor | None) -> AsyncIterator[Cursor]:
        if cursor is not None:
            yield cursor
            return
        async with self._open_cursor() as cursor:
            yield cursor

    # Garbage collection

    @property
    def block_limit(self) -> int:
        return self.cache_size // DEFAULT_BLOCK_SIZE

    async def clear_all_blocks(self) -> None:
        async with self._open_cursor() as cursor:
            cursor.execute("DELETE FROM chunks")

    # Upgraded set method

    async def set_chunk(self, chunk_id: ChunkID, raw: bytes) -> None:
        ciphered = self.local_symkey.encrypt(raw)

        # Update database
        async with self._open_cursor() as cursor:

            # Insert the chunk
            # Use a thread as executing a statement that modifies the content of the database might,
            # in some case, block for several hundreds of milliseconds
            await self.localdb.run_in_thread(
                cursor.execute,
                """INSERT OR REPLACE INTO
                chunks (chunk_id, size, offline, accessed_on, data)
                VALUES (?, ?, ?, ?, ?)""",
                (chunk_id.bytes, len(ciphered), False, time.time(), ciphered),
            )

            # Perform cleanup if necessary
            await self.cleanup(cursor)

    async def cleanup(self, cursor: Cursor | None = None) -> None:

        # Update database
        async with self._reenter_cursor(cursor) as cursor:

            # Count the chunks
            cursor.execute("SELECT COUNT(*) FROM chunks")
            (nb_blocks,) = cursor.fetchone()
            extra_blocks = nb_blocks - self.block_limit

            # No clean up is needed
            if extra_blocks <= 0:
                return

            # Remove the extra block plus 10 % of the cache size, i.e about 100 blocks
            limit = extra_blocks + self.block_limit // 10
            # Use a thread as executing a statement that modifies the content of the database might,
            # in some case, block for several hundreds of milliseconds
            await self.localdb.run_in_thread(
                cursor.execute,
                """
                DELETE FROM chunks WHERE chunk_id IN (
                    SELECT chunk_id FROM chunks ORDER BY accessed_on ASC LIMIT ?
                )
                """,
                (limit,),
            )
