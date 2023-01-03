# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import functools
import threading
from enum import Enum
from queue import Queue
from typing import Any, Awaitable, Callable, Dict, Tuple, TypeVar

import outcome
import trio
from trio.lowlevel import TrioToken
from typing_extensions import ParamSpec

from parsec._parsec import CoreEvent
from parsec.core.fs import AnyPath, WorkspaceFS
from parsec.core.fs.path import FsPath
from parsec.core.fs.workspacefs.file_transactions import FileDescriptor
from parsec.core.fs.workspacefs.workspacefs import EntryID
from parsec.event_bus import EventBus


class TrioDealockTimeoutError(Exception):
    pass


T = TypeVar("T")
P = ParamSpec("P")


def _run_from_thread_to_trio(
    trio_token: TrioToken,
    afn: Callable[P, Awaitable[T]],
    deadlock_timeout: Any = None,
    *args: P.args,
    **kwargs: P.kwargs,
) -> Any:
    """
    Most of this code is directly taken from:
    - `trio._threads.from_thread_run`
    - `trio._threads._run_fn_as_system_task`

    It's been altered to add a deadlock detection with proper cancellation.
    """
    # Condition and flag to safely deal with cancellation
    cancelled = False
    condition = threading.Condition()

    # Queue to receive the outcome of the task to run
    queue: Queue[outcome.Outcome[T]] = Queue()

    # Sync callback running in the trio thread
    def callback() -> None:
        # Protect against race-conditions
        with condition:
            # Notify the fuse thread/winfsp that the call has started
            condition.notify()
            # Check for cancellation in a thread-safe way
            if cancelled:
                return

        # Wrapper for the asynchronous function
        # Keyboard interrupt is enabled when using `trio_token.run_sync_soon`,
        # So let's disable it as we're heading back to user code.
        @trio.lowlevel.disable_ki_protection
        async def unprotected_afn() -> T:
            return await afn(*args)

        # System task target
        async def await_in_trio_thread_task() -> None:
            queue.put_nowait(await outcome.acapture(unprotected_afn))

        # Spawn system task
        try:
            trio.lowlevel.spawn_system_task(await_in_trio_thread_task, name=afn)
        # System nursery is closed
        except RuntimeError:
            queue.put_nowait(outcome.Error(trio.RunFinishedError("system nursery is closed")))

    # Protect against race conditions
    with condition:
        # Run the trio callback soon
        trio_token.run_sync_soon(callback)
        # Detect deadlock and cancel the callback in a thread-safe way
        if not condition.wait(deadlock_timeout):
            cancelled = True
            raise TrioDealockTimeoutError

    # Wait for the outcome and unwrap it
    return queue.get().unwrap()


class ThreadFSAccess:

    # One second seems like a sensible value, as it is very unlikely to
    # produce false positive (the trio loop would have to be busy processing
    # other earlier callbacks for more than a second, which would probably be
    # the indicator of another bug). It's also small enough to fail fast
    # enough from the user perspective.
    DEADLOCK_TIMEOUT = 1.0  # second

    def __init__(
        self, trio_token: TrioToken, workspace_fs: WorkspaceFS, event_bus: EventBus
    ) -> None:
        self._trio_token = trio_token
        self.workspace_fs = workspace_fs
        self.event_bus = event_bus

    def _run(self, afn: Callable[P, Awaitable[T]], *args: P.args, **kwargs: P.kwargs) -> T:
        return _run_from_thread_to_trio(self._trio_token, afn, self.DEADLOCK_TIMEOUT, *args)

    def _run_sync(self, fn: Callable[P, T], *args: P.args, **kwargs: P.kwargs) -> T:
        # This `_run_sync` method is only used for the `can_delete` winfsp operation,
        # so wrapping the sync function as an async one is good enough ¯\_(ツ)_/¯
        @functools.wraps(fn)
        async def afn(*args: P.args, **kwargs: P.kwargs) -> T:
            return fn(*args)

        return self._run(afn, *args)

    # Events

    def send_event(self, event: Enum | CoreEvent, **kwargs: Any) -> None:
        # Keyboard interrupt is enabled when using `trio_token.run_sync_soon`,
        # So let's disable it as we're heading back to user code.
        @trio.lowlevel.disable_ki_protection
        def callback() -> None:
            self.event_bus.send(event, **kwargs)

        # Do not use any kind of synchronization, events are fire-and-forget
        # This also means that there is no risk of deadlock.
        self._trio_token.run_sync_soon(callback)

    # Rights check

    def check_read_rights(self, path: FsPath) -> None:
        return self._run_sync(self.workspace_fs.transactions.check_read_rights, path)

    def check_write_rights(self, path: FsPath) -> None:
        return self._run_sync(self.workspace_fs.transactions.check_write_rights, path)

    # Entry transactions

    def entry_info(self, path: FsPath) -> Dict[str, Any]:
        return self._run(self.workspace_fs.transactions.entry_info, path)

    def entry_rename(
        self, source: FsPath, destination: FsPath, *, overwrite: bool = True
    ) -> EntryID | None:
        return self._run(
            self.workspace_fs.transactions.entry_rename, source, destination, overwrite
        )

    # Folder transactions

    def folder_create(self, path: FsPath) -> EntryID:
        return self._run(self.workspace_fs.transactions.folder_create, path)

    def folder_delete(self, path: FsPath) -> EntryID:
        return self._run(self.workspace_fs.transactions.folder_delete, path)

    # File transactions

    def file_create(
        self, path: FsPath, *, open: bool = True
    ) -> Tuple[EntryID, FileDescriptor | None]:
        return self._run(self.workspace_fs.transactions.file_create, path, open)

    def file_open(self, path: FsPath, *, write_mode: bool) -> Tuple[EntryID, FileDescriptor]:
        return self._run(self.workspace_fs.transactions.file_open, path, write_mode)

    def file_delete(self, path: FsPath) -> EntryID:
        return self._run(self.workspace_fs.transactions.file_delete, path)

    def file_resize(self, path: FsPath, length: int) -> EntryID:
        return self._run(self.workspace_fs.transactions.file_resize, path, length)

    # File descriptor transactions

    def fd_close(self, fh: FileDescriptor) -> None:
        return self._run(self.workspace_fs.transactions.fd_close, fh)

    def fd_read(self, fh: FileDescriptor, size: int, offset: int, raise_eof: bool = False) -> bytes:
        return self._run(self.workspace_fs.transactions.fd_read, fh, size, offset, raise_eof)

    def fd_write(
        self, fh: FileDescriptor, data: bytes, offset: int, constrained: bool = False
    ) -> int:
        return self._run(self.workspace_fs.transactions.fd_write, fh, data, offset, constrained)

    def fd_resize(self, fh: FileDescriptor, length: int, truncate_only: bool = False) -> None:
        return self._run(self.workspace_fs.transactions.fd_resize, fh, length, truncate_only)

    def fd_flush(self, fh: FileDescriptor) -> None:
        return self._run(self.workspace_fs.transactions.fd_flush, fh)

    # High-level helpers

    def workspace_move(self, source: AnyPath, destination: AnyPath) -> None:
        return self._run(self.workspace_fs.move, source, destination)
