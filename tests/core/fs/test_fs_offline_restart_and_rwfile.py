# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest
from hypothesis import strategies as st
from hypothesis_trio.stateful import initialize, rule

from parsec.api.data import EntryName
from tests.common import FileOracle


BLOCK_SIZE = 16
PLAYGROUND_SIZE = BLOCK_SIZE * 10


@pytest.mark.slow
def test_fs_offline_restart_and_rwfile(user_fs_offline_state_machine, alice):
    class FSOfflineRestartAndRWFile(user_fs_offline_state_machine):
        @initialize()
        async def init(self):
            await self.reset_all()
            self.device = alice
            await self.restart_user_fs(self.device)
            self.wid = await self.user_fs.workspace_create(EntryName("w"))
            self.workspace = self.user_fs.get_workspace(self.wid)
            await self.workspace.touch("/foo.txt")
            self.file_oracle = FileOracle()

        @rule()
        async def restart(self):
            await self.restart_user_fs(self.device)
            self.workspace = self.user_fs.get_workspace(self.wid)

        @rule(
            size=st.integers(min_value=0, max_value=PLAYGROUND_SIZE),
            offset=st.integers(min_value=0, max_value=PLAYGROUND_SIZE),
        )
        async def atomic_read(self, size, offset):
            async with await self.workspace.open_file("/foo.txt", "rb") as f:
                await f.seek(offset)
                content = await f.read(size)
            expected_content = self.file_oracle.read(size, offset)
            assert content == expected_content

        @rule(
            offset=st.integers(min_value=0, max_value=PLAYGROUND_SIZE),
            content=st.binary(max_size=PLAYGROUND_SIZE),
        )
        async def atomic_write(self, offset, content):
            async with await self.workspace.open_file("/foo.txt", "rb+") as f:
                await f.seek(offset)
                await f.write(content)
            self.file_oracle.write(offset, content)

        @rule(length=st.integers(min_value=0, max_value=PLAYGROUND_SIZE))
        async def atomic_truncate(self, length):
            await self.workspace.truncate("/foo.txt", length=length)
            self.file_oracle.truncate(length)

    FSOfflineRestartAndRWFile.run_as_test()
