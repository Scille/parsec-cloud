import pytest
from hypothesis import strategies as st, note
from hypothesis.stateful import rule

from parsec.utils import to_jsonb64, from_jsonb64

from tests.common import connect_core, core_factory


class FileOracle:
    def __init__(self):
        self._buffer = bytearray()

    def read(self, size, offset):
        return self._buffer[offset : size + offset]

    def write(self, offset, content):
        self._buffer[offset : len(content) + offset] = content

    def truncate(self, length):
        self._buffer = self._buffer[:length]


@pytest.mark.slow
@pytest.mark.trio
async def test_core_offline_rwfile(
    TrioDriverRuleBasedStateMachine, mocked_local_storage_connection, backend_addr, tmpdir, alice
):
    class CoreOfflineRWFile(TrioDriverRuleBasedStateMachine):
        count = 0

        async def trio_runner(self, task_status):
            mocked_local_storage_connection.reset()
            type(self).count += 1
            config = {
                "base_settings_path": tmpdir.mkdir("try-%s" % self.count).strpath,
                "backend_addr": backend_addr,
            }

            async with core_factory(**config) as core:
                await core.login(alice)
                async with connect_core(core) as sock:

                    await sock.send({"cmd": "file_create", "path": "/foo.txt"})
                    rep = await sock.recv()
                    assert rep == {"status": "ok"}
                    self.file_oracle = FileOracle()

                    self.core_cmd = self.communicator.send
                    task_status.started()

                    while True:
                        msg = await self.communicator.trio_recv()
                        await sock.send(msg)
                        rep = await sock.recv()
                        await self.communicator.trio_respond(rep)

        @rule(size=st.integers(min_value=0), offset=st.integers(min_value=0))
        def read(self, size, offset):
            rep = self.core_cmd(
                {"cmd": "file_read", "path": "/foo.txt", "offset": offset, "size": size}
            )
            note(rep)
            assert rep["status"] == "ok"
            expected_content = self.file_oracle.read(size, offset)
            assert from_jsonb64(rep["content"]) == expected_content

        @rule()
        def flush(self):
            rep = self.core_cmd({"cmd": "flush", "path": "/foo.txt"})
            note(rep)
            assert rep["status"] == "ok"

        @rule(offset=st.integers(min_value=0), content=st.binary())
        def write(self, offset, content):
            b64content = to_jsonb64(content)
            rep = self.core_cmd(
                {"cmd": "file_write", "path": "/foo.txt", "offset": offset, "content": b64content}
            )
            note(rep)
            assert rep["status"] == "ok"
            self.file_oracle.write(offset, content)

        @rule(length=st.integers(min_value=0, max_value=100))
        def truncate(self, length):
            rep = self.core_cmd(
                {"cmd": "file_truncate", "path": "/foo.txt", "length": length}
            )
            note(rep)
            assert rep["status"] == "ok"
            self.file_oracle.truncate(length)

    await CoreOfflineRWFile.run_test()
