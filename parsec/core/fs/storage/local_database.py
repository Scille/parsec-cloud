# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import inspect
import functools
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
from typing import AsyncIterator, Callable, Optional, Union, TypeVar, Awaitable, cast

import trio
import outcome
from async_generator import asynccontextmanager
from sqlite3 import Connection, Cursor, connect as sqlite_connect

R = TypeVar("R")
C = TypeVar("C", bound=Callable)


@asynccontextmanager
async def thread_pool_runner(
    max_workers: Optional[int] = None
) -> AsyncIterator[Callable[[Callable[[], R]], Awaitable[R]]]:
    """A trio-managed thread pool.

    This should be removed if trio decides to add support for thread pools:
    https://github.com/python-trio/trio/blob/c5497c5ac4/trio/_threads.py#L32-L128
    """
    executor = ThreadPoolExecutor(max_workers=max_workers)
    trio_token = trio.lowlevel.current_trio_token()

    async def run_in_thread(fn: Callable[[], R]) -> R:
        send_channel: trio.MemorySendChannel[outcome.Outcome[R]]
        receive_channel: trio.MemoryReceiveChannel[outcome.Outcome[R]]
        send_channel, receive_channel = trio.open_memory_channel(1)

        def target() -> None:
            result = outcome.capture(fn)
            trio.from_thread.run_sync(send_channel.send_nowait, result, trio_token=trio_token)

        executor.submit(target)
        result = await receive_channel.receive()
        return result.unwrap()

    # The thread pool executor cannot be used as a sync context here, as it would
    # block the trio loop. Instead, we shut the executor down in a worker thread.
    try:
        yield run_in_thread
    finally:
        with trio.CancelScope(shield=True):
            await trio.to_thread.run_sync(executor.shutdown)


def protect_context_with_lock(fn: C) -> C:
    """Use as a decorator to protect an async context with `self._lock`."""
    assert inspect.isasyncgenfunction(fn)

    @functools.wraps(fn)
    async def wrapper(
        self: "LocalDatabase", *args: object, **kwargs: object
    ) -> AsyncIterator[object]:
        async with self._lock:
            async for item in fn(self, *args, **kwargs):
                yield item

    return cast(C, wrapper)


def protect_method_with_lock(fn: C) -> C:
    """Use as a decorator to protect an async method with `self._lock`."""
    assert inspect.iscoroutinefunction(fn)

    @functools.wraps(fn)
    async def wrapper(self: "LocalDatabase", *args: object, **kwargs: object) -> object:
        async with self._lock:
            return await fn(self, *args, **kwargs)

    return cast(C, wrapper)


class LocalDatabase:
    """Base class for managing an sqlite3 connection."""

    def __init__(self, path: Union[str, Path], vacuum_threshold: Optional[int] = None):
        self._lock = trio.Lock()

        # Those attributes are set by the `run` async context manager
        self._conn: Connection
        self._run_in_thread: Callable[[Callable[[], R]], Awaitable[R]]

        self.path = Path(path)
        self.vacuum_threshold = vacuum_threshold

    @classmethod
    @asynccontextmanager
    async def run(
        cls, path: Union[str, Path], vacuum_threshold: Optional[int] = None
    ) -> AsyncIterator["LocalDatabase"]:
        # Instanciate the local database
        self = cls(path, vacuum_threshold)

        # Run a pool with single worker thread
        # (although the lock already protects against concurrent access to the pool)
        async with thread_pool_runner(max_workers=1) as self._run_in_thread:

            # Create the connection to the sqlite database
            try:
                await self._connect()

                # Yield the instance
                yield self

            # Safely flush and close the connection
            finally:
                with trio.CancelScope(shield=True):
                    await self._close()

    # Life cycle

    async def _create_connection(self) -> Connection:
        # Create directories
        self.path.parent.mkdir(mode=0o700, parents=True, exist_ok=True)

        # Create sqlite connection
        conn = sqlite_connect(str(self.path), check_same_thread=False)

        # The default isolation level ("") lets python manage the transaction
        # so we can periodically commit the pending changes.
        assert conn.isolation_level == ""

        # The combination of WAL journal mode and NORMAL synchronous mode
        # is a great combination: it allows for fast commits (~10 us compare
        # to 15 ms the default mode) but still protects the database against
        # corruption in the case of OS crash or power failure.
        conn.execute("PRAGMA journal_mode=WAL")
        conn.execute("PRAGMA synchronous=NORMAL")

        # Return connection
        return conn

    @protect_method_with_lock
    async def _connect(self) -> None:
        # Connect and initialize database
        self._conn = await self._create_connection()

    @protect_method_with_lock
    async def _close(self) -> None:
        # Commit and close
        await self._run_in_thread(self._conn.commit)
        self._conn.close()

    # Cursor management

    @asynccontextmanager
    @protect_context_with_lock
    async def open_cursor(self, commit: bool = True) -> AsyncIterator[Cursor]:
        # Get a cursor
        cursor = self._conn.cursor()
        try:

            # Execute SQL commands
            yield cursor

            # Commit the transaction when finished
            if commit and self._conn.in_transaction:
                await self._run_in_thread(self._conn.commit)

        # Close cursor
        finally:
            cursor.close()

    @protect_method_with_lock
    async def commit(self) -> None:
        await self._run_in_thread(self._conn.commit)

    # Vacuum

    def get_disk_usage(self) -> int:
        disk_usage = 0
        for suffix in (".sqlite", ".sqlite-wal", ".sqlite-shm"):
            try:
                disk_usage += self.path.with_suffix(suffix).stat().st_size
            except OSError:
                pass
        return disk_usage

    @protect_method_with_lock
    async def run_vacuum(self) -> None:
        # Vacuum disabled
        if self.vacuum_threshold is None:
            return

        # Flush to disk
        await self._run_in_thread(self._conn.commit)

        # No reason to vacuum yet
        if self.get_disk_usage() < self.vacuum_threshold:
            return

        # Run vacuum
        await self._run_in_thread(lambda: self._conn.execute("VACUUM"))

        # The connection needs to be recreated
        try:
            self._conn.close()
        finally:
            self._conn = await self._create_connection()
