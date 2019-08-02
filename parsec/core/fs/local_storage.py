# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from pathlib import Path
from collections import defaultdict

import trio
from trio import hazmat
from typing import Tuple, Dict

from pendulum import Pendulum

from structlog import get_logger
from async_generator import asynccontextmanager

from parsec.types import DeviceID
from parsec.crypto import SecretKey
from parsec.core.types import (
    EntryID,
    BlockID,
    FileDescriptor,
    LocalManifest,
    LocalFileManifest,
    local_manifest_serializer,
)
from parsec.core.fs.persistent_storage import PersistentStorage
from parsec.core.fs.exceptions import FSError, FSLocalMissError, FSInvalidFileDescriptor


logger = get_logger()


class LocalStorage:
    """Manage the access to the local storage.

    That includes:
    - a cache in memory for fast access to deserialized data
    - the persistent storage to keep serialized data on the disk
    - a lock mecanism to protect against race conditions
    """

    persistent_storage_class = PersistentStorage

    def __init__(self, device_id: DeviceID, key: SecretKey, path: Path, **kwargs):
        self.device_id = device_id

        # File descriptors
        self.open_fds: Dict[FileDescriptor, EntryID] = {}
        self.fd_counter = 0

        # Locking structures
        self.locking_tasks = {}
        self.entry_locks = defaultdict(trio.Lock)

        # Manifest and block storage
        self.cache_ahead_of_persistance_ids = set()
        self.local_manifest_cache = {}
        self.persistent_storage = self.persistent_storage_class(key, path, **kwargs)

    def _get_next_fd(self) -> FileDescriptor:
        self.fd_counter += 1
        return FileDescriptor(self.fd_counter)

    def __enter__(self):
        self.persistent_storage.__enter__()
        return self

    def __exit__(self, *args):
        for entry_id in self.cache_ahead_of_persistance_ids.copy():
            self.ensure_manifest_flushed_on_disk(entry_id)
        self.persistent_storage.__exit__(*args)

    def clear_memory_cache(self):
        self.local_manifest_cache.clear()
        self.cache_ahead_of_persistance_ids.clear()

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
            yield self.get_manifest(entry_id)

    def _check_lock_status(self, entry_id: EntryID) -> None:
        task = self.locking_tasks.get(entry_id)
        # TODO: remove `task is None` to ensure that the lock is taken
        assert task is None or task == hazmat.current_task()

    # Manifest interface

    def get_manifest(self, entry_id: EntryID) -> LocalManifest:
        """Raises: FSLocalMissError"""
        assert isinstance(entry_id, EntryID)
        try:
            return self.local_manifest_cache[entry_id]
        except KeyError:
            pass
        raw = self.persistent_storage.get_manifest(entry_id)
        manifest = local_manifest_serializer.loads(raw)
        self.local_manifest_cache[entry_id] = manifest
        return manifest

    def set_manifest(
        self,
        entry_id: EntryID,
        manifest: LocalManifest,
        cache_only: bool = False,
        check_lock_status=True,
    ) -> None:
        assert isinstance(entry_id, EntryID)
        assert manifest.author == self.device_id
        if check_lock_status:
            self._check_lock_status(entry_id)
        if not cache_only:
            raw = local_manifest_serializer.dumps(manifest)
            self.persistent_storage.set_manifest(entry_id, raw)
        else:
            self.cache_ahead_of_persistance_ids.add(entry_id)
        self.local_manifest_cache[entry_id] = manifest

    def ensure_manifest_persistant(self, entry_id: EntryID) -> None:
        assert isinstance(entry_id, EntryID)
        self._check_lock_status(entry_id)
        if entry_id not in self.cache_ahead_of_persistance_ids:
            return
        manifest = self.local_manifest_cache[entry_id]
        raw = local_manifest_serializer.dumps(manifest)
        self.persistent_storage.set_manifest(entry_id, raw)
        self.cache_ahead_of_persistance_ids.remove(entry_id)

    def clear_manifest(self, entry_id: EntryID) -> None:
        assert isinstance(entry_id, EntryID)
        self._check_lock_status(entry_id)
        try:
            self.persistent_storage.clear_manifest(entry_id)
        except FSLocalMissError:
            pass
        self.local_manifest_cache.pop(entry_id, None)
        self.cache_ahead_of_persistance_ids.discard(entry_id)

    # Block interface

    def is_dirty_block(self, block_id: BlockID):
        assert isinstance(block_id, BlockID)
        return self.persistent_storage.is_dirty_block(block_id)

    def get_block(self, block_id: BlockID) -> bytes:
        assert isinstance(block_id, BlockID)
        try:
            return self.persistent_storage.get_dirty_block(block_id)
        except FSLocalMissError:
            return self.persistent_storage.get_clean_block(block_id)

    def set_dirty_block(self, block_id: BlockID, block: bytes) -> None:
        assert isinstance(block_id, BlockID)
        return self.persistent_storage.set_dirty_block(block_id, block)

    def set_clean_block(self, block_id: BlockID, block: bytes) -> None:
        assert isinstance(block_id, BlockID)
        return self.persistent_storage.set_clean_block(block_id, block)

    def clear_block(self, block_id: BlockID) -> None:
        assert isinstance(block_id, BlockID)
        try:
            self.persistent_storage.clear_dirty_block(block_id)
        except FSLocalMissError:
            try:
                self.persistent_storage.clear_clean_block(block_id)
            except FSLocalMissError as exc:
                logger.warning(f"Block {exc.id} is absent from the storage")

    # File management interface

    def _assert_consistent_file_entry(self, entry_id):
        try:
            manifest = self.get_manifest(entry_id)
        except FSLocalMissError:
            pass
        else:
            assert isinstance(manifest, LocalFileManifest)

    def create_file_descriptor(self, entry_id) -> FileDescriptor:
        self._assert_consistent_file_entry(entry_id)
        fd = self._get_next_fd()
        self.open_fds[fd] = entry_id
        return fd

    def load_file_descriptor(self, fd: FileDescriptor) -> Tuple[EntryID, LocalFileManifest]:
        try:
            entry_id = self.open_fds[fd]
        except KeyError:
            raise FSInvalidFileDescriptor(fd)
        manifest = self.get_manifest(entry_id)
        assert isinstance(manifest, LocalFileManifest)
        return entry_id, manifest

    def remove_file_descriptor(self, fd: FileDescriptor, manifest: LocalFileManifest) -> None:
        assert isinstance(manifest, LocalFileManifest)
        try:
            self.open_fds.pop(fd)
        except KeyError:
            raise FSInvalidFileDescriptor(fd)

    # Timestamped workspace

    def to_timestamped(self, timestamp: Pendulum):
        return LocalStorageTimestamped(self, timestamp)


class LocalStorageTimestamped(LocalStorage):
    """Timestamped version to access a local storage as it was at a given timestamp

    That includes:
    - another cache in memory for fast access to deserialized data
    - the timestamped persistent storage to keep serialized data on the disk :
      vlobs are in common, not manifests. Actually only vlobs are used, manifests are mocked
    - the same lock mecanism to protect against race conditions, although it is useless there
    """

    def __init__(self, local_storage: LocalStorage, timestamp: Pendulum):
        super().__init__(local_storage.device_id, None, "")
        self.persistent_storage = local_storage.persistent_storage

        self.set_dirty_block = self._throw_permission_error
        self.clear_block = self._throw_permission_error

    def _throw_permission_error(*args, **kwargs):
        raise FSError("Not implemented : LocalStorage is timestamped")

    # Manifest interface

    def get_manifest(self, entry_id: EntryID) -> LocalManifest:
        """Raises: FSLocalMissError"""
        assert isinstance(entry_id, EntryID)
        try:
            return self.local_manifest_cache[entry_id]
        except KeyError:
            raise FSLocalMissError(entry_id)

    def set_manifest(
        self, entry_id: EntryID, manifest: LocalManifest, cache_only: bool = False
    ) -> None:  # initially for clean
        assert isinstance(entry_id, EntryID)
        if manifest.need_sync:
            return self._throw_permission_error()
        self._check_lock_status(entry_id)
        self.local_manifest_cache[entry_id] = manifest

    def clear_manifest(self, entry_id: EntryID) -> None:
        assert isinstance(entry_id, EntryID)
        self._check_lock_status(entry_id)
        self.local_manifest_cache.pop(entry_id, None)

    def ensure_manifest_persistant(self, entry_id: EntryID) -> None:
        pass
