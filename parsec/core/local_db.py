# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
from pathlib import Path
from contextlib import contextmanager
from sqlite3 import Connection, connect as sqlite_connect

import pendulum

from parsec.core.types.access import ManifestAccess
from parsec.crypto import encrypt_raw_with_secret_key, decrypt_raw_with_secret_key


# Alias now
now = pendulum.Pendulum.now

# TODO: shouldn't use core.fs.types.Acces here
# from parsec.core.fs.types import Access
Access = None  # TODO: hack to fix recursive import

# TODO: should be in config.py
DEFAULT_MAX_CACHE_SIZE = 128 * 1024 * 1024
DEFAULT_BLOCK_SIZE = 2 ** 16


class LocalDBError(Exception):
    pass


class LocalDBMissingEntry(LocalDBError):
    def __init__(self, access):
        self.access = access


class LocalDB:
    def __init__(self, path: Path, max_cache_size: int = DEFAULT_MAX_CACHE_SIZE):
        self.local_conn = None
        self.remote_conn = None
        self._path = Path(path)
        self._remote_db_files = self._path / "remote_cache"
        self._local_db_files = self._path / "local_data"
        self.max_cache_size = max_cache_size

    @property
    def path(self):
        return str(self._path)

    @property
    def block_limit(self):
        return self.max_cache_size // DEFAULT_BLOCK_SIZE

    # Life cycle

    def connect(self):
        if self.local_conn is not None or self.remote_conn is not None:
            raise RuntimeError("Already connected")

        # Create directories
        self._path.mkdir(parents=True, exist_ok=True)
        self._remote_db_files.mkdir(parents=True, exist_ok=True)
        self._local_db_files.mkdir(parents=True, exist_ok=True)

        # Connect and initialize database
        self.local_conn = sqlite_connect(str(self._path / "local_data.sqlite"))
        self.remote_conn = sqlite_connect(str(self._path / "remote_cache.sqlite"))

        # Use auto-commit for local data since it is very sensitive
        self.local_conn.isolation_level = None

        # Initialize
        self.create_db()

    def close(self):
        # Idempotency
        if self.local_conn is None and self.remote_conn is None:
            return

        # Write changes to the disk and close the connections
        try:
            # Local connection uses auto-commit
            # But let's perform a commit anyway, just in case
            self.local_conn.commit()
            self.local_conn.close()
            self.local_conn = None
        finally:
            self.remote_conn.commit()
            self.remote_conn.close()
            self.remote_conn = None

    # Context management

    def __enter__(self):
        self.connect()
        return self

    def __exit__(self, *args):
        self.close()

    # Cursor management

    @contextmanager
    def _open_cursor(self, conn: Connection):
        try:
            cursor = conn.cursor()
            yield cursor
        finally:
            cursor.close()

    def open_local_cursor(self):
        return self._open_cursor(self.local_conn)

    def open_remote_cursor(self):
        return self._open_cursor(self.remote_conn)

    # Database initialization

    def create_db(self):
        with self.open_local_cursor() as local_cursor, self.open_remote_cursor() as remote_cursor:

            # User table
            local_cursor.execute(
                """CREATE TABLE IF NOT EXISTS users
                    (access_id UUID PRIMARY KEY NOT NULL,
                     blob BYTEA NOT NULL,
                     inserted_on TIMESTAMPTZ);"""
            )

            for cursor in (local_cursor, remote_cursor):

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

    @property
    def nb_remote_blocks(self):
        with self.open_remote_cursor() as cursor:
            cursor.execute("SELECT COUNT(*) FROM blocks")
            result, = cursor.fetchone()
            return result

    def get_cache_size(self):
        with self.open_remote_cursor() as cursor:
            cursor.execute("SELECT COALESCE(SUM(size), 0) FROM blocks")
            result, = cursor.fetchone()
            return result

    def get_block_cache_size(self):
        cache = str(self._remote_db_files)
        return sum(
            os.path.getsize(os.path.join(cache, f))
            for f in os.listdir(cache)
            if os.path.isfile(os.path.join(cache, f))
        )

    # User operations

    def get_user(self, access: Access):
        with self.open_local_cursor() as cursor:
            cursor.execute(
                "SELECT access_id, blob, inserted_on FROM users WHERE access_id = ?",
                (str(access.id),),
            )
            user_row = cursor.fetchone()
        if not user_row or pendulum.parse(user_row[2]).add(hours=1) <= now():
            raise LocalDBMissingEntry(access)
        access_id, blob, created_on = user_row
        return decrypt_raw_with_secret_key(access.key, blob)

    def set_user(self, access: Access, raw: bytes):
        assert isinstance(raw, (bytes, bytearray))
        ciphered = encrypt_raw_with_secret_key(access.key, raw)
        with self.open_local_cursor() as cursor:
            cursor.execute(
                """INSERT OR REPLACE INTO users (access_id, blob, inserted_on)
                VALUES (?, ?, ?)""",
                (str(access.id), ciphered, str(now())),
            )

    # Generic manifest operations

    def _check_presence(self, conn: Connection, access: Access):
        with self._open_cursor(conn) as cursor:
            cursor.execute(
                "SELECT manifest_id FROM manifests WHERE manifest_id = ?", (str(access.id),)
            )
            row = cursor.fetchone()
        return bool(row)

    def _get_manifest(self, conn: Connection, access: Access):
        with self._open_cursor(conn) as cursor:
            cursor.execute(
                "SELECT manifest_id, blob FROM manifests WHERE manifest_id = ?", (str(access.id),)
            )
            manifest_row = cursor.fetchone()
        if not manifest_row:
            raise LocalDBMissingEntry(access)
        manifest_id, blob = manifest_row
        return decrypt_raw_with_secret_key(access.key, blob)

    def _set_manifest(self, conn: Connection, access: Access, raw: bytes):
        assert isinstance(raw, (bytes, bytearray))
        ciphered = encrypt_raw_with_secret_key(access.key, raw)

        with self._open_cursor(conn) as cursor:
            cursor.execute(
                """INSERT OR REPLACE INTO manifests (manifest_id, blob)
                VALUES (?, ?)""",
                (str(access.id), ciphered),
            )

    def _clear_manifest(self, conn: Connection, access: Access):
        with self._open_cursor(conn) as cursor:
            cursor.execute("DELETE FROM manifests WHERE manifest_id = ?", (str(access.id),))
            cursor.execute("SELECT changes()")
            deleted, = cursor.fetchone()
        if not deleted:
            raise LocalDBMissingEntry(access)

    # Remote manifest operations

    def get_remote_manifest(self, access: Access):
        return self._get_manifest(self.remote_conn, access)

    def set_remote_manifest(self, access: Access, raw: bytes):
        if self._check_presence(self.local_conn, access):
            raise ValueError("Cannot set remote manifest: a local manifest is already present")
        self._set_manifest(self.remote_conn, access, raw)

    def clear_remote_manifest(self, access: Access):
        self._clear_manifest(self.remote_conn, access)

    # Local manifest operations

    def get_local_manifest(self, access: Access):
        return self._get_manifest(self.local_conn, access)

    def set_local_manifest(self, access: Access, raw: bytes):
        if self._check_presence(self.remote_conn, access):
            raise ValueError("Cannot set local manifest: a remote manifest is already present")
        self._set_manifest(self.local_conn, access, raw)

    def clear_local_manifest(self, access: Access):
        self._clear_manifest(self.local_conn, access)

    # Generic block operations

    def _get_block(self, conn: Connection, path: Path, access: Access):
        with self._open_cursor(conn) as cursor:
            cursor.execute(
                """UPDATE blocks SET accessed_on = ? WHERE block_id = ?""",
                (str(now()), str(access.id)),
            )
            cursor.execute("SELECT changes()")
            changes, = cursor.fetchone()

        if not changes:
            raise LocalDBMissingEntry(access)

        ciphered = self._read_file(access, path)
        return decrypt_raw_with_secret_key(access.key, ciphered)

    def _set_block(self, conn: Connection, path: Path, access: Access, raw: bytes):
        assert isinstance(raw, (bytes, bytearray))
        filepath = path / str(access.id)
        ciphered = encrypt_raw_with_secret_key(access.key, raw)

        # Update database
        with self._open_cursor(conn) as cursor:
            cursor.execute(
                """INSERT OR REPLACE INTO
                blocks (block_id, size, offline, accessed_on, file_path)
                VALUES (?, ?, ?, ?, ?)""",
                (str(access.id), len(ciphered), False, str(now()), str(filepath)),
            )

        # Write file
        self._write_file(access, ciphered, path)

    def _clear_block(self, conn: Connection, path: Path, access: Access):
        with self._open_cursor(conn) as cursor:
            cursor.execute("DELETE FROM blocks WHERE block_id = ?", (str(access.id),))
            cursor.execute("SELECT changes()")
            changes, = cursor.fetchone()

        if not changes:
            raise LocalDBMissingEntry(access)

        self._remove_file(access, path)

    # Remote block operations

    def get_remote_block(self, access: Access):
        return self._get_block(self.remote_conn, self._remote_db_files, access)

    def set_remote_block(self, access: Access, raw: bytes):
        self._set_block(self.remote_conn, self._remote_db_files, access, raw)

        # Clean up if necessary
        limit = self.nb_remote_blocks - self.block_limit
        if limit > 0:
            self.cleanup_remote_blocks(limit=limit)

    def clear_remote_block(self, access):
        self._clear_block(self.remote_conn, self._remote_db_files, access)

    # Local block operations

    def get_local_block(self, access: Access):
        return self._get_block(self.local_conn, self._local_db_files, access)

    def set_local_block(self, access: Access, raw: bytes):
        self._set_block(self.local_conn, self._local_db_files, access, raw)

    def clear_local_block(self, access):
        self._clear_block(self.local_conn, self._local_db_files, access)

    # Block file operations

    def _read_file(self, access: Access, path: Path):
        filepath = path / str(access.id)
        try:
            return filepath.read_bytes()
        except FileNotFoundError:
            raise LocalDBMissingEntry(access)

    def _write_file(self, access: Access, content: bytes, path: Path):
        filepath = path / str(access.id)
        filepath.write_bytes(content)

    def _remove_file(self, access: Access, path: Path):
        filepath = path / str(access.id)
        try:
            filepath.unlink()
        except FileNotFoundError:
            raise LocalDBMissingEntry(access)

    # Garbage collection

    def cleanup_remote_blocks(self, limit=None):
        limit_string = f" LIMIT {limit}" if limit is not None else ""

        with self.open_remote_cursor() as cursor:
            cursor.execute(
                """
                SELECT block_id FROM blocks ORDER BY accessed_on ASC
                """
                + limit_string
            )
            block_ids = [block_id for (block_id,) in cursor.fetchall()]

        for block_id in block_ids:
            self.clear_remote_block(ManifestAccess(block_id))

    def run_block_garbage_collector(self):
        self.cleanup_remote_blocks()
