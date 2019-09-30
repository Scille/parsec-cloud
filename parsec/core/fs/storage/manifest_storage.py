# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from pathlib import Path
from typing import Dict, Tuple, Set
from structlog import get_logger
from sqlite3 import connect as sqlite_connect
from async_generator import asynccontextmanager

from parsec.core.fs.exceptions import FSLocalMissError
from parsec.core.types import EntryID, LocalDevice, LocalManifest


logger = get_logger()


class ManifestStorage:
    """Persistent storage with cache for storing manifests.

    Also stores the checkpoint.
    """

    def __init__(self, device: LocalDevice, path: Path, realm_id: EntryID):
        self.device = device
        self.path = Path(path)
        self.realm_id = realm_id

        self._cache = {}
        self._cache_ahead_of_persistance_ids = set()
        self._conn = None

    @classmethod
    @asynccontextmanager
    async def run(cls, *args, **kwargs):
        self = cls(*args, **kwargs)
        try:
            await self._connect()
            try:
                yield self
            finally:
                await self._flush_cache_ahead_of_persistance()
        finally:
            await self._close()

    # Life cycle

    async def _create_connection(self):
        # Create directories
        self.path.parent.mkdir(parents=True, exist_ok=True)

        # Create sqlite connection
        conn = sqlite_connect(str(self.path))

        # Set fast auto-commit mode
        conn.isolation_level = None
        conn.execute("pragma journal_mode=wal")
        conn.execute("PRAGMA synchronous = OFF")

        # Return connection
        return conn

    async def _connect(self):
        if self._conn is not None:
            raise RuntimeError("Already connected")

        # Connect and initialize database
        self._conn = await self._create_connection()

        # Initialize
        await self._create_db()

    async def _close(self):
        # Idempotency
        if self._conn is None:
            return

        # Auto-commit is used but do it once more just in case
        self._conn.commit()
        self._conn.close()
        self._conn = None

    def clear_memory_cache(self):
        self._cache.clear()
        self._cache_ahead_of_persistance_ids.clear()

    # Cursor management

    @asynccontextmanager
    async def _open_cursor(self):
        cursor = self._conn.cursor()
        # Automatic rollback on exception
        with self._conn:
            try:
                yield cursor
            finally:
                cursor.close()

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
            cursor.execute("BEGIN")
            cursor.executemany(
                "UPDATE vlobs SET remote_version = ? WHERE vlob_id = ?",
                ((version, entry_id.bytes) for entry_id, version in changed_vlobs.items()),
            )
            cursor.execute(
                """INSERT OR REPLACE INTO realm_checkpoint(_id, checkpoint)
                VALUES (0, ?)""",
                (new_checkpoint,),
            )
            cursor.execute("END")

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

        if not cache_only:
            await self._set_manifest_in_db(entry_id, manifest)

        else:
            self._cache_ahead_of_persistance_ids.add(entry_id)
        self._cache[entry_id] = manifest

    async def _set_manifest_in_db(self, entry_id: EntryID, manifest: LocalManifest) -> None:
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

    async def ensure_manifest_persistent(self, entry_id: EntryID) -> None:
        """
        Raises: Nothing !
        """
        assert isinstance(entry_id, EntryID)
        if entry_id not in self._cache_ahead_of_persistance_ids:
            return
        await self._ensure_manifest_persistent(entry_id)

    async def _flush_cache_ahead_of_persistance(self) -> None:
        for entry_id in self._cache_ahead_of_persistance_ids.copy():
            await self._ensure_manifest_persistent(entry_id)

    async def _ensure_manifest_persistent(self, entry_id: EntryID) -> None:
        manifest = self._cache[entry_id]
        await self._set_manifest_in_db(entry_id, manifest)
        self._cache_ahead_of_persistance_ids.remove(entry_id)

    async def clear_manifest(self, entry_id: EntryID) -> None:
        """
        Raises:
            FSLocalMissError
        """
        in_cache = bool(self._cache.pop(entry_id, None))
        if in_cache:
            self._cache_ahead_of_persistance_ids.discard(entry_id)
        async with self._open_cursor() as cursor:
            cursor.execute("DELETE FROM vlobs WHERE vlob_id = ?", (entry_id.bytes,))
            cursor.execute("SELECT changes()")
            deleted, = cursor.fetchone()
        if not deleted and not in_cache:
            raise FSLocalMissError(entry_id)
