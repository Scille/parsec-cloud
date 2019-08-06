# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import sqlite3
from unittest.mock import Mock
from inspect import iscoroutinefunction
from contextlib import ExitStack, contextmanager

import trio
import attr
import pendulum

from parsec.core.types import WorkspaceRole
from parsec.core.logged_core import LoggedCore
from parsec.core.fs import UserFS
from parsec.core.fs.persistent_storage import PersistentStorage
from parsec.api.transport import Transport, TransportError


def addr_with_device_subdomain(addr, device_id):
    """
    Useful to have each device access the same backend with a different hostname
    so tcp_stream_spy can put some offline and leave others online
    """
    device_specific_hostname = f"{device_id.user_id}.{device_id.device_name}.{addr.hostname}"
    return type(addr)(addr.replace(addr.hostname, device_specific_hostname, 1))


class InMemoryPersistentStorage(PersistentStorage):
    """An in-memory version of the local database.

    It doesn't perform any access to the file system
    and has very permissive life cycle.
    """

    # Mockup

    cache = None

    @classmethod
    @contextmanager
    def mockup_context(cls):
        try:
            cls.cache = {}
            yield cls.cache
        finally:
            cls.cache = None

    # Init

    def __init__(self, key, path, **kwargs):
        super().__init__(key, path, **kwargs)

        if self.cache is None:
            raise RuntimeError(
                "Cannot use the in-memory persistent storage outside of its mockup context."
                " Try adding the persistent_mockup fixture to your test."
            )

        try:
            self.dirty_conn, self.clean_conn = self.cache[path]
        except KeyError:
            self.dirty_conn = sqlite3.connect(":memory:")
            self.clean_conn = sqlite3.connect(":memory:")
            self.cache[path] = self.dirty_conn, self.clean_conn

        self.create_db()

    # Disable life cycle

    def connect(self):
        pass

    def close(self):
        pass


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
    workspace's vlob group events.
    """
    spies = []
    fss = []

    with ExitStack() as stack:
        for x in (creator, *shared_with):
            if isinstance(x, LoggedCore):
                # In case core has been passed
                spies.append(stack.enter_context(x.event_bus.listen()))
                fss.append(x.user_fs)
            elif isinstance(x, UserFS):
                fss.append(x)
            else:
                raise ValueError(f"{x!r} is not a {UserFS!r} or a {LoggedCore!r}")

        creator_user_fs, *shared_with_fss = fss
        wid = await creator_user_fs.workspace_create(name)
        await creator_user_fs.sync()
        workspace = creator_user_fs.get_workspace(wid)
        await workspace.sync("/")

        for recipient_user_fs in shared_with_fss:
            if recipient_user_fs.device.user_id == creator_user_fs.device.user_id:
                await recipient_user_fs.sync()
            else:
                await creator_user_fs.workspace_share(
                    wid, recipient_user_fs.device.user_id, WorkspaceRole.MANAGER
                )
                await recipient_user_fs.process_last_messages()
                await recipient_user_fs.sync()

        with trio.fail_after(1):
            for spy in spies:
                await spy.wait("backend.realm.roles_updated")

        return wid


def compare_fs_dumps(entry_1, entry_2):
    entry_1.pop("author", None)
    entry_2.pop("author", None)

    def cook_entry(entry):
        if "children" in entry:
            return {**entry, "children": {k: v["id"] for k, v in entry["children"].items()}}
        else:
            return entry

    assert not entry_1.get("need_sync", False)
    assert not entry_2.get("need_sync", False)

    if "need_sync" not in entry_1 or "need_sync" not in entry_2:
        # One of the entry is not loaded
        return

    assert cook_entry(entry_1) == cook_entry(entry_2)
    if "children" in entry_1:
        for key, child_for_entry_1 in entry_1["children"].items():
            child_for_entry_2 = entry_2["children"][key]
            compare_fs_dumps(child_for_entry_1, child_for_entry_2)
