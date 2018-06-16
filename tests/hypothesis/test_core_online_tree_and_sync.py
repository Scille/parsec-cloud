import os
import pytest
from string import ascii_lowercase
from hypothesis import strategies as st, note
from hypothesis.stateful import Bundle

from tests.hypothesis.common import rule, initialize, failure_reproducer, reproduce_rule


# The point is not to find breaking filenames here, so keep it simple
st_entry_name = st.text(alphabet=ascii_lowercase, min_size=1, max_size=3)


@pytest.fixture
def oracle_fs_with_sync_factory(oracle_fs_factory):
    class OracleFSWithSync:
        def __init__(self):
            self.core_fs = oracle_fs_factory()
            self.core_fs.sync("/")
            self.synced_fs = oracle_fs_factory()
            self.synced_fs.sync("/")

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
            synced_items = []

            def sync_cb(path, stat):
                synced_items.append((path, stat["base_version"], stat["type"]))

            res = self.core_fs.sync(path, sync_cb=sync_cb)
            if res == "ok":
                new_synced = self.core_fs.copy()

                def _recursive_keep_synced(path):
                    stat = new_synced.entries_stats[path]
                    if stat["type"] == "folder":
                        for child in path.iterdir():
                            _recursive_keep_synced(child)
                    stat["need_sync"] = False
                    if stat["is_placeholder"]:
                        del new_synced.entries_stats[path]
                        if stat["type"] == "file":
                            path.unlink()
                        else:
                            path.rmdir()

                _recursive_keep_synced(new_synced.root)
                self.synced_fs = new_synced
            return res

        def stat(self, path):
            return self.core_fs.stat(path)

        def reset_core(self):
            self.core_fs = self.synced_fs.copy()

    def _oracle_fs_with_sync_factory():
        return OracleFSWithSync()

    return _oracle_fs_with_sync_factory


@pytest.mark.slow
@pytest.mark.trio
async def test_online_core_tree_and_sync(
    TrioDriverRuleBasedStateMachine,
    server_factory,
    backend_factory_cm,
    core_factory_cm,
    core_sock_factory,
    device_factory,
    oracle_fs_with_sync_factory,
):
    class RestartCore(Exception):
        def __init__(self, reset_local_storage=False):
            self.reset_local_storage = reset_local_storage

    @failure_reproducer(
        """
import pytest
import os

from tests.hypothesis.test_core_online_tree_and_sync import *
from tests.hypothesis.conftest import *

class RestartCore(Exception):
    pass

class ResetCore(Exception):
    pass

@pytest.mark.trio
async def test_reproduce(
    running_backend, alice, core_factory_cm, core_sock_factory, oracle_fs_with_sync_factory
):
    oracle_fs = oracle_fs_with_sync_factory()
    to_run_rules = rule_selector()
    need_reset_sync = False
    done = False

    while not done:
        try:
            async with core_factory_cm(config={{"auto_sync": False}}) as core:
                await core.login(alice)
                if need_reset_sync:
                    need_reset_sync = False
                    await core.fs.sync('/')

                sock = core_sock_factory(core)
                while True:
                    afunc = next(to_run_rules, None)
                    if not afunc:
                        done = True
                        break
                    await afunc(sock, oracle_fs)

        except RestartCore:
            pass

        except ResetCore:
            need_reset_sync = True
            alice.local_db._data.clear()  # TODO: improve this

def rule_selector():
    {body}
"""
    )
    class CoreOnlineTreeAndSync(TrioDriverRuleBasedStateMachine):
        Files = Bundle("file")
        Folders = Bundle("folder")

        async def trio_runner(self, task_status):
            self.sys_cmd = lambda x: self.communicator.send(("sys", x))
            self.core_cmd = lambda x: self.communicator.send(("core", x))
            self.oracle_fs = oracle_fs_with_sync_factory()

            device = device_factory()

            async def run_core(on_ready):
                async with core_factory_cm(
                    devices=[device], config={"backend_addr": server.addr, "auto_sync": False}
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
                task_status.started()

            async def reset_core_done(core):
                # # Core won't try to fetch the user manifest from backend when
                # # starting (given a modified version can be present on disk,
                # # or we could be offline).
                # # If we reset local storage however, we want to force the core
                # # to load the data from the backend.
                # await core.fs.sync("/")
                await self.communicator.trio_respond(True)

            async def restart_core_done(core):
                await self.communicator.trio_respond(True)

            async with backend_factory_cm(devices=[device]) as backend:
                server = server_factory(backend.handle_client, nursery=backend.nursery)

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

        @initialize(target=Folders)
        def get_root(self):
            return "/"

        @rule(target=Files, parent=Folders, name=st_entry_name)
        @reproduce_rule(
            """
async def afunc(sock, oracle_fs):
    path = os.path.join({parent}, {name})
    await sock.send({{"cmd": "file_create", "path": path}})
    rep = await sock.recv()
    expected_status = oracle_fs.create_file(path)
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
        assert rep["type"] == expected["type"]
        assert rep["base_version"] == expected["base_version"]
        assert rep["is_placeholder"] == expected["is_placeholder"]
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
                assert rep["type"] == expected["type"]
                assert rep["base_version"] == expected["base_version"]
                assert rep["is_placeholder"] == expected["is_placeholder"]
                assert rep["need_sync"] == expected["need_sync"]

        @rule(path=st.one_of(Files, Folders))
        @reproduce_rule(
            """
async def afunc(sock, oracle_fs):
    await sock.send({{"cmd": "delete", "path": {path}}})
    rep = await sock.recv()
    expected_status = oracle_fs.delete({path})
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
    expected_status = oracle_fs.move({src}, dst)
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
    expected_status = oracle_fs.move({src}, dst)
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

        # TODO: really complex to implement...
        #         @rule(path=st.one_of(Folders, Files))
        #         @reproduce_rule(
        #             """
        # async def afunc(sock, oracle_fs):
        #     await sock.send({{"cmd": "synchronize", "path": {path}}})
        #     rep = await sock.recv()
        #     expected_status = oracle_fs.sync({path})
        #     assert rep["status"] == expected_status
        # yield afunc
        # """
        #         )
        #         def sync(self, path):
        #             rep = self.core_cmd({"cmd": "synchronize", "path": path})
        #             note(rep)
        #             expected_status = self.oracle_fs.sync(path)
        #             assert rep["status"] == expected_status

        @rule()
        @reproduce_rule(
            """
async def afunc(sock, oracle_fs):
    await sock.send({{"cmd": "synchronize", "path": '/'}})
    rep = await sock.recv()
    expected_status = oracle_fs.sync('/')
    assert rep["status"] == expected_status
yield afunc
"""
        )
        def sync_root(self):
            rep = self.core_cmd({"cmd": "synchronize", "path": "/"})
            note(rep)
            expected_status = self.oracle_fs.sync("/")
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
