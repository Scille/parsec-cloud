# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import secrets
import time
from contextlib import asynccontextmanager
from pathlib import Path
from typing import AsyncContextManager, AsyncIterator, List, TypeVar

import trio

from parsec.core.fs.exceptions import FSLocalMissError, FSLocalStorageClosedError
from parsec.core.fs.storage.local_database import Cursor, LocalDatabase
from parsec.core.types import DEFAULT_BLOCK_SIZE, ChunkID, LocalDevice

T = TypeVar("T", bound="ChunkStorage")


class ChunkStorage:
    """Interface to access the local chunks of data."""

    def __init__(self, device: LocalDevice, localdb: LocalDatabase):
        self.local_symkey = device.local_symkey
        self.localdb = localdb

    @property
    def path(self) -> Path:
        return Path(self.localdb.path)

    @classmethod
    @asynccontextmanager
    async def run(
        cls, device: LocalDevice, localdb: LocalDatabase
    ) -> AsyncIterator["ChunkStorage"]:
        async with cls(device, localdb)._run() as self:
            yield self

    @asynccontextmanager
    async def _run(self: T) -> AsyncIterator[T]:
        await self._create_db()
        try:
            yield self
        finally:
            with trio.CancelScope(shield=True):
                # Commit the pending changes in the local database
                try:
                    await self.localdb.commit()
                # Ignore storage closed exceptions, since it follows an operational error
                except FSLocalStorageClosedError:
                    pass

    def _open_cursor(self) -> AsyncContextManager[Cursor]:
        # There is no point in committing dirty chunks:
        # they are referenced by a manifest that will get committed
        # soon after them. This greatly improves the performance of
        # writing file using the mountpoint as the OS will typically
        # writes data as blocks of 4K. The manifest being kept in
        # memory during the writing, this means that the data and
        # metadata is typically not flushed to the disk until an
        # an actual flush operation is performed.
        return self.localdb.open_cursor(commit=False)

    # Database initialization

    async def _create_db(self) -> None:
        async with self._open_cursor() as cursor:
            cursor.execute(
                """CREATE TABLE IF NOT EXISTS chunks
                    (chunk_id BLOB PRIMARY KEY NOT NULL, -- UUID
                     size INTEGER NOT NULL,
                     offline INTEGER NOT NULL,  -- Boolean
                     accessed_on REAL, -- Timestamp
                     data BLOB NOT NULL
                );"""
            )

    # Size and chunks

    async def get_nb_blocks(self) -> int:
        async with self._open_cursor() as cursor:
            cursor.execute("SELECT COUNT(*) FROM chunks")
            (result,) = cursor.fetchone()
            return result

    async def get_total_size(self) -> int:
        async with self._open_cursor() as cursor:
            cursor.execute("SELECT COALESCE(SUM(size), 0) FROM chunks")
            (result,) = cursor.fetchone()
            return result

    # Generic chunk operations

    async def is_chunk(self, chunk_id: ChunkID) -> bool:
        async with self._open_cursor() as cursor:
            cursor.execute("SELECT chunk_id FROM chunks WHERE chunk_id = ?", (chunk_id.bytes,))
            manifest_row = cursor.fetchone()
        return bool(manifest_row)

    async def get_local_chunk_ids(self, chunk_id: List[ChunkID]) -> List[ChunkID]:

        bytes_id_list = [(id.bytes,) for id in chunk_id]

        async with self._open_cursor() as cursor:
            # Can't use execute many with SELECT so we have to make a temporary table filled with the needed chunk_id
            # and intersect it with the normal table
            # create random name for the temporary table to avoid asynchronous errors
            table_name = "temp" + secrets.token_hex(12)
            assert table_name.isalnum()

            cursor.execute(f"""DROP TABLE IF EXISTS {table_name}""")
            cursor.execute(
                f"""CREATE TABLE IF NOT EXISTS {table_name}
                    (chunk_id BLOB PRIMARY KEY NOT NULL -- UUID
                );"""
            )

            cursor.executemany(
                f"""INSERT OR REPLACE INTO
            {table_name} (chunk_id)
            VALUES (?)""",
                iter(bytes_id_list),
            )

            cursor.execute(
                f"""SELECT chunk_id FROM chunks INTERSECT SELECT chunk_id FROM {table_name}"""
            )

            intersect_rows = cursor.fetchall()
            cursor.execute(f"""DROP TABLE IF EXISTS {table_name}""")

        return [ChunkID.from_bytes(id_bytes) for (id_bytes,) in intersect_rows]

    async def get_chunk(self, chunk_id: ChunkID) -> bytes:
        async with self._open_cursor() as cursor:
            # Use a thread as executing a statement that modifies the content of the database might,
            # in some case, block for several hundreds of milliseconds
            await self.localdb.run_in_thread(
                cursor.execute,
                """
                UPDATE chunks SET accessed_on = ? WHERE chunk_id = ?;
                """,
                (time.time(), chunk_id.bytes),
            )
            cursor.execute("SELECT changes()")
            (changes,) = cursor.fetchone()
            if not changes:
                raise FSLocalMissError(chunk_id)
            cursor.execute("""SELECT data FROM chunks WHERE chunk_id = ?""", (chunk_id.bytes,))
            (ciphered,) = cursor.fetchone()

        return self.local_symkey.decrypt(ciphered)

    async def set_chunk(self, chunk_id: ChunkID, raw: bytes) -> None:
        ciphered = self.local_symkey.encrypt(raw)

        # Update database
        async with self._open_cursor() as cursor:
            # Use a thread as executing a statement that modifies the content of the database might,
            # in some case, block for several hundreds of milliseconds
            await self.localdb.run_in_thread(
                cursor.execute,
                """INSERT OR REPLACE INTO
                chunks (chunk_id, size, offline, accessed_on, data)
                VALUES (?, ?, ?, ?, ?)""",
                (chunk_id.bytes, len(ciphered), False, time.time(), ciphered),
            )

    async def clear_chunk(self, chunk_id: ChunkID) -> None:
        async with self._open_cursor() as cursor:
            # Use a thread as executing a statement that modifies the content of the database might,
            # in some case, block for several hundreds of milliseconds
            await self.localdb.run_in_thread(
                cursor.execute, "DELETE FROM chunks WHERE chunk_id = ?", (chunk_id.bytes,)
            )
            cursor.execute("SELECT changes()")
            (changes,) = cursor.fetchone()

        if not changes:
            raise FSLocalMissError(chunk_id)


class BlockStorage(ChunkStorage):
    """Interface for caching the data blocks."""

    def __init__(self, device: LocalDevice, localdb: LocalDatabase, cache_size: int):
        super().__init__(device, localdb)
        self.cache_size = cache_size

    @classmethod
    @asynccontextmanager
    async def run(  # type: ignore[override]
        cls, device: LocalDevice, localdb: LocalDatabase, cache_size: int
    ) -> AsyncIterator["BlockStorage"]:
        async with cls(device, localdb, cache_size)._run() as self:
            yield self

    def _open_cursor(self) -> AsyncContextManager[Cursor]:
        # It doesn't matter for blocks to be committed as soon as they're added
        # since they exists in the remote storage anyway. But it's simply more
        # convenient to perform the commit right away as it does't cost much (at
        # least compare to the downloading of the block).
        return self.localdb.open_cursor(commit=True)

    @asynccontextmanager
    async def _reenter_cursor(self, cursor: Cursor | None) -> AsyncIterator[Cursor]:
        if cursor is not None:
            yield cursor
            return
        async with self._open_cursor() as cursor:
            yield cursor

    # Garbage collection

    @property
    def block_limit(self) -> int:
        return self.cache_size // DEFAULT_BLOCK_SIZE

    async def clear_all_blocks(self) -> None:
        async with self._open_cursor() as cursor:
            cursor.execute("DELETE FROM chunks")

    # Upgraded set method

    async def set_chunk(self, chunk_id: ChunkID, raw: bytes) -> None:
        ciphered = self.local_symkey.encrypt(raw)

        # Update database
        async with self._open_cursor() as cursor:

            # Insert the chunk
            # Use a thread as executing a statement that modifies the content of the database might,
            # in some case, block for several hundreds of milliseconds
            await self.localdb.run_in_thread(
                cursor.execute,
                """INSERT OR REPLACE INTO
                chunks (chunk_id, size, offline, accessed_on, data)
                VALUES (?, ?, ?, ?, ?)""",
                (chunk_id.bytes, len(ciphered), False, time.time(), ciphered),
            )

            # Perform cleanup if necessary
            await self.cleanup(cursor)

    async def cleanup(self, cursor: Cursor | None = None) -> None:

        # Update database
        async with self._reenter_cursor(cursor) as cursor:

            # Count the chunks
            cursor.execute("SELECT COUNT(*) FROM chunks")
            (nb_blocks,) = cursor.fetchone()
            extra_blocks = nb_blocks - self.block_limit

            # No clean up is needed
            if extra_blocks <= 0:
                return

            # Remove the extra block plus 10 % of the cache size, i.e about 100 blocks
            limit = extra_blocks + self.block_limit // 10
            # Use a thread as executing a statement that modifies the content of the database might,
            # in some case, block for several hundreds of milliseconds
            await self.localdb.run_in_thread(
                cursor.execute,
                """
                DELETE FROM chunks WHERE chunk_id IN (
                    SELECT chunk_id FROM chunks ORDER BY accessed_on ASC LIMIT ?
                )
                """,
                (limit,),
            )
