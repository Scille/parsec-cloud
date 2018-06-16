import pytest
from copy import deepcopy
from hypothesis import strategies as st, note

from parsec.utils import to_jsonb64, from_jsonb64

from tests.hypothesis.common import rule, failure_reproducer, reproduce_rule


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


@pytest.mark.slow
@pytest.mark.trio
async def test_core_online_rw_and_sync(
    TrioDriverRuleBasedStateMachine,
    server_factory,
    backend_factory,
    core_factory_cm,
    core_sock_factory,
    device_factory,
):
    class RestartCore(Exception):
        def __init__(self, reset_local_storage=False):
            self.reset_local_storage = reset_local_storage

    @failure_reproducer(
        """
import pytest
import os
from copy import deepcopy

from parsec.utils import to_jsonb64, from_jsonb64

from tests.hypothesis.test_core_online_rw_and_sync import FileOracle, BLOCK_SIZE

class RestartCore(Exception):
    pass

class ResetCore(Exception):
    pass

@pytest.mark.trio
async def test_reproduce(running_backend, alice, core_factory_cm, core_sock_factory):
    bootstrapped = False
    file_oracle = FileOracle(base_version=1)
    to_run_rules = rule_selector()
    done = False
    need_boostrap_sync = True

    while not done:
        try:
            async with core_factory_cm(devices=[alice], config={{"block_size": BLOCK_SIZE}}) as core:
                await core.login(alice)
                if not bootstrapped:
                    await core.fs.file_create("/foo.txt")
                    bootstrapped = True
                if need_boostrap_sync:
                    need_boostrap_sync = False
                    await core.fs.sync("/")

                sock = core_sock_factory(core, nursery=core.nursery)
                while True:
                    afunc = next(to_run_rules, None)
                    if not afunc:
                        done = True
                        break
                    await afunc(sock, file_oracle)

        except RestartCore:
            pass

        except ResetCore:
            alice.local_db._data.clear()  # TODO: improve this
            need_boostrap_sync = True

def rule_selector():
    {body}
"""
    )
    class CoreOnlineRwAndSync(TrioDriverRuleBasedStateMachine):
        count = 0

        async def trio_runner(self, task_status):
            self.sys_cmd = lambda x: self.communicator.send(("sys", x))
            self.core_cmd = lambda x: self.communicator.send(("core", x))
            self.file_oracle = FileOracle(base_version=1)

            device = device_factory()
            backend = await backend_factory(devices=[device])
            server = server_factory(backend.handle_client)

            async def run_core(on_ready):
                async with core_factory_cm(
                    devices=[device], config={"backend_addr": server.addr, "block_size": BLOCK_SIZE}
                ) as core:
                    await core.login(device)
                    sock = core_sock_factory(core, nursery=core.nursery)

                    await on_ready(core)
                    while True:
                        target, msg = await self.communicator.trio_recv()
                        if target == "core":
                            await sock.send(msg)
                            rep = await sock.recv()
                            await self.communicator.trio_respond(rep)
                        elif msg == "restart_core!":
                            raise RestartCore()

                        elif msg == "reset_core!":
                            raise RestartCore(reset_local_storage=True)

            async def bootstrap_core(core):
                await core.fs.file_create("/foo.txt")
                await core.fs.sync("/")

                task_status.started()

            async def reset_core_done(core):
                # Core won't try to fetch the user manifest from backend when
                # starting (given a modified version can be present on disk,
                # or we could be offline).
                # If we reset local storage however, we want to force the core
                # to load the data from the backend.
                await core.fs.sync("/")
                await self.communicator.trio_respond(True)

            async def restart_core_done(core):
                await self.communicator.trio_respond(True)

            on_ready = bootstrap_core
            while True:
                try:
                    await run_core(on_ready)
                except RestartCore as exc:
                    if exc.reset_local_storage:
                        on_ready = reset_core_done
                        device.local_db._data.clear()  # TODO: improve this
                    else:
                        on_ready = restart_core_done

        @rule(
            size=st.integers(min_value=0, max_value=PLAYGROUND_SIZE),
            offset=st.integers(min_value=0, max_value=PLAYGROUND_SIZE),
        )
        @reproduce_rule(
            """
async def afunc(sock, file_oracle):
    await sock.send({{"cmd": "file_read", "path": "/foo.txt", "offset": {offset}, "size": {size}}})
    rep = await sock.recv()
    assert rep["status"] == "ok"
    expected_content = file_oracle.read({size}, {offset})
    assert from_jsonb64(rep["content"]) == expected_content
yield afunc
"""
        )
        def read(self, size, offset):
            rep = self.core_cmd(
                {"cmd": "file_read", "path": "/foo.txt", "offset": offset, "size": size}
            )
            note(rep)
            assert rep["status"] == "ok"
            expected_content = self.file_oracle.read(size, offset)
            assert from_jsonb64(rep["content"]) == expected_content

        @rule()
        @reproduce_rule(
            """
async def afunc(sock, file_oracle):
    await sock.send({{"cmd": "synchronize", "path": "/"}})
    rep = await sock.recv()
    assert rep["status"] == "ok"
    file_oracle.sync()
yield afunc
"""
        )
        def sync(self):
            rep = self.core_cmd({"cmd": "synchronize", "path": "/"})
            note(rep)
            assert rep["status"] == "ok"
            self.file_oracle.sync()

        @rule(
            offset=st.integers(min_value=0, max_value=PLAYGROUND_SIZE),
            content=st.binary(max_size=PLAYGROUND_SIZE),
        )
        @reproduce_rule(
            """
async def afunc(sock, file_oracle):
    b64content = to_jsonb64({content})
    await sock.send({{"cmd": "file_write", "path": "/foo.txt", "offset": {offset}, "content": b64content}})
    rep = await sock.recv()
    assert rep["status"] == "ok"
    file_oracle.write({offset}, {content})
yield afunc
"""
        )
        def write(self, offset, content):
            b64content = to_jsonb64(content)
            rep = self.core_cmd(
                {"cmd": "file_write", "path": "/foo.txt", "offset": offset, "content": b64content}
            )
            note(rep)
            assert rep["status"] == "ok"
            self.file_oracle.write(offset, content)

        @rule(length=st.integers(min_value=0, max_value=PLAYGROUND_SIZE))
        @reproduce_rule(
            """
async def afunc(sock, file_oracle):
    await sock.send({{"cmd": "file_truncate", "path": "/foo.txt", "length": {length}}})
    rep = await sock.recv()
    assert rep["status"] == "ok"
    file_oracle.truncate({length})
yield afunc
"""
        )
        def truncate(self, length):
            rep = self.core_cmd({"cmd": "file_truncate", "path": "/foo.txt", "length": length})
            note(rep)
            assert rep["status"] == "ok"
            self.file_oracle.truncate(length)

        @rule()
        @reproduce_rule(
            """
async def afunc(sock, file_oracle):
    await sock.send({{"cmd": "stat", "path": "/foo.txt"}})
    rep = await sock.recv()
    assert rep["status"] == "ok"
    assert rep["base_version"] == file_oracle.base_version
    assert not rep["is_placeholder"]
    assert rep["need_sync"] == file_oracle.need_sync
yield afunc
"""
        )
        def stat(self):
            rep = self.core_cmd({"cmd": "stat", "path": "/foo.txt"})
            note(rep)
            assert rep["status"] == "ok"
            assert rep["base_version"] == self.file_oracle.base_version
            assert not rep["is_placeholder"]
            assert rep["need_sync"] == self.file_oracle.need_sync

        @rule()
        @reproduce_rule(
            """
async def afunc(sock, file_oracle):
    file_oracle.restart_core()
    raise RestartCore()
yield afunc
"""
        )
        def restart_core(self):
            rep = self.sys_cmd("restart_core!")
            assert rep is True
            self.file_oracle.restart_core()

        @rule()
        @reproduce_rule(
            """
async def afunc(sock, file_oracle):
    file_oracle.reset_core()
    raise ResetCore()
yield afunc
"""
        )
        def reset_core(self):
            rep = self.sys_cmd("reset_core!")
            assert rep is True
            self.file_oracle.reset_core()

    await CoreOnlineRwAndSync.run_test()
