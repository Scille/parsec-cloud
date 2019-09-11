# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from pathlib import Path
from typing import Dict, Tuple, Set, Callable
from contextlib import contextmanager
from structlog import get_logger
from sqlite3 import connect as sqlite_connect

from parsec.core.fs.exceptions import FSLocalMissError
from parsec.core.types import EntryID, LocalDevice, LocalManifest, LocalUserManifest


logger = get_logger()


def _init_db(cursor):
    # Should only store the user manifest
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


@contextmanager
def _connect(dbpath: str):
    # Connect and initialize database
    conn = sqlite_connect(dbpath)

    # Tune database access
    conn.isolation_level = None
    conn.execute("pragma journal_mode=wal")
    conn.execute("PRAGMA synchronous = OFF")

    @contextmanager
    def _open_cursor():
        cursor = conn.cursor()
        # Automatic rollback on exception
        with conn:
            try:
                yield cursor
            finally:
                cursor.close()

    try:
        # Initialize db if needed
        with _open_cursor() as cursor:
            _init_db(cursor)

        yield _open_cursor

    finally:
        conn.commit()
        conn.close()


class RealmStorage:
    def __init__(self, device: LocalDevice, open_cursor: Callable, realm_id: EntryID):
        self.device = device
        self.realm_id = realm_id
        self._open_cursor = open_cursor
        self._cache = {}
        self._cache_ahead_of_persistance_ids = set()

    @classmethod
    @contextmanager
    def factory(cls, device: LocalDevice, path: Path, realm_id: EntryID):
        # Create directories if needed
        path.mkdir(parents=True, exist_ok=True)

        with _connect(str(path / "realmdb.sqlite")) as open_cursor:
            try:
                storage = cls(device, open_cursor, realm_id)
                yield storage

            finally:
                storage._flush_cache_ahead_of_persistance()

    # Manifest operations

    def get_realm_checkpoint(self) -> int:
        """
        Raises: Nothing !
        """
        with self._open_cursor() as cursor:
            cursor.execute("SELECT checkpoint FROM realm_checkpoint WHERE _id = 0")
            rep = cursor.fetchone()
            return rep[0] if rep else 0

    def update_realm_checkpoint(
        self, new_checkpoint: int, changed_vlobs: Dict[EntryID, int]
    ) -> None:
        """
        Raises: Nothing !
        """
        with self._open_cursor() as cursor:
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

    def get_need_sync_entries(self) -> Tuple[Set[EntryID], Set[EntryID]]:
        """
        Raises: Nothing !
        """
        with self._open_cursor() as cursor:
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

    def get_manifest(self, entry_id: EntryID) -> LocalManifest:
        """
        Raises:
            FSLocalMissError
        """
        try:
            return self._cache[entry_id]
        except KeyError:
            pass

        with self._open_cursor() as cursor:
            cursor.execute("SELECT blob FROM vlobs WHERE vlob_id = ?", (entry_id.bytes,))
            manifest_row = cursor.fetchone()
        if not manifest_row:
            raise FSLocalMissError(entry_id)

        manifest = LocalManifest.decrypt_and_load(manifest_row[0], key=self.device.local_symkey)
        self._cache[entry_id] = manifest

        return manifest

    def set_manifest(
        self, entry_id: EntryID, manifest: LocalManifest, cache_only: bool = False
    ) -> None:
        """
        Raises: Nothing !
        """
        assert isinstance(entry_id, EntryID)

        if not cache_only:
            self._set_manifest_in_db(entry_id, manifest)

        else:
            self._cache_ahead_of_persistance_ids.add(entry_id)
        self._cache[entry_id] = manifest

    def _set_manifest_in_db(self, entry_id: EntryID, manifest: LocalManifest) -> None:
        ciphered = manifest.dump_and_encrypt(self.device.local_symkey)
        with self._open_cursor() as cursor:
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

    def ensure_manifest_persistent(self, entry_id: EntryID) -> None:
        """
        Raises: Nothing !
        """
        assert isinstance(entry_id, EntryID)
        self._check_lock_status(entry_id)
        if entry_id not in self._cache_ahead_of_persistance_ids:
            return
        self._ensure_manifest_persistent(entry_id)

    def _flush_cache_ahead_of_persistance(self) -> None:
        for entry_id in self._cache_ahead_of_persistance_ids.copy():
            self._ensure_manifest_persistent(entry_id)

    def _ensure_manifest_persistent(self, entry_id: EntryID) -> None:
        manifest = self.local_manifest_cache[entry_id]
        self._set_manifest_in_db(entry_id, manifest)
        self._cache_ahead_of_persistance_ids.remove(entry_id)

    def clear_manifest(self, entry_id: EntryID) -> None:
        """
        Raises:
            FSLocalMissError
        """
        in_cache = bool(self._cache.pop(entry_id, None))
        if in_cache:
            self._cache_ahead_of_persistance_ids.discard(entry_id)
        with self._open_cursor() as cursor:
            cursor.execute("DELETE FROM vlobs WHERE vlob_id = ?", (entry_id.bytes,))
            cursor.execute("SELECT changes()")
            deleted, = cursor.fetchone()
        if not deleted and not in_cache:
            raise FSLocalMissError(entry_id)

    def run_vacuum(self) -> None:
        pass


class UserStorage(RealmStorage):
    @property
    def user_manifest_id(self):
        return self.realm_id

    @classmethod
    @contextmanager
    def factory(cls, device: LocalDevice, path: Path, user_manifest_id: EntryID):
        with super().factory(device, path, user_manifest_id) as storage:

            # Load the user manifest
            try:
                storage.get_manifest(storage.user_manifest_id)
            except FSLocalMissError:
                pass
            else:
                assert storage.user_manifest_id in storage._cache

            yield storage

    def get_user_manifest(self):
        try:
            return self._cache[self.user_manifest_id]
        except KeyError:
            # In the unlikely event the user manifest is not present in
            # local (e.g. device just created or during tests), we fall
            # back on an empty manifest which is a good aproximation of
            # the very first version of the manifest (field `created` is
            # invalid, but it will be corrected by the merge during sync).
            return LocalUserManifest.new_placeholder(id=self.device.user_manifest_id)

    def set_user_manifest(self, user_manifest):
        self.set_manifest(self.user_manifest_id, user_manifest)
