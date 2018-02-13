import pytest
from hypothesis import strategies as st, note
from hypothesis.stateful import rule
from functools import partial

from parsec.utils import to_jsonb64, from_jsonb64

from tests.common import connect_core, core_factory, QuitTestDueToBrokenStream
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
async def test_offline_core_rwfile(
    TrioDriverRuleBasedStateMachine,
    mocked_local_storage_connection,
    backend_addr,
    tmpdir,
    alice
):

    class RestartCore(Exception):
        pass

    class CoreOfflineRWFile(TrioDriverRuleBasedStateMachine):
        count = 0

        async def trio_runner(self, task_status):
            print('+++++++ INIT ++++++++++')
            mocked_local_storage_connection.reset()
            type(self).count += 1
            config = {
                'base_settings_path': tmpdir.mkdir('try-%s' % self.count).strpath,
                'backend_addr': backend_addr,
            }

            with self.open_communicator() as communicator:
                self.sys_cmd = partial(communicator.send, 'sys')
                self.file_oracle = FileOracle()

                async def run_core(on_ready):
                    async with core_factory(**config) as core:
                        await core.login(alice)
                        async with connect_core(core) as sock:

                            await on_ready(sock)
                            self.core_cmd = partial(communicator.send, 'core')

                            while True:
                                print('WAITING, queue:', communicator.queue.queue)
                                target, msg = await communicator.trio_recv()
                                print('PROCESSING, queue:', communicator.queue.queue)
                                if target == 'core':
                                    await sock.send(msg)
                                    rep = await sock.recv()
                                    await communicator.trio_respond(rep)
                                elif msg == 'restart!':
                                    raise RestartCore()

                async def bootstrap_core(sock):
                    await sock.send({'cmd': 'file_create', 'path': '/foo.txt'})
                    rep = await sock.recv()
                    assert rep == {'status': 'ok'}
                    task_status.started()

                async def restart_core_done(sock):
                    await communicator.trio_respond(42)

                on_ready = bootstrap_core
                while True:
                    try:
                        await run_core(on_ready)
                    except RestartCore:
                        print('restarting...')
                        on_ready = restart_core_done

        @rule()
        @skip_on_broken_stream
        def restart(self):
            print('//// RESTART')
            print('before restart')
            self.sys_cmd('restart!')
            self.file_oracle.restart()
            print('after restart')

        @rule(size=st.integers(min_value=0), offset=st.integers(min_value=0))
        @skip_on_broken_stream
        def read(self, size, offset):
            print('//// READ')
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
            print('//// FLUSH')
            rep = self.core_cmd({'cmd': 'flush', 'path': '/foo.txt'})
            note(rep)
            assert rep['status'] == 'ok'

        @rule(offset=st.integers(min_value=0), content=st.binary())
        @skip_on_broken_stream
        def write(self, offset, content):
            print('//// WRITE')
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

    await CoreOfflineRWFile.run_test()
