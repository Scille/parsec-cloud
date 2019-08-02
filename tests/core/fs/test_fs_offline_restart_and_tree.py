# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os
import pytest
from hypothesis import strategies as st
from hypothesis_trio.stateful import (
    initialize,
    rule,
    run_state_machine_as_test,
    TrioAsyncioRuleBasedStateMachine,
    Bundle,
)
from string import ascii_lowercase

from tests.common import call_with_control


def get_path(path):
    return path[2:] if path[2:] else "/"


# The point is not to find breaking filenames here, so keep it simple
st_entry_name = st.text(alphabet=ascii_lowercase, min_size=1, max_size=3)


@pytest.mark.slow
@pytest.mark.skipif(os.name == "nt", reason="Windows path style not compatible with oracle")
def test_fs_offline_restart_and_tree(
    hypothesis_settings, reset_testbed, oracle_fs_factory, user_fs_factory, alice
):
    class FSOfflineRestartAndTree(TrioAsyncioRuleBasedStateMachine):
        Files = Bundle("file")
        Folders = Bundle("folder")

        async def restart_user_fs(self):
            try:
                await self.fs_controller.stop()
            except AttributeError:
                pass

            async def _user_fs_controlled_cb(started_cb):
                async with user_fs_factory(device=self.device) as user_fs:
                    await started_cb(user_fs=user_fs)

            self.user_fs_controller = await self.get_root_nursery().start(
                call_with_control, _user_fs_controlled_cb
            )

        @property
        def user_fs(self):
            return self.user_fs_controller.user_fs

        @initialize(target=Folders)
        async def init(self):
            await reset_testbed()
            self.device = alice
            await self.restart_user_fs()
            wid = await self.user_fs.workspace_create("w")
            self.workspace = self.user_fs.get_workspace(wid)

            self.oracle_fs = oracle_fs_factory()
            self.oracle_fs.create_workspace("/w")
            return "/w"

        @rule()
        async def restart(self):
            await self.restart_user_fs()

        @rule(target=Files, parent=Folders, name=st_entry_name)
        async def create_file(self, parent, name):
            path = os.path.join(parent, name)
            expected_status = self.oracle_fs.create_file(path)
            if expected_status == "ok":
                await self.workspace.touch(path=get_path(path), exist_ok=False)
            else:
                with pytest.raises((FileExistsError, FileNotFoundError, NotADirectoryError)):
                    await self.workspace.touch(path=get_path(path), exist_ok=False)
            return path

        @rule(target=Folders, parent=Folders, name=st_entry_name)
        async def create_folder(self, parent, name):
            path = os.path.join(parent, name)
            expected_status = self.oracle_fs.create_folder(path)
            if expected_status == "ok":
                await self.workspace.mkdir(path=get_path(path), exist_ok=False)
            else:
                with pytest.raises((FileExistsError, FileNotFoundError, NotADirectoryError)):
                    await self.workspace.mkdir(path=get_path(path), exist_ok=False)
            return path

        @rule(path=Files)
        async def delete_file(self, path):
            expected_status = self.oracle_fs.unlink(path)
            if expected_status == "ok":
                await self.workspace.unlink(path=get_path(path))
            else:
                with pytest.raises(OSError):
                    await self.workspace.unlink(path=get_path(path))
            return path

        @rule(path=Folders)
        async def delete_folder(self, path):
            expected_status = self.oracle_fs.rmdir(path)
            if expected_status == "ok":
                await self.workspace.rmdir(path=get_path(path))
            else:
                with pytest.raises(OSError):
                    await self.workspace.rmdir(path=get_path(path))
            return path

        @rule(target=Files, src=Files, dst_parent=Folders, dst_name=st_entry_name)
        async def move_file(self, src, dst_parent, dst_name):
            dst = os.path.join(dst_parent, dst_name)
            expected_status = self.oracle_fs.move(src, dst)
            if expected_status == "ok":
                await self.workspace.rename(get_path(src), get_path(dst))
            else:
                with pytest.raises(OSError):
                    await self.workspace.rename(get_path(src), get_path(dst))
            return dst

        @rule(target=Folders, src=Folders, dst_parent=Folders, dst_name=st_entry_name)
        async def move_folder(self, src, dst_parent, dst_name):
            dst = os.path.join(dst_parent, dst_name)
            expected_status = self.oracle_fs.move(src, dst)
            if expected_status == "ok":
                await self.workspace.rename(get_path(src), get_path(dst))
            else:
                with pytest.raises(OSError):
                    await self.workspace.rename(get_path(src), get_path(dst))
            return dst

    run_state_machine_as_test(FSOfflineRestartAndTree, settings=hypothesis_settings)
