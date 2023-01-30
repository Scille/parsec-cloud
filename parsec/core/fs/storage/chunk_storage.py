# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import secrets
import time
from contextlib import asynccontextmanager
from pathlib import Path
from typing import AsyncContextManager, AsyncIterator, List, TypeVar, cast

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

            # Singleton for storing remanence information
            cursor.execute(
                """CREATE TABLE IF NOT EXISTS remanence
                    (_id INTEGER PRIMARY KEY NOT NULL,
                     block_remanent BOOL NOT NULL
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

    async def set_chunk(self, chunk_id: ChunkID, raw: bytes) -> list[ChunkID]:
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
            # No chunks are removed in ChunkStorage implementation
            return []

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

    async def clear_chunks(self, chunk_ids: list[ChunkID]) -> None:
        async with self._open_cursor() as cursor:
            # Use a thread as executing a statement that modifies the content of the database might,
            # in some case, block for several hundreds of milliseconds
            chunk_id_as_bytes = [(chunk_id.bytes,) for chunk_id in chunk_ids]
            await self.localdb.run_in_thread(
                cursor.executemany,
                "DELETE FROM chunks WHERE chunk_id = ?",
                chunk_id_as_bytes,
            )


class BlockStorage(ChunkStorage):
    """Interface for caching the data blocks."""

    def __init__(self, device: LocalDevice, localdb: LocalDatabase, cache_size: int):
        super().__init__(device, localdb)
        self.cache_size = cache_size
        self._block_remanent = False

    @classmethod
    @asynccontextmanager
    async def run(  # type: ignore[override]
        cls, device: LocalDevice, localdb: LocalDatabase, cache_size: int
    ) -> AsyncIterator["BlockStorage"]:
        async with cls(device, localdb, cache_size)._run() as self:
            await self._load_block_remanent()
            yield self

    def _open_cursor(self) -> AsyncContextManager[Cursor]:
        # It doesn't matter for blocks to be committed as soon as they're added
        # since they exists in the remote storage anyway. But it's simply more
        # convenient to perform the commit right away as it doesn't cost much (at
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

    async def set_chunk(self, chunk_id: ChunkID, raw: bytes) -> list[ChunkID]:
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
            return await self.cleanup(cursor)

    async def cleanup(self, cursor: Cursor | None = None) -> list[ChunkID]:
        # No cleanup for remanent storage
        if self._block_remanent:
            return []

        # Update database
        async with self._reenter_cursor(cursor) as cursor:

            # Count the chunks
            cursor.execute("SELECT COUNT(*) FROM chunks")
            (nb_blocks,) = cursor.fetchone()
            extra_blocks = nb_blocks - self.block_limit

            # No clean up is needed
            if extra_blocks <= 0:
                return []

            # Remove the extra block plus 10 % of the cache size, i.e about 100 blocks
            limit = extra_blocks + self.block_limit // 10

            # Use a thread as executing a statement that modifies the content of the database might,
            # in some case, block for several hundreds of milliseconds
            def _thread_target(cursor: Cursor) -> list[ChunkID]:

                # Select before delete
                cursor.execute(
                    """
                    SELECT chunk_id FROM chunks ORDER BY accessed_on ASC LIMIT ?
                    """,
                    (limit,),
                )
                rows = cursor.fetchall()

                # And then actual delete
                cursor.execute(
                    """
                    DELETE FROM chunks WHERE chunk_id IN (
                        SELECT chunk_id FROM chunks ORDER BY accessed_on ASC LIMIT ?
                    )
                    """,
                    (limit,),
                )

                return [ChunkID.from_bytes(id_bytes) for (id_bytes,) in rows]

            result = await self.localdb.run_in_thread(_thread_target, cursor)

            # Type detection is broken here for some reason
            return cast(list[ChunkID], result)

    # Remanent interface

    async def _load_block_remanent(self) -> None:
        async with self._open_cursor() as cursor:
            cursor.execute("SELECT block_remanent FROM remanence")
            rep = cursor.fetchone()
            self._block_remanent = False if rep is None else bool(rep[0])

    def is_block_remanent(self) -> bool:
        return self._block_remanent

    async def enable_block_remanence(self) -> bool:
        async with self._open_cursor() as cursor:
            # Check if remanence is already enabled
            if self._block_remanent:
                return False

            # Use a thread as executing a statement that modifies the content of the database might,
            # in some case, block for several hundreds of milliseconds
            await self.localdb.run_in_thread(
                cursor.execute,
                """INSERT OR REPLACE INTO
                    remanence (_id, block_remanent)
                    VALUES (0, 1)""",
            )

            # Set remanent flag
            self._block_remanent = True

        # Indicate that the state has changed
        return True

    async def disable_block_remanence(self) -> list[ChunkID] | None:
        async with self._open_cursor() as cursor:
            # Check if remanence is already disabled
            if not self._block_remanent:
                return None

            # Use a thread as executing a statement that modifies the content of the database might,
            # in some case, block for several hundreds of milliseconds
            await self.localdb.run_in_thread(
                cursor.execute,
                """INSERT OR REPLACE INTO
                    remanence (_id, block_remanent)
                    VALUES (0, 0)""",
            )

            # Set remanent flag
            self._block_remanent = False

            # Cleanup blocks and indicate that the state has changed
            return await self.cleanup(cursor)

    async def clear_unreferenced_chunks(
        self, chunk_ids: list[ChunkID], not_accessed_after: float
    ) -> None:
        if not chunk_ids:
            return

        bytes_id_list = [(id.bytes,) for id in chunk_ids]

        async with self._open_cursor() as cursor:

            def _thread_target() -> None:
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
                    f"""DELETE FROM chunks where chunk_id NOT IN (SELECT chunk_id FROM {table_name}) AND accessed_on < ?""",
                    (not_accessed_after,),
                )

                cursor.execute(f"""DROP TABLE IF EXISTS {table_name}""")

            await self.localdb.run_in_thread(_thread_target)
