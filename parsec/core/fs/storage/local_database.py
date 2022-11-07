# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from pathlib import Path
from typing import AsyncIterator, Optional, Union
import trio
from contextlib import asynccontextmanager
from sqlite3 import Connection, Cursor, OperationalError, connect as sqlite_connect
from parsec.core.core_events import CoreEvent

from parsec.core.fs.exceptions import FSLocalStorageClosedError, FSLocalStorageOperationalError
from parsec.event_bus import EventBus


class LocalDatabase:
    """Base class for managing an sqlite3 connection."""

    # Make the trio run_sync function patchable for the tests
    run_in_thread = staticmethod(trio.to_thread.run_sync)

    def __init__(
        self,
        path: Union[str, Path, trio.Path],
        event_bus: EventBus,
        vacuum_threshold: Optional[int] = None,
    ):
        # Make sure only a single task access the connection object at a time
        self._lock = trio.Lock()

        # Those attributes are set by the `run` async context manager
        self._conn: Connection

        self.path = trio.Path(path)
        self.vacuum_threshold = vacuum_threshold
        self.event_bus = event_bus

    @classmethod
    @asynccontextmanager
    async def run(
        cls, path: Union[str, Path], event_bus: EventBus, vacuum_threshold: Optional[int] = None
    ) -> AsyncIterator["LocalDatabase"]:
        # Instanciate the local database
        self = cls(path, event_bus, vacuum_threshold)

        # Create the connection to the sqlite database
        try:
            await self._connect()

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
        discarding any uncommited data.
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
            with trio.CancelScope(shield=True):

                # Close the sqlite3 connection
                try:
                    await self.run_in_thread(self._conn.close)

                # Ignore second operational error (it should not happen though)
                except OperationalError:
                    pass

                # Mark the local database as closed
                finally:
                    del self._conn

                self.event_bus.send(CoreEvent.FS_LOCALDATABASE_OPERATIONAL_ERROR, error=exception)
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
