import os
import pytest
from string import ascii_lowercase
from hypothesis import strategies as st, note
from hypothesis.stateful import Bundle

from tests.hypothesis.common import rule, rule_once


# The point is not to find breaking filenames here, so keep it simple
st_entry_name = st.text(alphabet=ascii_lowercase, min_size=1, max_size=3)


@pytest.mark.slow
@pytest.mark.trio
async def test_core_offline_restart_and_tree(
    TrioDriverRuleBasedStateMachine,
    oracle_fs_factory,
    core_factory,
    core_sock_factory,
    device_factory,
):
    class RestartCore(Exception):
        pass

    class CoreOfflineRestartAndTree(TrioDriverRuleBasedStateMachine):
        Files = Bundle("file")
        Folders = Bundle("folder")

        async def trio_runner(self, task_status):
            device = device_factory()

            self.core = None
            self.sys_cmd = lambda x: self.communicator.send(("sys", x))
            self.core_cmd = lambda x: self.communicator.send(("core", x))
            self.oracle_fs = oracle_fs_factory()

            async def run_core(on_ready):
                self.core = await core_factory(devices=[device], config={"auto_sync": False})
                try:
                    await self.core.login(device)
                    sock = core_sock_factory(self.core)

                    await on_ready(sock)

                    while True:
                        target, msg = await self.communicator.trio_recv()
                        if target == "core":
                            await sock.send(msg)
                            rep = await sock.recv()
                            await self.communicator.trio_respond(rep)
                        elif msg == "restart!":
                            raise RestartCore()

                finally:
                    await self.core.teardown()

            async def bootstrap_core(sock):
                task_status.started()

            async def restart_core_done(sock):
                await self.communicator.trio_respond(True)

            on_ready = bootstrap_core
            while True:
                try:
                    await run_core(on_ready)
                except RestartCore:
                    on_ready = restart_core_done

        @rule_once(target=Folders)
        def init_root(self):
            return "/"

        @rule(target=Files, parent=Folders, name=st_entry_name)
        def create_file(self, parent, name):
            path = os.path.join(parent, name)
            rep = self.core_cmd({"cmd": "file_create", "path": path})
            note(rep)
            expected_status = self.oracle_fs.create_file(path)
            assert rep["status"] == expected_status
            return path

        @rule(target=Folders, parent=Folders, name=st_entry_name)
        def create_folder(self, parent, name):
            path = os.path.join(parent, name)
            rep = self.core_cmd({"cmd": "folder_create", "path": path})
            note(rep)
            expected_status = self.oracle_fs.create_folder(path)
            assert rep["status"] == expected_status
            return path

        @rule(path=Files)
        def delete_file(self, path):
            rep = self.core_cmd({"cmd": "delete", "path": path})
            note(rep)
            expected_status = self.oracle_fs.delete(path)
            assert rep["status"] == expected_status

        @rule(path=Folders)
        def delete_folder(self, path):
            rep = self.core_cmd({"cmd": "delete", "path": path})
            note(rep)
            expected_status = self.oracle_fs.delete(path)
            assert rep["status"] == expected_status

        @rule(target=Files, src=Files, dst_parent=Folders, dst_name=st_entry_name)
        def move_file(self, src, dst_parent, dst_name):
            dst = os.path.join(dst_parent, dst_name)
            rep = self.core_cmd({"cmd": "move", "src": src, "dst": dst})
            note(rep)
            expected_status = self.oracle_fs.move(src, dst)
            assert rep["status"] == expected_status
            return dst

        @rule(target=Folders, src=Folders, dst_parent=Folders, dst_name=st_entry_name)
        def move_folder(self, src, dst_parent, dst_name):
            dst = os.path.join(dst_parent, dst_name)
            rep = self.core_cmd({"cmd": "move", "src": src, "dst": dst})
            note(rep)
            expected_status = self.oracle_fs.move(src, dst)
            assert rep["status"] == expected_status
            return dst

        @rule()
        def restart(self):
            rep = self.sys_cmd("restart!")
            assert rep is True

    await CoreOfflineRestartAndTree.run_test()
