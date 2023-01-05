# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from contextlib import asynccontextmanager
from pathlib import Path
from typing import AsyncContextManager, AsyncIterator, Dict, Set, Tuple, Union

import trio
from structlog import get_logger

from parsec._parsec import Regex
from parsec.core.fs.exceptions import FSLocalMissError, FSLocalStorageClosedError
from parsec.core.fs.storage.local_database import Cursor, LocalDatabase
from parsec.core.types import BlockID, ChunkID, EntryID, LocalDevice
from parsec.core.types.manifest import AnyLocalManifest, local_manifest_decrypt_and_load

logger = get_logger()

EMPTY_PATTERN = r"^\b$"  # Do not match anything (https://stackoverflow.com/a/2302992/2846140)


class ManifestStorage:
    """Persistent storage with cache for storing manifests.

    Also stores the checkpoint.
    """

    def __init__(self, device: LocalDevice, localdb: LocalDatabase, realm_id: EntryID):
        self.device = device
        self.localdb = localdb
        self.realm_id = realm_id

        # This cache contains all the manifests that have been set or accessed
        # since the last call to `clear_memory_cache`
        self._cache: Dict[EntryID, AnyLocalManifest] = {}

        # This dictionary keeps track of all the entry ids of the manifests
        # that have been added to the cache but still needs to be written to
        # the localdb. The corresponding value is a set with the ids of all
        # the chunks that needs to be removed from the localdb after the
        # manifest is written. Note: this set might be empty but the manifest
        # still requires to be flushed.
        self._cache_ahead_of_localdb: Dict[EntryID, Set[Union[ChunkID, BlockID]]] = {}

    @property
    def path(self) -> Path:
        return Path(self.localdb.path)

    @classmethod
    @asynccontextmanager
    async def run(
        cls, device: LocalDevice, localdb: LocalDatabase, realm_id: EntryID
    ) -> AsyncIterator["ManifestStorage"]:
        self = cls(device, localdb, realm_id)
        await self._create_db()
        try:
            yield self
        finally:
            with trio.CancelScope(shield=True):
                # Flush the in-memory cache before closing the storage
                try:
                    await self._flush_cache_ahead_of_persistance()
                # Ignore storage closed exceptions, since it follows an operational error
                except FSLocalStorageClosedError:
                    pass

    def _open_cursor(self) -> AsyncContextManager[Cursor]:
        # We want the manifest to be written to the disk as soon as possible
        # (unless they are purposely kept out of the local database)
        return self.localdb.open_cursor(commit=True)

    async def clear_memory_cache(self, flush: bool = True) -> None:
        if flush:
            await self._flush_cache_ahead_of_persistance()
        self._cache_ahead_of_localdb.clear()
        self._cache.clear()

    # Database initialization

    async def _create_db(self) -> None:
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
            # Singleton storing the prevent_sync_pattern
            cursor.execute(
                """
                CREATE TABLE IF NOT EXISTS prevent_sync_pattern
                (
                  _id INTEGER PRIMARY KEY NOT NULL,
                  pattern TEXT NOT NULL,
                  fully_applied INTEGER NOT NULL  -- Boolean
                );
                """
            )
            # Set the default "prevent sync" pattern if it doesn't exist
            cursor.execute(
                """
                INSERT OR IGNORE INTO prevent_sync_pattern(_id, pattern, fully_applied)
                VALUES (0, ?, 0)""",
                (EMPTY_PATTERN,),
            )

    # "Prevent sync" pattern operations

    async def set_prevent_sync_pattern(self, pattern: Regex) -> bool:
        """Set the "prevent sync" pattern for the corresponding workspace.

        This operation is idempotent,
        i.e it does not reset the `fully_applied` flag if the pattern hasn't changed.
        """
        async with self._open_cursor() as cursor:
            cursor.execute(
                """UPDATE prevent_sync_pattern SET pattern = ?, fully_applied = 0 WHERE _id = 0 AND pattern != ?""",
                (pattern.pattern, pattern.pattern),
            )
            cursor.execute("SELECT fully_applied FROM prevent_sync_pattern WHERE _id = 0")
            reply = cursor.fetchone()
            (fully_applied,) = reply
            return fully_applied

    async def mark_prevent_sync_pattern_fully_applied(self, pattern: Regex) -> bool:
        """Mark the provided pattern as fully applied.

        This is meant to be called after one made sure that all the manifests in the
        workspace are compliant with the new pattern. The applied pattern is provided
        as an argument in order to avoid concurrency issues.
        """
        async with self._open_cursor() as cursor:
            cursor.execute(
                """UPDATE prevent_sync_pattern SET fully_applied = 1 WHERE _id = 0 AND pattern = ?""",
                (pattern.pattern,),
            )
            cursor.execute("SELECT fully_applied FROM prevent_sync_pattern WHERE _id = 0")
            reply = cursor.fetchone()
            (fully_applied,) = reply
            return fully_applied

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
        remote_changes = set()
        local_changes = {
            entry_id for entry_id, manifest in self._cache.items() if manifest.need_sync
        }

        async with self._open_cursor() as cursor:
            cursor.execute(
                "SELECT vlob_id, need_sync, base_version, remote_version "
                "FROM vlobs WHERE need_sync = 1 OR base_version != remote_version"
            )
            for manifest_id, need_sync, bv, rv in cursor.fetchall():
                manifest_id = EntryID.from_bytes(manifest_id)
                if need_sync:
                    local_changes.add(manifest_id)
                if bv != rv:
                    remote_changes.add(manifest_id)
            return local_changes, remote_changes

    # Manifest operations

    async def get_manifest(self, entry_id: EntryID) -> AnyLocalManifest:
        """
        Raises:
            FSLocalMissError
        """
        # Look in cache first
        try:
            return self._cache[entry_id]
        except KeyError:
            pass

        # Look into the database
        async with self._open_cursor() as cursor:
            cursor.execute("SELECT blob FROM vlobs WHERE vlob_id = ?", (entry_id.bytes,))
            manifest_row = cursor.fetchone()

        # Not found
        if not manifest_row:
            raise FSLocalMissError(entry_id)

        # Safely fill the cache
        if entry_id not in self._cache:
            self._cache[entry_id] = local_manifest_decrypt_and_load(
                manifest_row[0], key=self.device.local_symkey
            )

        # Always return the cached value
        return self._cache[entry_id]

    async def set_manifest(
        self,
        entry_id: EntryID,
        manifest: AnyLocalManifest,
        cache_only: bool = False,
        removed_ids: Set[ChunkID] | None = None,
    ) -> None:
        """
        Raises: Nothing !
        """
        assert isinstance(entry_id, EntryID)

        # Set the cache first
        self._cache[entry_id] = manifest

        # Tag the entry as ahead of localdb
        self._cache_ahead_of_localdb.setdefault(entry_id, set())

        # Cleanup
        if removed_ids:
            self._cache_ahead_of_localdb[entry_id] |= removed_ids

        # Flush the cached value to the localdb
        if not cache_only:
            await self._ensure_manifest_persistent(entry_id)

    async def _ensure_manifest_persistent(self, entry_id: EntryID) -> None:

        # Get cursor
        async with self._open_cursor() as cursor:

            # Flushing is not necessary
            if entry_id not in self._cache_ahead_of_localdb:
                return

            # Safely get the manifest and other information
            manifest = self._cache[entry_id]
            pending_chunks_ids = [
                (chunk_id.bytes,) for chunk_id in self._cache_ahead_of_localdb[entry_id]
            ]
            local_symkey = self.device.local_symkey

            def _thread_target() -> None:
                # Dump and decrypt the manifest
                ciphered = manifest.dump_and_encrypt(local_symkey)

                # Insert into the local database
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

                # Clean all the pending chunks
                if pending_chunks_ids:
                    cursor.executemany("DELETE FROM chunks WHERE chunk_id = ?", pending_chunks_ids)

            # Run CPU and IO expensive logic in a thread
            await self.localdb.run_in_thread(_thread_target)

        # Tag entry as up-to-date only if no new manifest has been written in the meantime
        if manifest == self._cache[entry_id]:
            self._cache_ahead_of_localdb.pop(entry_id)

    async def ensure_manifest_persistent(self, entry_id: EntryID) -> None:
        """
        Raises: Nothing !
        """
        assert isinstance(entry_id, EntryID)
        # Flush if necessary
        if entry_id in self._cache_ahead_of_localdb:
            await self._ensure_manifest_persistent(entry_id)

    async def _flush_cache_ahead_of_persistance(self) -> None:
        # Flush until the all the cache is gone
        while self._cache_ahead_of_localdb:
            entry_id = next(iter(self._cache_ahead_of_localdb))
            await self._ensure_manifest_persistent(entry_id)

    # This method is not used in the code base but it is still tested
    # as it might come handy in a cleanup routine later

    async def clear_manifest(self, entry_id: EntryID) -> None:
        """
        Raises:
            FSLocalMissError
        """

        async with self._open_cursor() as cursor:

            # Safely remove from cache
            in_cache = bool(self._cache.pop(entry_id, None))

            # Remove from local database
            cursor.execute("DELETE FROM vlobs WHERE vlob_id = ?", (entry_id.bytes,))
            cursor.execute("SELECT changes()")
            (deleted,) = cursor.fetchone()

            # Clean all the pending chunks
            # TODO: should also add the content of the popped manifest
            pending_chunk_ids = self._cache_ahead_of_localdb.pop(entry_id, ())
            for chunk_id in pending_chunk_ids:
                cursor.execute("DELETE FROM chunks WHERE chunk_id = ?", (chunk_id.bytes,))

        # Raise a miss if the entry wasn't found
        if not deleted and not in_cache:
            raise FSLocalMissError(entry_id)
