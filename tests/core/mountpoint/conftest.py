# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import threading
import trio
import pathlib
from contextlib import contextmanager
from inspect import iscoroutinefunction

from parsec.utils import start_task
from parsec.core.mountpoint import mountpoint_manager_factory


@pytest.fixture
def base_mountpoint(tmpdir):
    return pathlib.Path(tmpdir / "base_mountpoint")


@pytest.fixture
@pytest.mark.mountpoint
def mountpoint_service_factory(tmpdir, alice, user_fs_factory, reset_testbed):
    """
    Run a trio loop with fs and mountpoint manager in a separate thread to
    allow blocking operations on the mountpoint in the test
    """

    do_reset_testbed = reset_testbed

    class MountpointService:
        def __init__(self, base_mountpoint):
            self.base_mountpoint = pathlib.Path(base_mountpoint)
            # Provide a default workspace given we cannot create it through FUSE
            self.default_workspace_name = "w"
            self._task = None
            self._trio_token = None
            self._need_stop = trio.Event()
            self._stopped = trio.Event()
            self._need_start = trio.Event()
            self._started = trio.Event()
            self._ready = threading.Event()

        def execute(self, cb):
            if iscoroutinefunction(cb):
                trio.from_thread.run(cb, *self._task.value, trio_token=self._trio_token)
            else:
                trio.from_thread.run_sync(cb, *self._task.value, trio_token=self._trio_token)

        def get_default_workspace_mountpoint(self):
            return self.get_workspace_mountpoint(self.default_workspace_name)

        def get_workspace_mountpoint(self, workspace_name):
            return self.base_mountpoint / workspace_name

        def start(self):
            async def _start():
                self._need_start.set()
                await self._started.wait()

            trio.from_thread.run(_start, trio_token=self._trio_token)

        def stop(self, reset_testbed=True):
            async def _stop():
                if self._started.is_set():
                    self._need_stop.set()
                    await self._stopped.wait()
                    if reset_testbed:
                        await do_reset_testbed(keep_logs=True)

            trio.from_thread.run(_stop, trio_token=self._trio_token)

        def init(self):
            self._thread = threading.Thread(target=trio.run, args=(self._service,))
            self._thread.setName("MountpointService")
            self._thread.start()
            self._ready.wait()

        async def _teardown(self):
            self._nursery.cancel_scope.cancel()

        def teardown(self):
            if self._trio_token is None:
                return
            self.stop(reset_testbed=False)
            trio.from_thread.run(self._teardown, trio_token=self._trio_token)
            self._thread.join()

        async def _service(self):
            self._trio_token = trio.hazmat.current_trio_token()

            async def _mountpoint_controlled_cb(*, task_status=trio.TASK_STATUS_IGNORED):
                async with user_fs_factory(alice, offline=True) as user_fs:

                    self.default_workspace_id = await user_fs.workspace_create(
                        f"{self.default_workspace_name}"
                    )

                    async with mountpoint_manager_factory(
                        user_fs, user_fs.event_bus, self.base_mountpoint, debug=False
                    ) as mountpoint_manager:

                        await mountpoint_manager.mount_workspace(self.default_workspace_id)

                        task_status.started((user_fs, mountpoint_manager))
                        await trio.sleep_forever()

            async with trio.open_service_nursery() as self._nursery:
                self._ready.set()
                while True:
                    await self._need_start.wait()
                    self._need_stop = trio.Event()
                    self._stopped = trio.Event()

                    self._task = await start_task(self._nursery, _mountpoint_controlled_cb)
                    self._started.set()

                    await self._need_stop.wait()
                    self._need_start = trio.Event()
                    self._started = trio.Event()
                    await self._task.cancel_and_join()
                    self._stopped.set()

    count = 0

    @contextmanager
    def _mountpoint_service_factory(base_mountpoint=None):
        nonlocal count
        count += 1
        if not base_mountpoint:
            base_mountpoint = tmpdir / f"mountpoint-svc-{count}"

        mountpoint_service = MountpointService(str(base_mountpoint))
        mountpoint_service.init()
        try:
            yield mountpoint_service

        finally:
            mountpoint_service.teardown()

    return _mountpoint_service_factory


@pytest.fixture
def mountpoint_service(mountpoint_service_factory):
    with mountpoint_service_factory() as fuse:
        yield fuse
