# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import hashlib
import random

import pendulum
from structlog import get_logger

from parsec.types import UserID
from parsec.core.types import (
    Access,
    LocalManifest,
    local_manifest_serializer,
    remote_user_serializer,
    BlockAccess,
    ManifestAccess,
)
from .persistent_storage import PersistentStorage, LocalStorageMissingEntry

logger = get_logger()


# Alias now
now = pendulum.Pendulum.now

# TODO: should be in config.py
DEFAULT_MAX_CACHE_SIZE = 128 * 1024 * 1024
DEFAULT_BLOCK_SIZE = 2 ** 16


class LocalStorage:
    """Manage the access to the local storage.

    That includes:
    - a cache in memory for fast access to deserialized data
    - the persistent storage to keep serialized data on the disk
    """

    def __init__(self, path, max_memory_cache_size: int = DEFAULT_MAX_CACHE_SIZE, **kwargs):
        self.local_symkey = None
        self.manifest_cache = {}
        self.clean_block_cache = {}
        self.dirty_block_cache = {}
        self.persistent_storage = PersistentStorage(path, **kwargs)
        self.max_cache_size = max_memory_cache_size

    def __enter__(self):
        self.persistent_storage.__enter__()
        return self

    def __exit__(self, *args):
        for value in self.clean_block_cache.values():
            if not value["persisted"]:
                self.persistent_storage.set_clean_block(value["access"], value["block"])
            self.persistent_storage.update_block_accessed_on(
                True, value["access"], value["accessed_on"]
            )
        self.persistent_storage.__exit__(*args)

    @property
    def block_limit(self):
        return self.max_cache_size // DEFAULT_BLOCK_SIZE

    def get_nb_blocks(self, clean: bool):
        cache = self.clean_block_cache if clean else self.dirty_block_cache
        return len(cache)

    # User interface

    def _build_remote_user_local_access(self, user_id: UserID) -> ManifestAccess:
        return ManifestAccess(
            id=hashlib.sha256(user_id.encode("utf8")).hexdigest(), key=self.local_symkey
        )

    def get_user(self, user_id):
        access = self._build_remote_user_local_access(user_id)
        raw_user_data = self.persistent_storage.get_user(access)
        return remote_user_serializer.loads(raw_user_data)

    def set_user(self, user_id, user):
        access = self._build_remote_user_local_access(user_id)
        raw = remote_user_serializer.dumps(user)
        self.persistent_storage.set_user(access, raw)

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

    def set_clean_manifest(self, access: Access, manifest: LocalManifest, force=False):
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

    def set_dirty_manifest(self, access: Access, manifest: LocalManifest):
        try:
            self.persistent_storage.clear_clean_manifest(access)
        except LocalStorageMissingEntry:
            pass
        raw = local_manifest_serializer.dumps(manifest)
        self.persistent_storage.set_dirty_manifest(access, raw)
        self.manifest_cache[access.id] = manifest

    def clear_manifest(self, access: Access):
        try:
            self.persistent_storage.clear_clean_manifest(access)
        except LocalStorageMissingEntry:
            try:
                self.persistent_storage.clear_dirty_manifest(access)
            except LocalStorageMissingEntry:
                pass
        self.manifest_cache.pop(access.id, None)

    # Block interface

    def add_block_in_dirty_cache(self, access: BlockAccess, block: bytes):
        if self.get_nb_blocks(False) >= self.block_limit:
            key = random.choice(list(self.dirty_block_cache.keys()))
            del self.dirty_block_cache[key]
        self.dirty_block_cache[access.id] = block

    def get_block(self, access: BlockAccess) -> bytes:
        result = None
        try:
            return self.dirty_block_cache[access.id]
        except KeyError:
            pass
        try:
            self.clean_block_cache[access.id]["accessed_on"] = str(now())
            return self.clean_block_cache[access.id]["block"]
        except KeyError:
            pass
        try:
            result = self.persistent_storage.get_dirty_block(access)
        except LocalStorageMissingEntry:
            result = self.persistent_storage.get_clean_block(access)
            self.clean_block_cache[access.id] = {
                "access": access,
                "accessed_on": str(now()),
                "block": result,
                "persisted": True,
            }
        else:
            self.add_block_in_dirty_cache(access, result)
        return result

    def set_dirty_block(self, access: BlockAccess, block: bytes) -> None:
        self.add_block_in_dirty_cache(access, block)
        self.persistent_storage.set_dirty_block(access, block)

    def set_clean_block(self, access: BlockAccess, block: bytes) -> None:
        if self.get_nb_blocks(True) >= self.block_limit:
            key = random.choice(list(self.clean_block_cache.keys()))
            deleted = self.clean_block_cache.pop(key)
            if deleted["persisted"]:
                self.persistent_storage.update_block_accessed_on(
                    True, deleted["access"], deleted["accessed_on"]
                )
            else:
                self.persistent_storage.set_clean_block(deleted["access"], deleted["block"])
        self.clean_block_cache[access.id] = {
            "access": access,
            "accessed_on": None,
            "block": block,
            "persisted": False,
        }

    def clear_dirty_block(self, access: BlockAccess) -> None:
        try:
            self.persistent_storage.clear_dirty_block(access)
        except LocalStorageMissingEntry:
            logger.warning("Tried to remove a dirty block that doesn't exist anymore")
        self.dirty_block_cache.pop(access.id, None)

    def clear_clean_block(self, access: BlockAccess) -> None:
        try:
            self.persistent_storage.clear_clean_block(access)
        except LocalStorageMissingEntry:
            pass
        self.clean_block_cache.pop(access.id, None)
