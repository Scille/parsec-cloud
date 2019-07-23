# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from hypothesis import strategies as st
from hypothesis_trio.stateful import (
    initialize,
    rule,
    run_state_machine_as_test,
    TrioAsyncioRuleBasedStateMachine,
)

from tests.common import call_with_control
from tests.oracles import FileOracle


BLOCK_SIZE = 16
PLAYGROUND_SIZE = BLOCK_SIZE * 10


@pytest.mark.slow
def test_fs_online_rwfile_and_sync(
    hypothesis_settings,
    reset_testbed,
    backend_addr,
    backend_factory,
    server_factory,
    local_storage_factory,
    user_fs_factory,
    alice,
):
    class FSOnlineRwFileAndSync(TrioAsyncioRuleBasedStateMachine):
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

        @property
        def user_fs(self):
            return self.user_fs_controller.user_fs

        @property
        def backend(self):
            return self.backend_controller.backend

        @initialize()
        async def init(self):
            await reset_testbed()
            self.device = alice
            self.local_storage = await local_storage_factory(self.device)
            await self.start_backend()
            await self.restart_user_fs(self.device, self.local_storage)
            self.wid = await self.user_fs.workspace_create("w")
            workspace = await self.user_fs.get_workspace(self.wid)
            await workspace.touch("/foo.txt")
            await workspace.sync("/")
            await self.user_fs.sync()
            self.file_oracle = FileOracle(base_version=1)

        @rule()
        async def restart(self):
            await self.restart_user_fs(self.device, self.local_storage)

        @rule()
        async def reset(self):
            # TODO: would be cleaner to recreate a new device...
            self.local_storage = await local_storage_factory(self.device, force=True)
            await self.restart_user_fs(self.device, self.local_storage)
            await self.user_fs.sync()
            self.file_oracle.reset()

        @rule()
        async def sync(self):
            workspace = await self.user_fs.get_workspace(self.wid)
            await workspace.sync("/")
            self.file_oracle.sync()

        @rule(
            size=st.integers(min_value=0, max_value=PLAYGROUND_SIZE),
            offset=st.integers(min_value=0, max_value=PLAYGROUND_SIZE),
        )
        async def atomic_read(self, size, offset):
            workspace = await self.user_fs.get_workspace(self.wid)
            content = await workspace.read_bytes("/foo.txt", size=size, offset=offset)
            expected_content = self.file_oracle.read(size, offset)
            assert content == expected_content

        @rule(
            offset=st.integers(min_value=0, max_value=PLAYGROUND_SIZE),
            content=st.binary(max_size=PLAYGROUND_SIZE),
        )
        async def atomic_write(self, offset, content):
            workspace = await self.user_fs.get_workspace(self.wid)
            await workspace.write_bytes("/foo.txt", data=content, offset=offset)
            self.file_oracle.write(offset, content)

        @rule(length=st.integers(min_value=0, max_value=PLAYGROUND_SIZE))
        async def atomic_truncate(self, length):
            workspace = await self.user_fs.get_workspace(self.wid)
            await workspace.truncate("/foo.txt", length=length)
            self.file_oracle.truncate(length)

        @rule()
        async def stat(self):
            workspace = await self.user_fs.get_workspace(self.wid)
            path_info = await workspace.path_info("/foo.txt")
            assert path_info["type"] == "file"
            assert path_info["base_version"] == self.file_oracle.base_version
            assert not path_info["is_placeholder"]
            assert path_info["need_sync"] == self.file_oracle.need_sync
            assert path_info["size"] == self.file_oracle.size

    run_state_machine_as_test(FSOnlineRwFileAndSync, settings=hypothesis_settings)
