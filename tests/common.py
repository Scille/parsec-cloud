# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import sqlite3
from unittest.mock import Mock
from contextlib import ExitStack
from inspect import iscoroutinefunction

import trio
import attr
import pendulum

from parsec.core.logged_core import LoggedCore
from parsec.core.fs import FS
from parsec.core.local_db import LocalDB, LocalDBMissingEntry
from parsec.api.transport import Transport, TransportError


class InMemoryLocalDB(LocalDB):
    """An in-memory version of the local database.

    It doesn't perform any access to the file system
    and has very permissive life cycle.
    """

    def __init__(self, **kwargs):
        self._data = {}
        super().__init__("unused", **kwargs)

        self.dirty_conn = sqlite3.connect(":memory:")
        self.clean_conn = sqlite3.connect(":memory:")
        self.create_db()

    # Disable life cycle

    def connect(self):
        pass

    def close(self):
        pass

    # File systeme interface

    def _read_file(self, access, path):
        filepath = path / str(access.id)
        try:
            return self._data[filepath]
        except KeyError:
            raise LocalDBMissingEntry(access)

    def _write_file(self, access, content, path):
        filepath = path / str(access.id)
        self._data[filepath] = content

    def _remove_file(self, access, path):
        filepath = path / str(access.id)
        try:
            del self._data[filepath]
        except KeyError:
            raise LocalDBMissingEntry(access)


def freeze_time(time):
    if isinstance(time, str):
        time = pendulum.parse(time)
    return pendulum.test(time)


class AsyncMock(Mock):
    @property
    def is_async(self):
        return self.__dict__.get("is_async", False)

    @is_async.setter
    def is_async(self, val):
        self.__dict__["is_async"] = val

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.__dict__["is_async"] = False
        spec = kwargs.get("spec") or kwargs.get("spec_set")
        if spec:
            if callable(spec):
                self.is_async = True
            for field in dir(spec):
                if iscoroutinefunction(getattr(spec, field)):
                    getattr(self, field).is_async = True

    async def __async_call(self, *args, **kwargs):
        return super().__call__(*args, **kwargs)

    def __call__(self, *args, **kwargs):
        if getattr(self, "is_async", False) is True:
            if iscoroutinefunction(self.side_effect):
                return self.side_effect(*args, **kwargs)

            else:
                return self.__async_call(*args, **kwargs)

        else:
            return super().__call__(*args, **kwargs)


class FreezeTestOnTransportError(Transport):
    """
    When a server crashes during test, it is possible the client coroutine
    receives a `TransportError` exception. Hence we end up with two
    exceptions: the server crash (i.e. the original exception we are interested
    into) and the client not receiving an answer.
    The solution is simply to freeze the coroutine receiving the broken stream
    error until it will be cancelled by the original exception bubbling up.
    """

    def __init__(self, transport):
        self.transport = transport

    @property
    def stream(self):
        return self.transport.stream

    async def send(self, msg):
        try:
            return await self.transport.send(msg)

        except TransportError:
            # Wait here until this coroutine is cancelled
            await trio.sleep_forever()

    async def recv(self):
        try:
            return await self.transport.recv()

        except TransportError:
            # Wait here until this coroutine is cancelled
            await trio.sleep_forever()


@attr.s
class CallController:
    need_stop = attr.ib(factory=trio.Event)
    stopped = attr.ib(factory=trio.Event)

    async def stop(self):
        self.need_stop.set()
        await self.stopped.wait()


async def call_with_control(controlled_fn, *, task_status=trio.TASK_STATUS_IGNORED):
    controller = CallController()

    async def _started_cb(**kwargs):
        controller.__dict__.update(kwargs)
        task_status.started(controller)
        await controller.need_stop.wait()

    try:
        await controlled_fn(_started_cb)

    finally:
        controller.stopped.set()


async def create_shared_workspace(name, creator, *shared_with):
    """
    Create a workspace and share it with the given Cores/FSs.
    This is more tricky than it seems given all Cores/FSs must agree on the
    workspace version and (only for the Cores) be ready to listen to the
    workpace's vlob group events.
    """
    spies = []
    fss = []

    with ExitStack() as stack:
        for x in (creator, *shared_with):
            if isinstance(x, LoggedCore):
                # In case core has been passed
                spies.append(stack.enter_context(x.event_bus.listen()))
                fss.append(x.fs)
            elif isinstance(x, FS):
                fss.append(x)
            else:
                raise ValueError(f"{x!r} is not a {FS!r} or a {LoggedCore!r}")

        creator_fs, *shared_with_fss = fss
        path = f"/{name}"
        await creator_fs.workspace_create(path)
        await creator_fs.sync(path)
        for recipient_fs in shared_with_fss:
            if recipient_fs.device.user_id == creator_fs.device.user_id:
                await recipient_fs.sync("/")
            else:
                await creator_fs.share(path, recipient_fs.device.user_id)
                await recipient_fs.process_last_messages()
                await recipient_fs.sync("/")

        with trio.fail_after(1):
            for spy in spies:
                await spy.wait("backend.listener.restarted")
