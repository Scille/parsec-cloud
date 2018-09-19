import os
import pytest
from string import ascii_lowercase
from hypothesis import strategies as st, note
from hypothesis.stateful import Bundle
from hypothesis_trio.stateful import run_state_machine_as_test

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
def test_online_core_tree_and_sync(
    BaseCoreWithBackendStateMachine, oracle_fs_with_sync_factory, hypothesis_settings
):
    @failure_reproducer(
        """
import pytest
import os
from copy import deepcopy

from parsec.utils import to_jsonb64, from_jsonb64

from tests.hypothesis.test_core_online_tree_and_sync import FileOracle, BLOCK_SIZE

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
        core = await core_factory(config={{"auto_sync": False}})
        await core.login(alice)
        sock = core_sock_factory(core)
        if old_core:
            await core.fs.sync("/")
        return core, sock

    core, sock = await restart_core()

    {body}
"""
    )
    class CoreOnlineTreeAndSync(BaseCoreWithBackendStateMachine):
        Files = Bundle("file")
        Folders = Bundle("folder")

        @initialize(target=Folders)
        async def init_oracle_and_bundle(self):
            self.oracle_fs = oracle_fs_with_sync_factory()
            return "/"

        @rule(target=Files, parent=Folders, name=st_entry_name)
        @reproduce_rule(
            """
path = os.path.join({parent}, {name})
await sock.send({{"cmd": "file_create", "path": path}})
rep = await sock.recv()
expected_status = oracle_fs.create_file(path)
assert rep["status"] == expected_status
"""
        )
        async def create_file(self, parent, name):
            path = os.path.join(parent, name)
            rep = await self.core_cmd({"cmd": "file_create", "path": path})
            note(rep)
            expected_status = self.oracle_fs.create_file(path)
            assert rep["status"] == expected_status
            return path

        @rule(target=Folders, parent=Folders, name=st_entry_name)
        @reproduce_rule(
            """
path = os.path.join({parent}, {name})
await sock.send({{"cmd": "folder_create", "path": path}})
rep = await sock.recv()
expected_status = oracle_fs.create_folder(path)
assert rep["status"] == expected_status
"""
        )
        async def create_folder(self, parent, name):
            path = os.path.join(parent, name)
            rep = await self.core_cmd({"cmd": "folder_create", "path": path})
            note(rep)
            expected_status = self.oracle_fs.create_folder(path)
            assert rep["status"] == expected_status
            return path

        @rule(path=st.one_of(Files, Folders))
        @reproduce_rule(
            """
await sock.send({{"cmd": "stat", "path": {path}}})
rep = await sock.recv()
expected = oracle_fs.stat({path})
assert rep["status"] == expected["status"]
if expected["status"] == "ok":
    assert rep["type"] == expected["type"]
    assert rep["base_version"] == expected["base_version"]
    assert rep["is_placeholder"] == expected["is_placeholder"]
    assert rep["need_sync"] == expected["need_sync"]
"""
        )
        async def stat(self, path):
            rep = await self.core_cmd({"cmd": "stat", "path": path})
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
await sock.send({{"cmd": "delete", "path": {path}}})
rep = await sock.recv()
expected_status = oracle_fs.delete({path})
assert rep["status"] == expected_status
"""
        )
        async def delete(self, path):
            rep = await self.core_cmd({"cmd": "delete", "path": path})
            note(rep)
            expected_status = self.oracle_fs.delete(path)
            assert rep["status"] == expected_status

        @rule(target=Files, src=Files, dst_parent=Folders, dst_name=st_entry_name)
        @reproduce_rule(
            """
dst = os.path.join({dst_parent}, {dst_name})
await sock.send({{"cmd": "move", "src": {src}, "dst": dst}})
rep = await sock.recv()
expected_status = oracle_fs.move({src}, dst)
assert rep["status"] == expected_status
"""
        )
        async def move_file(self, src, dst_parent, dst_name):
            dst = os.path.join(dst_parent, dst_name)
            rep = await self.core_cmd({"cmd": "move", "src": src, "dst": dst})
            note(rep)
            expected_status = self.oracle_fs.move(src, dst)
            assert rep["status"] == expected_status
            return dst

        @rule(target=Folders, src=Folders, dst_parent=Folders, dst_name=st_entry_name)
        @reproduce_rule(
            """
dst = os.path.join({dst_parent}, {dst_name})
await sock.send({{"cmd": "move", "src": {src}, "dst": dst}})
rep = await sock.recv()
expected_status = oracle_fs.move({src}, dst)
assert rep["status"] == expected_status
"""
        )
        async def move_folder(self, src, dst_parent, dst_name):
            dst = os.path.join(dst_parent, dst_name)
            rep = await self.core_cmd({"cmd": "move", "src": src, "dst": dst})
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
        #             rep = await self.core_cmd({"cmd": "synchronize", "path": path})
        #             note(rep)
        #             expected_status = self.oracle_fs.sync(path)
        #             assert rep["status"] == expected_status

        @rule()
        @reproduce_rule(
            """
await sock.send({{"cmd": "synchronize", "path": '/'}})
rep = await sock.recv()
expected_status = oracle_fs.sync('/')
assert rep["status"] == expected_status
"""
        )
        async def sync_root(self):
            rep = await self.core_cmd({"cmd": "synchronize", "path": "/"})
            note(rep)
            expected_status = self.oracle_fs.sync("/")
            assert rep["status"] == expected_status

        @rule()
        @reproduce_rule("""core, sock = await restart_core(core)""")
        async def restart(self):
            await self.restart_core()

        @rule()
        @reproduce_rule("""core, sock = await restart_core(core, reset_local_db=True)""")
        async def reset(self):
            await self.restart_core(reset_local_db=True)
            self.oracle_fs.reset_core()

    run_state_machine_as_test(CoreOnlineTreeAndSync, settings=hypothesis_settings)
