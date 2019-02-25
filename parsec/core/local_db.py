# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
from pathlib import Path
import pendulum
from pendulum import Pendulum
import sqlite3

from parsec.core.types.access import ManifestAccess
from parsec.crypto import encrypt_raw_with_secret_key, decrypt_raw_with_secret_key

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
        self.conn = None
        self.nb_blocks = 0
        self._path = Path(path)
        self._db_files = self._path / "cache"
        self.max_cache_size = max_cache_size

    @property
    def path(self):
        return str(self._path)

    @property
    def block_limit(self):
        return self.max_cache_size // DEFAULT_BLOCK_SIZE

    # Life cycle

    def connect(self):
        if self.conn is not None:
            raise RuntimeError("Already connected")

        # Create directories
        self._path.mkdir(parents=True, exist_ok=True)
        self._db_files.mkdir(parents=True, exist_ok=True)

        # Connect and initialize database
        self.conn = sqlite3.connect(str(self._path / "cache.sqlite"))

        # Use auto-commit
        self.conn.isolation_level = None

        # Initialize
        self.create_db()
        self.nb_blocks = self.get_nb_blocks()

    def close(self):
        # Idempotency
        if self.conn is None:
            return

        # Commit, just in case
        self.conn.commit()

        # Close the connection
        self.conn.close()
        self.conn = None

    # Context management

    def __enter__(self):
        self.connect()
        return self

    def __exit__(self, *args):
        self.close()

    # Database initialization

    def create_db(self):
        cursor = self.conn.cursor()
        cursor.execute(
            """CREATE TABLE IF NOT EXISTS users
                (_id SERIAL PRIMARY KEY,
                 user_id UUID NOT NULL,
                 blob BYTEA NOT NULL,
                 created_on TIMESTAMPTZ);"""
        )
        cursor.execute(
            """CREATE TABLE IF NOT EXISTS vlobs
                (_id SERIAL PRIMARY KEY,
                 vlob_id UUID NOT NULL,
                 blob BYTEA NOT NULL,
                 deletable BOOLEAN NOT NULL);"""
        )
        cursor.execute(
            """CREATE TABLE IF NOT EXISTS blocks
                (block_id INT PRIMARY KEY NOT NULL,
                 size INT NOT NULL,
                 deletable BOOLEAN NOT NULL,
                 offline BOOLEAN NOT NULL,
                 accessed_on TIMESTAMPTZ,
                 file_path TEXT NOT NULL);"""
        )
        cursor.close()

    # Size and blocks

    def get_nb_blocks(self):
        cursor = self.conn.cursor()
        res = cursor.execute("SELECT COUNT(block_id) FROM blocks WHERE deletable = 1")
        res = res.fetchone()
        cursor.close()
        return res[0]

    def get_cache_size(self):
        cursor = self.conn.cursor()
        res = cursor.execute("SELECT SUM(size) FROM blocks WHERE deletable = 1")
        res = res.fetchone()
        cursor.close()
        return res[0] if res[0] else 0

    # User operations

    def get_user(self, access: Access):
        cursor = self.conn.cursor()
        res = cursor.execute(
            "SELECT user_id, blob, created_on FROM users WHERE user_id = ?", (str(access.id),)
        )
        user = res.fetchone()
        cursor.close()
        if not user or pendulum.parse(user[2]).add(hours=1) <= Pendulum.now():
            raise LocalDBMissingEntry(access)
        return decrypt_raw_with_secret_key(access.key, user[1])

    def set_user(self, access: Access, raw: bytes):
        assert isinstance(raw, (bytes, bytearray))
        ciphered = encrypt_raw_with_secret_key(access.key, raw)
        cursor = self.conn.cursor()
        cursor.execute("DELETE FROM users WHERE user_id = ? ", (str(access.id),))
        cursor.execute(
            """INSERT INTO users (user_id, blob, created_on)
               VALUES (?, ?, ?)""",
            (str(access.id), ciphered, str(Pendulum.now())),
        )
        cursor.close()

    # Manifest operations

    def get_manifest(self, access: Access):
        cursor = self.conn.cursor()
        res = cursor.execute("SELECT vlob_id, blob FROM vlobs WHERE vlob_id = ?", (str(access.id),))
        vlob = res.fetchone()
        cursor.close()
        if not vlob:
            raise LocalDBMissingEntry(access)
        return decrypt_raw_with_secret_key(access.key, vlob[1])

    def set_manifest(self, access: Access, raw: bytes, deletable: bool = True):
        assert isinstance(raw, (bytes, bytearray))
        ciphered = encrypt_raw_with_secret_key(access.key, raw)

        if deletable:
            pass
            # TODO clean
            # if self.get_block_cache_size() + len(ciphered) > self.max_cache_size:
            #     self.run_block_garbage_collector()

        cursor = self.conn.cursor()
        cursor.execute("DELETE FROM vlobs WHERE vlob_id = ? ", (str(access.id),))
        cursor.execute(
            """INSERT INTO vlobs (vlob_id, blob, deletable)
               VALUES (?, ?, ?)""",
            (str(access.id), ciphered, deletable),
        )
        cursor.close()

    # Block operations

    def get_block_cache_size(self):
        cache = str(self._db_files)
        return sum(
            os.path.getsize(os.path.join(cache, f))
            for f in os.listdir(cache)
            if os.path.isfile(os.path.join(cache, f))
        )

    def get_block(self, access: Access):
        file = self._db_files / str(access.id)
        if not file:
            raise LocalDBMissingEntry(access)
        cursor = self.conn.cursor()
        cursor.execute(
            """UPDATE blocks SET accessed_on = ? WHERE block_id = ? and deletable = 0""",
            (str(Pendulum.now()), str(access.id)),
        )
        cursor.close()
        ciphered = self._read_file(access)
        return decrypt_raw_with_secret_key(access.key, ciphered)

    def set_block(self, access: Access, raw: bytes, deletable: bool = True):
        assert isinstance(raw, (bytes, bytearray))
        file = self._db_files / str(access.id)
        ciphered = encrypt_raw_with_secret_key(access.key, raw)

        # Update database
        cursor = self.conn.cursor()
        cursor.execute(
            """INSERT OR REPLACE INTO blocks (block_id, size, deletable, offline, file_path)
            VALUES (?, ?, ?, ?, ?)""",
            (str(access.id), len(ciphered), deletable, False, str(file)),
        )
        cursor.close()

        # Write file
        self._write_file(access, ciphered)
        self.nb_blocks += 1

        # Clean up if necessary
        if deletable and self.nb_blocks > self.block_limit:
            limit = self.nb_blocks - self.block_limit
            self.cleanup_blocks(limit=limit)

        #  TODO offline

    # Clear operations

    def clear_block(self, access):
        cursor = self.conn.cursor()
        cursor.execute("DELETE FROM blocks WHERE block_id = ?", (str(access.id),))
        cursor.close()
        self.nb_blocks -= 1
        self._remove_file(access)

    def clear_manifest(self, access: Access):
        cursor = self.conn.cursor()
        cursor.execute("DELETE FROM vlobs WHERE vlob_id = ?", (str(access.id),))
        cursor.close()

    def clear_non_deletable_blocks_and_manifests(self):
        cursor = self.conn.cursor()
        cursor.execute("DELETE FROM vlobs WHERE deletable = 0")
        cursor.execute("DELETE FROM blocks WHERE deletable = 0")
        cursor.close()

    # Block file operations

    def _write_file(self, access: Access, content: bytes):
        try:
            self._remove_file(access)
        except LocalDBMissingEntry:
            pass
        file = self._db_files / str(access.id)
        file.write_bytes(content)

    def _read_file(self, access: Access):
        file = self._db_files / str(access.id)
        if file.exists():
            return file.read_bytes()
        else:
            raise LocalDBMissingEntry(access)

    def _remove_file(self, access: Access):
        file = self._db_files / str(access.id)
        if file.exists():
            file.unlink()
        else:
            raise LocalDBMissingEntry(access)

    # Garbage collection

    def cleanup_blocks(self, limit=None):
        cursor = self.conn.cursor()
        limit_string = f" limit {limit}" if limit is not None else ""
        cursor.execute("SELECT block_id from blocks WHERE deletable = 1" + limit_string)
        block_ids = [block_id for (block_id,) in cursor.fetchall()]
        cursor.close()
        for block_id in block_ids:
            self.clear_block(ManifestAccess(block_id))

    def run_block_garbage_collector(self):
        self.cleanup_blocks()
