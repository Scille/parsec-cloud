import os
import pytest
from string import ascii_lowercase
from hypothesis import strategies as st, note
from hypothesis_trio.stateful import Bundle, run_state_machine_as_test

from tests.hypothesis.common import initialize, rule


# The point is not to find breaking filenames here, so keep it simple
st_entry_name = st.text(alphabet=ascii_lowercase, min_size=1, max_size=3)


@pytest.mark.slow
def test_core_offline_restart_and_tree(
    BaseCoreAloneStateMachine, oracle_fs_factory, hypothesis_settings
):
    class CoreOfflineRestartAndTree(BaseCoreAloneStateMachine):
        Files = Bundle("file")
        Folders = Bundle("folder")

        @initialize(target=Folders)
        async def init_oracle_and_bundle(self):
            self.oracle_fs = oracle_fs_factory()
            return "/"

        @rule(target=Files, parent=Folders, name=st_entry_name)
        async def create_file(self, parent, name):
            path = os.path.join(parent, name)
            rep = await self.core_cmd({"cmd": "file_create", "path": path})
            note(rep)
            expected_status = self.oracle_fs.create_file(path)
            assert rep["status"] == expected_status
            return path

        @rule(target=Folders, parent=Folders, name=st_entry_name)
        async def create_folder(self, parent, name):
            path = os.path.join(parent, name)
            rep = await self.core_cmd({"cmd": "folder_create", "path": path})
            note(rep)
            expected_status = self.oracle_fs.create_folder(path)
            assert rep["status"] == expected_status
            return path

        @rule(path=Files)
        async def delete_file(self, path):
            rep = await self.core_cmd({"cmd": "delete", "path": path})
            note(rep)
            expected_status = self.oracle_fs.delete(path)
            assert rep["status"] == expected_status

        @rule(path=Folders)
        async def delete_folder(self, path):
            rep = await self.core_cmd({"cmd": "delete", "path": path})
            note(rep)
            expected_status = self.oracle_fs.delete(path)
            assert rep["status"] == expected_status

        @rule(target=Files, src=Files, dst_parent=Folders, dst_name=st_entry_name)
        async def move_file(self, src, dst_parent, dst_name):
            dst = os.path.join(dst_parent, dst_name)
            rep = await self.core_cmd({"cmd": "move", "src": src, "dst": dst})
            note(rep)
            expected_status = self.oracle_fs.move(src, dst)
            assert rep["status"] == expected_status
            return dst

        @rule(target=Folders, src=Folders, dst_parent=Folders, dst_name=st_entry_name)
        async def move_folder(self, src, dst_parent, dst_name):
            dst = os.path.join(dst_parent, dst_name)
            rep = await self.core_cmd({"cmd": "move", "src": src, "dst": dst})
            note(rep)
            expected_status = self.oracle_fs.move(src, dst)
            assert rep["status"] == expected_status
            return dst

        @rule()
        async def restart(self):
            await self.restart_core()

    run_state_machine_as_test(CoreOfflineRestartAndTree, settings=hypothesis_settings)
