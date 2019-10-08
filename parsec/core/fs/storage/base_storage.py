# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from pathlib import Path

from async_generator import asynccontextmanager
from sqlite3 import connect as sqlite_connect


class BaseStorage:
    """Base class for managing an sqlite3 connection."""

    def __init__(self, path):
        self._path = path
        self._conn = None

    @property
    def path(self):
        return Path(self._path)

    @classmethod
    @asynccontextmanager
    async def run(cls, *args, **kwargs):
        self = cls(*args, **kwargs)
        try:
            await self._connect()

            try:
                yield self
            finally:
                await self._flush()

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
        conn.execute("PRAGMA synchronous=OFF")

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

        # Commit and close
        self._conn.commit()
        self._conn.close()
        self._conn = None

    async def _create_db(self):
        raise NotImplementedError

    async def _flush(self):
        pass

    # Cursor management

    @asynccontextmanager
    async def _open_cursor(self):

        # Commit (or rollback) the transaction when finished
        with self._conn:

            # Get a cursor
            cursor = self._conn.cursor()
            try:

                # Execute SQL commands
                yield cursor

            # Close cursor
            finally:
                cursor.close()
