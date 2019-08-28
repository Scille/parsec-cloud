# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from time import time
from typing import Set, Dict, Tuple
from pathlib import Path
from contextlib import contextmanager
from sqlite3 import Connection, connect as sqlite_connect

from parsec.core.types import EntryID, ChunkID, BlockID
from parsec.core.fs.exceptions import FSLocalMissError
from parsec.crypto import SecretKey, encrypt_raw_with_secret_key, decrypt_raw_with_secret_key
from parsec.core.types.local_manifests import DEFAULT_BLOCK_SIZE

# TODO: should be in config.py
DEFAULT_MAX_CACHE_SIZE = 128 * 1024 * 1024
DIRTY_VACUUM_THRESHOLD = 128 * 1024 * 1024


class PersistentStorage:
    """Manage the access to the persistent storage.

    That includes:
    - the sqlite database for clean data (block data)
    - the sqlite database for dirty data (manifests and chunk data)

    The only data that might be subject to garbage collection is the clean
    chunks since they are large and non-sensitive.

    The data is always bytes, this class doesn't have knowledge about object
    types nor serialization processes.
    """

    def __init__(self, key: SecretKey, path: Path, max_cache_size: int = DEFAULT_MAX_CACHE_SIZE):
        self.local_symkey = key
        self.path = Path(path)
        self.max_cache_size = max_cache_size
        self.dirty_conn = None
        self.clean_conn = None

    @property
    def dirty_data_path(self):
        return self.path / "dirty_data.sqlite"

    @property
    def clean_cache_path(self):
        return self.path / "clean_cache.sqlite"

    @property
    def block_limit(self):
        return self.max_cache_size // DEFAULT_BLOCK_SIZE

    # Life cycle

    def connect(self):
        if self.dirty_conn is not None or self.clean_conn is not None:
            raise RuntimeError("Already connected")

        # Create directories
        self.path.mkdir(parents=True, exist_ok=True)

        # Connect and initialize database
        self.dirty_conn = sqlite_connect(str(self.dirty_data_path))
        self.clean_conn = sqlite_connect(str(self.clean_cache_path))

        # Tune database access
        # Use auto-commit for dirty data since it is very sensitive
        self.dirty_conn.isolation_level = None
        self.dirty_conn.execute("pragma journal_mode=wal")
        self.dirty_conn.execute("PRAGMA synchronous = OFF")
        self.clean_conn.isolation_level = None
        self.clean_conn.execute("PRAGMA synchronous = OFF")
        self.clean_conn.execute("pragma journal_mode=wal")

        # Initialize
        self.create_db()

    def close(self):
        # Idempotency
        if self.dirty_conn is None and self.clean_conn is None:
            return

        # Write changes to the disk and close the connections
        try:
            # Dirty connection uses auto-commit
            # But let's perform a commit anyway, just in case
            self.dirty_conn.commit()
            self.dirty_conn.close()
            self.dirty_conn = None
        finally:
            self.clean_conn.commit()
            self.clean_conn.close()
            self.clean_conn = None

    # Context management

    def __enter__(self):
        self.connect()
        return self

    def __exit__(self, *args):
        self.close()

    # Cursor management

    @contextmanager
    def _open_cursor(self, conn: Connection):
        cursor = conn.cursor()
        # Automatic rollback on exception
        with conn:
            try:
                yield cursor
            finally:
                cursor.close()

    def open_dirty_cursor(self):
        return self._open_cursor(self.dirty_conn)

    def open_clean_cursor(self):
        return self._open_cursor(self.clean_conn)

    # Database initialization

    def create_db(self):
        with self.open_dirty_cursor() as dirty_cursor, self.open_clean_cursor() as clean_cursor:

            # Manifest tables
            dirty_cursor.execute(
                """CREATE TABLE IF NOT EXISTS manifests
                    (
                     manifest_id BLOB PRIMARY KEY NOT NULL, -- UUID
                     base_version INTEGER NOT NULL,
                     remote_version INTEGER NOT NULL,
                     need_sync INTEGER NOT NULL,  -- Boolean
                     blob BLOB NOT NULL);"""
            )

            # Singleton storing the checkpoint
            dirty_cursor.execute(
                """CREATE TABLE IF NOT EXISTS realm_checkpoint
                    (_id INTEGER PRIMARY KEY NOT NULL,
                     checkpoint INTEGER NOT NULL);"""
            )

            for cursor in (dirty_cursor, clean_cursor):

                # Chunks tables
                cursor.execute(
                    """CREATE TABLE IF NOT EXISTS chunks
                        (chunk_id BLOB PRIMARY KEY NOT NULL, -- UUID
                         size INTEGER NOT NULL,
                         offline INTEGER NOT NULL,  -- Boolean
                         accessed_on INTEGER, -- Timestamp
                         data BLOB NOT NULL
                    );"""
                )

    # Size and chunks

    def get_nb_clean_blocks(self):
        with self.open_clean_cursor() as cursor:
            cursor.execute("SELECT COUNT(*) FROM chunks")
            result, = cursor.fetchone()
            return result

    def get_cache_size(self):
        with self.open_clean_cursor() as cursor:
            cursor.execute("SELECT COALESCE(SUM(size), 0) FROM chunks")
            result, = cursor.fetchone()
            return result

    # Manifest operations

    def get_realm_checkpoint(self) -> int:
        with self.open_dirty_cursor() as cursor:
            cursor.execute("SELECT checkpoint FROM realm_checkpoint WHERE _id = 0")
            rep = cursor.fetchone()
            return rep[0] if rep else 0

    def update_realm_checkpoint(
        self, new_checkpoint: int, changed_vlobs: Dict[EntryID, int]
    ) -> None:
        with self.open_dirty_cursor() as cursor:
            cursor.execute("BEGIN")
            cursor.executemany(
                "UPDATE manifests SET remote_version = ? WHERE manifest_id = ?",
                ((version, entry_id.bytes) for entry_id, version in changed_vlobs.items()),
            )
            cursor.execute(
                """INSERT OR REPLACE INTO realm_checkpoint(_id, checkpoint)
                VALUES (0, ?)""",
                (new_checkpoint,),
            )
            cursor.execute("END")

    def get_need_sync_entries(self) -> Tuple[Set[EntryID], Set[EntryID]]:
        with self.open_dirty_cursor() as cursor:
            cursor.execute(
                "SELECT manifest_id, need_sync FROM manifests WHERE need_sync = 1 OR base_version != remote_version"
            )
            local_changes = set()
            remote_changes = set()
            for manifest_id, need_sync in cursor.fetchall():
                if need_sync:
                    local_changes.add(EntryID(manifest_id))
                else:
                    remote_changes.add(EntryID(manifest_id))
            return local_changes, remote_changes

    def get_manifest(self, entry_id: EntryID):
        with self.open_dirty_cursor() as cursor:
            cursor.execute(
                "SELECT manifest_id, blob FROM manifests WHERE manifest_id = ?", (entry_id.bytes,)
            )
            manifest_row = cursor.fetchone()
        if not manifest_row:
            raise FSLocalMissError(entry_id)
        manifest_id, blob = manifest_row
        return decrypt_raw_with_secret_key(self.local_symkey, blob)

    def set_manifest(self, entry_id: EntryID, base_version: int, need_sync: bool, raw: bytes):
        assert isinstance(raw, (bytes, bytearray))
        ciphered = encrypt_raw_with_secret_key(self.local_symkey, raw)

        with self.open_dirty_cursor() as cursor:
            cursor.execute(
                """INSERT OR REPLACE INTO manifests (manifest_id, blob, need_sync, base_version, remote_version)
                VALUES (
                    ?, ?, ?, ?,
                    max(
                        ?,
                        IFNULL((SELECT remote_version FROM manifests WHERE manifest_id=?), 0)
                    )
                )""",
                (entry_id.bytes, ciphered, need_sync, base_version, base_version, entry_id.bytes),
            )

    def clear_manifest(self, entry_id: EntryID):
        with self.open_dirty_cursor() as cursor:
            cursor.execute("DELETE FROM manifests WHERE manifest_id = ?", (entry_id.bytes,))
            cursor.execute("SELECT changes()")
            deleted, = cursor.fetchone()
        if not deleted:
            raise FSLocalMissError(entry_id)

    # Generic chunk operations

    def _is_chunk(self, conn: Connection, chunk_id: ChunkID):
        with self._open_cursor(conn) as cursor:
            cursor.execute("SELECT chunk_id FROM chunks WHERE chunk_id = ?", (chunk_id.bytes,))
            manifest_row = cursor.fetchone()
        return bool(manifest_row)

    def _get_chunk(self, conn: Connection, chunk_id: ChunkID):
        with self._open_cursor(conn) as cursor:
            cursor.execute("BEGIN")
            cursor.execute(
                """
                UPDATE chunks SET accessed_on = ? WHERE chunk_id = ?;
                """,
                (time(), chunk_id.bytes),
            )
            cursor.execute("SELECT changes()")
            changes, = cursor.fetchone()
            if not changes:
                raise FSLocalMissError(chunk_id)

            cursor.execute("""SELECT data FROM chunks WHERE chunk_id = ?""", (chunk_id.bytes,))
            ciphered, = cursor.fetchone()
            cursor.execute("END")

        return decrypt_raw_with_secret_key(self.local_symkey, ciphered)

    def _set_chunk(self, conn: Connection, chunk_id: ChunkID, raw: bytes):
        assert isinstance(raw, (bytes, bytearray))
        ciphered = encrypt_raw_with_secret_key(self.local_symkey, raw)

        # Update database
        with self._open_cursor(conn) as cursor:
            cursor.execute(
                """INSERT OR REPLACE INTO
                chunks (chunk_id, size, offline, accessed_on, data)
                VALUES (?, ?, ?, ?, ?)""",
                (chunk_id.bytes, len(ciphered), False, time(), ciphered),
            )

    def _clear_chunk(self, conn: Connection, chunk_id: ChunkID):
        with self._open_cursor(conn) as cursor:
            cursor.execute("BEGIN")
            cursor.execute("DELETE FROM chunks WHERE chunk_id = ?", (chunk_id.bytes,))
            cursor.execute("SELECT changes()")
            changes, = cursor.fetchone()
            cursor.execute("END")

        if not changes:
            raise FSLocalMissError(chunk_id)

    # Clean block operations

    def get_clean_block(self, chunk_id: BlockID):
        return self._get_chunk(self.clean_conn, chunk_id)

    def set_clean_block(self, chunk_id: BlockID, raw: bytes):
        self._set_chunk(self.clean_conn, chunk_id, raw)

        # Clean up if necessary
        limit = self.get_nb_clean_blocks() - self.block_limit
        if limit > 0:
            self.clear_clean_blocks(limit=limit)

    def clear_clean_block(self, chunk_id: BlockID):
        self._clear_chunk(self.clean_conn, chunk_id)

    # Dirty chunk operations

    def is_dirty_chunk(self, chunk_id: ChunkID):
        return self._is_chunk(self.dirty_conn, chunk_id)

    def get_dirty_chunk(self, chunk_id: ChunkID):
        return self._get_chunk(self.dirty_conn, chunk_id)

    def set_dirty_chunk(self, chunk_id: ChunkID, raw: bytes):
        self._set_chunk(self.dirty_conn, chunk_id, raw)

    def clear_dirty_chunk(self, chunk_id: ChunkID):
        self._clear_chunk(self.dirty_conn, chunk_id)

    # Garbage collection

    def clear_clean_blocks(self, limit=None):
        with self.open_clean_cursor() as cursor:
            if not limit:
                cursor.execute("DELETE FROM chunks")
            else:
                cursor.execute(
                    """
                    DELETE FROM chunks WHERE chunk_id IN (
                        SELECT chunk_id FROM chunks ORDER BY accessed_on ASC LIMIT ?
                    )
                    """,
                    (limit,),
                )

    def run_block_garbage_collector(self):
        self.clear_clean_blocks()

    # Vacuum

    def run_vacuum(self):
        # Vacuum is only necessary for the dirty database
        if self.dirty_data_path.stat().st_size > DIRTY_VACUUM_THRESHOLD:
            self.dirty_conn.execute("VACUUM")
