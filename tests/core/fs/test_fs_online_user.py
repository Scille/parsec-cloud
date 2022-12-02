# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import sys
from string import ascii_lowercase

import pytest
from hypothesis import strategies as st
from hypothesis_trio.stateful import Bundle, initialize, rule

from parsec.api.data import EntryID, EntryName
from parsec.core.fs import FSWorkspaceNotFoundError
from tests.common import customize_fixtures

# The point is not to find breaking filenames here, so keep it simple
st_entry_name = st.text(alphabet=ascii_lowercase, min_size=1, max_size=3)


@pytest.mark.slow
@pytest.mark.skipif(sys.platform == "win32", reason="Windows path style not compatible with oracle")
@customize_fixtures(real_data_storage=True, alternate_workspace_storage=True)
def test_fs_online_user(user_fs_online_state_machine, oracle_fs_with_sync_factory, alice):
    class FSOfflineUser(user_fs_online_state_machine):
        Workspaces = Bundle("workspace")

        @initialize()
        async def init(self):
            await self.reset_all()
            await self.start_backend()

            self.device = self.backend_controller.server.correct_addr(alice)
            self.oracle_fs = oracle_fs_with_sync_factory()
            self.workspace = None

            await self.restart_user_fs(self.device)

        @rule()
        async def restart(self):
            await self.restart_user_fs(self.device)

        @rule()
        async def reset(self):
            await self.reset_user_fs(self.device)
            await self.user_fs.sync()
            self.oracle_fs.reset()

        async def stat(self):
            expected = self.oracle_fs.stat("/")
            path_info = await self.workspace.path_info("/")
            assert path_info["type"] == expected["type"]
            # TODO: oracle's `base_version` is broken (synchronization
            # strategy with parent placeholder make it complex to get right)
            # assert path_info["base_version"] == expected["base_version"]
            if not path_info["need_sync"]:
                assert path_info["base_version"] > 0
            assert path_info["is_placeholder"] == expected["is_placeholder"]
            assert path_info["need_sync"] == expected["need_sync"]

        @rule(target=Workspaces, name=st_entry_name)
        async def create_workspace(self, name):
            self.oracle_fs.create_workspace(f"/{name}")
            wid = await self.user_fs.workspace_create(EntryName(name))
            self.workspace = self.user_fs.get_workspace(wid)
            await self.user_fs.sync()
            return wid, name

        @rule(target=Workspaces, workspace=Workspaces, new_name=st_entry_name)
        async def rename_workspace(self, workspace, new_name):
            wid, workspace = workspace
            src = f"/{workspace}"
            dst = f"/{new_name}"
            expected_status = self.oracle_fs.rename_workspace(src, dst)
            if expected_status == "ok":
                await self.user_fs.workspace_rename(wid, EntryName(new_name))
            else:
                with pytest.raises(FSWorkspaceNotFoundError):
                    await self.user_fs.workspace_rename(EntryID.new(), EntryName(new_name))
            return wid, new_name

        @rule(workspace=Workspaces)
        async def sync(self, workspace):
            await self.user_fs.sync()
            self.oracle_fs.sync()

    FSOfflineUser.run_as_test()
