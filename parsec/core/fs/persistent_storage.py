# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from time import time
from pathlib import Path
from contextlib import contextmanager
from sqlite3 import Connection, connect as sqlite_connect

from parsec.core.types import EntryID, BlockID
from parsec.core.fs.exceptions import FSLocalMissError
from parsec.crypto import SecretKey, encrypt_raw_with_secret_key, decrypt_raw_with_secret_key


# TODO: should be in config.py
DEFAULT_MAX_CACHE_SIZE = 128 * 1024 * 1024
DEFAULT_BLOCK_SIZE = 2 ** 16


class PersistentStorage:
    """Manage the access to the persistent storage.

    That includes:
    - the sqlite database for clean data (block metadata)
    - the sqlite database for dirty data (manifests and block metadata)
    - the clean block files
    - the dirty block files

    The only data that might be subject to garbage collection is the clean
    blocks since they are large and non-sensitive.

    The data is always bytes, this class doesn't have knowledge about object
    types nor serialization processes.
    """

    def __init__(self, key: SecretKey, path: Path, max_cache_size: int = DEFAULT_MAX_CACHE_SIZE):
        self.local_symkey = key
        self._path = Path(path)
        self.max_cache_size = max_cache_size
        self.dirty_conn = None
        self.clean_conn = None

    # TODO: really needed ?
    @property
    def path(self):
        return str(self._path)

    @property
    def block_limit(self):
        return self.max_cache_size // DEFAULT_BLOCK_SIZE

    # Life cycle

    def connect(self):
        if self.dirty_conn is not None or self.clean_conn is not None:
            raise RuntimeError("Already connected")

        # Create directories
        self._path.mkdir(parents=True, exist_ok=True)

        # Connect and initialize database
        self.dirty_conn = sqlite_connect(str(self._path / "dirty_data.sqlite"))
        self.clean_conn = sqlite_connect(str(self._path / "clean_cache.sqlite"))

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

            for cursor in (dirty_cursor, clean_cursor):

                # Manifest tables
                cursor.execute(
                    """CREATE TABLE IF NOT EXISTS manifests
                        (manifest_id UUID PRIMARY KEY NOT NULL,
                         blob BYTEA NOT NULL);"""
                )

                # Blocks tables
                cursor.execute(
                    """CREATE TABLE IF NOT EXISTS blocks
                        (block_id UUID PRIMARY KEY NOT NULL,
                         size INT NOT NULL,
                         offline BOOLEAN NOT NULL,
                         accessed_on TIMESTAMPTZ,
                         data BLOB NOT NULL
                    );"""
                )

    # Size and blocks

    def get_nb_clean_blocks(self):
        with self.open_clean_cursor() as cursor:
            cursor.execute("SELECT COUNT(*) FROM blocks")
            result, = cursor.fetchone()
            return result

    def get_cache_size(self):
        with self.open_clean_cursor() as cursor:
            cursor.execute("SELECT COALESCE(SUM(size), 0) FROM blocks")
            result, = cursor.fetchone()
            return result

    # Manifest operations

    def get_manifest(self, entry_id: EntryID):
        with self.open_dirty_cursor() as cursor:
            cursor.execute(
                "SELECT manifest_id, blob FROM manifests WHERE manifest_id = ?", (str(entry_id),)
            )
            manifest_row = cursor.fetchone()
        if not manifest_row:
            raise FSLocalMissError(entry_id)
        manifest_id, blob = manifest_row
        return decrypt_raw_with_secret_key(self.local_symkey, blob)

    def set_manifest(self, entry_id: EntryID, raw: bytes):
        assert isinstance(raw, (bytes, bytearray))
        ciphered = encrypt_raw_with_secret_key(self.local_symkey, raw)

        with self.open_dirty_cursor() as cursor:
            cursor.execute(
                """INSERT OR REPLACE INTO manifests (manifest_id, blob)
                VALUES (?, ?)""",
                (str(entry_id), ciphered),
            )

    def clear_manifest(self, entry_id: EntryID):
        with self.open_dirty_cursor() as cursor:
            cursor.execute("DELETE FROM manifests WHERE manifest_id = ?", (str(entry_id),))
            cursor.execute("SELECT changes()")
            deleted, = cursor.fetchone()
        if not deleted:
            raise FSLocalMissError(entry_id)

    # Generic block operations

    def _is_block(self, conn: Connection, block_id: BlockID):
        with self._open_cursor(conn) as cursor:
            cursor.execute("SELECT block_id FROM blocks WHERE block_id = ?", (str(block_id),))
            manifest_row = cursor.fetchone()
        return bool(manifest_row)

    def _get_block(self, conn: Connection, block_id: BlockID):
        with self._open_cursor(conn) as cursor:
            cursor.execute("BEGIN")
            cursor.execute(
                """
                UPDATE blocks SET accessed_on = ? WHERE block_id = ?;
                """,
                (time(), str(block_id)),
            )
            cursor.execute("SELECT changes()")
            changes, = cursor.fetchone()
            if not changes:
                raise FSLocalMissError(block_id)

            cursor.execute("""SELECT data FROM blocks WHERE block_id = ?""", (str(block_id),))
            ciphered, = cursor.fetchone()
            cursor.execute("END")

        return decrypt_raw_with_secret_key(self.local_symkey, ciphered)

    def _set_block(self, conn: Connection, block_id: BlockID, raw: bytes):
        assert isinstance(raw, (bytes, bytearray))
        ciphered = encrypt_raw_with_secret_key(self.local_symkey, raw)

        # Update database
        with self._open_cursor(conn) as cursor:
            cursor.execute(
                """INSERT OR REPLACE INTO
                blocks (block_id, size, offline, accessed_on, data)
                VALUES (?, ?, ?, ?, ?)""",
                (str(block_id), len(ciphered), False, time(), ciphered),
            )

    def _clear_block(self, conn: Connection, block_id: BlockID):
        with self._open_cursor(conn) as cursor:
            cursor.execute("BEGIN")
            cursor.execute("DELETE FROM blocks WHERE block_id = ?", (str(block_id),))
            cursor.execute("SELECT changes()")
            changes, = cursor.fetchone()
            cursor.execute("END")

        if not changes:
            raise FSLocalMissError(block_id)

    # Clean block operations

    def get_clean_block(self, block_id: BlockID):
        return self._get_block(self.clean_conn, block_id)

    def set_clean_block(self, block_id: BlockID, raw: bytes):
        self._set_block(self.clean_conn, block_id, raw)

        # Clean up if necessary
        limit = self.get_nb_clean_blocks() - self.block_limit
        if limit > 0:
            self.clear_clean_blocks(limit=limit)

    def clear_clean_block(self, block_id: BlockID):
        self._clear_block(self.clean_conn, block_id)

    # Dirty block operations

    def is_dirty_block(self, block_id: BlockID):
        return self._is_block(self.dirty_conn, block_id)

    def get_dirty_block(self, block_id: BlockID):
        return self._get_block(self.dirty_conn, block_id)

    def set_dirty_block(self, block_id: BlockID, raw: bytes):
        self._set_block(self.dirty_conn, block_id, raw)

    def clear_dirty_block(self, block_id: BlockID):
        self._clear_block(self.dirty_conn, block_id)

    # Garbage collection

    def clear_clean_blocks(self, limit=None):
        with self.open_clean_cursor() as cursor:
            if not limit:
                cursor.execute("DELETE FROM blocks")
            else:
                cursor.execute(
                    """
                    DELETE FROM blocks WHERE block_id IN (
                        SELECT block_id FROM blocks ORDER BY accessed_on ASC LIMIT ?
                    )
                    """,
                    (limit,),
                )

    def run_block_garbage_collector(self):
        self.clear_clean_blocks()
