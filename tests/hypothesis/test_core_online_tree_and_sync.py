import os
import pytest
from hypothesis import strategies as st, note

# from hypothesis.stateful import Bundle, rule
from hypothesis.stateful import Bundle
from copy import deepcopy

from tests.common import connect_core, core_factory, backend_factory, run_app
from tests.hypothesis.common import OracleFS, rule, normalize_path, rule_once


class OracleFSWithSync:
    def __init__(self):
        self.core_fs = OracleFS()
        self.synced_fs = OracleFS()

    def create_file(self, path):
        res = self.core_fs.create_file(path)
        return res

    def create_folder(self, path):
        res = self.core_fs.create_folder(path)
        return res

    def delete(self, path):
        res = self.core_fs.delete(path)
        return res

    def move(self, src, dst):
        res = self.core_fs.move(src, dst)
        return res

    def flush(self, path):
        return self.core_fs.flush(path)

    def sync(self, path):
        res = self.core_fs.sync(path)
        if res == "ok":
            self._sync_oracles(path)
        return res

    def reset_core(self):
        self.core_fs = deepcopy(self.synced_fs)

    def _sync_oracles(self, path):
        path = normalize_path(path)
        if path != "/":
            raise NotImplementedError("Oracle can only sync root yet :'(")

        self.synced_fs = deepcopy(self.core_fs)


# TODO: allow arbitrary path sync
# if path == '/':
#     self.synced_fs = deepcopy(self.core_fs)
# else:
#     entry = self.core_fs.get_path(path)
#     *parents, entry_name = path.strip('/').split('/')
#     # Make sure folder path exists in sync
#     folder_path = '/'
#     for parent in parents:
#         cur_path = _normalize_path('%s/%s' % (folder_path, parent))
#         if cur_path in self._just_deleted:
#             self.synced_fs.delete(cur_path)
#         self.synced_fs.create_folder(folder_path, parent)
#         folder_path += '%s/' % parent
#     sync_parent_entry = self.synced_fs.get_path(folder_path)
#     sync_parent_entry[entry_name] = deepcopy(entry)


@pytest.mark.slow
@pytest.mark.trio
async def test_online_core_tree_and_sync(
    TrioDriverRuleBasedStateMachine,
    mocked_local_storage_connection,
    tcp_stream_spy,
    backend_addr,
    tmpdir,
    alice,
    monitor,
):
    class RestartCore(Exception):
        def __init__(self, reset_local_storage=False):
            self.reset_local_storage = reset_local_storage

    st_entry_name = st.text(min_size=1).filter(lambda x: "/" not in x)

    class CoreOnlineRWFile(TrioDriverRuleBasedStateMachine):
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
        def create_file(self, parent, name):
            path = os.path.join(parent, name)
            rep = self.core_cmd({"cmd": "file_create", "path": path})
            note(rep)
            expected_status = self.oracle_fs.create_file(path)
            assert rep["status"] == expected_status
            return path

        @rule(target=Folders, parent=Folders, name=st_entry_name)
        def create_folder(self, parent, name):
            path = os.path.join(parent, name)
            rep = self.core_cmd({"cmd": "folder_create", "path": path})
            note(rep)
            expected_status = self.oracle_fs.create_folder(path)
            assert rep["status"] == expected_status
            return path

        @rule(path=Files)
        def delete_file(self, path):
            rep = self.core_cmd({"cmd": "delete", "path": path})
            note(rep)
            expected_status = self.oracle_fs.delete(path)
            assert rep["status"] == expected_status

        @rule(path=Folders)
        def delete_folder(self, path):
            rep = self.core_cmd({"cmd": "delete", "path": path})
            note(rep)
            expected_status = self.oracle_fs.delete(path)
            assert rep["status"] == expected_status

        @rule(target=Files, src=Files, dst_parent=Folders, dst_name=st_entry_name)
        def move_file(self, src, dst_parent, dst_name):
            dst = os.path.join(dst_parent, dst_name)
            rep = self.core_cmd({"cmd": "move", "src": src, "dst": dst})
            note(rep)
            expected_status = self.oracle_fs.move(src, dst)
            assert rep["status"] == expected_status
            return dst

        @rule(target=Folders, src=Folders, dst_parent=Folders, dst_name=st_entry_name)
        def move_folder(self, src, dst_parent, dst_name):
            dst = os.path.join(dst_parent, dst_name)
            rep = self.core_cmd({"cmd": "move", "src": src, "dst": dst})
            note(rep)
            expected_status = self.oracle_fs.move(src, dst)
            assert rep["status"] == expected_status
            return dst

        @rule()
        def sync_root(self):
            rep = self.core_cmd({"cmd": "synchronize", "path": "/"})
            note(rep)
            expected_status = self.oracle_fs.sync("/")
            assert rep["status"] == expected_status

        # @rule(path=Files)
        # def sync_file(self, path):
        #     rep = self.core_cmd({'cmd': 'synchronize', 'path': path})
        #     note(rep)
        #     expected_status = self.oracle_fs.sync(*path.rsplit('/', 1))
        #     assert rep['status'] == expected_status

        # @rule(path=Folders)
        # def sync_folder(self, path):
        #     rep = self.core_cmd({'cmd': 'synchronize', 'path': path})
        #     note(rep)
        #     if path == '/':
        #         expected_status = 'ok'
        #     else:
        #         expected_status = self.oracle_fs.sync(*path.rsplit('/', 1))
        #     assert rep['status'] == expected_status

        @rule()
        def restart_core(self):
            rep = self.sys_cmd("restart_core!")
            assert rep is True

        @rule()
        def reset_core(self):
            rep = self.sys_cmd("reset_core!")
            assert rep is True
            self.oracle_fs.reset_core()

    await CoreOnlineRWFile.run_test()
