import os
import pytest
from hypothesis import strategies as st, note

from hypothesis.stateful import Bundle
from copy import deepcopy

from tests.common import connect_core, core_factory, backend_factory, run_app
from tests.hypothesis.common import (
    OracleFS,
    OracleFSFolder,
    rule,
    normalize_path,
    rule_once,
    failure_reproducer,
    reproduce_rule,
)


class OracleFSWithSync:
    def __init__(self):
        self.core_fs = OracleFS()
        self.synced_fs = OracleFS()

    def create_file(self, path):
        return self.core_fs.create_file(path)

    def create_folder(self, path):
        return self.core_fs.create_folder(path)

    def delete(self, path):
        return self.core_fs.delete(path)

    def move(self, src, dst):
        return self.core_fs.move(src, dst)

    def flush(self, path):
        return self.core_fs.flush(path)

    def sync(self, path):
        res = self.core_fs.sync(path)
        if res == "ok":
            self._sync_oracles(path)
        return res

    def stat(self, path):
        return self.core_fs.stat(path)

    def reset_core(self):
        self.core_fs = deepcopy(self.synced_fs)
        # Mimic what is done in CoreOnlineRWFile.reset_core_done
        self.core_fs.sync("/")

    def _sync_oracles(self, path):
        path = normalize_path(path)
        if path == "/":
            self.synced_fs = deepcopy(self.core_fs)
        else:
            *parent_hops, name = path.split("/")
            parent_dir = self.synced_fs.root
            for hop in parent_hops:
                if hop and hop not in parent_dir:
                    parent_dir[hop] = OracleFSFolder(False, False, 1)
                    parent_dir = parent_dir[hop]
            entry = self.core_fs.get_path(path)
            parent_dir[name] = deepcopy(entry)


@pytest.mark.slow
@pytest.mark.trio
async def test_online_core_tree_and_sync(
    TrioDriverRuleBasedStateMachine,
    mocked_local_storage_connection,
    tcp_stream_spy,
    backend_addr,
    tmpdir,
    alice,
):
    class RestartCore(Exception):
        def __init__(self, reset_local_storage=False):
            self.reset_local_storage = reset_local_storage

    st_entry_name = st.text(min_size=1).filter(lambda x: "/" not in x)

    @failure_reproducer(
        """
import pytest
import os

from tests.common import connect_core, core_factory
from tests.hypothesis.test_core_online_tree_and_sync import OracleFSWithSync

class RestartCore(Exception):
    pass

class ResetCore(Exception):
    pass

@pytest.mark.trio
async def test_reproduce(tmpdir, running_backend, backend_addr, alice, mocked_local_storage_connection):
    config = {{
        "base_settings_path": tmpdir.strpath,
        "backend_addr": backend_addr,
    }}
    oracle_fs = OracleFSWithSync()
    to_run_rules = rule_selector()
    done = False

    while not done:
        try:
            async with core_factory(**config) as core:
                await core.login(alice)

                async with connect_core(core) as sock:
                    while True:
                        afunc = next(to_run_rules, None)
                        if not afunc:
                            done = True
                            break
                        await afunc(sock, oracle_fs)

        except RestartCore:
            pass

        except ResetCore:
            mocked_local_storage_connection.reset()

def rule_selector():
    {body}
"""
    )
    class CoreOnlineTreeAndSync(TrioDriverRuleBasedStateMachine):
        Files = Bundle("file")
        Folders = Bundle("folder")
        count = 0

        async def trio_runner(self, task_status):
            mocked_local_storage_connection.reset()

            type(self).count += 1
            backend_config = {"blockstore_postgresql": True}
            core_config = {
                "base_settings_path": tmpdir.mkdir("try-%s" % self.count).strpath,
                "backend_addr": backend_addr,
            }

            self.sys_cmd = lambda x: self.communicator.send(("sys", x))
            self.core_cmd = lambda x: self.communicator.send(("core", x))
            self.oracle_fs = OracleFSWithSync()

            async def run_core(on_ready):
                async with core_factory(**core_config) as core:
                    self.core = core

                    await core.login(alice)
                    async with connect_core(core) as sock:

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

            async with backend_factory(**backend_config) as backend:

                await backend.user.create(
                    author="<backend-fixture>",
                    user_id=alice.user_id,
                    broadcast_key=alice.user_pubkey.encode(),
                    devices=[(alice.device_name, alice.device_verifykey.encode())],
                )

                async with run_app(backend) as backend_connection_factory:

                    tcp_stream_spy.install_hook(backend_addr, backend_connection_factory)
                    try:

                        on_ready = bootstrap_core
                        while True:
                            try:
                                await run_core(on_ready)
                            except RestartCore as exc:
                                if exc.reset_local_storage:
                                    on_ready = reset_core_done
                                    mocked_local_storage_connection.reset()
                                else:
                                    on_ready = restart_core_done

                    finally:
                        tcp_stream_spy.install_hook(backend_addr, None)

        @rule_once(target=Folders)
        def get_root(self):
            return "/"

        @rule(target=Files, parent=Folders, name=st_entry_name)
        @reproduce_rule(
            """
async def afunc(sock, oracle_fs):
    path = os.path.join({parent}, {name})
    await sock.send({{"cmd": "file_create", "path": path}})
    rep = await sock.recv()
    expected_status = oracle_fs.create_folder(path)
    assert rep["status"] == expected_status
yield afunc
"""
        )
        def create_file(self, parent, name):
            path = os.path.join(parent, name)
            rep = self.core_cmd({"cmd": "file_create", "path": path})
            note(rep)
            expected_status = self.oracle_fs.create_file(path)
            assert rep["status"] == expected_status
            return path

        @rule(target=Folders, parent=Folders, name=st_entry_name)
        @reproduce_rule(
            """
async def afunc(sock, oracle_fs):
    path = os.path.join({parent}, {name})
    await sock.send({{"cmd": "folder_create", "path": path}})
    rep = await sock.recv()
    expected_status = oracle_fs.create_folder(path)
    assert rep["status"] == expected_status
yield afunc
"""
        )
        def create_folder(self, parent, name):
            path = os.path.join(parent, name)
            rep = self.core_cmd({"cmd": "folder_create", "path": path})
            note(rep)
            expected_status = self.oracle_fs.create_folder(path)
            assert rep["status"] == expected_status
            return path

        @rule(path=st.one_of(Files, Folders))
        @reproduce_rule(
            """
async def afunc(sock, oracle_fs):
    await sock.send({{"cmd": "stat", "path": {path}}})
    rep = await sock.recv()
    expected = oracle_fs.stat({path})
    assert rep["status"] == expected["status"]
    if expected["status"] == "ok":
        assert rep["base_version"] == expected["base_version"]
        assert rep["is_placeholder"] == expected["is_placeholder"]
        assert rep["need_flush"] == expected["need_flush"]
        assert rep["need_sync"] == expected["need_sync"]
yield afunc
"""
        )
        def stat(self, path):
            rep = self.core_cmd({"cmd": "stat", "path": path})
            note(rep)
            expected = self.oracle_fs.stat(path)
            assert rep["status"] == expected["status"]
            if expected["status"] == "ok":
                assert rep["base_version"] == expected["base_version"]
                assert rep["is_placeholder"] == expected["is_placeholder"]
                assert rep["need_flush"] == expected["need_flush"]
                assert rep["need_sync"] == expected["need_sync"]

        @rule(path=st.one_of(Files, Folders))
        @reproduce_rule(
            """
async def afunc(sock, oracle_fs):
    await sock.send({{"cmd": "delete", "path": {path}}})
    rep = await sock.recv()
    expected_status = oracle_fs.delete(path)
    assert rep["status"] == expected_status
yield afunc
"""
        )
        def delete(self, path):
            rep = self.core_cmd({"cmd": "delete", "path": path})
            note(rep)
            expected_status = self.oracle_fs.delete(path)
            assert rep["status"] == expected_status

        @rule(target=Files, src=Files, dst_parent=Folders, dst_name=st_entry_name)
        @reproduce_rule(
            """
async def afunc(sock, oracle_fs):
    dst = os.path.join({dst_parent}, {dst_name})
    await sock.send({{"cmd": "move", "src": {src}, "dst": dst}})
    rep = await sock.recv()
    expected_status = oracle_fs.move(src, dst)
    assert rep["status"] == expected_status
yield afunc
"""
        )
        def move_file(self, src, dst_parent, dst_name):
            dst = os.path.join(dst_parent, dst_name)
            rep = self.core_cmd({"cmd": "move", "src": src, "dst": dst})
            note(rep)
            expected_status = self.oracle_fs.move(src, dst)
            assert rep["status"] == expected_status
            return dst

        @rule(target=Folders, src=Folders, dst_parent=Folders, dst_name=st_entry_name)
        @reproduce_rule(
            """
async def afunc(sock, oracle_fs):
    dst = os.path.join({dst_parent}, {dst_name})
    await sock.send({{"cmd": "move", "src": {src}, "dst": dst}})
    rep = await sock.recv()
    expected_status = oracle_fs.move(src, dst)
    assert rep["status"] == expected_status
yield afunc
"""
        )
        def move_folder(self, src, dst_parent, dst_name):
            dst = os.path.join(dst_parent, dst_name)
            rep = self.core_cmd({"cmd": "move", "src": src, "dst": dst})
            note(rep)
            expected_status = self.oracle_fs.move(src, dst)
            assert rep["status"] == expected_status
            return dst

        @rule(path=st.one_of(Folders, Files))
        @reproduce_rule(
            """
async def afunc(sock, oracle_fs):
    await sock.send({{"cmd": "synchronize", "path": {path}}})
    rep = await sock.recv()
    expected_status = oracle_fs.sync({path})
    assert rep["status"] == expected_status
yield afunc
"""
        )
        def sync(self, path):
            rep = self.core_cmd({"cmd": "synchronize", "path": path})
            note(rep)
            expected_status = self.oracle_fs.sync(path)
            assert rep["status"] == expected_status

        @rule()
        @reproduce_rule(
            """
async def afunc(sock, oracle_fs):
    raise RestartCore()
yield afunc
"""
        )
        def restart_core(self):
            rep = self.sys_cmd("restart_core!")
            assert rep is True

        @rule()
        @reproduce_rule(
            """
async def afunc(sock, oracle_fs):
    oracle_fs.reset_core()
    raise ResetCore()
yield afunc
"""
        )
        def reset_core(self):
            rep = self.sys_cmd("reset_core!")
            assert rep is True
            self.oracle_fs.reset_core()

    await CoreOnlineTreeAndSync.run_test()
