# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from pathlib import Path

from async_generator import asynccontextmanager
from sqlite3 import connect as sqlite_connect


class LocalDatabase:
    """Base class for managing an sqlite3 connection."""

    def __init__(self, path, vacuum_threshold=None):
        self._conn = None
        self.path = Path(path)
        self.vacuum_threshold = vacuum_threshold

    @classmethod
    @asynccontextmanager
    async def run(cls, *args, **kwargs):
        self = cls(*args, **kwargs)
        try:
            await self._connect()
            yield self
        finally:
            await self._close()

    # Life cycle

    async def _create_connection(self):
        # Create directories
        self.path.parent.mkdir(parents=True, exist_ok=True)

        # Create sqlite connection
        conn = sqlite_connect(str(self.path))

        # Set WAL mode
        conn.execute("PRAGMA journal_mode=WAL")
        conn.execute("PRAGMA synchronous=NORMAL")

        # Return connection
        return conn

    async def _connect(self):
        if self._conn is not None:
            raise RuntimeError("Already connected")

        # Connect and initialize database
        self._conn = await self._create_connection()

    async def _close(self):
        # Idempotency
        if self._conn is None:
            return

        # Commit and close
        self._conn.commit()
        self._conn.close()
        self._conn = None

    # Cursor management

    @asynccontextmanager
    async def open_cursor(self, commit=True):
        # Manage transaction
        try:

            # Get a cursor
            cursor = self._conn.cursor()
            try:

                # Execute SQL commands
                yield cursor

            # Close cursor
            finally:
                cursor.close()

        # Commit the transaction when finished
        finally:
            if commit:
                self._conn.commit()

    async def commit(self):
        self._conn.commit()

    # Vacuum

    def get_disk_usage(self):
        disk_usage = 0
        for suffix in (".sqlite", ".sqlite-wal", ".sqlite-shm"):
            try:
                disk_usage += self.path.with_suffix(suffix).stat().st_size
            except OSError:
                pass
        return disk_usage

    async def run_vacuum(self):
        # Vacuum disabled
        if self.vacuum_threshold is None:
            return

        # Flush to disk
        self._conn.commit()

        # No reason to vacuum yet
        if self.get_disk_usage() < self.vacuum_threshold:
            return

        # Run vacuum
        self._conn.execute("VACUUM")

        # The connection needs to be recreated
        try:
            await self._close()
        finally:
            self._conn = await self._create_connection()
