import pytest
from hypothesis import strategies as st, note
from hypothesis_trio.stateful import run_state_machine_as_test

from parsec.utils import to_jsonb64, from_jsonb64

from tests.hypothesis.common import initialize, rule, FileOracle


BLOCK_SIZE = 16
PLAYGROUND_SIZE = BLOCK_SIZE * 10


@pytest.mark.slow
def test_core_offline_rwfile(BaseCoreAloneStateMachine, hypothesis_settings):
    class CoreOfflineRWFile(BaseCoreAloneStateMachine):
        @initialize(core=BaseCoreAloneStateMachine.Cores)
        async def init_oracle_and_fs(self, core):
            await core.fs.file_create("/foo.txt")
            self.file_oracle = FileOracle()

        @rule(
            size=st.integers(min_value=0, max_value=PLAYGROUND_SIZE),
            offset=st.integers(min_value=0, max_value=PLAYGROUND_SIZE),
        )
        async def atomic_read(self, size, offset):
            rep = await self.core_cmd(
                {"cmd": "file_read", "path": "/foo.txt", "offset": offset, "size": size}
            )
            note(rep)
            assert rep["status"] == "ok"
            expected_content = self.file_oracle.read(size, offset)
            assert from_jsonb64(rep["content"]) == expected_content

        @rule(
            offset=st.integers(min_value=0, max_value=PLAYGROUND_SIZE),
            content=st.binary(max_size=PLAYGROUND_SIZE),
        )
        async def atomic_write(self, offset, content):
            b64content = to_jsonb64(content)
            rep = await self.core_cmd(
                {"cmd": "file_write", "path": "/foo.txt", "offset": offset, "content": b64content}
            )
            note(rep)
            assert rep["status"] == "ok"
            self.file_oracle.write(offset, content)

        @rule(length=st.integers(min_value=0, max_value=PLAYGROUND_SIZE))
        async def atomic_truncate(self, length):
            rep = await self.core_cmd(
                {"cmd": "file_truncate", "path": "/foo.txt", "length": length}
            )
            note(rep)
            assert rep["status"] == "ok"
            self.file_oracle.truncate(length)

    run_state_machine_as_test(CoreOfflineRWFile, settings=hypothesis_settings)
