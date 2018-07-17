import pytest
from hypothesis import strategies as st, note
from hypothesis_trio.stateful import run_state_machine_as_test

from parsec.utils import to_jsonb64, from_jsonb64

from tests.hypothesis.common import initialize, rule, failure_reproducer, reproduce_rule, FileOracle


BLOCK_SIZE = 16
PLAYGROUND_SIZE = BLOCK_SIZE * 10


@pytest.mark.slow
def test_core_offline_restart_and_rwfile(BaseCoreAloneStateMachine, hypothesis_settings):
    @failure_reproducer(
        """
import pytest

from parsec.utils import to_jsonb64, from_jsonb64

from tests.hypothesis.test_core_offline_restart_and_rwfile import FileOracle, BLOCK_SIZE

@pytest.mark.trio
async def test_reproduce(alice, core_factory, core_sock_factory):
    file_oracle = FileOracle()

    async def restart_core(old_core=None):
        if old_core:
            await old_core.teardown()
        file_oracle.restart_core()
        core = await core_factory(config={{"block_size": BLOCK_SIZE}})
        await core.login(alice)
        sock = core_sock_factory(core)
        return core, sock

    core, sock = await restart_core()
    await core.fs.file_create("/foo.txt")

    {body}
"""
    )
    class CoreOfflineRestartAndRWFile(BaseCoreAloneStateMachine):
        @initialize(core=BaseCoreAloneStateMachine.Cores)
        async def init_oracle_and_fs(self, core):
            await core.fs.file_create("/foo.txt")
            self.file_oracle = FileOracle()

        @rule()
        @reproduce_rule("""core, sock = await restart_core(core)""")
        async def restart(self):
            await self.restart_core()

        @rule(
            size=st.integers(min_value=0, max_value=PLAYGROUND_SIZE),
            offset=st.integers(min_value=0, max_value=PLAYGROUND_SIZE),
        )
        @reproduce_rule(
            """
await sock.send({{"cmd": "file_read", "path": "/foo.txt", "offset": {offset}, "size": {size}}})
rep = await sock.recv()
assert rep["status"] == "ok"
expected_content = file_oracle.read({size}, {offset})
assert from_jsonb64(rep["content"]) == expected_content
"""
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
        @reproduce_rule(
            """
b64content = to_jsonb64({content})
await sock.send({{"cmd": "file_write", "path": "/foo.txt", "offset": {offset}, "content": b64content}})
rep = await sock.recv()
assert rep["status"] == "ok"
file_oracle.write({offset}, {content})
"""
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
        @reproduce_rule(
            """
await sock.send({{"cmd": "file_truncate", "path": "/foo.txt", "length": {length}}})
rep = await sock.recv()
assert rep["status"] == "ok"
file_oracle.truncate({length})
"""
        )
        async def atomic_truncate(self, length):
            rep = await self.core_cmd(
                {"cmd": "file_truncate", "path": "/foo.txt", "length": length}
            )
            note(rep)
            assert rep["status"] == "ok"
            self.file_oracle.truncate(length)

    run_state_machine_as_test(CoreOfflineRestartAndRWFile, settings=hypothesis_settings)
