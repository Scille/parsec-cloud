# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import threading
import trio
import pathlib
from contextlib import contextmanager
from inspect import iscoroutinefunction

from parsec.core.fs import FS
from parsec.utils import start_task
from parsec.core.mountpoint import mountpoint_manager_factory


@pytest.fixture
def base_mountpoint(tmpdir):
    return pathlib.Path(tmpdir / "base_mountpoint")


@pytest.fixture
@pytest.mark.mountpoint
def mountpoint_service_factory(tmpdir, alice, user_fs_factory):
    """
    Run a trio loop with fs and mountpoint manager in a separate thread to
    allow blocking operations on the mountpoint in the test
    """

    class MountpointService:
        def __init__(self, base_mountpoint):
            self.base_mountpoint = pathlib.Path(base_mountpoint)
            # Provide a default workspace given we cannot create it through FUSE
            self.default_workspace_name = "w"
            self._task = None
            self._portal = None
            self._need_stop = trio.Event()
            self._stopped = trio.Event()
            self._need_start = trio.Event()
            self._started = trio.Event()
            self._ready = threading.Event()

        def execute(self, cb):
            if iscoroutinefunction(cb):
                self._portal.run(cb, *self._task.value)
            else:
                self._portal.run_sync(cb, *self._task.value)

        def get_default_workspace_mountpoint(self):
            return self.get_workspace_mountpoint(self.default_workspace_name)

        def get_workspace_mountpoint(self, workspace_name):
            return self.base_mountpoint / workspace_name

        def start(self):
            async def _start():
                self._need_start.set()
                await self._started.wait()

            self._portal.run(_start)

        def stop(self):
            async def _stop():
                if self._started.is_set():
                    self._need_stop.set()
                    await self._stopped.wait()

            self._portal.run(_stop)

        def init(self):
            self._thread = threading.Thread(target=trio.run, args=(self._service,))
            self._thread.setName("MountpointService")
            self._thread.start()
            self._ready.wait()

        async def _teardown(self):
            self._nursery.cancel_scope.cancel()

        def teardown(self):
            if not self._portal:
                return
            self.stop()
            self._portal.run(self._teardown)
            self._thread.join()

        async def _service(self):
            self._portal = trio.BlockingTrioPortal()

            async def _mountpoint_controlled_cb(*, task_status=trio.TASK_STATUS_IGNORED):
                async with user_fs_factory(alice) as user_fs:

                    self.default_workspace_id = await user_fs.workspace_create(
                        f"{self.default_workspace_name}"
                    )

                    async with mountpoint_manager_factory(
                        user_fs, user_fs.event_bus, self.base_mountpoint, debug=False
                    ) as mountpoint_manager:

                        await mountpoint_manager.mount_workspace(self.default_workspace_id)

                        fs = FS(user_fs)
                        task_status.started((user_fs, fs, mountpoint_manager))
                        await trio.sleep_forever()

            async with trio.open_nursery() as self._nursery:
                self._ready.set()
                while True:
                    await self._need_start.wait()
                    self._need_stop.clear()
                    self._stopped.clear()

                    self._task = await start_task(self._nursery, _mountpoint_controlled_cb)
                    self._started.set()

                    await self._need_stop.wait()
                    self._need_start.clear()
                    self._started.clear()
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
