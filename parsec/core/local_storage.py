# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from typing import Tuple
from itertools import count
from collections import defaultdict

import trio
from trio import hazmat

from structlog import get_logger
from async_generator import asynccontextmanager

from parsec.core.types import (
    FileDescriptor,
    Access,
    BlockAccess,
    LocalManifest,
    LocalFileManifest,
    FileCursor,
    local_manifest_serializer,
)
from parsec.core.persistent_storage import PersistentStorage, LocalStorageMissingEntry


logger = get_logger()


class FSInvalidFileDescriptor(Exception):
    pass


class LocalStorage:
    """Manage the access to the local storage.

    That includes:
    - a cache in memory for fast access to deserialized data
    - the persistent storage to keep serialized data on the disk
    """

    def __init__(self, path, **kwargs):

        # Cursors and file descriptors
        self.open_cursors = {}
        self.file_references = defaultdict(set)
        self.file_descriptor_counter = count(1)

        # Locking structures
        self.locking_tasks = {}
        self.access_locks = defaultdict(trio.Lock)

        # Manifest and block storage
        self.manifest_cache = {}
        self.persistent_storage = PersistentStorage(path, **kwargs)

    def __enter__(self):
        self.persistent_storage.__enter__()
        return self

    def __exit__(self, *args):
        self.persistent_storage.__exit__(*args)

    # Locking helpers

    @asynccontextmanager
    async def lock_manifest(self, access: Access):
        async with self.access_locks[access]:
            try:
                self.locking_tasks[access] = hazmat.current_task()
                yield self.get_manifest(access)
            finally:
                del self.locking_tasks[access]

    def _check_lock_status(self, access: Access) -> None:
        task = self.locking_tasks.get(access)
        assert task is None or task == hazmat.current_task()

    # Manifest interface

    def get_manifest(self, access: Access) -> LocalManifest:
        try:
            return self.manifest_cache[access.id]
        except KeyError:
            pass
        # Try local manifest first, although it should not matter
        try:
            raw = self.persistent_storage.get_dirty_manifest(access)
        except LocalStorageMissingEntry:
            raw = self.persistent_storage.get_clean_manifest(access)
        manifest = local_manifest_serializer.loads(raw)
        self.manifest_cache[access.id] = manifest
        return manifest

    def set_clean_manifest(self, access: Access, manifest: LocalManifest, force=False) -> None:
        self._check_lock_status(access)
        # Remove the corresponding local manifest if it exists
        if force:
            try:
                self.persistent_storage.clear_dirty_manifest(access)
            except LocalStorageMissingEntry:
                pass
        # Serialize and set remote manifest
        raw = local_manifest_serializer.dumps(manifest)
        self.persistent_storage.set_clean_manifest(access, raw)
        self.manifest_cache[access.id] = manifest

    def set_dirty_manifest(self, access: Access, manifest: LocalManifest) -> None:
        self._check_lock_status(access)
        try:
            self.persistent_storage.clear_clean_manifest(access)
        except LocalStorageMissingEntry:
            pass
        raw = local_manifest_serializer.dumps(manifest)
        self.persistent_storage.set_dirty_manifest(access, raw)
        self.manifest_cache[access.id] = manifest

    def clear_manifest(self, access: Access) -> None:
        self._check_lock_status(access)
        try:
            self.persistent_storage.clear_clean_manifest(access)
        except LocalStorageMissingEntry:
            try:
                self.persistent_storage.clear_dirty_manifest(access)
            except LocalStorageMissingEntry:
                pass
        self.manifest_cache.pop(access.id, None)

    # Block interface

    def get_block(self, access: BlockAccess) -> bytes:
        try:
            return self.persistent_storage.get_dirty_block(access)
        except LocalStorageMissingEntry:
            return self.persistent_storage.get_clean_block(access)

    def set_dirty_block(self, access: BlockAccess, block: bytes) -> None:
        return self.persistent_storage.set_dirty_block(access, block)

    def set_clean_block(self, access: BlockAccess, block: bytes) -> None:
        return self.persistent_storage.set_clean_block(access, block)

    def clear_dirty_block(self, access: BlockAccess) -> None:
        try:
            self.persistent_storage.clear_dirty_block(access)
        except LocalStorageMissingEntry:
            logger.warning("Tried to remove a dirty block that doesn't exist anymore")

    def clear_clean_block(self, access: BlockAccess) -> None:
        try:
            self.persistent_storage.clear_clean_block(access)
        except LocalStorageMissingEntry:
            pass

    # File management interface

    def _assert_consistent_file_access(self, access, manifest=None):
        try:
            local_manifest = self.get_manifest(access)
        except LocalStorageMissingEntry:
            local_manifest = None
        if manifest and local_manifest:
            assert local_manifest == manifest
        if manifest or local_manifest:
            assert isinstance(manifest or local_manifest, LocalFileManifest)

    def _assert_consistent_file_descriptor(self, fd, manifest=None):
        cursor = self.open_cursors.get(fd)
        if cursor is not None:
            return self._assert_consistent_file_access(cursor.access, manifest)
        if manifest is not None:
            assert isinstance(manifest, LocalFileManifest)

    def create_file_descriptor(self, cursor: FileCursor) -> FileDescriptor:
        self._assert_consistent_file_access(cursor.access)
        fd = next(self.file_descriptor_counter)
        self.open_cursors[fd] = cursor
        self.file_references[cursor.access].add(fd)
        return fd

    def load_file_descriptor(self, fd: FileDescriptor) -> Tuple[FileCursor, LocalFileManifest]:
        self._assert_consistent_file_descriptor(fd)
        try:
            cursor = self.open_cursors[fd]
        except KeyError:
            raise FSInvalidFileDescriptor(fd)
        manifest = self.get_manifest(cursor.access)
        return cursor, manifest

    def remove_file_descriptor(self, fd: FileDescriptor, manifest: LocalFileManifest) -> None:
        self._assert_consistent_file_descriptor(fd, manifest)
        try:
            cursor = self.open_cursors.pop(fd)
        except KeyError:
            raise FSInvalidFileDescriptor(fd)
        self.file_references[cursor.access].discard(fd)
        self.cleanup_unreferenced_file(cursor.access, manifest)

    def add_file_reference(self, access: Access) -> None:
        self._assert_consistent_file_access(access)
        self.file_references[access].add(access)

    def remove_file_reference(self, access: Access, manifest: LocalFileManifest) -> None:
        self._assert_consistent_file_access(access, manifest)
        self.file_references[access].discard(access)
        self.cleanup_unreferenced_file(access, manifest)

    def cleanup_unreferenced_file(self, access: Access, manifest: LocalFileManifest) -> None:
        self._assert_consistent_file_access(access, manifest)
        if self.file_references[access]:
            return
        self.clear_manifest(access)
        for block_access in manifest.dirty_blocks:
            self.clear_dirty_block(block_access)
        for block_access in manifest.blocks:
            self.clear_clean_block(block_access)
