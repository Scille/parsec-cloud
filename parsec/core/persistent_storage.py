# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
from pathlib import Path
from contextlib import contextmanager
from sqlite3 import Connection, connect as sqlite_connect

import pendulum

from parsec.core.types import EntryID, BlockID
from parsec.crypto import SecretKey, encrypt_raw_with_secret_key, decrypt_raw_with_secret_key


# TODO: should be in config.py
DEFAULT_MAX_CACHE_SIZE = 128 * 1024 * 1024
DEFAULT_BLOCK_SIZE = 2 ** 16


class LocalStorageError(Exception):
    pass


class LocalStorageMissingError(LocalStorageError):
    def __init__(self, id):
        self.id = id


class PersistentStorage:
    """Manage the access to the persistent storage.

    That includes:
    - the sqlite database for clean data (manifests and block metadata)
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
        self._clean_db_files = self._path / "clean_data_cache"
        self._dirty_db_files = self._path / "dirty_data_storage"

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
        self._clean_db_files.mkdir(parents=True, exist_ok=True)
        self._dirty_db_files.mkdir(parents=True, exist_ok=True)

        # Connect and initialize database
        self.dirty_conn = sqlite_connect(str(self._path / "dirty_data.sqlite"))
        self.clean_conn = sqlite_connect(str(self._path / "clean_cache.sqlite"))

        # Use auto-commit for dirty data since it is very sensitive
        self.dirty_conn.isolation_level = None

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
                         file_path TEXT NOT NULL);"""
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

    def get_block_cache_size(self):
        cache = str(self._clean_db_files)
        return sum(
            os.path.getsize(os.path.join(cache, f))
            for f in os.listdir(cache)
            if os.path.isfile(os.path.join(cache, f))
        )

    # Generic manifest operations

    def _get_manifest(self, conn: Connection, entry_id: EntryID):
        with self._open_cursor(conn) as cursor:
            cursor.execute(
                "SELECT manifest_id, blob FROM manifests WHERE manifest_id = ?", (str(entry_id),)
            )
            manifest_row = cursor.fetchone()
        if not manifest_row:
            raise LocalStorageMissingError(entry_id)
        manifest_id, blob = manifest_row
        return decrypt_raw_with_secret_key(self.local_symkey, blob)

    def _set_manifest(self, conn: Connection, entry_id: EntryID, raw: bytes):
        assert isinstance(raw, (bytes, bytearray))
        ciphered = encrypt_raw_with_secret_key(self.local_symkey, raw)

        with self._open_cursor(conn) as cursor:
            cursor.execute(
                """INSERT OR REPLACE INTO manifests (manifest_id, blob)
                VALUES (?, ?)""",
                (str(entry_id), ciphered),
            )

    def _clear_manifest(self, conn: Connection, entry_id: EntryID):
        with self._open_cursor(conn) as cursor:
            cursor.execute("DELETE FROM manifests WHERE manifest_id = ?", (str(entry_id),))
            cursor.execute("SELECT changes()")
            deleted, = cursor.fetchone()
        if not deleted:
            raise LocalStorageMissingError(entry_id)

    # Clean manifest operations

    def get_clean_manifest(self, entry_id: EntryID):
        return self._get_manifest(self.clean_conn, entry_id)

    def set_clean_manifest(self, entry_id: EntryID, raw: bytes):
        self._set_manifest(self.clean_conn, entry_id, raw)

    def clear_clean_manifest(self, entry_id: EntryID):
        self._clear_manifest(self.clean_conn, entry_id)

    # Dirty manifest operations

    def get_dirty_manifest(self, entry_id: EntryID):
        return self._get_manifest(self.dirty_conn, entry_id)

    def set_dirty_manifest(self, entry_id: EntryID, raw: bytes):
        self._set_manifest(self.dirty_conn, entry_id, raw)

    def clear_dirty_manifest(self, entry_id: EntryID):
        self._clear_manifest(self.dirty_conn, entry_id)

    # Generic block operations

    def _get_block(self, conn: Connection, path: Path, block_id: BlockID):
        with self._open_cursor(conn) as cursor:
            cursor.execute(
                """UPDATE blocks SET accessed_on = ? WHERE block_id = ?""",
                (str(pendulum.now()), str(block_id)),
            )
            cursor.execute("SELECT changes()")
            changes, = cursor.fetchone()

        if not changes:
            raise LocalStorageMissingError(block_id)

        ciphered = self._read_file(block_id, path)
        return decrypt_raw_with_secret_key(self.local_symkey, ciphered)

    def _set_block(self, conn: Connection, path: Path, block_id: BlockID, raw: bytes):
        assert isinstance(raw, (bytes, bytearray))
        filepath = path / str(block_id)
        ciphered = encrypt_raw_with_secret_key(self.local_symkey, raw)

        # Update database
        with self._open_cursor(conn) as cursor:
            cursor.execute(
                """INSERT OR REPLACE INTO
                blocks (block_id, size, offline, accessed_on, file_path)
                VALUES (?, ?, ?, ?, ?)""",
                # TODO: better serialization of DateTime
                (str(block_id), len(ciphered), False, str(pendulum.now()), str(filepath)),
            )

        # Write file
        self._write_file(block_id, ciphered, path)

    def _clear_block(self, conn: Connection, path: Path, block_id: BlockID):
        with self._open_cursor(conn) as cursor:
            cursor.execute("DELETE FROM blocks WHERE block_id = ?", (str(block_id),))
            cursor.execute("SELECT changes()")
            changes, = cursor.fetchone()

        if not changes:
            raise LocalStorageMissingError(block_id)

        self._remove_file(block_id, path)

    # Clean block operations

    def get_clean_block(self, block_id: BlockID):
        return self._get_block(self.clean_conn, self._clean_db_files, block_id)

    def set_clean_block(self, block_id: BlockID, raw: bytes):
        self._set_block(self.clean_conn, self._clean_db_files, block_id, raw)

        # Clean up if necessary
        limit = self.get_nb_clean_blocks() - self.block_limit
        if limit > 0:
            self.clear_clean_blocks(limit=limit)

    def clear_clean_block(self, block_id: BlockID):
        self._clear_block(self.clean_conn, self._clean_db_files, block_id)

    # Dirty block operations

    def get_dirty_block(self, block_id: BlockID):
        return self._get_block(self.dirty_conn, self._dirty_db_files, block_id)

    def set_dirty_block(self, block_id: BlockID, raw: bytes):
        self._set_block(self.dirty_conn, self._dirty_db_files, block_id, raw)

    def clear_dirty_block(self, block_id: BlockID):
        self._clear_block(self.dirty_conn, self._dirty_db_files, block_id)

    # Block file operations

    def _read_file(self, block_id: BlockID, path: Path):
        filepath = path / str(block_id)
        try:
            return filepath.read_bytes()
        except FileNotFoundError:
            raise LocalStorageMissingError(block_id)

    def _write_file(self, block_id: BlockID, content: bytes, path: Path):
        filepath = path / str(block_id)
        filepath.write_bytes(content)

    def _remove_file(self, block_id: BlockID, path: Path):
        filepath = path / str(block_id)
        try:
            filepath.unlink()
        except FileNotFoundError:
            raise LocalStorageMissingError(block_id)

    # Garbage collection

    def clear_clean_blocks(self, limit=None):
        limit_string = f" LIMIT {limit}" if limit is not None else ""

        with self.open_clean_cursor() as cursor:
            cursor.execute(
                """
                SELECT block_id FROM blocks ORDER BY accessed_on ASC
                """
                + limit_string
            )
            block_ids = [BlockID(block_id) for (block_id,) in cursor.fetchall()]

        for block_id in block_ids:
            self.clear_clean_block(block_id)

    def run_block_garbage_collector(self):
        self.clear_clean_blocks()
