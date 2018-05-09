import os
import pytest
from hypothesis import strategies as st, note
from hypothesis.stateful import Bundle

from tests.common import connect_core, core_factory
from tests.hypothesis.common import OracleFS, rule, rule_once


@pytest.mark.slow
@pytest.mark.trio
async def test_core_offline_restart_and_tree(
    TrioDriverRuleBasedStateMachine, mocked_local_storage_connection, backend_addr, tmpdir, alice
):

    class RestartCore(Exception):
        pass

    st_entry_name = st.text(min_size=1).filter(lambda x: "/" not in x)

    class CoreOfflineRestartAndTree(TrioDriverRuleBasedStateMachine):
        Files = Bundle("file")
        Folders = Bundle("folder")
        count = 0

        async def trio_runner(self, task_status):
            mocked_local_storage_connection.reset()

            type(self).count += 1
            config = {
                "base_settings_path": tmpdir.mkdir("try-%s" % self.count).strpath,
                "backend_addr": backend_addr,
            }

            self.sys_cmd = lambda x: self.communicator.send(("sys", x))
            self.core_cmd = lambda x: self.communicator.send(("core", x))
            self.oracle_fs = OracleFS()

            async def run_core(on_ready):
                async with core_factory(**config) as core:
                    self.core = core

                    await core.login(alice)
                    async with connect_core(core) as sock:

                        await on_ready(sock)

                        while True:
                            target, msg = await self.communicator.trio_recv()
                            if target == "core":
                                await sock.send(msg)
                                rep = await sock.recv()
                                await self.communicator.trio_respond(rep)
                            elif msg == "restart!":
                                raise RestartCore()

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

        @rule(path=st.one_of(Folders, Files))
        def flush(self, path):
            # Note that fs api should automatically do the flush when creating
            # files/folders. So manual flush such as here has no effect.
            rep = self.core_cmd({"cmd": "flush", "path": path})
            note(rep)
            expected_status = self.oracle_fs.flush(path)
            assert rep["status"] == expected_status

    await CoreOfflineRestartAndTree.run_test()
