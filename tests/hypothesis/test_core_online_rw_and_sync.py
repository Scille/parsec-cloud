import pytest
from hypothesis import strategies as st, note
from hypothesis.stateful import rule

from parsec.utils import to_jsonb64, from_jsonb64

from tests.common import (
    connect_core, core_factory, backend_factory, run_app
)


class FileOracle:
    def __init__(self):
        self._buffer = bytearray()

    def read(self, size, offset):
        return self._buffer[offset:size + offset]

    def write(self, offset, content):
        self._buffer[offset:len(content) + offset] = content


@pytest.mark.slow
@pytest.mark.trio
async def test_online(
    TrioDriverRuleBasedStateMachine,
    mocked_local_storage_connection,
    tcp_stream_spy,
    backend_addr,
    tmpdir,
    alice
):

    class RestartCore(Exception):
        def __init__(self, reset_local_storage=False):
            self.reset_local_storage = reset_local_storage

    class CoreOnline(TrioDriverRuleBasedStateMachine):
        count = 0

        async def trio_runner(self, task_status):
            mocked_local_storage_connection.reset()
            type(self).count += 1
            backend_config = {
                'blockstore_url': 'backend://',
            }
            core_config = {
                'base_settings_path': tmpdir.mkdir('try-%s' % self.count).strpath,
                'backend_addr': backend_addr,
            }
            self.sys_cmd = lambda x: self.communicator.send(('sys', x))
            self.core_cmd = lambda x: self.communicator.send(('core', x))
            self.file_oracle = FileOracle()

            async def run_core(on_ready):
                async with core_factory(**core_config) as core:

                    await core.login(alice)
                    async with connect_core(core) as sock:

                        await on_ready(core)

                        while True:
                            target, msg = await self.communicator.trio_recv()
                            if target == 'core':
                                await sock.send(msg)
                                rep = await sock.recv()
                                await self.communicator.trio_respond(rep)
                            elif msg == 'restart_core!':
                                raise RestartCore()
                            elif msg == 'reset_core!':
                                raise RestartCore(reset_local_storage=True)

            async def bootstrap_core(core):
                await core.fs.root.create_file('foo.txt')
                await core.fs.root.sync(recursive=True)

                task_status.started()

            async def restart_core_done(core):
                await self.communicator.trio_respond(True)

            async with backend_factory(**backend_config) as backend:

                await backend.user.create(
                    author='<backend-fixture>',
                    user_id=alice.user_id,
                    broadcast_key=alice.user_pubkey.encode(),
                    devices=[(alice.device_name, alice.device_verifykey.encode())]
                )

                async with run_app(backend) as backend_connection_factory:

                    tcp_stream_spy.install_hook(backend_addr, backend_connection_factory)
                    try:

                        on_ready = bootstrap_core
                        while True:
                            try:
                                await run_core(on_ready)
                            except RestartCore as exc:
                                on_ready = restart_core_done
                                if exc.reset_local_storage:
                                    mocked_local_storage_connection.reset()

                    finally:
                        tcp_stream_spy.install_hook(backend_addr, None)

        @rule(size=st.integers(min_value=0, max_value=100),
              offset=st.integers(min_value=0, max_value=100))
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
        def flush(self):
            rep = self.core_cmd({'cmd': 'flush', 'path': '/foo.txt'})
            note(rep)
            assert rep['status'] == 'ok'

        @rule()
        def sync(self):
            rep = self.core_cmd({'cmd': 'synchronize', 'path': '/foo.txt'})
            note(rep)
            assert rep['status'] == 'ok'

        @rule(offset=st.integers(min_value=0, max_value=100), content=st.binary())
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

        @rule()
        def restart_core(self):
            rep = self.sys_cmd('restart_core!')
            assert rep is True

        @rule()
        def reset_core(self):
            rep = self.sys_cmd('reset_core!')
            assert rep is True

    await CoreOnline.run_test()
