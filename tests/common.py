# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from unittest.mock import Mock
from inspect import iscoroutinefunction
from contextlib import ExitStack, contextmanager, asynccontextmanager
import time

import trio
import attr
import pendulum

from parsec.core.core_events import CoreEvent
from parsec.core.types import WorkspaceRole
from parsec.core.logged_core import LoggedCore
from parsec.core.fs import UserFS
from parsec.api.transport import Transport, TransportError
from parsec.core.types.local_device import LocalDevice, DeviceID


def addr_with_device_subdomain(addr, device_id):
    """
    Useful to have each device access the same backend with a different hostname
    so tcp_stream_spy can put some offline and leave others online
    """
    device_specific_hostname = f"{device_id.user_id}_{device_id.device_name}.{addr.hostname}"
    return type(addr).from_url(addr.to_url().replace(addr.hostname, device_specific_hostname, 1))


@asynccontextmanager
async def real_clock_fail_after(seconds: float):
    # In tests we use a mock clock to make parsec code faster by not staying idle,
    # however we might also want ensure some test code doesn't take too long.
    # Hence using `trio.fail_after` in test code doesn't play nice with mock clock
    # (especially given the CI can be unpredictably slow)...
    # The solution is to have our own fail_after that use the real monotonic clock.

    # Starting a thread can be very slow (looking at you, Windows) so better
    # take the starting time here
    start = time.monotonic()
    event_occured = False
    async with trio.open_nursery() as nursery:

        def _run_until_timeout_or_event_occured():
            while not event_occured and time.monotonic() - start < seconds:
                # cancelling `_watchdog` coroutine doesn't stop the thread,
                # so we sleep only by a short amount of time in order to
                # detect early enough that we are no longer needed
                time.sleep(0.01)

        async def _watchdog():
            await trio.to_thread.run_sync(_run_until_timeout_or_event_occured)
            if not event_occured:
                raise trio.TooSlowError()

        # Note: We could have started the thread directly instead of using
        # trio's thread support.
        # This would allow us to use a non-async contextmanager to better mimic
        # `trio.fail_after`, however this would prevent us from using trio's
        # threadpool system which is good given it allows us to reuse the thread
        # and hence avoid most of it cost
        nursery.start_soon(_watchdog)
        try:
            yield
        finally:
            event_occured = True
        nursery.cancel_scope.cancel()


__freeze_time_dict = {}


def _timestamp_mockup(device):
    _, time = __freeze_time_dict.get(device.device_id, (None, None))
    return time if time is not None else pendulum.now()


@contextmanager
def freeze_device_time(device, current_time):
    # Parse time
    if isinstance(current_time, str):
        current_time = pendulum.parse(current_time)

    # Get device id
    if isinstance(device, LocalDevice):
        device_id = device.device_id
    elif isinstance(device, DeviceID):
        device_id = device
    else:
        assert False, device

    # Apply mockup (idempotent)
    type(device).timestamp = _timestamp_mockup

    # Save previous context
    previous_task, previous_time = __freeze_time_dict.get(device_id, (None, None))

    # Get current trio task
    try:
        current_task = trio.lowlevel.current_task()
    except RuntimeError:
        current_task = None

    # Ensure time has not been frozen from another coroutine
    assert previous_task in (None, current_task)

    try:
        # Set new context
        __freeze_time_dict[device_id] = (current_task, current_time)
        yield current_time
    finally:
        # Restore previous context
        __freeze_time_dict[device_id] = (previous_task, previous_time)


__freeze_time_task = None


@contextmanager
def freeze_time(time=None, device=None):
    # Get current time if not provided
    if time is None:
        time = pendulum.now()

    # Freeze a single device
    if device is not None:
        with freeze_device_time(device, time) as time:
            yield time
        return

    # Parse time
    global __freeze_time_task
    if isinstance(time, str):
        time = pendulum.parse(time)

    # Save previous context
    previous_task = __freeze_time_task
    previous_time = pendulum.get_test_now()

    # Get current trio task
    try:
        current_task = trio.lowlevel.current_task()
    except RuntimeError:
        current_task = None

    # Ensure time has not been frozen from another coroutine
    assert previous_task in (None, current_task)

    try:
        # Set new context
        __freeze_time_task = current_task
        pendulum.set_test_now(time)
        try:
            from libparsec.types import freeze_time as _Rs_freeze_time
        except ImportError:
            pass
        else:
            _Rs_freeze_time(time)

        yield time
    finally:
        # Restore previous context
        __freeze_time_task = previous_task
        pendulum.set_test_now(previous_time)
        try:
            from libparsec.types import freeze_time as _Rs_freeze_time
        except ImportError:
            pass
        else:
            _Rs_freeze_time(time)


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

        async with real_clock_fail_after(1):
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


_FIXTURES_CUSTOMIZATIONS = {
    "alice_profile",
    "alice_initial_local_user_manifest",
    "alice2_initial_local_user_manifest",
    "alice_initial_remote_user_manifest",
    "alice_has_human_handle",
    "alice_has_device_label",
    "bob_profile",
    "bob_initial_local_user_manifest",
    "bob_initial_remote_user_manifest",
    "bob_has_human_handle",
    "bob_has_device_label",
    "adam_profile",
    "adam_initial_local_user_manifest",
    "adam_initial_remote_user_manifest",
    "adam_has_human_handle",
    "adam_has_device_label",
    "mallory_profile",
    "mallory_initial_local_user_manifest",
    "mallory_has_human_handle",
    "mallory_has_device_label",
    "backend_not_populated",
    "backend_has_webhook",
    "backend_force_mocked",
    "backend_over_ssl",
    "backend_forward_proto_enforce_https",
    "backend_spontaneous_organization_boostrap",
    "logged_gui_as_admin",
    "fake_preferred_org_creation_backend_addr",
    "blockstore_mode",
    "real_data_storage",
    "gui_language",
}


def customize_fixtures(**customizations):
    """
    Should be used as a decorator on tests to provide custom settings to fixtures.
    """
    assert not customizations.keys() - _FIXTURES_CUSTOMIZATIONS

    def wrapper(fn):
        try:
            getattr(fn, "_fixtures_customization").update(customizations)
        except AttributeError:
            setattr(fn, "_fixtures_customization", customizations)
        return fn

    return wrapper
