import os
import pytest
from hypothesis import strategies as st
from hypothesis_trio.stateful import (
    initialize,
    rule,
    run_state_machine_as_test,
    TrioRuleBasedStateMachine,
    Bundle,
)
from string import ascii_lowercase

from tests.common import call_with_control


# The point is not to find breaking filenames here, so keep it simple
st_entry_name = st.text(alphabet=ascii_lowercase, min_size=1, max_size=3)


@pytest.mark.slow
@pytest.mark.skipif(os.name == "nt", reason="Windows path style not compatible with oracle")
def test_fs_offline_restart_and_tree(
    device_factory, oracle_fs_factory, unused_tcp_addr, fs_factory, hypothesis_settings
):
    class FSOfflineRestartAndTree(TrioRuleBasedStateMachine):
        Files = Bundle("file")
        Folders = Bundle("folder")

        async def restart_fs(self):
            try:
                await self.fs_controller.stop()
            except AttributeError:
                pass

            async def _fs_controlled_cb(started_cb):
                async with fs_factory(device=self.device, backend_addr=unused_tcp_addr) as fs:
                    await started_cb(fs=fs)

            self.fs_controller = await self.get_root_nursery().start(
                call_with_control, _fs_controlled_cb
            )

        @property
        def fs(self):
            return self.fs_controller.fs

        @initialize(target=Folders)
        async def init(self):
            self.device = device_factory()
            await self.restart_fs()
            await self.fs.workspace_create("/w")

            self.oracle_fs = oracle_fs_factory()
            self.oracle_fs.create_workspace("/w")
            return "/w"

        @rule()
        async def restart(self):
            await self.restart_fs()

        @rule(target=Files, parent=Folders, name=st_entry_name)
        async def create_file(self, parent, name):
            path = os.path.join(parent, name)
            expected_status = self.oracle_fs.create_file(path)
            if expected_status == "ok":
                await self.fs.file_create(path=path)
            else:
                with pytest.raises(OSError):
                    await self.fs.file_create(path=path)
            return path

        @rule(target=Folders, parent=Folders, name=st_entry_name)
        async def create_folder(self, parent, name):
            path = os.path.join(parent, name)
            expected_status = self.oracle_fs.create_folder(path)
            if expected_status == "ok":
                await self.fs.folder_create(path=path)
            else:
                with pytest.raises(OSError):
                    await self.fs.folder_create(path=path)
            return path

        @rule(target=Folders, name=st_entry_name)
        async def create_workspace(self, name):
            path = os.path.join("/", name)
            expected_status = self.oracle_fs.create_workspace(path)
            if expected_status == "ok":
                await self.fs.workspace_create(path)
            else:
                with pytest.raises(OSError):
                    await self.fs.workspace_create(path)
            return path

        @rule(path=Files)
        async def delete_file(self, path):
            # TODO: separate delete file from delete folder
            expected_status = self.oracle_fs.delete(path)
            if expected_status == "ok":
                await self.fs.delete(path=path)
            else:
                with pytest.raises(OSError):
                    await self.fs.delete(path=path)
            return path

        @rule(path=Folders)
        async def delete_folder(self, path):
            # TODO: separate delete file from delete folder
            expected_status = self.oracle_fs.delete(path)
            if expected_status == "ok":
                await self.fs.delete(path=path)
            else:
                with pytest.raises(OSError):
                    await self.fs.delete(path=path)
            return path

        @rule(target=Files, src=Files, dst_parent=Folders, dst_name=st_entry_name)
        async def move_file(self, src, dst_parent, dst_name):
            dst = os.path.join(dst_parent, dst_name)
            expected_status = self.oracle_fs.move(src, dst)
            if expected_status == "ok":
                await self.fs.move(src, dst)
            else:
                with pytest.raises(OSError):
                    await self.fs.move(src, dst)
            return dst

        @rule(target=Folders, src=Folders, dst_parent=Folders, dst_name=st_entry_name)
        async def move_folder(self, src, dst_parent, dst_name):
            dst = os.path.join(dst_parent, dst_name)
            expected_status = self.oracle_fs.move(src, dst)
            if expected_status == "ok":
                await self.fs.move(src, dst)
            else:
                with pytest.raises(OSError):
                    await self.fs.move(src, dst)
            return dst

        # TODO: test workspace_rename as well ?

    run_state_machine_as_test(FSOfflineRestartAndTree, settings=hypothesis_settings)
