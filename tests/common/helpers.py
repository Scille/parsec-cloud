# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import trio
import attr
import pytest
from unittest.mock import Mock
from inspect import iscoroutinefunction
from contextlib import ExitStack
import pendulum

from parsec.core.core_events import CoreEvent
from parsec.core.types import WorkspaceRole
from parsec.core.logged_core import LoggedCore
from parsec.core.fs import UserFS
from parsec.api.transport import Transport, TransportError

from tests.common.trio_clock import real_clock_timeout


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

    async def __aenter__(self, *args, **kwargs):
        return self

    async def __aexit__(self, *args, **kwargs):
        return True


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
    This is *even* more tricky considering we want the cores involved in the
    sharing to endup in a stable state (no event wildly fired or coroutine in
    the middle of a processing when leaving this function) to avoid polluting
    the actual test.
    """
    with ExitStack() as stack:
        if isinstance(creator, LoggedCore):
            creator_spy = stack.enter_context(creator.event_bus.listen())
            creator_user_fs = creator.user_fs
        elif isinstance(creator, UserFS):
            creator_user_fs = creator
            creator_spy = None
        else:
            raise ValueError(f"{creator!r} is not a {UserFS!r} or a {LoggedCore!r}")

        all_user_fss = []
        shared_with_spies = []
        shared_with_cores_and_user_fss = []
        for x in shared_with:
            if isinstance(x, LoggedCore):
                user_fs = x.user_fs
                core = x
                shared_with_spies.append(stack.enter_context(x.event_bus.listen()))
            elif isinstance(x, UserFS):
                user_fs = x
                core = None
            else:
                raise ValueError(f"{x!r} is not a {UserFS!r} or a {LoggedCore!r}")
            all_user_fss.append(user_fs)
            if user_fs.device.user_id != creator_user_fs.device.user_id:
                shared_with_cores_and_user_fss.append((core, user_fs))

        wid = await creator_user_fs.workspace_create(name)
        await creator_user_fs.sync()
        workspace = creator_user_fs.get_workspace(wid)
        await workspace.sync()

        for recipient_core, recipient_user_fs in shared_with_cores_and_user_fss:
            await creator_user_fs.workspace_share(
                wid, recipient_user_fs.device.user_id, WorkspaceRole.MANAGER
            )
            # Don't try to double-cross core's message monitor !
            if not recipient_core:
                await recipient_user_fs.process_last_messages()

        async with real_clock_timeout():
            if creator_spy:
                await creator_spy.wait_multiple(
                    [CoreEvent.FS_WORKSPACE_CREATED, CoreEvent.BACKEND_REALM_ROLES_UPDATED]
                )
            for spy in shared_with_spies:
                await spy.wait_multiple(
                    [
                        CoreEvent.BACKEND_REALM_ROLES_UPDATED,
                        CoreEvent.BACKEND_MESSAGE_RECEIVED,
                        CoreEvent.SHARING_UPDATED,
                    ]
                )

        for user_fs in all_user_fss:
            await user_fs.sync()

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


@pytest.fixture
def next_timestamp():
    """On windows, 2 calls to `pendulum.now()` can yield the same value.
    For some tests, this creates edges cases we want to avoid.
    """
    last_timestamp = None

    def _next_timestamp():
        if pendulum.has_test_now():
            return pendulum.now()
        nonlocal last_timestamp
        while last_timestamp == pendulum.now():
            pass
        last_timestamp = pendulum.now()
        return last_timestamp

    return _next_timestamp
