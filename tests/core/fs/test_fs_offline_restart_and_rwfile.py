# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from hypothesis import strategies as st
from hypothesis_trio.stateful import (
    initialize,
    rule,
    run_state_machine_as_test,
    TrioRuleBasedStateMachine,
)

from tests.common import call_with_control
from tests.oracles import FileOracle


BLOCK_SIZE = 16
PLAYGROUND_SIZE = BLOCK_SIZE * 10


@pytest.mark.slow
def test_fs_offline_restart_and_rwfile(
    hypothesis_settings, local_storage_factory, user_fs_factory, alice
):
    class FSOfflineRestartAndRWFile(TrioRuleBasedStateMachine):
        async def restart_user_fs(self):
            try:
                await self.fs_controller.stop()
            except AttributeError:
                pass

            async def _user_fs_controlled_cb(started_cb):
                async with user_fs_factory(
                    device=self.device, local_storage=self.local_storage
                ) as user_fs:
                    await started_cb(user_fs=user_fs)

            self.user_fs_controller = await self.get_root_nursery().start(
                call_with_control, _user_fs_controlled_cb
            )

        @property
        def user_fs(self):
            if self.user_fs_controller:
                return self.user_fs_controller.user_fs
            else:
                return None

        @initialize()
        async def init(self):
            self.device = alice
            self.local_storage = local_storage_factory(alice)
            await self.restart_user_fs()
            wid = await self.user_fs.workspace_create("w")
            self.workspace = self.user_fs.get_workspace(wid)
            await self.workspace.touch("/foo.txt")
            self.file_oracle = FileOracle()

        @rule()
        async def restart(self):
            await self.restart_user_fs()

        @rule(
            size=st.integers(min_value=0, max_value=PLAYGROUND_SIZE),
            offset=st.integers(min_value=0, max_value=PLAYGROUND_SIZE),
        )
        async def atomic_read(self, size, offset):
            content = await self.workspace.read_bytes("/foo.txt", size=size, offset=offset)
            expected_content = self.file_oracle.read(size, offset)
            assert content == expected_content

        @rule(
            offset=st.integers(min_value=0, max_value=PLAYGROUND_SIZE),
            content=st.binary(max_size=PLAYGROUND_SIZE),
        )
        async def atomic_write(self, offset, content):
            await self.workspace.write_bytes("/foo.txt", data=content, offset=offset)
            self.file_oracle.write(offset, content)

        @rule(length=st.integers(min_value=0, max_value=PLAYGROUND_SIZE))
        async def atomic_truncate(self, length):
            await self.workspace.truncate("/foo.txt", length=length)
            self.file_oracle.truncate(length)

    run_state_machine_as_test(FSOfflineRestartAndRWFile, settings=hypothesis_settings)
