import pytest
from hypothesis import strategies as st, note
from hypothesis_trio.stateful import run_state_machine_as_test

from parsec.utils import to_jsonb64, from_jsonb64

from tests.hypothesis.common import rule, initialize, failure_reproducer, reproduce_rule


BLOCK_SIZE = 16
PLAYGROUND_SIZE = BLOCK_SIZE * 10


class FileOracle:
    def __init__(self, base_version=0):
        self._buffer = bytearray()
        self._synced_buffer = bytearray()
        self.base_version = base_version
        self.need_sync = base_version == 0

    def read(self, size, offset):
        return self._buffer[offset : size + offset]

    def write(self, offset, content):
        if not content:
            return

        if offset > len(self._buffer):
            self.truncate(offset + len(content))
        self._buffer[offset : len(content) + offset] = content
        self.need_sync = True

    def truncate(self, length):
        if length == len(self._buffer):
            return
        new_buffer = bytearray(length)
        truncate_length = min(length, len(self._buffer))
        new_buffer[:truncate_length] = self._buffer[:truncate_length]
        self._buffer = new_buffer
        self.need_sync = True

    def sync(self):
        self._synced_buffer = self._buffer.copy()
        if self.need_sync:
            self.base_version += 1
        self.need_sync = False

    def reset_core(self):
        self._buffer = self._synced_buffer.copy()
        self.need_sync = False

    def restart_core(self):
        pass


@pytest.fixture
def CoreOnlineRwAndSync(BaseCoreWithBackendStateMachine):
    @failure_reproducer(
        """
import pytest
import os
from copy import deepcopy

from parsec.utils import to_jsonb64, from_jsonb64

from tests.hypothesis.test_core_online_rw_and_sync import FileOracle, BLOCK_SIZE

@pytest.mark.trio
async def test_reproduce(running_backend, alice, core_factory, core_sock_factory):
    file_oracle = FileOracle(base_version=1)

    async def restart_core(old_core=None, reset_local_db=False):
        if old_core:
            await old_core.teardown()
        if reset_local_db:
            alice.local_db._data.clear()
            file_oracle.reset_core()
        else:
            file_oracle.restart_core()
        core = await core_factory(config={{"block_size": BLOCK_SIZE}})
        await core.login(alice)
        sock = core_sock_factory(core)
        return core, sock

    core, sock = await restart_core()
    await core.fs.file_create("/foo.txt")
    await core.fs.sync("/")

    {body}
"""
    )
    class CoreOnlineRwAndSync(BaseCoreWithBackendStateMachine):
        @initialize(core=BaseCoreWithBackendStateMachine.Cores)
        async def init_oracle_and_fs(self, core):
            await core.fs.file_create("/foo.txt")
            await core.fs.sync("/")
            self.file_oracle = FileOracle(base_version=1)

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
        async def read(self, size, offset):
            rep = await self.core_cmd(
                {"cmd": "file_read", "path": "/foo.txt", "offset": offset, "size": size}
            )
            note(rep)
            assert rep["status"] == "ok"
            expected_content = self.file_oracle.read(size, offset)
            assert from_jsonb64(rep["content"]) == expected_content

        @rule()
        @reproduce_rule(
            """
await sock.send({{"cmd": "synchronize", "path": "/"}})
rep = await sock.recv()
assert rep["status"] == "ok"
file_oracle.sync()
"""
        )
        async def sync(self):
            rep = await self.core_cmd({"cmd": "synchronize", "path": "/"})
            note(rep)
            assert rep["status"] == "ok"
            self.file_oracle.sync()

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
        async def write(self, offset, content):
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
        async def truncate(self, length):
            rep = await self.core_cmd(
                {"cmd": "file_truncate", "path": "/foo.txt", "length": length}
            )
            note(rep)
            assert rep["status"] == "ok"
            self.file_oracle.truncate(length)

        @rule()
        @reproduce_rule(
            """
await sock.send({{"cmd": "stat", "path": "/foo.txt"}})
rep = await sock.recv()
assert rep["status"] == "ok"
assert rep["base_version"] == file_oracle.base_version
assert not rep["is_placeholder"]
assert rep["need_sync"] == file_oracle.need_sync
"""
        )
        async def stat(self):
            rep = await self.core_cmd({"cmd": "stat", "path": "/foo.txt"})
            note(rep)
            assert rep["status"] == "ok"
            assert rep["base_version"] == self.file_oracle.base_version
            assert not rep["is_placeholder"]
            assert rep["need_sync"] == self.file_oracle.need_sync

        @rule()
        @reproduce_rule("""core, sock = await restart_core(core)""")
        async def restart(self):
            await self.restart_core()
            self.file_oracle.restart_core()

        @rule()
        @reproduce_rule("""core, sock = await restart_core(core, reset_local_db=True)""")
        async def reset(self):
            await self.restart_core(reset_local_db=True)
            self.file_oracle.reset_core()

    return CoreOnlineRwAndSync


@pytest.mark.slow
def test_core_online_rw_and_sync(CoreOnlineRwAndSync, hypothesis_settings):
    run_state_machine_as_test(CoreOnlineRwAndSync, settings=hypothesis_settings)
