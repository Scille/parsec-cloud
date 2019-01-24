import os
import pytest
import threading
import trio
import pathlib
from contextlib import contextmanager

from parsec.core.mountpoint import mountpoint_manager

from tests.common import call_with_control


@pytest.fixture(params=("thread", "process"))
def fuse_mode(request):
    if request.param == "process":
        pytest.skip("Quick fix for CI...")
    if request.param == "thread" and os.name == "nt":
        pytest.skip("Windows doesn't support threaded fuse")
    if request.param == "process" and os.name == "posix":
        pytest.skip("Nobody ain't not time for this on POSIX !!!")
    return request.param


@pytest.fixture
@pytest.mark.fuse
def fuse_service_factory(tmpdir, unused_tcp_addr, alice, event_bus_factory, fs_factory, fuse_mode):
    """
    Run a trio loop with fs and fuse in a separate thread to allow
    blocking operations on the mountpoint in the test
    """

    class FuseService:
        def __init__(self, mountpoint):
            self.mountpoint = pathlib.Path(mountpoint)
            # Provide a default workspace given we cannot create it through FUSE
            self.default_workspace_name = "w"
            self._portal = None
            self._need_stop = trio.Event()
            self._stopped = trio.Event()
            self._need_start = trio.Event()
            self._started = trio.Event()
            self._ready = threading.Event()

        @property
        def default_workpsace(self):
            return self.mountpoint / self.default_workspace_name

        async def _start(self):
            self._need_start.set()
            await self._started.wait()

        async def _stop(self):
            if self._started.is_set():
                self._need_stop.set()
                await self._stopped.wait()

        def start(self):
            self._portal.run(self._start)

        def stop(self):
            self._portal.run(self._stop)

        def init(self):
            self._thread = threading.Thread(target=trio.run, args=(self._service,))
            self._thread.setName("FuseService")
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

            async def _fuse_controlled_cb(started_cb):
                async with fs_factory(alice) as fs:

                    await fs.workspace_create(f"/{self.default_workspace_name}")

                    async with trio.open_nursery() as nursery:
                        async with mountpoint_manager(
                            fs, fs.event_bus, self.mountpoint, nursery, mode=fuse_mode
                        ) as fuse_task:
                            await started_cb(fs=fs, fuse=fuse_task)

            async with trio.open_nursery() as self._nursery:
                self._ready.set()
                while True:
                    await self._need_start.wait()
                    self._need_stop.clear()
                    self._stopped.clear()

                    fuse_controller = await self._nursery.start(
                        call_with_control, _fuse_controlled_cb
                    )
                    self._started.set()

                    await self._need_stop.wait()
                    self._need_start.clear()
                    self._started.clear()
                    await fuse_controller.stop()
                    self._stopped.set()

    count = 0

    @contextmanager
    def _fuse_service_factory(mountpoint=None):
        nonlocal count
        count += 1
        if not mountpoint:
            mountpoint = tmpdir / f"mountpoint-{count}"
            mountpoint.mkdir()

        fuse_service = FuseService(str(mountpoint))
        fuse_service.init()
        try:
            yield fuse_service
        finally:
            fuse_service.teardown()

    return _fuse_service_factory


@pytest.fixture
def fuse_service(fuse_service_factory):
    with fuse_service_factory() as fuse:
        yield fuse
