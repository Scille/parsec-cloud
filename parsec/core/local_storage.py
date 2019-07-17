# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from collections import defaultdict
from pathlib import Path
from typing import Dict, Set, Tuple, Union

import trio
from async_generator import asynccontextmanager
from pendulum import Pendulum
from structlog import get_logger
from trio import hazmat

from parsec.core.persistent_storage import (
    LocalStorageError,
    LocalStorageMissingError,
    PersistentStorage,
)
from parsec.core.types import (
    BlockID,
    EntryID,
    FileCursor,
    FileDescriptor,
    LocalFileManifest,
    LocalManifest,
    RemoteManifest,
    local_manifest_serializer,
    remote_manifest_serializer,
)
from parsec.crypto import SecretKey
from parsec.types import DeviceID

logger = get_logger()


class FSInvalidFileDescriptor(Exception):
    pass


class LocalStorage:
    """Manage the access to the local storage.

    That includes:
    - a cache in memory for fast access to deserialized data
    - the persistent storage to keep serialized data on the disk
    - a lock mecanism to protect against race conditions
    """

    def __init__(self, device_id: DeviceID, key: SecretKey, path: Path, **kwargs):
        self.device_id = device_id

        # Cursors and file descriptors
        self.open_cursors: Dict[FileDescriptor, FileCursor] = {}
        self.file_references: Dict[EntryID, Set[Union[EntryID, FileDescriptor]]] = defaultdict(set)
        self._fd_counter = 0

        # Locking structures
        self.locking_tasks = {}
        self.entry_locks = defaultdict(trio.Lock)

        # Manifest and block storage
        self.base_manifest_cache = {}
        self.local_manifest_cache = {}
        self.persistent_storage = PersistentStorage(key, path, **kwargs)

    def _get_next_fd(self) -> FileDescriptor:
        self._fd_counter += 1
        return FileDescriptor(self._fd_counter)

    def __enter__(self):
        self.persistent_storage.__enter__()
        return self

    def __exit__(self, *args):
        self.persistent_storage.__exit__(*args)

    def clear_memory_cache(self):
        self.base_manifest_cache.clear()
        self.local_manifest_cache.clear()

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

    def get_base_manifest(self, entry_id: EntryID) -> RemoteManifest:
        """Raises: LocalStorageMissingError"""
        assert isinstance(entry_id, EntryID)
        try:
            return self.base_manifest_cache[entry_id]
        except KeyError:
            pass
        raw = self.persistent_storage.get_clean_manifest(entry_id)
        manifest = remote_manifest_serializer.loads(raw)
        self.base_manifest_cache[entry_id] = manifest
        return manifest

    def get_manifest(self, entry_id: EntryID) -> LocalManifest:
        """Raises: LocalStorageMissingError"""
        assert isinstance(entry_id, EntryID)
        try:
            return self.local_manifest_cache[entry_id]
        except KeyError:
            pass
        try:
            raw = self.persistent_storage.get_dirty_manifest(entry_id)
        except LocalStorageMissingError:
            manifest = self.get_base_manifest(entry_id).to_local(self.device_id)
        else:
            manifest = local_manifest_serializer.loads(raw)
        self.local_manifest_cache[entry_id] = manifest
        return manifest

    def set_base_manifest(self, entry_id: EntryID, manifest: RemoteManifest) -> None:
        assert isinstance(entry_id, EntryID)
        self._check_lock_status(entry_id)
        # Remove the corresponding local manifest if it exists
        try:
            self.persistent_storage.clear_dirty_manifest(entry_id)
        except LocalStorageMissingError:
            pass
        self.local_manifest_cache.pop(entry_id, None)
        # Serialize and set remote manifest
        raw = remote_manifest_serializer.dumps(manifest)
        self.persistent_storage.set_clean_manifest(entry_id, raw)
        self.base_manifest_cache[entry_id] = manifest

    def set_manifest(self, entry_id: EntryID, manifest: LocalManifest) -> None:
        assert isinstance(entry_id, EntryID)
        assert manifest.author == self.device_id
        self._check_lock_status(entry_id)
        raw = local_manifest_serializer.dumps(manifest)
        self.persistent_storage.set_dirty_manifest(entry_id, raw)
        self.local_manifest_cache[entry_id] = manifest

    def clear_manifest(self, entry_id: EntryID) -> None:
        assert isinstance(entry_id, EntryID)
        self._check_lock_status(entry_id)
        try:
            self.persistent_storage.clear_clean_manifest(entry_id)
        except LocalStorageMissingError:
            pass
        try:
            self.persistent_storage.clear_dirty_manifest(entry_id)
        except LocalStorageMissingError:
            pass
        self.base_manifest_cache.pop(entry_id, None)
        self.local_manifest_cache.pop(entry_id, None)

    # Block interface

    def is_dirty_block(self, block_id: BlockID):
        assert isinstance(block_id, BlockID)
        return self.persistent_storage.is_dirty_block(block_id)

    def get_block(self, block_id: BlockID) -> bytes:
        assert isinstance(block_id, BlockID)
        try:
            return self.persistent_storage.get_dirty_block(block_id)
        except LocalStorageMissingError:
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
        except LocalStorageMissingError:
            try:
                self.persistent_storage.clear_clean_block(block_id)
            except LocalStorageMissingError:
                logger.warning("Tried to remove a dirty block that doesn't exist anymore")

    # File management interface

    def _assert_consistent_file_entry(self, entry_id, manifest=None):
        try:
            local_manifest = self.get_manifest(entry_id)
        except LocalStorageMissingError:
            local_manifest = None
        if manifest and local_manifest:
            assert local_manifest == manifest
        if manifest or local_manifest:
            assert isinstance(manifest or local_manifest, LocalFileManifest)

    def _assert_consistent_file_descriptor(self, fd, manifest=None):
        cursor = self.open_cursors.get(fd)
        if cursor is not None:
            return self._assert_consistent_file_entry(cursor.entry_id, manifest)
        if manifest is not None:
            assert isinstance(manifest, LocalFileManifest)

    def create_file_descriptor(self, cursor: FileCursor) -> FileDescriptor:
        self._assert_consistent_file_entry(cursor.entry_id)
        fd = self._get_next_fd()
        self.open_cursors[fd] = cursor
        self.file_references[cursor.entry_id].add(fd)
        return fd

    def load_file_descriptor(self, fd: FileDescriptor) -> Tuple[FileCursor, LocalFileManifest]:
        self._assert_consistent_file_descriptor(fd)
        try:
            cursor = self.open_cursors[fd]
        except KeyError:
            raise FSInvalidFileDescriptor(fd)
        manifest = self.get_manifest(cursor.entry_id)
        return cursor, manifest

    def remove_file_descriptor(self, fd: FileDescriptor, manifest: LocalFileManifest) -> None:
        self._assert_consistent_file_descriptor(fd, manifest)
        try:
            cursor = self.open_cursors.pop(fd)
        except KeyError:
            raise FSInvalidFileDescriptor(fd)
        self.file_references[cursor.entry_id].discard(fd)
        self.cleanup_unreferenced_file(cursor.entry_id, manifest)

    def add_file_reference(self, entry_id: EntryID) -> None:
        self._assert_consistent_file_entry(entry_id)
        self.file_references[entry_id].add(entry_id)

    def remove_file_reference(self, entry_id: EntryID, manifest: LocalFileManifest) -> None:
        self._assert_consistent_file_entry(entry_id, manifest)
        self.file_references[entry_id].discard(entry_id)
        self.cleanup_unreferenced_file(entry_id, manifest)

    def cleanup_unreferenced_file(self, entry_id: EntryID, manifest: LocalFileManifest) -> None:
        self._assert_consistent_file_entry(entry_id, manifest)
        if self.file_references[entry_id]:
            return
        self.clear_manifest(entry_id)
        for block_access in manifest.dirty_blocks:
            self.clear_block(block_access.id)
        for block_access in manifest.blocks:
            self.clear_block(block_access.id)

    # Cursor helper

    def create_cursor(self, entry_id: EntryID) -> FileDescriptor:
        cursor = FileCursor(entry_id)
        self.add_file_reference(entry_id)
        return self.create_file_descriptor(cursor)

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
        self.device_id = local_storage.device_id

        # Cursors and file descriptors
        self.open_cursors: Dict[FileDescriptor, FileCursor] = {}
        self.file_references: Dict[EntryID, Set[Union[EntryID, FileDescriptor]]] = defaultdict(set)
        self._fd_counter = 0

        # Locking structures
        self.locking_tasks = {}
        self.entry_locks = defaultdict(trio.Lock)

        # Manifest and block storage
        self.base_manifest_cache = {}
        self.local_manifest_cache = {}  # should delete? seems used for dirty, base for clean...

        self.persistent_storage = local_storage.persistent_storage

        self.set_dirty_block = self._throw_permission_error
        self.clear_block = self._throw_permission_error
        self.set_manifest = self._throw_permission_error

    def _throw_permission_error(*e, **ke):
        raise LocalStorageError("Not implemented : LocalStorage is timestamped")

    # Manifest interface

    def get_base_manifest(self, entry_id: EntryID) -> RemoteManifest:
        """Raises: LocalStorageMissingError"""
        assert isinstance(entry_id, EntryID)
        try:
            return self.base_manifest_cache[entry_id]
        except KeyError:
            raise LocalStorageMissingError(entry_id)

    def get_manifest(self, entry_id: EntryID) -> LocalManifest:
        """Raises: LocalStorageMissingError"""
        assert isinstance(entry_id, EntryID)
        try:
            return self.local_manifest_cache[entry_id]
        except KeyError:
            pass
        manifest = self.get_base_manifest(entry_id).to_local(self.device_id)
        self.local_manifest_cache[entry_id] = manifest
        return manifest

    def set_base_manifest(
        self, entry_id: EntryID, manifest: RemoteManifest
    ) -> None:  # initially for clean
        assert isinstance(entry_id, EntryID)
        self._check_lock_status(entry_id)
        self.base_manifest_cache[entry_id] = manifest

    def clear_manifest(self, entry_id: EntryID) -> None:
        assert isinstance(entry_id, EntryID)
        self._check_lock_status(entry_id)
        self.base_manifest_cache.pop(entry_id, None)
        self.local_manifest_cache.pop(entry_id, None)
