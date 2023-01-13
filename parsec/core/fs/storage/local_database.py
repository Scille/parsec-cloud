# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from contextlib import asynccontextmanager
from pathlib import Path
from sqlite3 import Connection, Cursor, OperationalError
from sqlite3 import connect as sqlite_connect
from typing import AsyncIterator, Union

import trio
from trio_typing import TaskStatus

from parsec.core.fs.exceptions import FSLocalStorageClosedError, FSLocalStorageOperationalError
from parsec.utils import open_service_nursery


class LocalDatabase:
    """Base class for managing an sqlite3 connection."""

    # Make the trio run_sync function patchable for the tests
    run_in_thread = staticmethod(trio.to_thread.run_sync)

    def __init__(
        self,
        path: Union[str, Path, trio.Path],
        vacuum_threshold: int | None = None,
        auto_vacuum: bool = False,
    ):

        # Make sure only a single task access the connection object at a time
        self._lock = trio.Lock()

        # Those attributes are set by the `run` async context manager
        self._conn: Connection
        self._abort_service_send_channel: trio.MemorySendChannel[Exception]

        # Set arguments
        if auto_vacuum:
            assert vacuum_threshold is None
        self.path = trio.Path(path)
        self.auto_vacuum = auto_vacuum
        self.vacuum_threshold = vacuum_threshold

    @asynccontextmanager
    async def _service_abort_context(self) -> AsyncIterator[None]:
        async def _service_abort_task(
            task_status: TaskStatus[trio.MemorySendChannel[Exception]] = trio.TASK_STATUS_IGNORED,
        ) -> None:
            send, receive = trio.open_memory_channel[Exception](0)
            task_status.started(send)
            async with receive:
                async for item in receive:
                    raise item

        async with open_service_nursery() as nursery:
            async with await nursery.start(_service_abort_task) as self._abort_service_send_channel:
                yield
            nursery.cancel_scope.cancel()

    async def _send_abort(self, exception: Exception) -> None:
        # Send the exception to be raised in the `run` context
        try:
            await self._abort_service_send_channel.send(exception)
        # The service has already exited
        except trio.BrokenResourceError:
            pass

    @classmethod
    @asynccontextmanager
    async def run(
        cls,
        path: Union[str, Path],
        vacuum_threshold: int | None = None,
        auto_vacuum: bool = False,
    ) -> AsyncIterator["LocalDatabase"]:
        # Instantiate the local database
        self = cls(path, vacuum_threshold, auto_vacuum)

        # Create the connection to the sqlite database
        try:
            await self._connect()

            async with self._service_abort_context():

                # Yield the instance
                yield self

        # Safely flush and close the connection
        finally:
            with trio.CancelScope(shield=True):
                try:
                    await self._close()
                except FSLocalStorageClosedError:
                    pass

    # Operational error protection

    @asynccontextmanager
    async def _manage_operational_error(self, allow_commit: bool = False) -> AsyncIterator[None]:
        """Close the local database when an operational error is detected

        Operational errors have to be treated with care since they usually indicate
        that the current transaction has been rolled back. Since parsec has its own
        in-memory cache for manifests, this can lead to complicated bugs and possibly
        database corruptions (in the sense that a committed manifest might reference a
        chunk of data that has not been successfully committed). See issue #1535 for an
        example of such problem.

        If an operational error is detected we simply close the connection and invalidate
        the local database object while raising an FSLocalStorageOperationalError exception.
        This way, we force the core to create new workspace storage objects and therefore
        discarding any uncommitted data.
        """
        in_transaction_before = self._conn.in_transaction
        # Safe context for operational errors
        try:
            try:
                yield

            # Extra checks for end of transaction
            finally:
                end_of_transaction_detected = (
                    in_transaction_before and not self._conn.in_transaction
                )
                if not allow_commit and end_of_transaction_detected:
                    raise OperationalError("A forbidden commit/rollback has been detected")

        # An operational error has been detected
        except OperationalError as exception:

            # Make sure the local database won't be used by another task
            _conn = self._conn
            del self._conn

            # Protect against cancellation
            with trio.CancelScope(shield=True):

                # Notify the service that it can no longer be used
                await self._send_abort(FSLocalStorageOperationalError(*exception.args))

                # Close the sqlite3 connection
                try:
                    await self.run_in_thread(_conn.close)

                # Ignore second operational error (it should not happen though)
                except OperationalError:
                    pass

                # Raise the dedicated operational error
                raise FSLocalStorageOperationalError from exception

    # Life cycle

    async def _create_connection(self) -> None:
        # Create directories
        await self.path.parent.mkdir(mode=0o700, parents=True, exist_ok=True)

        # Create sqlite connection
        self._conn = sqlite_connect(str(self.path), check_same_thread=False)

        # The default isolation level ("") lets python manage the transaction
        # so we can periodically commit the pending changes.
        assert self._conn.isolation_level == ""

        # The combination of WAL journal mode and NORMAL synchronous mode
        # is a great combination: it allows for fast commits (~10 us compare
        # to 15 ms the default mode) but still protects the database against
        # corruption in the case of OS crash or power failure.
        async with self._manage_operational_error():
            self._conn.execute("PRAGMA journal_mode=WAL")
            self._conn.execute("PRAGMA synchronous=NORMAL")

            # Manage auto_vacuum
            cursor = self._conn.execute("PRAGMA auto_vacuum")
            rep = cursor.fetchone()
            current_auto_vacuum = rep and rep[0]

            if self.auto_vacuum != current_auto_vacuum:
                self._conn.execute(f"PRAGMA auto_vacuum={int(self.auto_vacuum)}")
                self._conn.execute("VACUUM")

    async def _connect(self) -> None:
        # Lock the access to the connection object
        async with self._lock:

            # Connect and initialize database
            await self._create_connection()

    async def _close(self) -> None:
        # Lock the access to the connection object
        async with self._lock:

            # Local database is already closed
            if self._is_closed():
                return

            # Commit the current transaction
            try:
                await self._commit()

            finally:
                # Local database is not already closed
                if not self._is_closed():

                    # Close the sqlite3 connection
                    try:
                        await self.run_in_thread(self._conn.close)

                    # Mark the local database as closed
                    finally:
                        del self._conn

    async def _commit(self) -> None:
        # Close the local database if an operational error is detected
        async with self._manage_operational_error(allow_commit=True):
            await self.run_in_thread(self._conn.commit)

    def _is_closed(self) -> bool:
        return not hasattr(self, "_conn")

    def _check_open(self) -> None:
        if self._is_closed():
            raise FSLocalStorageClosedError

    # Cursor management

    @asynccontextmanager
    async def open_cursor(self, commit: bool = True) -> AsyncIterator[Cursor]:
        # Lock the access to the connection object
        async with self._lock:

            # Check connection state
            self._check_open()

            # Close the local database if an operational error is detected
            async with self._manage_operational_error():

                # Execute SQL commands
                cursor = self._conn.cursor()
                try:
                    yield cursor
                finally:
                    cursor.close()

            # Commit the transaction when finished
            if commit and self._conn.in_transaction:
                await self._commit()

    async def commit(self) -> None:
        # Lock the access to the connection object
        async with self._lock:

            # Check connection state
            self._check_open()

            # Run the commit
            await self._commit()

    # Vacuum

    async def get_disk_usage(self) -> int:
        disk_usage = 0
        for suffix in (".sqlite", ".sqlite-wal", ".sqlite-shm"):
            try:
                stat = await self.path.with_suffix(suffix).stat()
            except OSError:
                pass
            else:
                disk_usage += stat.st_size
        return disk_usage

    async def run_vacuum(self) -> None:
        if self.auto_vacuum:
            return

        # Lock the access to the connection object
        async with self._lock:

            # Check connection state
            self._check_open()

            # Vacuum disabled
            if self.vacuum_threshold is None:
                return

            # Flush to disk before computing disk usage
            await self._commit()

            # No reason to vacuum yet
            if await self.get_disk_usage() < self.vacuum_threshold:
                return

            # Run vacuum
            await self.run_in_thread(self._conn.execute, "VACUUM")

            # The connection needs to be recreated
            try:
                await self.run_in_thread(self._conn.close)
            finally:
                await self._create_connection()
