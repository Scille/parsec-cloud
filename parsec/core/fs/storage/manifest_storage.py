# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
from structlog import get_logger
from typing import Dict, Tuple, Set, Optional
from async_generator import asynccontextmanager

from parsec.core.fs.exceptions import FSLocalMissError
from parsec.core.types import EntryID, ChunkID, LocalDevice, LocalManifest
from parsec.core.fs.storage.local_database import LocalDatabase

logger = get_logger()


class ManifestStorage:
    """Persistent storage with cache for storing manifests.

    Also stores the checkpoint.
    """

    def __init__(self, device: LocalDevice, localdb: LocalDatabase, realm_id: EntryID):
        self.device = device
        self.localdb = localdb
        self.realm_id = realm_id

        self._cache = {}
        self._cache_ahead_of_localdb = {}

    @property
    def path(self):
        return self.localdb.path

    @classmethod
    @asynccontextmanager
    async def run(cls, *args, **kwargs):
        self = cls(*args, **kwargs)
        await self._create_db()
        try:
            yield self
        finally:
            with trio.CancelScope(shield=True):
                await self._flush_cache_ahead_of_persistance()

    def _open_cursor(self):
        # We want the manifest to be written to the disk as soon as possible
        # (unless they are purposely kept out of the local database)
        return self.localdb.open_cursor(commit=True)

    def clear_memory_cache(self):
        self._cache.clear()
        self._cache_ahead_of_localdb.clear()

    # Database initialization

    async def _create_db(self):
        async with self._open_cursor() as cursor:
            cursor.execute(
                """
                CREATE TABLE IF NOT EXISTS vlobs
                (
                  vlob_id BLOB PRIMARY KEY NOT NULL, -- UUID
                  base_version INTEGER NOT NULL,
                  remote_version INTEGER NOT NULL,
                  need_sync INTEGER NOT NULL,  -- Boolean
                  blob BLOB NOT NULL
                );
                """
            )

            # Singleton storing the checkpoint
            cursor.execute(
                """
                CREATE TABLE IF NOT EXISTS realm_checkpoint
                (
                  _id INTEGER PRIMARY KEY NOT NULL,
                  checkpoint INTEGER NOT NULL
                );
                """
            )

    # Checkpoint operations

    async def get_realm_checkpoint(self) -> int:
        """
        Raises: Nothing !
        """
        async with self._open_cursor() as cursor:
            cursor.execute("SELECT checkpoint FROM realm_checkpoint WHERE _id = 0")
            rep = cursor.fetchone()
            return rep[0] if rep else 0

    async def update_realm_checkpoint(
        self, new_checkpoint: int, changed_vlobs: Dict[EntryID, int]
    ) -> None:
        """
        Raises: Nothing !
        """
        async with self._open_cursor() as cursor:
            cursor.executemany(
                "UPDATE vlobs SET remote_version = ? WHERE vlob_id = ?",
                ((version, entry_id.bytes) for entry_id, version in changed_vlobs.items()),
            )
            cursor.execute(
                """INSERT OR REPLACE INTO realm_checkpoint(_id, checkpoint)
                VALUES (0, ?)""",
                (new_checkpoint,),
            )

    async def get_need_sync_entries(self) -> Tuple[Set[EntryID], Set[EntryID]]:
        """
        Raises: Nothing !
        """
        async with self._open_cursor() as cursor:
            cursor.execute(
                "SELECT vlob_id, need_sync FROM vlobs WHERE need_sync = 1 OR base_version != remote_version"
            )
            local_changes = set()
            remote_changes = set()
            for manifest_id, need_sync in cursor.fetchall():
                if need_sync:
                    local_changes.add(EntryID(manifest_id))
                else:
                    remote_changes.add(EntryID(manifest_id))
            return local_changes, remote_changes

    # Manifest operations

    async def get_manifest(self, entry_id: EntryID) -> LocalManifest:
        """
        Raises:
            FSLocalMissError
        """
        try:
            return self._cache[entry_id]
        except KeyError:
            pass

        async with self._open_cursor() as cursor:
            cursor.execute("SELECT blob FROM vlobs WHERE vlob_id = ?", (entry_id.bytes,))
            manifest_row = cursor.fetchone()
        if not manifest_row:
            raise FSLocalMissError(entry_id)

        manifest = LocalManifest.decrypt_and_load(manifest_row[0], key=self.device.local_symkey)
        self._cache[entry_id] = manifest

        return manifest

    async def set_manifest(
        self, entry_id: EntryID, manifest: LocalManifest, cache_only: bool = False
    ) -> None:
        """
        Raises: Nothing !
        """
        assert isinstance(entry_id, EntryID)
        self._cache[entry_id] = manifest
        self._cache_ahead_of_localdb.setdefault(entry_id, set())

        if not cache_only:
            await self._ensure_manifest_persistent(entry_id)

    async def _set_manifest_in_db(
        self, entry_id: EntryID, manifest: LocalManifest, chunk_ids: Optional[Set[ChunkID]] = None
    ) -> None:
        ciphered = manifest.dump_and_encrypt(self.device.local_symkey)
        async with self._open_cursor() as cursor:
            cursor.execute(
                """INSERT OR REPLACE INTO vlobs (vlob_id, blob, need_sync, base_version, remote_version)
                VALUES (
                    ?, ?, ?, ?,
                    max(
                        ?,
                        IFNULL((SELECT remote_version FROM vlobs WHERE vlob_id=?), 0)
                    )
                )""",
                (
                    entry_id.bytes,
                    ciphered,
                    manifest.need_sync,
                    manifest.base_version,
                    manifest.base_version,
                    entry_id.bytes,
                ),
            )

            # No cleanup necessary
            if not chunk_ids:
                return

            # Clean all the pending chunks
            for chunk_id in chunk_ids:
                cursor.execute("DELETE FROM chunks WHERE chunk_id = ?", (chunk_id.bytes,))

    async def ensure_manifest_persistent(self, entry_id: EntryID) -> None:
        """
        Raises: Nothing !
        """
        assert isinstance(entry_id, EntryID)
        if entry_id not in self._cache_ahead_of_localdb:
            return
        await self._ensure_manifest_persistent(entry_id)

    async def _flush_cache_ahead_of_persistance(self) -> None:
        for entry_id in self._cache_ahead_of_localdb.copy():
            await self._ensure_manifest_persistent(entry_id)

    async def _ensure_manifest_persistent(self, entry_id: EntryID) -> None:
        manifest = self._cache[entry_id]
        chunk_ids = self._cache_ahead_of_localdb[entry_id]
        await self._set_manifest_in_db(entry_id, manifest, chunk_ids)
        self._cache_ahead_of_localdb.pop(entry_id)

    async def clear_manifest(self, entry_id: EntryID) -> None:
        """
        Raises:
            FSLocalMissError
        """
        if entry_id in self._cache_ahead_of_localdb:
            await self._ensure_manifest_persistent(entry_id)
        in_cache = bool(self._cache.pop(entry_id, None))
        async with self._open_cursor() as cursor:
            cursor.execute("DELETE FROM vlobs WHERE vlob_id = ?", (entry_id.bytes,))
            cursor.execute("SELECT changes()")
            deleted, = cursor.fetchone()
        if not deleted and not in_cache:
            raise FSLocalMissError(entry_id)

    # Chunk clearing logic

    def postpone_clear_chunks(self, chunk_ids: Set[ChunkID], postpone: EntryID) -> bool:
        if postpone not in self._cache_ahead_of_localdb:
            return False
        self._cache_ahead_of_localdb[postpone] |= chunk_ids
        return True
