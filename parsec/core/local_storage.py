# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from typing import Dict

from structlog import get_logger

from parsec.core.types import (
    Access,
    LocalManifest,
    local_manifest_serializer,
    remote_user_serializer,
    BlockAccess,
)
from .persistent_storage import PersistentStorage, LocalStorageMissingEntry

logger = get_logger()


class LocalStorage:
    """Manage the access to the local storage.

    That includes:
    - a cache in memory for fast access to deserialized data
    - the persistent storage to keep serialized data on the disk
    """

    def __init__(self, path, **kwargs):
        self.manifest_cache = {}
        self.persistent_storage = PersistentStorage(path, **kwargs)

    def __enter__(self):
        self.persistent_storage.__enter__()
        return self

    def __exit__(self, *args):
        self.persistent_storage.__exit__(*args)

    # User interface

    def get_user(self, access: Access) -> Dict:
        raw_user_data = self.persistent_storage.get_user(access)
        return remote_user_serializer.loads(raw_user_data)

    def set_user(self, access: Access, user: Dict) -> None:
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

    def set_clean_manifest(self, access: Access, manifest: LocalManifest, force=False) -> None:
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
        try:
            self.persistent_storage.clear_clean_manifest(access)
        except LocalStorageMissingEntry:
            pass
        raw = local_manifest_serializer.dumps(manifest)
        self.persistent_storage.set_dirty_manifest(access, raw)
        self.manifest_cache[access.id] = manifest

    def clear_manifest(self, access: Access) -> None:
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
