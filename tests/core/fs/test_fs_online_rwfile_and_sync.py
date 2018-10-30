import pytest
from parsec.core.local_db import LocalDB
from shutil import rmtree
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
def test_fs_online_rwfile_and_sync(
    hypothesis_settings,
    unused_tcp_addr,
    device_factory,
    backend_factory,
    server_factory,
    fs_factory,
    backend_addr,
    tmpdir,
):
    class FSOnlineRwFileAndSync(TrioRuleBasedStateMachine):
        async def restart_fs(self, device):
            try:
                await self.fs_controller.stop()
            except AttributeError:
                pass

            async def _fs_controlled_cb(started_cb):
                async with fs_factory(device=device, backend_addr=backend_addr) as fs:
                    await started_cb(fs=fs)

            self.fs_controller = await self.get_root_nursery().start(
                call_with_control, _fs_controlled_cb
            )

        async def start_backend(self):
            async def _backend_controlled_cb(started_cb):
                async with backend_factory(devices=[self.device]) as backend:
                    async with server_factory(backend.handle_client, backend_addr) as server:
                        await started_cb(backend=backend, server=server)

            self.backend_controller = await self.get_root_nursery().start(
                call_with_control, _backend_controlled_cb
            )

        @property
        def fs(self):
            return self.fs_controller.fs

        @property
        def backend(self):
            return self.backend_controller.backend

        @initialize()
        async def init(self):
            self.device = device_factory(local_db=LocalDB(tmpdir))
            await self.start_backend()
            await self.restart_fs(self.device)
            await self.fs.file_create("/foo.txt")
            await self.fs.sync("/")
            self.file_oracle = FileOracle(base_version=1)

        @rule()
        async def restart(self):
            await self.restart_fs(self.device)

        @rule()
        async def reset(self):
            rmtree(tmpdir)
            self.device = device_factory(user_id=self.device.user_id, local_db=LocalDB(tmpdir))
            await self.backend.user.create_device(
                user_id=self.device.user_id,
                device_name=self.device.device_name,
                verify_key=self.device.device_verifykey.encode(),
            )
            await self.restart_fs(self.device)
            await self.fs.sync("/")
            self.file_oracle.reset()

        @rule()
        async def run_garbage_collector(self):
            self.device.local_db.run_garbage_collector()

        @rule()
        async def sync(self):
            await self.fs.sync("/")
            self.file_oracle.sync()

        @rule(
            size=st.integers(min_value=0, max_value=PLAYGROUND_SIZE),
            offset=st.integers(min_value=0, max_value=PLAYGROUND_SIZE),
        )
        async def atomic_read(self, size, offset):
            content = await self.fs.file_read("/foo.txt", size=size, offset=offset)
            expected_content = self.file_oracle.read(size, offset)
            assert content == expected_content

        @rule(
            offset=st.integers(min_value=0, max_value=PLAYGROUND_SIZE),
            content=st.binary(max_size=PLAYGROUND_SIZE),
        )
        async def atomic_write(self, offset, content):
            await self.fs.file_write("/foo.txt", content=content, offset=offset)
            self.file_oracle.write(offset, content)

        @rule(length=st.integers(min_value=0, max_value=PLAYGROUND_SIZE))
        async def atomic_truncate(self, length):
            await self.fs.file_truncate("/foo.txt", length=length)
            self.file_oracle.truncate(length)

        @rule()
        async def stat(self):
            stat = await self.fs.stat("/foo.txt")
            assert stat["type"] == "file"
            assert stat["base_version"] == self.file_oracle.base_version
            assert not stat["is_placeholder"]
            assert stat["need_sync"] == self.file_oracle.need_sync
            assert stat["size"] == self.file_oracle.size

    run_state_machine_as_test(FSOnlineRwFileAndSync, settings=hypothesis_settings)
