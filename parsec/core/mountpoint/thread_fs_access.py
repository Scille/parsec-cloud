# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import trio
import outcome
import functools
import threading
from queue import Queue


class TrioDealockTimeoutError(Exception):
    pass


def _run_from_thread_to_trio(trio_token, afn, *args, deadlock_timeout=None):
    # Thread-safe communication
    queue = Queue()
    cancelled = False
    condition = threading.Condition()

    def callback():
        # Notify the call has started and check for cancellation in a thread-safe way
        with condition:
            condition.notify()
            if cancelled:
                queue.put_nowait(RuntimeError("Cancelled"))
                return

        # Wrapper for the asynchronous function
        @trio.lowlevel.disable_ki_protection
        async def unprotected_afn():
            coro = trio._util.coroutine_or_error(afn, *args)
            return await coro

        # Task target
        async def await_in_trio_thread_task():
            queue.put_nowait(await outcome.acapture(unprotected_afn))

        # Spawn system task
        try:
            trio.lowlevel.spawn_system_task(await_in_trio_thread_task, name=afn)
        except RuntimeError:  # system nursery is closed
            queue.put_nowait(outcome.Error(trio.RunFinishedError("system nursery is closed")))

    with condition:
        # Run the callback soon
        trio_token.run_sync_soon(callback)
        # Detect deadlock and cancel the callback in a thread-safe way
        if not condition.wait(deadlock_timeout):
            cancelled = True
            raise TrioDealockTimeoutError

    # Wait for the outcome and unwrap it
    return queue.get().unwrap()


class ThreadFSAccess:

    DEADLOCK_TIMEOUT = 1.0  # second

    def __init__(self, trio_token, workspace_fs, event_bus):
        self._trio_token = trio_token
        self.workspace_fs = workspace_fs
        self.event_bus = event_bus

    def _run(self, afn, *args):
        return _run_from_thread_to_trio(
            self._trio_token, afn, *args, deadlock_timeout=self.DEADLOCK_TIMEOUT
        )

    def _run_sync(self, fn, *args):
        @functools.wraps(fn)
        async def afn(*args):
            return fn(*args)

        return self._run(afn, *args)

    # Events

    def send_event(self, event, **kwargs):
        @trio.lowlevel.disable_ki_protection
        def callback():
            self.event_bus.send(event, **kwargs)

        # Do not use any kind of synchronization, events are fire-and-forget
        self._trio_token.run_sync_soon(callback)

    # Rights check

    def check_read_rights(self, path):
        return self._run_sync(self.workspace_fs.transactions.check_read_rights, path)

    def check_write_rights(self, path):
        return self._run_sync(self.workspace_fs.transactions.check_write_rights, path)

    # Entry transactions

    def entry_info(self, path):
        return self._run(self.workspace_fs.transactions.entry_info, path)

    def entry_rename(self, source, destination, *, overwrite):
        return self._run(
            self.workspace_fs.transactions.entry_rename, source, destination, overwrite
        )

    # Folder transactions

    def folder_create(self, path):
        return self._run(self.workspace_fs.transactions.folder_create, path)

    def folder_delete(self, path):
        return self._run(self.workspace_fs.transactions.folder_delete, path)

    # File transactions

    def file_create(self, path, *, open):
        return self._run(self.workspace_fs.transactions.file_create, path, open)

    def file_open(self, path, *, write_mode):
        return self._run(self.workspace_fs.transactions.file_open, path, write_mode)

    def file_delete(self, path):
        return self._run(self.workspace_fs.transactions.file_delete, path)

    def file_resize(self, path, length):
        return self._run(self.workspace_fs.transactions.file_resize, path, length)

    # File descriptor transactions

    def fd_close(self, fh):
        return self._run(self.workspace_fs.transactions.fd_close, fh)

    def fd_seek(self, fh, offset):
        return self._run(self.workspace_fs.transactions.fd_seek, fh, offset)

    def fd_read(self, fh, size, offset, raise_eof=False):
        return self._run(self.workspace_fs.transactions.fd_read, fh, size, offset, raise_eof)

    def fd_write(self, fh, data, offset, constrained=False):
        return self._run(self.workspace_fs.transactions.fd_write, fh, data, offset, constrained)

    def fd_resize(self, fh, length, truncate_only=False):
        return self._run(self.workspace_fs.transactions.fd_resize, fh, length, truncate_only)

    def fd_flush(self, fh):
        return self._run(self.workspace_fs.transactions.fd_flush, fh)
