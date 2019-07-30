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

from parsec.core.fs import FSWorkspaceNotFoundError
from tests.common import call_with_control


# The point is not to find breaking filenames here, so keep it simple
st_entry_name = st.text(alphabet=ascii_lowercase, min_size=1, max_size=3)


@pytest.mark.slow
@pytest.mark.skipif(os.name == "nt", reason="Windows path style not compatible with oracle")
def test_fs_online_user(
    hypothesis_settings,
    reset_testbed,
    backend_addr,
    backend_factory,
    server_factory,
    local_storage_factory,
    oracle_fs_with_sync_factory,
    user_fs_factory,
    alice,
):
    class FSOfflineUser(TrioAsyncioRuleBasedStateMachine):
        Workspaces = Bundle("workspace")

        async def restart_user_fs(self, device, local_storage):
            try:
                await self.user_fs_controller.stop()
            except AttributeError:
                pass

            async def _user_fs_controlled_cb(started_cb):
                async with user_fs_factory(device=device, local_storage=local_storage) as user_fs:
                    await started_cb(user_fs=user_fs)

            self.user_fs_controller = await self.get_root_nursery().start(
                call_with_control, _user_fs_controlled_cb
            )

        async def start_backend(self):
            async def _backend_controlled_cb(started_cb):
                async with backend_factory() as backend:
                    async with server_factory(backend.handle_client, backend_addr) as server:
                        await started_cb(backend=backend, server=server)

            self.backend_controller = await self.get_root_nursery().start(
                call_with_control, _backend_controlled_cb
            )

        @initialize()
        async def init(self):
            await reset_testbed()
            self.device = alice
            self.local_storage = await local_storage_factory(self.device)
            self.oracle_fs = oracle_fs_with_sync_factory()
            self.workspace = None

            await self.start_backend()
            await self.restart_user_fs(self.device, self.local_storage)

        @property
        def user_fs(self):
            return self.user_fs_controller.user_fs

        @property
        def backend(self):
            return self.backend_controller.backend

        @rule()
        async def restart(self):
            await self.restart_user_fs(self.device, self.local_storage)

        @rule()
        async def reset(self):
            # TODO: would be cleaner to recreate a new device...
            self.local_storage = await local_storage_factory(self.device, force=True)
            await self.restart_user_fs(self.device, self.local_storage)
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
            path = os.path.join("/", name)
            self.oracle_fs.create_workspace(path)
            wid = await self.user_fs.workspace_create(name)
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
                await self.user_fs.workspace_rename(wid, new_name)
            else:
                with pytest.raises(FSWorkspaceNotFoundError):
                    await self.user_fs.workspace_rename(workspace, new_name)
            return wid, new_name

        @rule(workspace=Workspaces)
        async def sync(self, workspace):
            await self.user_fs.sync()
            self.oracle_fs.sync("/")

    run_state_machine_as_test(FSOfflineUser, settings=hypothesis_settings)
