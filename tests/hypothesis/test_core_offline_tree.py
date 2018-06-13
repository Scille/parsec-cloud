import os
import pytest
from string import ascii_lowercase
from hypothesis import strategies as st, note
from hypothesis.stateful import Bundle

from tests.common import connect_core, core_factory
from tests.hypothesis.common import OracleFS, rule_once, rule


# The point is not to find breaking filenames here, so keep it simple
st_entry_name = st.text(alphabet=ascii_lowercase, min_size=1, max_size=3)


@pytest.mark.slow
@pytest.mark.trio
async def test_offline_core_tree(
    TrioDriverRuleBasedStateMachine, mocked_local_storage_connection, backend_addr, tmpdir, alice
):
    class CoreOfflineRWFile(TrioDriverRuleBasedStateMachine):
        Files = Bundle("file")
        Folders = Bundle("folder")
        count = 0

        async def trio_runner(self, task_status):
            mocked_local_storage_connection.reset()
            self.oracle_fs = OracleFS()

            type(self).count += 1
            config = {
                "base_settings_path": tmpdir.mkdir("try-%s" % self.count).strpath,
                "backend_addr": backend_addr,
            }

            async with core_factory(**config) as core:
                await core.login(alice)
                async with connect_core(core) as sock:

                    self.core_cmd = self.communicator.send
                    task_status.started()

                    while True:
                        msg = await self.communicator.trio_recv()
                        await sock.send(msg)
                        rep = await sock.recv()
                        await self.communicator.trio_respond(rep)

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

    await CoreOfflineRWFile.run_test()
