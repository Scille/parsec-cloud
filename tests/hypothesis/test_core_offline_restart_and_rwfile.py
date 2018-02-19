import pytest
from hypothesis import strategies as st, note
from hypothesis.stateful import rule

from parsec.utils import to_jsonb64, from_jsonb64

from tests.common import connect_core, core_factory
from tests.hypothesis.conftest import skip_on_broken_stream


class FileOracle:
    def __init__(self):
        self._buffer = bytearray()

    def read(self, size, offset):
        return self._buffer[offset:size + offset]

    def write(self, offset, content):
        self._buffer[offset:len(content) + offset] = content

    def restart(self):
        # TODO: comment me and look where it crash...
        self._buffer = bytearray()


@pytest.mark.slow
@pytest.mark.trio
async def test_core_offline_restart_and_rwfile(
    TrioDriverRuleBasedStateMachine,
    mocked_local_storage_connection,
    backend_addr,
    tmpdir,
    alice
):

    class RestartCore(Exception):
        pass

    class CoreOfflineRestartAndRWFile(TrioDriverRuleBasedStateMachine):
        count = 0

        async def trio_runner(self, task_status):
            mocked_local_storage_connection.reset()
            type(self).count += 1
            config = {
                'base_settings_path': tmpdir.mkdir('try-%s' % self.count).strpath,
                'backend_addr': backend_addr,
            }

            self.sys_cmd = lambda x: self.communicator.send(('sys', x))
            self.file_oracle = FileOracle()

            async def run_core(on_ready):
                async with core_factory(**config) as core:
                    await core.login(alice)
                    async with connect_core(core) as sock:

                        await on_ready(sock)
                        self.core_cmd = lambda x: self.communicator.send(('core', x))

                        while True:
                            target, msg = await self.communicator.trio_recv()
                            if target == 'core':
                                await sock.send(msg)
                                rep = await sock.recv()
                                await self.communicator.trio_respond(rep)
                            elif msg == 'restart!':
                                raise RestartCore()

            async def bootstrap_core(sock):
                await sock.send({'cmd': 'file_create', 'path': '/foo.txt'})
                rep = await sock.recv()
                assert rep == {'status': 'ok'}
                task_status.started()

            async def restart_core_done(sock):
                await self.communicator.trio_respond(42)

            on_ready = bootstrap_core
            while True:
                try:
                    await run_core(on_ready)
                except RestartCore:
                    on_ready = restart_core_done

        @rule()
        @skip_on_broken_stream
        def restart(self):
            self.sys_cmd('restart!')
            self.file_oracle.restart()

        @rule(size=st.integers(min_value=0), offset=st.integers(min_value=0))
        @skip_on_broken_stream
        def read(self, size, offset):
            rep = self.core_cmd({
                'cmd': 'file_read',
                'path': '/foo.txt',
                'offset': offset,
                'size': size,
            })
            note(rep)
            assert rep['status'] == 'ok'
            expected_content = self.file_oracle.read(size, offset)
            assert from_jsonb64(rep['content']) == expected_content

        @rule()
        @skip_on_broken_stream
        def flush(self):
            rep = self.core_cmd({'cmd': 'flush', 'path': '/foo.txt'})
            note(rep)
            assert rep['status'] == 'ok'

        @rule(offset=st.integers(min_value=0), content=st.binary())
        @skip_on_broken_stream
        def write(self, offset, content):
            b64content = to_jsonb64(content)
            rep = self.core_cmd({
                'cmd': 'file_write',
                'path': '/foo.txt',
                'offset': offset,
                'content': b64content,
            })
            note(rep)
            assert rep['status'] == 'ok'
            self.file_oracle.write(offset, content)

    await CoreOfflineRestartAndRWFile.run_test()
