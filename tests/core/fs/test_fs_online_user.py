# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

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
def test_fs_online_user(
    hypothesis_settings,
    backend_addr,
    backend_factory,
    server_factory,
    local_storage_factory,
    oracle_fs_with_sync_factory,
    fs_factory,
    alice,
):
    class FSOfflineUser(TrioRuleBasedStateMachine):
        Workspaces = Bundle("workspace")

        async def restart_fs(self, device, local_storage):
            try:
                await self.fs_controller.stop()
            except AttributeError:
                pass

            async def _fs_controlled_cb(started_cb):
                async with fs_factory(device=device, local_storage=local_storage) as fs:
                    await started_cb(fs=fs)

            self.fs_controller = await self.get_root_nursery().start(
                call_with_control, _fs_controlled_cb
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
            self.device = alice
            self.local_storage = local_storage_factory(self.device)
            self.oracle_fs = oracle_fs_with_sync_factory()

            await self.start_backend()
            await self.restart_fs(self.device, self.local_storage)

        @property
        def fs(self):
            return self.fs_controller.fs

        @property
        def backend(self):
            return self.backend_controller.backend

        @rule()
        async def restart(self):
            await self.restart_fs(self.device, self.local_storage)

        @rule()
        async def reset(self):
            # TODO: would be cleaner to recreate a new device...
            self.local_storage = local_storage_factory(self.device, force=True)
            await self.restart_fs(self.device, self.local_storage)
            await self.fs.sync("/")
            self.oracle_fs.reset()

        async def stat(self):
            expected = self.oracle_fs.stat("/")
            stat = await self.fs.stat("/")
            assert stat["type"] == expected["type"]
            # TODO: oracle's `base_version` is broken (synchronization
            # strategy with parent placeholder make it complex to get right)
            # assert stat["base_version"] == expected["base_version"]
            if not stat["need_sync"]:
                assert stat["base_version"] > 0
            assert stat["is_placeholder"] == expected["is_placeholder"]
            assert stat["need_sync"] == expected["need_sync"]

        @rule(target=Workspaces, name=st_entry_name)
        async def create_workspace(self, name):
            path = os.path.join("/", name)
            expected_status = self.oracle_fs.create_workspace(path)
            if expected_status == "ok":
                await self.fs.workspace_create(path)
            else:
                with pytest.raises(OSError):
                    await self.fs.workspace_create(path)
            return name

        @rule(target=Workspaces, workspace=Workspaces, new_name=st_entry_name)
        async def rename_workspace(self, workspace, new_name):
            src = f"/{workspace}"
            dst = f"/{new_name}"
            expected_status = self.oracle_fs.rename_workspace(src, dst)
            if expected_status == "ok":
                await self.fs.workspace_rename(src, dst)
            else:
                with pytest.raises(OSError):
                    await self.fs.workspace_rename(src, dst)
            return new_name

        @rule()
        async def sync(self):
            await self.fs.sync("/")
            self.oracle_fs.sync("/")

    run_state_machine_as_test(FSOfflineUser, settings=hypothesis_settings)
