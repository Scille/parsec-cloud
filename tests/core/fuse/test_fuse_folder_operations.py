import os
import trio
import attr
import threading
import pytest
from pathlib import Path
from string import ascii_lowercase
from hypothesis.stateful import (
    RuleBasedStateMachine,
    Bundle,
    initialize,
    rule,
    run_state_machine_as_test,
)
from hypothesis import strategies as st

from tests.common import call_with_control


# The point is not to find breaking filenames here, so keep it simple
st_entry_name = st.text(alphabet=ascii_lowercase, min_size=1, max_size=3)


class expect_raises:
    def __init__(self, expected_exc):
        self.expected_exc = expected_exc

    def __enter__(self):
        __tracebackhide__ = True
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        __tracebackhide__ = True

        if self.expected_exc is None:
            return False

        if not exc_type:
            raise AssertionError(f"DID NOT RAISED {self.expected_exc!r}")

        if not isinstance(exc_value, type(self.expected_exc)):
            raise AssertionError(
                f"RAISED {exc_value!r} BUT EXPECTED {self.expected_exc!r}"
            ) from exc_value

        return True


@attr.s
class PathElement:
    absolute_path = attr.ib()
    parsec_root = attr.ib()
    oracle_root = attr.ib()

    def to_oracle(self):
        return self.oracle_root / self.absolute_path[1:]

    def to_parsec(self):
        return self.parsec_root / self.absolute_path[1:]

    def __truediv__(self, path):
        return PathElement(
            os.path.join(self.absolute_path, path), self.parsec_root, self.oracle_root
        )


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
                await core.fuse_manager.start(str(mountpoint))
                await started_cb(core=core)

        async def _watchdog(nursery):
            await core_service_need_kill.wait()
            nursery.cancel_scope.cancel()

        async with trio.open_nursery() as nursery:

            async def _core_service_kill():
                nursery.cancel_scope.cancel()

            core_service_kill = lambda: portal.run(_core_service_kill)

            nursery.start_soon(_watchdog, nursery)
            core_service_ready.set()
            while True:
                await core_service_need_start.wait()
                core_service_need_stop.clear()
                core_service_stopped.clear()
                core_controller = await nursery.start(call_with_control, _core_controlled_cb)
                core_service_started.set()

                await core_service_need_stop.wait()
                core_service_need_start.clear()
                core_service_started.clear()
                await core_controller.stop()
                core_service_stopped.set()

    thread = threading.Thread(target=trio.run, args=(_core_service,))
    thread.start()
    core_service_ready.wait()

    tentative = 0

    class FuseFileOperationsStateMachine(RuleBasedStateMachine):
        Files = Bundle("file")
        Folders = Bundle("folder")

        @initialize(target=Folders)
        def init(self):
            nonlocal tentative
            tentative += 1

            core_service_start()

            self.folder_oracle = Path(tmpdir / f"oracle-test-{tentative}")
            self.folder_oracle.mkdir()
            oracle_root = self.folder_oracle / "root"
            oracle_root.mkdir()
            self.folder_oracle.chmod(0o500)  # Root oracle can no longer be removed this way

            return PathElement("/", Path(str(mountpoint)), oracle_root)

        def teardown(self):
            core_service_stop()

        @rule(target=Files, parent=Folders, name=st_entry_name)
        def touch(self, parent, name):
            path = parent / name

            expected_exc = None
            try:
                path.to_oracle().touch(exist_ok=False)
            except OSError as exc:
                expected_exc = exc

            with expect_raises(expected_exc):
                path.to_parsec().touch(exist_ok=False)

            return path

        @rule(target=Folders, parent=Folders, name=st_entry_name)
        def mkdir(self, parent, name):
            path = parent / name

            expected_exc = None
            try:
                path.to_oracle().mkdir(exist_ok=False)
            except OSError as exc:
                expected_exc = exc

            with expect_raises(expected_exc):
                path.to_parsec().mkdir(exist_ok=False)

            return path

        @rule(path=Files)
        def unlink(self, path):
            expected_exc = None
            try:
                path.to_oracle().unlink()
            except OSError as exc:
                expected_exc = exc

            with expect_raises(expected_exc):
                path.to_parsec().unlink()

        # TODO: trying to remove root gives a DeviceBusy error instead of PermissionError...
        @rule(path=Folders.filter(lambda x: x.absolute_path != "/"))
        def rmdir(self, path):
            expected_exc = None
            try:
                path.to_oracle().rmdir()
            except OSError as exc:
                expected_exc = exc

            with expect_raises(expected_exc):
                path.to_parsec().rmdir()

        def _move(self, src, dst_parent, dst_name):
            dst = dst_parent / dst_name

            expected_exc = None
            try:
                src.to_oracle().rename(str(dst.to_oracle()))
            except OSError as exc:
                expected_exc = exc

            with expect_raises(expected_exc):
                src.to_parsec().rename(str(dst.to_parsec()))

            return dst

        @rule(target=Files, src=Files, dst_parent=Folders, dst_name=st_entry_name)
        def move_file(self, src, dst_parent, dst_name):
            return self._move(src, dst_parent, dst_name)

        @rule(target=Folders, src=Folders, dst_parent=Folders, dst_name=st_entry_name)
        def move_folder(self, src, dst_parent, dst_name):
            return self._move(src, dst_parent, dst_name)

    try:
        run_state_machine_as_test(FuseFileOperationsStateMachine, settings=hypothesis_settings)
    finally:
        core_service_kill()
        thread.join()
