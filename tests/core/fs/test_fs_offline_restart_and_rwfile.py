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
    device_factory, unused_tcp_addr, fs_factory, hypothesis_settings
):
    class FSOfflineRestartAndRWFile(TrioRuleBasedStateMachine):
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
            if self.fs_controller:
                return self.fs_controller.fs
            else:
                return None

        @initialize()
        async def init(self):
            self.device = device_factory()
            await self.restart_fs()
            await self.fs.file_create("/foo.txt")
            self.file_oracle = FileOracle()

        @rule()
        async def restart(self):
            await self.restart_fs()

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

    run_state_machine_as_test(FSOfflineRestartAndRWFile, settings=hypothesis_settings)
