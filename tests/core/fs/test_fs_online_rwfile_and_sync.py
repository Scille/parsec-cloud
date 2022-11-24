# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest

import structlog
from hypothesis import strategies as st, settings
from hypothesis_trio.stateful import initialize, rule
from parsec._parsec import LocalDevice

from parsec.api.data import EntryName
from tests.common import FileOracle

BLOCK_SIZE = 16
PLAYGROUND_SIZE = BLOCK_SIZE * 10


@pytest.fixture
def fs_online_rw_file_and_sync(user_fs_online_state_machine, alice: LocalDevice):
    @settings(max_examples=50)
    class FSOnlineRwFileAndSync(user_fs_online_state_machine):
        @initialize()
        async def init(self):
            await self.reset_all()
            await self.start_backend()
            self.device = self.correct_addr(alice)
            await self.restart_user_fs(self.device)
            self.wid = await self.user_fs.workspace_create(EntryName("w"))
            workspace = self.user_fs.get_workspace(self.wid)
            await workspace.touch("/foo.txt")
            await workspace.sync()
            await self.user_fs.sync()
            self.file_oracle = FileOracle(base_version=1)
            self.log: structlog.BoundLogger = structlog.get_logger(FSOnlineRwFileAndSync.__name__)

        @rule()
        async def restart(self):
            await self.restart_user_fs(self.device)

        @rule()
        async def reset(self):
            self.log.debug(
                "reset", version=self.file_oracle.base_version, need_sync=self.file_oracle.need_sync
            )
            await self.reset_user_fs(self.device)
            await self.user_fs.sync()
            # Retrieve workspace manifest v1 to replace the default empty speculative placeholder
            await self.user_fs.get_workspace(self.wid).sync()
            self.file_oracle.reset()

        @rule()
        async def sync(self):
            self.log.debug(
                "before syn",
                version=self.file_oracle.base_version,
                need_sync=self.file_oracle.need_sync,
            )
            workspace = self.user_fs.get_workspace(self.wid)
            await workspace.sync()
            self.file_oracle.sync()
            self.log.debug(
                "after sync",
                version=self.file_oracle.base_version,
                need_sync=self.file_oracle.need_sync,
            )

        @rule(
            size=st.integers(min_value=0, max_value=PLAYGROUND_SIZE),
            offset=st.integers(min_value=0, max_value=PLAYGROUND_SIZE),
        )
        async def atomic_read(self, size, offset):
            workspace = self.user_fs.get_workspace(self.wid)
            async with await workspace.open_file("/foo.txt", "rb") as f:
                await f.seek(offset)
                content = await f.read(size)
            expected_content = self.file_oracle.read(size, offset)
            assert content == expected_content

        @rule(
            offset=st.integers(min_value=0, max_value=PLAYGROUND_SIZE),
            content=st.binary(max_size=PLAYGROUND_SIZE),
        )
        async def atomic_write(self, offset, content):
            workspace = self.user_fs.get_workspace(self.wid)
            async with await workspace.open_file("/foo.txt", "rb+") as f:
                await f.seek(offset)
                await f.write(content)
            self.log.debug(
                "write", version=self.file_oracle.base_version, need_sync=self.file_oracle.need_sync
            )
            self.file_oracle.write(offset, content)

        @rule(length=st.integers(min_value=0, max_value=PLAYGROUND_SIZE))
        async def atomic_truncate(self, length):
            workspace = self.user_fs.get_workspace(self.wid)
            await workspace.truncate("/foo.txt", length=length)
            self.log.debug(
                "truncate",
                version=self.file_oracle.base_version,
                need_sync=self.file_oracle.need_sync,
            )
            self.file_oracle.truncate(length)

        @rule()
        async def stat(self):
            workspace = self.user_fs.get_workspace(self.wid)
            path_info = await workspace.path_info("/foo.txt")
            self.log.debug(
                "stat",
                version=self.file_oracle.base_version,
                need_sync=self.file_oracle.need_sync,
                info=path_info,
            )
            assert path_info["type"] == "file"
            assert path_info["base_version"] == self.file_oracle.base_version
            assert not path_info["is_placeholder"]
            assert path_info["need_sync"] == self.file_oracle.need_sync
            assert path_info["size"] == self.file_oracle.size

    return FSOnlineRwFileAndSync


@pytest.mark.slow
def test_fs_online_rwfile_and_sync(fs_online_rw_file_and_sync):
    fs_online_rw_file_and_sync.run_as_test()


@pytest.mark.slow
def test_fixture_working(fs_online_rw_file_and_sync):
    state = fs_online_rw_file_and_sync()

    async def steps():
        for _ in range(5):
            await state.init()
            await state.atomic_write(content=b"\x00", offset=0)
            await state.reset()
            await state.stat()

    state.trio_run(steps)
