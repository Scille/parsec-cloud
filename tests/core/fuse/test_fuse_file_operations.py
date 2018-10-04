import os
import trio
import threading
import pytest
from hypothesis.stateful import RuleBasedStateMachine, initialize, rule, run_state_machine_as_test
from hypothesis import strategies as st

from tests.common import call_with_control


@pytest.mark.slow
def test_fuse_file_operations(
    tmpdir, unused_tcp_addr, hypothesis_settings, device_factory, core_factory
):
    mountpoint = tmpdir / "mountpoint"
    mountpoint.mkdir()

    # Run core in a separate thread to avoid deadlock with blocking fs calls

    core_service_stop = None
    core_service_start = None
    core_service_kill = None
    core_service_ready = threading.Event()

    async def _core_service():
        nonlocal core_service_stop, core_service_start, core_service_kill

        core_service_need_stop = trio.Event()
        core_service_stopped = trio.Event()
        core_service_need_start = trio.Event()
        core_service_started = trio.Event()
        core_service_need_kill = trio.Event()

        async def _core_service_stop():
            core_service_need_stop.set()
            await core_service_stopped.wait()

        async def _core_service_start():
            core_service_need_start.set()
            await core_service_started.wait()

        portal = trio.BlockingTrioPortal()
        core_service_stop = lambda: portal.run(_core_service_stop)
        core_service_start = lambda: portal.run(_core_service_start)

        async def _core_controlled_cb(started_cb):
            device = device_factory()
            async with core_factory(
                devices=[device], config={"backend_addr": unused_tcp_addr}
            ) as core:
                await core.login(device)
                await core.fs.workspace_create("/foo")
                await core.fs.file_create("/foo/bar.txt")
                await core.fuse_manager.start(str(mountpoint))
                await started_cb(core=core)

        async def _watchdog(nursery):
            print("watchdog ready")
            await core_service_need_kill.wait()
            nursery.cancel_scope.cancel()

        async with trio.open_nursery() as nursery:

            async def _core_service_kill():
                nursery.cancel_scope.cancel()

            core_service_kill = lambda: portal.run(_core_service_kill)

            nursery.start_soon(_watchdog, nursery)
            print("core service idle")
            core_service_ready.set()
            while True:
                await core_service_need_start.wait()
                print("starting core")
                core_service_need_stop.clear()
                core_service_stopped.clear()
                core_controller = await nursery.start(call_with_control, _core_controlled_cb)
                core_service_started.set()

                print("core ready")
                await core_service_need_stop.wait()
                print("stopping core")
                core_service_need_start.clear()
                core_service_started.clear()
                await core_controller.stop()
                core_service_stopped.set()
                print("core stopped")

    thread = threading.Thread(target=trio.run, args=(_core_service,))
    thread.start()
    core_service_ready.wait()

    class FuseFileOperationsStateMachine(RuleBasedStateMachine):
        @initialize()
        def init(self):
            print("init")
            core_service_start()
            print("init ok")

            self.fd = os.open(mountpoint / "foo/bar.txt", os.O_RDWR)

        def teardown(self):
            core_service_stop()

        @rule(size=st.integers(min_value=0))
        def read(self, size):
            print("read")
            data = os.read(self.fd, size)
            assert len(data) <= size

        @rule(content=st.binary())
        def write(self, content):
            print("write")
            ret = os.write(self.fd, content)
            assert ret == len(content)

        @rule(
            length=st.integers(min_value=0),
            seek_type=st.one_of(st.just(os.SEEK_SET), st.just(os.SEEK_CUR), st.just(os.SEEK_END)),
        )
        def seek(self, length, seek_type):
            print("seek")
            os.lseek(self.fd, length, seek_type)

        @rule(length=st.integers(min_value=0))
        def truncate(self, length):
            print("truncate")
            os.ftruncate(self.fd, length)

        @rule()
        def reopen(self):
            print("reopen")
            os.close(self.fd)
            self.fd = os.open(mountpoint / "foo/bar.txt", os.O_RDWR)

    try:
        run_state_machine_as_test(FuseFileOperationsStateMachine, settings=hypothesis_settings)
    finally:
        core_service_kill()
        thread.join()
